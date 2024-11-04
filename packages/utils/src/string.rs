use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use rand::{rngs::OsRng, RngCore};

pub fn generate_api_key(length: usize) -> String {
    let mut key = vec![0u8; length];
    OsRng.fill_bytes(&mut key); // Use a secure random number generator
    BASE64_URL_SAFE_NO_PAD.encode(key)
}

#[cfg(test)]
mod tests {
    use super::generate_api_key;

    #[test]
    fn test_generate_api_key() {
        let mut keys = std::collections::HashSet::new();

        // Generate 100000 keys to check for uniqueness
        for _ in 0..100000 {
            let key = generate_api_key(32);
            // Insert into the HashSet, which only keeps unique values
            assert!(keys.insert(key), "Duplicate API key generated!");
        }
    }
}
