//! Define supported ISAs; includes ISA-specific instructions, encodings, registers, settings, etc.
use crate::cdsl::isa::TargetIsa;
use std::fmt;

mod arm64;
mod riscv64;
mod s390x;
pub(crate) mod x86;
mod zkasm;

/// Represents known ISA target.
#[derive(PartialEq, Copy, Clone)]
pub enum Isa {
    X86,
    Arm64,
    S390x,
    Riscv64,
    ZkAsm,
}

impl Isa {
    /// Creates isa target using name.
    pub fn from_name(name: &str) -> Option<Self> {
        Isa::all()
            .iter()
            .cloned()
            .find(|isa| isa.to_string() == name)
    }

    /// Creates isa target from arch.
    pub fn from_arch(arch: &str) -> Option<Self> {
        match arch {
            "aarch64" => Some(Isa::Arm64),
            "zkasm" => Some(Isa::ZkAsm),
            "s390x" => Some(Isa::S390x),
            x if ["x86_64", "i386", "i586", "i686"].contains(&x) => Some(Isa::X86),
            "riscv64" | "riscv64gc" | "riscv64imac" => Some(Isa::Riscv64),
            _ => None,
        }
    }

    /// Returns all supported isa targets.
    pub fn all() -> &'static [Isa] {
        &[Isa::X86, Isa::Arm64, Isa::S390x, Isa::Riscv64, Isa::ZkAsm]
    }
}

impl fmt::Display for Isa {
    // These names should be kept in sync with the crate features.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Isa::X86 => write!(f, "x86"),
            Isa::Arm64 => write!(f, "arm64"),
            Isa::S390x => write!(f, "s390x"),
            Isa::Riscv64 => write!(f, "riscv64"),
            Isa::ZkAsm => write!(f, "zkasm"),
        }
    }
}

pub(crate) fn define(isas: &[Isa]) -> Vec<TargetIsa> {
    isas.iter()
        .map(|isa| match isa {
            Isa::X86 => x86::define(),
            Isa::Arm64 => arm64::define(),
            Isa::S390x => s390x::define(),
            Isa::Riscv64 => riscv64::define(),
            Isa::ZkAsm => zkasm::define(),
        })
        .collect()
}
