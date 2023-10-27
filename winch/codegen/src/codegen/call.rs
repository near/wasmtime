//! Function call emission.  For more details around the ABI and
//! calling convention, see [ABI].
//!
//! This module exposes a single function [`FnCall::emit`], which is responsible
//! of orchestrating the emission of calls. In general such orchestration
//! takes place in 4 steps:
//!
//! 1. [`Callee`] resolution.
//! 2. Mapping of the [`Callee`] to the [`CalleeKind`].
//! 3. Calculation of the stack space consumed by the call.
//! 4. Emission.
//!
//! The stack space consumed by the function call; that is,
//! the sum of:
//!
//! 1. The amount of stack space created by saving any live
//!    registers at the callsite.
//! 2. The amount of space used by any memory entries in the value
//!    stack present at the callsite, that will be used as
//!    arguments for the function call. Any memory values in the
//!    value stack that are needed as part of the function
//!    arguments, will be consumed by the function call (either by
//!    assigning those values to a register or by storing those
//!    values to a memory location if the callee argument is on
//!    the stack), so we track that stack space to reclaim it once
//!    the function call has ended. This could also be done in
//!    when assigning arguments everytime a memory entry needs to be assigned
//!    to a particular location, but doing so, will incur in more
//!    instructions (e.g. a pop per argument that needs to be
//!    assigned); it's more efficient to track the space needed by
//!    those memory values and reclaim it at once.
//!
//! The machine stack throghout the function call is as follows:
//! ┌──────────────────────────────────────────────────┐
//! │                                                  │
//! │                  1                               │
//! │  Stack space created by any previous spills      │
//! │  from the value stack; and which memory values   │
//! │  are used as function arguments.                 │
//! │                                                  │
//! ├──────────────────────────────────────────────────┤ ---> The Wasm value stack at this point in time would look like:
//! │                                                  │      [ Reg | Reg | Mem(offset) | Mem(offset) ]
//! │                   2                              │
//! │   Stack space created by saving                  │
//! │   any live registers at the callsite.            │
//! │                                                  │
//! │                                                  │
//! ├─────────────────────────────────────────────────┬┤ ---> The Wasm value stack at this point in time would look like:
//! │                                                  │      [ Mem(offset) | Mem(offset) | Mem(offset) | Mem(offset) ]
//! │                                                  │      Assuming that the callee takes 4 arguments, we calculate
//! │                                                  │      2 spilled registers + 2 memory values; all of which will be used
//! │   Stack space allocated for                      │      as arguments to the call via `assign_args`, thus the memory they represent is
//! │   the callee function arguments in the stack;    │      is considered to be consumed by the call.
//! │   represented by `arg_stack_space`               │
//! │                                                  │
//! │                                                  │
//! │                                                  │
//! └──────────────────────────────────────────────────┘ ------> Stack pointer when emitting the call

use crate::{
    abi::{ABIArg, ABISig, ABI},
    codegen::{
        ptr_type_from_ptr_size, BuiltinFunction, BuiltinType, Callee, CalleeInfo, CodeGenContext,
        TypedReg,
    },
    masm::{CalleeKind, MacroAssembler, OperandSize},
    reg::Reg,
    CallingConvention,
};
use smallvec::SmallVec;
use std::borrow::Cow;
use wasmtime_environ::{PtrSize, VMOffsets, WasmType};

/// All the information needed to emit a function call.
#[derive(Copy, Clone)]
pub(crate) struct FnCall {}

impl FnCall {
    /// Orchestrates the emission of a function call:
    /// 1. Resolves the [`Callee`] through the given callback.
    /// 2. Maps the resolved [`Callee`] to the [`CalleeKind`].
    /// 3. Saves any live registers and calculates the stack space consumed
    ///    by the function call.
    /// 4. Emits the call.
    pub fn emit<M: MacroAssembler, P: PtrSize, R>(
        masm: &mut M,
        context: &mut CodeGenContext,
        mut resolve: R,
    ) where
        R: FnMut(&mut CodeGenContext) -> Callee,
    {
        let callee = resolve(context);
        let ptr_type = ptr_type_from_ptr_size(context.vmoffsets.ptr.size());
        let sig = Self::get_sig::<M>(&callee, ptr_type);
        let sig = sig.as_ref();

        let arg_stack_space = sig.stack_bytes;
        let kind = Self::map(&context.vmoffsets, &callee, sig, context, masm);
        let call_stack_space = Self::save(context, masm, &sig);

        let reserved_stack = masm.call(arg_stack_space, |masm| {
            let scratch = <M::ABI as ABI>::scratch_reg();
            Self::assign(sig, context, masm, scratch);
            kind
        });

        match kind {
            CalleeKind::Indirect(r) => context.free_reg(r),
            _ => {}
        }
        Self::cleanup(
            sig,
            call_stack_space.checked_add(reserved_stack).unwrap(),
            masm,
            context,
        );
    }

