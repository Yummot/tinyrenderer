#[allow(dead_code)]
pub fn u32_from_be(buf: &[u8]) -> u32 {
    if buf.len() == 4 {
        (buf[0] as u32) << 24 | (buf[1] as u32) << 16 | (buf[2] as u32) << 8 | buf[3] as u32
    } else if buf.len() == 3 {
        (buf[0] as u32) << 16 | (buf[1] as u32) << 8 | buf[2] as u32
    } else if buf.len() == 2 {
        (buf[1] as u32) << 8 | buf[1] as u32
    } else if buf.len() == 1 {
        buf[0] as u32
    } else {
        panic!("Error: tga_image::base::u32_from_be bad buf parameter.")
    }
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

