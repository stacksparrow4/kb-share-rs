pub fn bytes_to_u16(b: &[u8]) -> u16 {
    return ((b[0] as u16) << 8) | (b[1] as u16);
}

pub fn u16_to_bytes(d: u16) -> [u8; 2] {
    return [(d >> 8) as u8, (d & 0xff) as u8];
}
