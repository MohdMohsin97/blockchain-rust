use std::time::{SystemTime, UNIX_EPOCH};

use crate::prelude::*;

pub fn now() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}

pub fn u32_bytes (u: &u32) -> [u8; 4] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,
    ]
}

pub fn u64_bytes (u: &u64) -> [u8; 8] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,

        (u >> 8 * 0x4) as u8,
        (u >> 8 * 0x5) as u8,
        (u >> 8 * 0x6) as u8,
        (u >> 8 * 0x7) as u8,
    ]
}

pub fn u128_bytes (u: &u128) -> [u8; 16] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,

        (u >> 8 * 0x4) as u8,
        (u >> 8 * 0x5) as u8,
        (u >> 8 * 0x6) as u8,
        (u >> 8 * 0x7) as u8,

        (u >> 8 * 0x8) as u8,
        (u >> 8 * 0x9) as u8,
        (u >> 8 * 0xa) as u8,
        (u >> 8 * 0xb) as u8,

        (u >> 8 * 0xc) as u8,
        (u >> 8 * 0xd) as u8,
        (u >> 8 * 0xe) as u8,
        (u >> 8 * 0xf) as u8,
    ]
}

pub fn difficulty_bytes_as_u128 (v: &BlockHash) -> u128 {
    ((v[31] as u128) << 0xf * 8) |
    ((v[30] as u128) << 0xe * 8) |
    ((v[29] as u128) << 0xd * 8) |
    ((v[28] as u128) << 0xc * 8) |
    ((v[27] as u128) << 0xb * 8) |
    ((v[26] as u128) << 0xa * 8) |
    ((v[25] as u128) << 0x9 * 8) |
    ((v[24] as u128) << 0x8 * 8) |
    ((v[23] as u128) << 0x7 * 8) |
    ((v[22] as u128) << 0x6 * 8) |
    ((v[21] as u128) << 0x5 * 8) |
    ((v[20] as u128) << 0x4 * 8) |
    ((v[19] as u128) << 0x3 * 8) |
    ((v[18] as u128) << 0x2 * 8) |
    ((v[17] as u128) << 0x1 * 8) |
    ((v[16] as u128) << 0x0 * 8)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn give_difficulty_back() {
        let difficulty_bytes: [u8; 32] = [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00,
        ];

        let difficulty_u128 = difficulty_bytes_as_u128(&difficulty_bytes);
        assert_eq!(2_u128.pow(112) - 1, difficulty_u128);

    }
}