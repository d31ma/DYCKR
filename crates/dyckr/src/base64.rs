const BASE64_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn base64(bytes: &[u8]) -> String {
    let mut encoded = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let mut triple = 0u32;
        for (index, byte) in chunk.iter().enumerate() {
            triple |= u32::from(*byte) << (16 - 8 * index);
        }
        for index in 0..4 {
            if index > chunk.len() {
                encoded.push('=');
                continue;
            }
            let sextet = (triple >> (18 - 6 * index)) & 0x3f;
            encoded.push(char::from(BASE64_ALPHABET[sextet as usize]));
        }
    }
    encoded
}
