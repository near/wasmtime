//! zkASM ISA definitions: immediate constants.

// Some variants are never constructed, but we still want them as options in the future.
use super::Inst;
#[allow(dead_code)]
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Copy, Clone, Debug, Default)]
pub struct Imm32 {
    pub bits: i32,
}

impl Imm32 {
    pub fn maybe_from_u64(val: u64) -> Option<Imm32> {
        let sign_bit = 1 << 31;
        if val == 0 {
            Some(Imm32 { bits: 0 })
        } else if (val & sign_bit) != 0 && (val >> 31) == 0xffff_ffff {
            Some(Imm32 {
                bits: (val & 0xffff_ffff) as i32,
            })
        } else if (val & sign_bit) == 0 && (val >> 32) == 0 {
            Some(Imm32 {
                bits: (val & 0xffff_ffff) as i32,
            })
        } else {
            None
        }
    }
}

impl Display for Imm32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:+}", self.bits)
    }
}

// singed
// TODO: remove
#[derive(Clone, Copy, Default)]
pub struct Imm20 {
    /// The immediate bits.
    pub bits: i32,
}

impl Imm20 {
    #[inline]
    pub fn from_bits(bits: i32) -> Self {
        Self {
            bits: bits & 0xf_ffff,
        }
    }
    #[inline]
    pub fn as_u32(&self) -> u32 {
        (self.bits as u32) & 0xf_ffff
    }
}

impl Debug for Imm20 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.bits)
    }
}

impl Display for Imm20 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.bits)
    }
}

/// An unsigned 5-bit immediate.
// TODO: remove
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UImm5 {
    value: u8,
}

impl UImm5 {
    /// Create an unsigned 5-bit immediate from u8.
    pub fn maybe_from_u8(value: u8) -> Option<UImm5> {
        if value < 32 {
            Some(UImm5 { value })
        } else {
            None
        }
    }

    /// Bits for encoding.
    pub fn bits(&self) -> u32 {
        u32::from(self.value)
    }
}

impl Display for UImm5 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}

/// A Signed 5-bit immediate.
// TODO: remove
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Imm5 {
    value: i8,
}

impl Imm5 {
    /// Create an signed 5-bit immediate from an i8.
    pub fn maybe_from_i8(value: i8) -> Option<Imm5> {
        if value >= -16 && value <= 15 {
            Some(Imm5 { value })
        } else {
            None
        }
    }

    pub fn from_bits(value: u8) -> Imm5 {
        assert_eq!(value & 0x1f, value);
        let signed = ((value << 3) as i8) >> 3;
        Imm5 { value: signed }
    }

    /// Bits for encoding.
    pub fn bits(&self) -> u8 {
        self.value as u8 & 0x1f
    }
}

impl Display for Imm5 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}

impl Inst {
    pub(crate) fn imm_min() -> i64 {
        let imm20_max: i64 = (1 << 19) << 12;
        let imm12_max = 1 << 11;
        -imm20_max - imm12_max
    }
    pub(crate) fn imm_max() -> i64 {
        let imm20_max: i64 = ((1 << 19) - 1) << 12;
        let imm12_max = (1 << 11) - 1;
        imm20_max + imm12_max
    }

    /// An imm20 immediate and an Imm12 immediate can generate a 32-bit immediate.
    /// This helper produces an imm12, imm20, or both to generate the value.
    ///
    /// `value` must be between `imm_min()` and `imm_max()`, or else
    /// this helper returns `None`.
    pub(crate) fn generate_imm<R>(
        value: u64,
        mut handle_imm: impl FnMut(Option<Imm20>) -> R,
    ) -> Option<R> {
        let value = value as i64;
        if !(value >= Self::imm_min() && value <= Self::imm_max()) {
            // not in range, return None.
            return None;
        }
        const MOD_NUM: i64 = 4096;
        let (imm20, imm12) = if value > 0 {
            let mut imm20 = value / MOD_NUM;
            let mut imm12 = value % MOD_NUM;
            if imm12 >= 2048 {
                imm12 -= MOD_NUM;
                imm20 += 1;
            }
            assert!(imm12 >= -2048 && imm12 <= 2047);
            (imm20, imm12)
        } else {
            // this is the abs value.
            let value_abs = value.abs();
            let imm20 = value_abs / MOD_NUM;
            let imm12 = value_abs % MOD_NUM;
            let mut imm20 = -imm20;
            let mut imm12 = -imm12;
            if imm12 < -2048 {
                imm12 += MOD_NUM;
                imm20 -= 1;
            }
            (imm20, imm12)
        };
        assert!(imm20 >= -(0x7_ffff + 1) && imm20 <= 0x7_ffff);
        assert!(imm20 != 0 || imm12 != 0);
        Some(handle_imm(if imm20 != 0 {
            Some(Imm20::from_bits(imm20 as i32))
        } else {
            None
        }))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn imm20_and_imm12() {
        assert!(Inst::imm_max() == (i32::MAX - 2048) as i64);
        assert!(Inst::imm_min() == i32::MIN as i64 - 2048);
    }
}
