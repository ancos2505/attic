// type BlockHash = Vec<u8>;

pub fn now_millis() -> anyhow::Result<u128> {
    // Reference: https://stackoverflow.com/a/44378174/2773837
    use std::time::{SystemTime, UNIX_EPOCH};
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis())
}

// pub fn u32_bytes(number_u32: &u32) -> [u8; 4] {
//     use byteorder::{ByteOrder, LittleEndian};

//     let mut buf = [0; 4];
//     LittleEndian::write_u32(&mut buf, *number_u32);
//     buf
// }

// pub fn u64_bytes(u: &u64) -> [u8; 8] {
//     [
//         (u >> 8 * 0x0) as u8,
//         (u >> 8 * 0x1) as u8,
//         (u >> 8 * 0x2) as u8,
//         (u >> 8 * 0x3) as u8,
//         (u >> 8 * 0x4) as u8,
//         (u >> 8 * 0x5) as u8,
//         (u >> 8 * 0x6) as u8,
//         (u >> 8 * 0x7) as u8,
//     ]
// }

// pub fn u128_bytes(u: &u128) -> [u8; 16] {
//     [
//         (u >> 8 * 0x0) as u8,
//         (u >> 8 * 0x1) as u8,
//         (u >> 8 * 0x2) as u8,
//         (u >> 8 * 0x3) as u8,
//         (u >> 8 * 0x4) as u8,
//         (u >> 8 * 0x5) as u8,
//         (u >> 8 * 0x6) as u8,
//         (u >> 8 * 0x7) as u8,
//         (u >> 8 * 0x8) as u8,
//         (u >> 8 * 0x9) as u8,
//         (u >> 8 * 0xa) as u8,
//         (u >> 8 * 0xb) as u8,
//         (u >> 8 * 0xc) as u8,
//         (u >> 8 * 0xd) as u8,
//         (u >> 8 * 0xe) as u8,
//         (u >> 8 * 0xf) as u8,
//     ]
// }
