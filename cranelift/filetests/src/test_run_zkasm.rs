//! Test command for interpreting CLIF files and verifying their results
//!
//! The `interpret` test command interprets each function on the host machine
//! using [RunCommand](cranelift_reader::RunCommand)s.

use crate::runone::FileUpdate;
use crate::subtest::SubTest;
use anyhow::Context;
use cranelift_codegen::data_value::DataValue;
use cranelift_codegen::ir::Function;
use cranelift_codegen::isa::TargetIsa;
use cranelift_codegen::settings::Flags;
use cranelift_codegen::{self, ir};
use cranelift_interpreter::environment::FunctionStore;
use cranelift_reader::{parse_run_command, Details, TestCommand, TestFile};
use log::{info, trace};
use std::borrow::Cow;

use std::collections::HashMap;

use cranelift_codegen::entity::EntityRef;
use cranelift_codegen::ir::function::FunctionParameters;
use cranelift_codegen::ir::ExternalName;
use cranelift_codegen::isa::zkasm;
use cranelift_codegen::{settings, FinalizedMachReloc, FinalizedRelocTarget};

struct TestRunZkasm;

pub fn subtest(parsed: &TestCommand) -> anyhow::Result<Box<dyn SubTest>> {
    assert_eq!(parsed.command, "run-zkasm");
    if !parsed.options.is_empty() {
        anyhow::bail!("No options allowed on {}", parsed);
    }
    Ok(Box::new(TestRunZkasm))
}

impl SubTest for TestRunZkasm {
    fn name(&self) -> &'static str {
        "run-zkasm"
    }

    fn is_mutating(&self) -> bool {
        false
    }

    fn needs_isa(&self) -> bool {
        false
    }

    /// Runs the entire subtest for a given target, invokes [Self::run] for running
    /// individual tests.
    fn run_target<'a>(
        &self,
        testfile: &TestFile,
        _: &mut FileUpdate,
        _: &'a str,
        _: &'a Flags,
        _: Option<&'a dyn TargetIsa>,
    ) -> anyhow::Result<()> {
        // We can build the FunctionStore once and reuse it
        let mut func_store = FunctionStore::default();
        for (func, _) in &testfile.functions {
            func_store.add(func.name.to_string(), &func);
        }

        for (func, details) in &testfile.functions {
            info!("Test: {}({}) runner-zkasm", self.name(), func.name);

            run_test(&func_store, func, details).context(self.name())?;
        }

        Ok(())
    }

    fn run(
        &self,
        _func: Cow<ir::Function>,
        _context: &crate::subtest::Context,
    ) -> anyhow::Result<()> {
        unreachable!()
    }
}

struct ZkAsmExecutor {}

impl ZkAsmExecutor {
    pub fn new() -> ZkAsmExecutor {
        ZkAsmExecutor {}
    }

    pub fn execute(self, zkasm_function: &Vec<String>, args: &Vec<DataValue>) -> DataValue {
        for line in zkasm_function {
            println!("{}", line);
        }
        for arg in args {
            println!("{:#?}", arg);
        }
        todo!();
    }
}

fn run_test(func_store: &FunctionStore, func: &Function, details: &Details) -> anyhow::Result<()> {
    let zkasm_function = generate_zkasm(func_store, func);
    for comment in details.comments.iter() {
        if let Some(command) = parse_run_command(comment.text, &func.signature)? {
            trace!("Parsed run command: {}", command);
            command
                .run(|_func_name, run_args| {
                    let mut args = Vec::with_capacity(run_args.len());
                    args.extend_from_slice(run_args);
                    let executor = ZkAsmExecutor::new();
                    let result = executor.execute(&zkasm_function, &args);
                    Ok(vec![result])
                })
                .map_err(|e| anyhow::anyhow!("{}", e))?;
        }
    }
    Ok(())
}

fn generate_zkasm(_func_store: &FunctionStore, func: &Function) -> Vec<String> {
    let flag_builder = settings::builder();
    let isa_builder = zkasm::isa_builder("zkasm-unknown-unknown".parse().unwrap());
    let isa = isa_builder
        .finish(settings::Flags::new(flag_builder))
        .unwrap();
    let mut comp_ctx = cranelift_codegen::Context::for_function(func.clone());
    let compiled_code = comp_ctx
        .compile(isa.as_ref(), &mut Default::default())
        .unwrap();
    let mut code_buffer = compiled_code.code_buffer().to_vec();
    fix_relocs(
        &mut code_buffer,
        &func.params,
        compiled_code.buffer.relocs(),
    );
    let code = std::str::from_utf8(&code_buffer).unwrap();
    let lines: Vec<String> = code.lines().map(|s| s.to_string()).collect();
    lines
}

