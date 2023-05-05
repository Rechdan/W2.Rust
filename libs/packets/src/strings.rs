use encoding_rs::WINDOWS_1252;

pub fn bytes_to_str(bytes: &[u8]) -> String {
    WINDOWS_1252.decode(bytes).0.to_string()
}

pub fn str_to_bytes(bytes: &mut [u8], value: &str) {
    let mut encoded = WINDOWS_1252
        .encode(value)
        .0
        .to_vec()
        .iter()
        .filter(|v| **v != 0)
        .map(|v| *v)
        .collect::<Vec<_>>();

    encoded.resize(bytes.len(), 0);

    bytes.fill(0);

    bytes.copy_from_slice(&encoded);
}
