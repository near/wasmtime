#![no_main]
#![no_std]

use panic_halt as _;

use sha3::{Digest, Keccak256};

extern "C" {
    fn assert_eq_i64(actual: u64, expected: u64);
}

fn assert_eq_array(expected: &[u8], actual: &[u8]) {
    for i in 0..32 {
        unsafe {
            assert_eq_i64(expected[i] as u64, actual[i] as u64);
        }
    }
}

#[no_mangle]
pub fn main() {
    let hash = Keccak256::digest(b"X");
    let expected = [
        85, 12, 100, 161, 80, 49, 195, 6, 68, 84, 193, 154, 220, 98, 67, 166, 18, 44, 19, 138, 36,
        46, 170, 9, 141, 165, 11, 177, 20, 252, 141, 86,
    ];

    assert_eq_array(&expected, &hash)
}
