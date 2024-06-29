pub fn bytes_to_hex_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .fold(String::with_capacity(bytes.len() * 2), |mut acc, &b| {
            acc.push_str(&format!("{:02X}", b));
            acc
        })
}
