use sha2::{Digest, Sha256};

/// Computes the SHA-256 hash of the given byte slice and returns it as a hex string.
pub fn compute_hash(content: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_hash() {
        let hash = compute_hash(b"hello world");
        assert_eq!(
            hash,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }
}
