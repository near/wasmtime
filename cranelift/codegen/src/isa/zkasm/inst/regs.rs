//! Riscv64 ISA definitions: registers.
//!

use crate::settings;

use crate::machinst::{Reg, Writable};

use crate::machinst::RealReg;
use alloc::vec;
use alloc::vec::Vec;

use regalloc2::VReg;
use regalloc2::{MachineEnv, PReg, RegClass};

#[inline]
pub fn a0() -> Reg {
    x_reg(1)
}

#[inline]
pub fn b0() -> Reg {
    x_reg(2)
}

// second argument of function call
#[inline]
pub fn a1() -> Reg {
    x_reg(11)
}

// third argument of function call
#[inline]
pub fn a2() -> Reg {
    x_reg(12)
}

#[inline]
pub fn writable_a0() -> Writable<Reg> {
    Writable::from_reg(a0())
}
#[inline]
pub fn writable_a1() -> Writable<Reg> {
    Writable::from_reg(a1())
}
#[inline]
pub fn writable_a2() -> Writable<Reg> {
    Writable::from_reg(a2())
}

#[inline]
pub fn fa0() -> Reg {
    f_reg(10)
}
#[inline]
pub fn writable_fa0() -> Writable<Reg> {
    Writable::from_reg(fa0())
}
#[inline]
pub fn writable_fa1() -> Writable<Reg> {
    Writable::from_reg(fa1())
}
#[inline]
pub fn fa1() -> Reg {
    f_reg(11)
}

#[inline]
pub fn fa7() -> Reg {
    f_reg(17)
}

/// Get a reference to the zero-register.
#[inline]
pub fn zero_reg() -> Reg {
    x_reg(0)
}

/// Get a writable reference to the zero-register (this discards a result).
#[inline]
pub fn writable_zero_reg() -> Writable<Reg> {
    Writable::from_reg(zero_reg())
}
#[inline]
pub fn stack_reg() -> Reg {
    x_reg(2)
}

/// Get a writable reference to the stack-pointer register.
#[inline]
pub fn writable_stack_reg() -> Writable<Reg> {
    Writable::from_reg(stack_reg())
}

/// Get a reference to the link register (x1).
pub fn link_reg() -> Reg {
    x_reg(1)
}

/// Get a writable reference to the link register.
#[inline]
pub fn writable_link_reg() -> Writable<Reg> {
    Writable::from_reg(link_reg())
}

/// Get a reference to the frame pointer (x29).
#[inline]
pub fn fp_reg() -> Reg {
    x_reg(8)
}

/// Get a writable reference to the frame pointer.
#[inline]
pub fn writable_fp_reg() -> Writable<Reg> {
    Writable::from_reg(fp_reg())
}

/// Get a reference to the first temporary, sometimes "spill temporary",
/// register. This register is used in various ways as a temporary.
#[inline]
pub fn spilltmp_reg() -> Reg {
    x_reg(31)
}

/// Get a writable reference to the spilltmp reg.
#[inline]
pub fn writable_spilltmp_reg() -> Writable<Reg> {
    Writable::from_reg(spilltmp_reg())
}

///spilltmp2
#[inline]
pub fn spilltmp_reg2() -> Reg {
    x_reg(30)
}

/// Get a writable reference to the spilltmp2 reg.
#[inline]
pub fn writable_spilltmp_reg2() -> Writable<Reg> {
    Writable::from_reg(spilltmp_reg2())
}

pub fn crate_reg_eviroment(_flags: &settings::Flags) -> MachineEnv {
    let preferred_regs_by_class: [Vec<PReg>; 3] = {
        // Registers are A, B, C, D, E.
        // TODO(akashin): Remove redundant registers.
        let x_registers: Vec<PReg> = (1..=15)
            .map(|i| PReg::new(i, RegClass::Int))
            .collect();

        let f_registers: Vec<PReg> = Vec::new();
            // (0..=7)
            // .chain(10..=17)
            // .chain(28..=31)
            // .map(|i| PReg::new(i, RegClass::Float))
            // .collect();

        let v_registers: Vec<PReg> = Vec::new();
            // (0..=31).map(|i| PReg::new(i, RegClass::Vector)).collect();

        [x_registers, f_registers, v_registers]
    };

    let non_preferred_regs_by_class: [Vec<PReg>; 3] = {
        let x_registers: Vec<PReg> = Vec::new();
            // (9..=9)
            // .chain(18..=27)
            // .map(|i| PReg::new(i, RegClass::Int))
            // .collect();

        let f_registers: Vec<PReg> = Vec::new();
            // (8..=9)
            // .chain(18..=27)
            // .map(|i| PReg::new(i, RegClass::Float))
            // .collect();

        let v_registers = vec![];

        [x_registers, f_registers, v_registers]
    };

    MachineEnv {
        preferred_regs_by_class,
        non_preferred_regs_by_class,
        fixed_stack_slots: vec![],
        scratch_by_class: [None, None, None],
    }
}

#[inline]
pub fn x_reg(enc: usize) -> Reg {
    let p_reg = PReg::new(enc, RegClass::Int);
    let v_reg = VReg::new(p_reg.index(), p_reg.class());
    Reg::from(v_reg)
}
pub const fn px_reg(enc: usize) -> PReg {
    PReg::new(enc, RegClass::Int)
}

#[inline]
pub fn f_reg(enc: usize) -> Reg {
    let p_reg = PReg::new(enc, RegClass::Float);
    let v_reg = VReg::new(p_reg.index(), p_reg.class());
    Reg::from(v_reg)
}
pub const fn pf_reg(enc: usize) -> PReg {
    PReg::new(enc, RegClass::Float)
}
#[inline]
pub(crate) fn real_reg_to_reg(x: RealReg) -> Reg {
    let v_reg = VReg::new(x.hw_enc() as usize, x.class());
    Reg::from(v_reg)
}

#[allow(dead_code)]
pub(crate) fn x_reg_range(start: usize, end: usize) -> Vec<Writable<Reg>> {
    let mut regs = vec![];
    for i in start..=end {
        regs.push(Writable::from_reg(x_reg(i)));
    }
    regs
}

#[inline]
pub fn v_reg(enc: usize) -> Reg {
    let p_reg = PReg::new(enc, RegClass::Vector);
    let v_reg = VReg::new(p_reg.index(), p_reg.class());
    Reg::from(v_reg)
}
pub const fn pv_reg(enc: usize) -> PReg {
    PReg::new(enc, RegClass::Vector)
}
