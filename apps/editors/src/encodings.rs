use encoding_rs::WINDOWS_1252;

pub fn windows1252_to_utf8(buf: &[u8]) -> String {
    let (cow, _, _) = WINDOWS_1252.decode(buf);
    cow.trim().to_string()
}

pub fn utf8_to_windows1252(string: &str) -> Vec<u8> {
    let (a, _, _) = WINDOWS_1252.encode(string);
    a.to_vec()
}
