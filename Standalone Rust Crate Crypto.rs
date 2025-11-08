//! Cryptographic utilities for hashing and checksums

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Simple hash function for data
pub fn simple_hash(input: &[u8]) -> [u8; 32] {
    let mut hash = [0u8; 32];
    
    for (i, &byte) in input.iter().enumerate() {
        hash[i % 32] ^= byte;
        hash[i % 32] = hash[i % 32].wrapping_add(byte);
    }
    
    for i in 0..32 {
        hash[i] = hash[i].wrapping_mul(31).wrapping_add(17);
    }
    
    hash
}

/// Calculate checksum for data
pub fn checksum(data: &[u8]) -> u32 {
    data.iter()
        .enumerate()
        .fold(0u32, |acc, (i, &byte)| {
            acc.wrapping_add((byte as u32).wrapping_mul((i as u32) + 1))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_hash() {
        let data = b"test";
        let hash = simple_hash(data);
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_checksum() {
        let data = b"test";
        let sum = checksum(data);
        assert!(sum > 0);
    }
}
