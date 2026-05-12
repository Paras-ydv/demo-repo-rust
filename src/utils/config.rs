use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};

pub fn hash_string(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn encode_base64(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(data)
}

pub fn decode_base64(encoded: &str) -> Result<Vec<u8>, base64::DecodeError> {
    general_purpose::STANDARD.decode(encoded)
}
// auto-commit: 1778586757023