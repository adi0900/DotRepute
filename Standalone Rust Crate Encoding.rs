//! Data encoding and decoding utilities

use crate::{Error, Result};

#[cfg(not(feature = "std"))]
use alloc::{vec::Vec, string::String};

/// Encode bytes to hexadecimal string
pub fn hex_encode(bytes: &[u8]) -> String {
    let mut hex = String::new();
    
    for &byte in bytes {
        hex.push(nibble_to_char((byte >> 4) & 0x0f));
        hex.push(nibble_to_char(byte & 0x0f));
    }
    
    hex
}

/// Decode hexadecimal string to bytes
pub fn hex_decode(hex: &str) -> Result<Vec<u8>> {
    if hex.len() % 2 != 0 {
        return Err(Error::EncodingError);
    }
    
    let mut bytes = Vec::new();
    let chars: Vec<char> = hex.chars().collect();
    
    for i in (0..chars.len()).step_by(2) {
        let high = char_to_nibble(chars[i])?;
        let low = char_to_nibble(chars[i + 1])?;
        bytes.push((high << 4) | low);
    }
    
    Ok(bytes)
}

fn nibble_to_char(nibble: u8) -> char {
    match nibble {
        0..=9 => (b'0' + nibble) as char,
        10..=15 => (b'a' + nibble - 10) as char,
        _ => '0',
    }
}

fn char_to_nibble(c: char) -> Result<u8> {
    match c {
        '0'..='9' => Ok(c as u8 - b'0'),
        'a'..='f' => Ok(c as u8 - b'a' + 10),
        'A'..='F' => Ok(c as u8 - b'A' + 10),
        _ => Err(Error::EncodingError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_encode() {
        assert_eq!(hex_encode(&[0xde, 0xad]), "dead");
    }

    #[test]
    fn test_hex_decode() {
        let result = hex_decode("dead").unwrap();
        assert_eq!(result, vec![0xde, 0xad]);
    }
}
