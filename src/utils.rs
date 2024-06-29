use num_bigint::BigUint;
use num_traits::Num;

pub fn bytes_to_hex_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .fold(String::with_capacity(bytes.len() * 2), |mut acc, &b| {
            acc.push_str(&format!("{:02X}", b));
            acc
        })
}

pub fn hex_to_bytes(hex_str: &str, bit_length: usize) -> Vec<u8> {
    // Remove "0x" prefix if present
    let cleaned_hex = hex_str.trim_start_matches("0x");
    // Check if the input exceeds the specified bit length
    if cleaned_hex.len() * 4 > bit_length {
        panic!("Input exceeds specified bit length");
    }

    // Parse the hex string
    let value = BigUint::from_str_radix(cleaned_hex, 16).unwrap();
    // Calculate number of bytes needed
    let byte_length = (bit_length + 7) / 8;
    // Convert to big-endian bytes
    let mut bytes = value.to_bytes_be();
    // Pad with leading zeros if necessary
    while bytes.len() < byte_length {
        bytes.insert(0, 0);
    }

    // Truncate if the result is longer than necessary (shouldn't happen with our check, but just in case)
    bytes.truncate(byte_length);
    bytes
}