    /// Derive the [`ABISig`] for a particulare [`Callee].
    fn get_sig<M: MacroAssembler>(callee: &Callee, ptr_type: WasmType) -> Cow<'_, ABISig> {
        match callee {
            Callee::Builtin(info) => Cow::Borrowed(info.sig()),
            Callee::Import(info) => {
                let mut params: SmallVec<[WasmType; 6]> =
                    SmallVec::with_capacity(info.ty.params().len() + 2);
                params.extend_from_slice(&[ptr_type, ptr_type]);
                params.extend_from_slice(info.ty.params());
                Cow::Owned(<M::ABI as ABI>::sig_from(
                    &params,
                    info.ty.returns(),
                    &CallingConvention::Default,
                ))
            }
            Callee::Local(info) => {
                Cow::Owned(<M::ABI as ABI>::sig(&info.ty, &CallingConvention::Default))
            }
            Callee::FuncRef(ty) => {
                Cow::Owned(<M::ABI as ABI>::sig(&ty, &CallingConvention::Default))
            }
        }
    }

    /// Maps the given [`Callee`] to a [`CalleeKind`].
    fn map<P: PtrSize, M: MacroAssembler>(
        vmoffsets: &VMOffsets<P>,
        callee: &Callee,
        sig: &ABISig,
        context: &mut CodeGenContext,
        masm: &mut M,
    ) -> CalleeKind {
        match callee {
            Callee::Builtin(b) => Self::load_builtin(b, context, masm),
            Callee::FuncRef(_) => Self::load_funcref(sig, vmoffsets.ptr.size(), context, masm),
            Callee::Local(i) => Self::map_local(i),
            Callee::Import(i) => Self::load_import(i, sig, context, masm, vmoffsets),
        }
    }

    /// Load a built-in function to the next available register.
    fn load_builtin<M: MacroAssembler>(
        builtin: &BuiltinFunction,
        context: &mut CodeGenContext,
        masm: &mut M,
    ) -> CalleeKind {
        match builtin.ty() {
            BuiltinType::Dynamic { index, base } => {
                let sig = builtin.sig();
                let callee = context.without::<Reg, _, _>(&sig.regs, masm, |cx, masm| {
                    let scratch = <M::ABI as ABI>::scratch_reg();
                    let builtins_base = masm.address_at_vmctx(base);
                    masm.load_ptr(builtins_base, scratch);
                    let addr = masm.address_at_reg(scratch, index);
                    let callee = cx.any_gpr(masm);
                    masm.load_ptr(addr, callee);
                    callee
                });
                CalleeKind::indirect(callee)
            }
            BuiltinType::Known(c) => CalleeKind::known(c),
        }
    }

    /// Map a local function to a [`CalleeKind`].
    fn map_local(info: &CalleeInfo) -> CalleeKind {
        CalleeKind::direct(info.index.as_u32())
    }

    /// Loads a function import to the next available register.
    fn load_import<M: MacroAssembler, P: PtrSize>(
        info: &CalleeInfo,
        sig: &ABISig,
        context: &mut CodeGenContext,
        masm: &mut M,
        vmoffsets: &VMOffsets<P>,
    ) -> CalleeKind {
        let ptr_type = ptr_type_from_ptr_size(vmoffsets.ptr.size());
        let caller_vmctx = <M::ABI as ABI>::vmctx_reg();
        let (callee, callee_vmctx) =
            context.without::<(Reg, Reg), M, _>(&sig.regs, masm, |context, masm| {
                (context.any_gpr(masm), context.any_gpr(masm))
            });
        let callee_vmctx_offset = vmoffsets.vmctx_vmfunction_import_vmctx(info.index);
        let callee_vmctx_addr = masm.address_at_vmctx(callee_vmctx_offset);
        masm.load_ptr(callee_vmctx_addr, callee_vmctx);

        let callee_body_offset = vmoffsets.vmctx_vmfunction_import_wasm_call(info.index);
        let callee_addr = masm.address_at_vmctx(callee_body_offset);
        masm.load_ptr(callee_addr, callee);

        // Put the callee / caller vmctx at the start of the
        // range of the stack so that they are used as first
        // and second arguments.
        let stack = &mut context.stack;
        let location = stack.len() - (sig.params.len() - 2);
        let values = [
            TypedReg::new(ptr_type, callee_vmctx).into(),
            TypedReg::new(ptr_type, caller_vmctx).into(),
        ]
        .into_iter();
        context.stack.insert_many(location, values);

        CalleeKind::indirect(callee)
    }

    /// Loads a function reference to the next available register.
    fn load_funcref<M: MacroAssembler>(
        sig: &ABISig,
        ptr: impl PtrSize,
        context: &mut CodeGenContext,
        masm: &mut M,
    ) -> CalleeKind {
        // Pop the funcref pointer to a register and allocate a register to hold the
        // address of the funcref. Since the callee is not addressed from a global non
        // allocatable register (like the vmctx in the case of an import), we load the
        // funcref to a register ensuring that it doesn't get assigned to a non-arg
        // register.
        let (funcref_ptr, funcref) = context.without::<_, M, _>(&sig.regs, masm, |cx, masm| {
            (cx.pop_to_reg(masm, None).into(), cx.any_gpr(masm))
        });

        masm.load_ptr(
            masm.address_at_reg(funcref_ptr, ptr.vm_func_ref_wasm_call().into()),
            funcref,
        );
        context.free_reg(funcref_ptr);
        CalleeKind::indirect(funcref)
    }

    /// Assign arguments for the function call.
    fn assign<M: MacroAssembler>(
        sig: &ABISig,
        context: &mut CodeGenContext,
        masm: &mut M,
        scratch: Reg,
    ) {
        let arg_count = sig.params.len();
        let stack = &context.stack;
        let mut stack_values = stack.peekn(arg_count);
        for arg in &sig.params {
            let val = stack_values
                .next()
                .unwrap_or_else(|| panic!("expected stack value for function argument"));
            match &arg {
                &ABIArg::Reg { ty: _, reg } => {
                    context.move_val_to_reg(&val, *reg, masm);
                }
                &ABIArg::Stack { ty, offset } => {
                    let addr = masm.address_at_sp(*offset);
                    let size: OperandSize = (*ty).into();
                    context.move_val_to_reg(val, scratch, masm);
                    masm.store(scratch.into(), addr, size);
                }
            }
        }
    }

    /// Save any live registers prior to emitting the call.
    //
    // Here we perform a "spill" of the register entries
    // in the Wasm value stack, we also count any memory
    // values that will be used used as part of the callee
    // arguments.  Saving the live registers is done by
    // emitting push operations for every `Reg` entry in
    // the Wasm value stack. We do this to be compliant
    // with Winch's internal ABI, in which all registers
    // are treated as caller-saved. For more details, see
    // [ABI].
    //
    // The next few lines, partition the value stack into
    // two sections:
    // +------------------+--+--- (Stack top)
    // |                  |  |
    // |                  |  | 1. The top `n` elements, which are used for
    // |                  |  |    function arguments; for which we save any
    // |                  |  |    live registers, keeping track of the amount of registers
    // +------------------+  |    saved plus the amount of memory values consumed by the function call;
    // |                  |  |    with this information we can later reclaim the space used by the function call.
    // |                  |  |
    // +------------------+--+---
    // |                  |  | 2. The rest of the items in the stack, for which
    // |                  |  |    we only save any live registers.
    // |                  |  |
    // +------------------+  |
    fn save<M: MacroAssembler>(context: &mut CodeGenContext, masm: &mut M, sig: &ABISig) -> u32 {
        let callee_params = &sig.params;
        let stack = &context.stack;
        match callee_params.len() {
            0 => {
                let _ = context.save_live_registers_and_calculate_sizeof(masm, ..);
                0u32
            }
            _ => {
                assert!(stack.len() >= callee_params.len());
                let partition = stack.len() - callee_params.len();
                let _ = context.save_live_registers_and_calculate_sizeof(masm, 0..partition);
                context.save_live_registers_and_calculate_sizeof(masm, partition..)
            }
        }
    }

    /// Cleanup stack space and free registers after emitting the call.
    fn cleanup<M: MacroAssembler>(
        sig: &ABISig,
        total_space: u32,
        masm: &mut M,
        context: &mut CodeGenContext,
    ) {
        masm.free_stack(total_space);
        // Only account for registers given that any memory entries
        // consumed by the call (assigned to a register or to a stack
        // slot) were freed by the previous call to
        // `masm.free_stack`, so we only care about dropping them
        // here.
        //
        // NOTE / TODO there's probably a path to getting rid of
        // `save_live_registers_and_calculate_sizeof` and
        // `call_stack_space`, making it a bit more obvious what's
        // happening here. We could:
        //
        // * Modify the `spill` implementation so that it takes a
        // filtering callback, to control which values the caller is
        // interested in saving (e.g. save all if no function is provided)
        // * Rely on the new implementation of `drop_last` to calcuate
        // the stack memory entries consumed by the call and then free
        // the calculated stack space.
        context.drop_last(sig.params.len(), |regalloc, v| {
            if v.is_reg() {
                regalloc.free(v.get_reg().into());
            }
        });
        context.push_abi_results(&sig.result, masm);
    }
}