// TODO: Labels optimization already happens in `MachBuffer`, we need to find a way to leverage
// it.
/// Label name is formatted as follows: <label_name>_<function_id>_<label_id>
/// Function id is unique through whole program while label id is unique only
/// inside given function.
/// Label name must begin from label_.
fn optimize_labels(code: &[&str], func_index: usize) -> Vec<String> {
    let mut label_definition: HashMap<String, usize> = HashMap::new();
    let mut label_uses: HashMap<String, Vec<usize>> = HashMap::new();
    let mut lines = Vec::new();
    for (index, line) in code.iter().enumerate() {
        let mut line = line.to_string();
        if line.starts_with(&"label_") {
            // Handles lines with a label marker, e.g.:
            //   <label_name>_XXX:
            let index_begin = line.rfind("_").expect("Failed to parse label index") + 1;
            let label_name: String = line[..line.len() - 1].to_string();
            line.insert_str(index_begin - 1, &format!("_{}", func_index));
            label_definition.insert(label_name, index);
        } else if line.contains(&"label_") {
            // Handles lines with a jump to label, e.g.:
            // A : JMPNZ(<label_name>_XXX)
            let pos = line.rfind(&"_").unwrap() + 1;
            let label_name = line[line
                .find("label_")
                .expect(&format!("Error parsing label line '{}'", line))
                ..line
                    .rfind(")")
                    .expect(&format!("Error parsing label line '{}'", line))]
                .to_string();
            line.insert_str(pos - 1, &format!("_{}", func_index));
            label_uses.entry(label_name).or_default().push(index);
        }
        lines.push(line);
    }

    let mut lines_to_delete = Vec::new();
    for (label, label_line) in label_definition {
        match label_uses.entry(label) {
            std::collections::hash_map::Entry::Occupied(uses) => {
                if uses.get().len() == 1 {
                    let use_line = uses.get()[0];
                    if use_line + 1 == label_line {
                        lines_to_delete.push(use_line);
                        lines_to_delete.push(label_line);
                    }
                }
            }
            std::collections::hash_map::Entry::Vacant(_) => {
                lines_to_delete.push(label_line);
            }
        }
    }
    lines_to_delete.sort();
    lines_to_delete.reverse();
    for index in lines_to_delete {
        lines.remove(index);
    }
    lines
}

fn generate_preamble(
    start_func_index: usize,
    globals: &[(cranelift_wasm::GlobalIndex, cranelift_wasm::GlobalInit)],
    data_segments: &[(u64, Vec<u8>)],
) -> Vec<String> {
    let mut program: Vec<String> = Vec::new();

    // Generate global variable definitions.
    for (key, _) in globals {
        program.push(format!("VAR GLOBAL global_{}", key.index()));
    }

    program.push("start:".to_string());
    for (key, init) in globals {
        match init {
            cranelift_wasm::GlobalInit::I32Const(v) => {
                // ZKASM stores constants in 2-complement form, so we need a cast to unsigned.
                program.push(format!(
                    "  {} :MSTORE(global_{})  ;; Global32({})",
                    *v as u32,
                    key.index(),
                    v
                ));
            }
            cranelift_wasm::GlobalInit::I64Const(v) => {
                // ZKASM stores constants in 2-complement form, so we need a cast to unsigned.
                program.push(format!(
                    "  {} :MSTORE(global_{})  ;; Global64({})",
                    *v as u64,
                    key.index(),
                    v
                ));
            }
            _ => unimplemented!("Global type is not supported"),
        }
    }

    // Generate const data segments definitions.
    for (offset, data) in data_segments {
        program.push(format!("  {offset} => E"));
        // Each slot stores 8 consecutive u8 numbers, with earlier addresses stored in lower
        // bits.
        for (i, chunk) in data.chunks(8).enumerate() {
            let mut chunk_data = 0u64;
            for c in chunk.iter().rev() {
                chunk_data <<= 8;
                chunk_data |= *c as u64;
            }
            program.push(format!("  {chunk_data}n :MSTORE(MEM:E + {i})"));
        }
    }

    // The total amount of stack available on ZKASM processor is 2^16 of 8-byte words.
    // Stack memory is a separate region that is independent from the heap.
    program.push("  0xffff => SP".to_string());
    program.push("  zkPC + 2 => RR".to_string());
    program.push(format!("  :JMP(function_{})", start_func_index));
    program.push("  :JMP(finalizeExecution)".to_string());
    program
}

fn generate_postamble() -> Vec<String> {
    let mut program: Vec<String> = Vec::new();
    // In the prover, the program always runs for a fixed number of steps (e.g. 2^23), so we
    // need an infinite loop at the end of the program to fill the execution trace to the
    // expected number of steps.
    // In the future we might need to put zero in all registers here.
    program.push("finalizeExecution:".to_string());
    program.push("  ${beforeLast()}  :JMPN(finalizeExecution)".to_string());
    program.push("                   :JMP(start)".to_string());
    program.push("INCLUDE \"helpers/2-exp.zkasm\"".to_string());
    program
}

// TODO: Relocations should be generated by a linker and/or clift itself.
fn fix_relocs(
    code_buffer: &mut Vec<u8>,
    params: &FunctionParameters,
    relocs: &[FinalizedMachReloc],
) {
    let mut delta = 0i32;
    for reloc in relocs {
        let start = (reloc.offset as i32 + delta) as usize;
        let mut pos = start;
        while code_buffer[pos] != b'\n' {
            pos += 1;
            delta -= 1;
        }

        let code =
            if let FinalizedRelocTarget::ExternalName(ExternalName::User(name)) = reloc.target {
                let name = &params.user_named_funcs()[name];
                if name.index == 0 {
                    b"  B :ASSERT".to_vec()
                } else {
                    format!("  zkPC + 2 => RR\n  :JMP(function_{})", name.index)
                        .as_bytes()
                        .to_vec()
                }
            } else {
                b"  UNKNOWN".to_vec()
            };
        delta += code.len() as i32;

        code_buffer.splice(start..pos, code);
    }
}
