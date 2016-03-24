#[allow(dead_code)]
pub fn u16_from_be(buf: &[u8]) -> u16 {
    (buf[0] as u16) << 8 | buf[1] as u16
}

#[allow(dead_code)]
pub fn u16_from_le(buf: &[u8]) -> u16 {
    (buf[1] as u16) << 8 | buf[0] as u16
}

#[allow(dead_code)]
pub fn u16_to_le(x: u16) -> [u8; 2] {
    [x as u8, (x >> 8) as u8]
}

#[allow(dead_code)]
pub fn u32_from_be(buf: &[u8]) -> u32 {
    (buf[0] as u32) << 24 | (buf[1] as u32) << 16 | (buf[2] as u32) << 8 | buf[3] as u32
}

#[allow(dead_code)]
pub fn u32_to_be(x: u32) -> [u8; 4] {
    [(x >> 24) as u8, (x >> 16) as u8, (x >> 8) as u8, (x) as u8]
}

#[allow(dead_code)]
pub fn u32_to_le(x: u32) -> [u8; 4] {
    [(x) as u8, (x >> 8) as u8, (x >> 16) as u8, (x >> 24) as u8]
}

#[allow(dead_code)]
pub fn u32_from_le(buf: &[u8]) -> u32 {
    if buf.len() == 4 {
        (buf[3] as u32) << 24 | (buf[2] as u32) << 16 | (buf[1] as u32) << 8 | buf[0] as u32
    } else if buf.len() == 3 {
        (buf[2] as u32) << 16 | (buf[1] as u32) << 8 | buf[0] as u32
    } else if buf.len() == 2 {
        (buf[1] as u32) << 8 | buf[0] as u32
    } else if buf.len() == 1 {
        buf[0] as u32
    } else {
        panic!("Error: tga_image::base::u32_from_le bad buf parameter.")
    }
}

#[allow(dead_code)]
pub fn i32_from_le(buf: &[u8]) -> i32 {
    ((buf[3] as u32) << 24 | (buf[2] as u32) << 16 | (buf[1] as u32) << 8 | buf[0] as u32)
        as i32
}
