use rand::{rngs::OsRng, Rng};

pub fn generate_api_key(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz\
                             0123456789";
    let mut key = String::with_capacity(length);

    for _ in 0..length {
        // Randomly select a character from the CHARSET
        let idx = OsRng.gen_range(0..CHARSET.len());
        key.push(CHARSET[idx] as char);
    }

    key
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
