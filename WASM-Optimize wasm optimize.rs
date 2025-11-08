//! WASM-Optimized Helper Modules
//! 
//! High-performance, modular utilities for cryptography, data parsing, and score calculations.
//! Designed for browser, serverless, and on-chain environments with no_std compatibility.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{vec::Vec, string::String};

#[cfg(feature = "std")]
use std::{vec::Vec, string::String};

pub mod score_normalizer {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct NormalizationConfig {
        pub min_value: f64,
        pub max_value: f64,
        pub scale_factor: f64,
    }

    impl Default for NormalizationConfig {
        fn default() -> Self {
            Self {
                min_value: 0.0,
                max_value: 100.0,
                scale_factor: 1.0,
            }
        }
    }

    pub fn normalize_score(value: f64, config: &NormalizationConfig) -> f64 {
        let clamped = clamp(value, config.min_value, config.max_value);
        let range = config.max_value - config.min_value;
        
        if range == 0.0 {
            return config.min_value;
        }
        
        ((clamped - config.min_value) / range) * config.scale_factor
    }

    pub fn denormalize_score(normalized: f64, config: &NormalizationConfig) -> f64 {
        let range = config.max_value - config.min_value;
        let value = (normalized / config.scale_factor) * range + config.min_value;
        clamp(value, config.min_value, config.max_value)
    }

    pub fn normalize_batch(values: &[f64], config: &NormalizationConfig) -> Vec<f64> {
        values.iter()
            .map(|&v| normalize_score(v, config))
            .collect()
    }

    pub fn z_score_normalize(value: f64, mean: f64, std_dev: f64) -> f64 {
        if std_dev == 0.0 {
            return 0.0;
        }
        (value - mean) / std_dev
    }

    pub fn min_max_normalize(value: f64, min: f64, max: f64) -> f64 {
        if max == min {
            return 0.0;
        }
        (value - min) / (max - min)
    }

    fn clamp(value: f64, min: f64, max: f64) -> f64 {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_normalize_score() {
            let config = NormalizationConfig::default();
            assert_eq!(normalize_score(50.0, &config), 0.5);
            assert_eq!(normalize_score(0.0, &config), 0.0);
            assert_eq!(normalize_score(100.0, &config), 1.0);
        }

        #[test]
        fn test_denormalize_score() {
            let config = NormalizationConfig::default();
            assert_eq!(denormalize_score(0.5, &config), 50.0);
        }

        #[test]
        fn test_z_score_normalize() {
            assert_eq!(z_score_normalize(10.0, 5.0, 2.0), 2.5);
        }
    }
}

pub mod data_cleaner {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct CleaningStats {
        pub total_records: u32,
        pub cleaned_records: u32,
        pub invalid_records: u32,
        pub duplicates_removed: u32,
    }

    impl CleaningStats {
        pub fn new() -> Self {
            Self {
                total_records: 0,
                cleaned_records: 0,
                invalid_records: 0,
                duplicates_removed: 0,
            }
        }
    }

    pub fn remove_whitespace(input: &str) -> String {
        input.chars().filter(|c| !c.is_whitespace()).collect()
    }

    pub fn trim_all(input: &str) -> String {
        input.trim().to_string()
    }

    pub fn normalize_text(input: &str) -> String {
        input.to_lowercase().trim().to_string()
    }

    pub fn remove_duplicates<T: PartialEq + Clone>(items: &[T]) -> Vec<T> {
        let mut result = Vec::new();
        
        for item in items {
            if !result.contains(item) {
                result.push(item.clone());
            }
        }
        
        result
    }

    pub fn validate_numeric_range(value: f64, min: f64, max: f64) -> bool {
        value >= min && value <= max
    }

    pub fn clean_numeric_string(input: &str) -> Option<f64> {
        let cleaned: String = input.chars()
            .filter(|c| c.is_numeric() || *c == '.' || *c == '-')
            .collect();
        
        cleaned.parse::<f64>().ok()
    }

    pub fn remove_outliers(values: &[f64], threshold: f64) -> Vec<f64> {
        if values.is_empty() {
            return Vec::new();
        }

        let mean = values.iter().sum::<f64>() / values.len() as f64;
        
        let variance = values.iter()
            .map(|&v| (v - mean).powi(2))
            .sum::<f64>() / values.len() as f64;
        
        let std_dev = variance.sqrt();
        
        values.iter()
            .filter(|&&v| (v - mean).abs() <= threshold * std_dev)
            .copied()
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_remove_whitespace() {
            assert_eq!(remove_whitespace("hello world"), "helloworld");
        }

        #[test]
        fn test_normalize_text() {
            assert_eq!(normalize_text("  Hello World  "), "hello world");
        }

        #[test]
        fn test_remove_duplicates() {
            let input = vec![1, 2, 2, 3, 3, 3, 4];
            let result = remove_duplicates(&input);
            assert_eq!(result, vec![1, 2, 3, 4]);
        }

        #[test]
        fn test_validate_numeric_range() {
            assert!(validate_numeric_range(50.0, 0.0, 100.0));
            assert!(!validate_numeric_range(150.0, 0.0, 100.0));
        }

        #[test]
        fn test_clean_numeric_string() {
            assert_eq!(clean_numeric_string("123.45"), Some(123.45));
            assert_eq!(clean_numeric_string("-67.89"), Some(-67.89));
        }
    }
}

pub mod identity_parser {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct IdentityData {
        pub account_id: String,
        pub is_verified: bool,
        pub verification_level: u8,
        pub metadata: Vec<u8>,
    }

    impl IdentityData {
        pub fn new(account_id: String) -> Self {
            Self {
                account_id,
                is_verified: false,
                verification_level: 0,
                metadata: Vec::new(),
            }
        }
    }

    pub fn parse_account_id(input: &str) -> Result<String, &'static str> {
        let cleaned = input.trim();
        
        if cleaned.is_empty() {
            return Err("Empty account ID");
        }
        
        if cleaned.len() < 32 {
            return Err("Account ID too short");
        }
        
        Ok(cleaned.to_string())
    }

    pub fn validate_identity_format(input: &str) -> bool {
        if input.len() < 32 || input.len() > 64 {
            return false;
        }
        
        input.chars().all(|c| c.is_alphanumeric())
    }

    pub fn extract_verification_level(score: u64) -> u8 {
        match score {
            0..=30 => 0,
            31..=60 => 1,
            61..=80 => 2,
            _ => 3,
        }
    }

    pub fn encode_metadata(name: &str, value: &str) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(name.as_bytes());
        result.push(b':');
        result.extend_from_slice(value.as_bytes());
        result
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parse_account_id() {
            let valid = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
            assert!(parse_account_id(valid).is_ok());
        }

        #[test]
        fn test_validate_identity_format() {
            assert!(validate_identity_format("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"));
            assert!(!validate_identity_format("invalid"));
        }

        #[test]
        fn test_extract_verification_level() {
            assert_eq!(extract_verification_level(25), 0);
            assert_eq!(extract_verification_level(50), 1);
            assert_eq!(extract_verification_level(75), 2);
            assert_eq!(extract_verification_level(95), 3);
        }
    }
}

pub mod crypto_utils {
    use super::*;

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

    pub fn xor_encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
        if key.is_empty() {
            return data.to_vec();
        }
        
        data.iter()
            .enumerate()
            .map(|(i, &byte)| byte ^ key[i % key.len()])
            .collect()
    }

    pub fn xor_decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
        xor_encrypt(data, key)
    }

    pub fn checksum(data: &[u8]) -> u32 {
        data.iter()
            .enumerate()
            .fold(0u32, |acc, (i, &byte)| {
                acc.wrapping_add((byte as u32).wrapping_mul((i as u32) + 1))
            })
    }

    pub fn verify_checksum(data: &[u8], expected: u32) -> bool {
        checksum(data) == expected
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_simple_hash() {
            let input = b"hello world";
            let hash = simple_hash(input);
            assert_eq!(hash.len(), 32);
        }

        #[test]
        fn test_xor_encryption() {
            let data = b"secret message";
            let key = b"key123";
            
            let encrypted = xor_encrypt(data, key);
            let decrypted = xor_decrypt(&encrypted, key);
            
            assert_eq!(decrypted, data);
        }

        #[test]
        fn test_checksum() {
            let data = b"test data";
            let sum = checksum(data);
            assert!(verify_checksum(data, sum));
        }
    }
}

pub mod math_helpers {
    pub fn fast_pow(base: u64, exp: u32) -> u64 {
        let mut result = 1u64;
        let mut b = base;
        let mut e = exp;
        
        while e > 0 {
            if e & 1 == 1 {
                result = result.saturating_mul(b);
            }
            b = b.saturating_mul(b);
            e >>= 1;
        }
        
        result
    }

    pub fn integer_sqrt(n: u64) -> u64 {
        if n < 2 {
            return n;
        }
        
        let mut x = n;
        let mut y = (x + 1) / 2;
        
        while y < x {
            x = y;
            y = (x + n / x) / 2;
        }
        
        x
    }

    pub fn integer_log2(mut n: u64) -> u64 {
        if n == 0 {
            return 0;
        }
        
        let mut log = 0;
        while n > 1 {
            n >>= 1;
            log += 1;
        }
        
        log
    }

    pub fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    pub fn lcm(a: u64, b: u64) -> u64 {
        if a == 0 || b == 0 {
            return 0;
        }
        (a / gcd(a, b)).saturating_mul(b)
    }

    pub fn abs_diff(a: i64, b: i64) -> u64 {
        a.wrapping_sub(b).unsigned_abs()
    }

    pub fn interpolate(start: f64, end: f64, t: f64) -> f64 {
        start + (end - start) * t.clamp(0.0, 1.0)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_fast_pow() {
            assert_eq!(fast_pow(2, 10), 1024);
            assert_eq!(fast_pow(5, 3), 125);
        }

        #[test]
        fn test_integer_sqrt() {
            assert_eq!(integer_sqrt(16), 4);
            assert_eq!(integer_sqrt(100), 10);
        }

        #[test]
        fn test_integer_log2() {
            assert_eq!(integer_log2(8), 3);
            assert_eq!(integer_log2(16), 4);
        }

        #[test]
        fn test_gcd() {
            assert_eq!(gcd(48, 18), 6);
        }

        #[test]
        fn test_lcm() {
            assert_eq!(lcm(4, 6), 12);
        }
    }
}

pub mod time_utils {
    pub const SECONDS_PER_DAY: u64 = 86400;
    pub const SECONDS_PER_HOUR: u64 = 3600;
    pub const SECONDS_PER_MINUTE: u64 = 60;

    pub fn days_to_seconds(days: u64) -> u64 {
        days.saturating_mul(SECONDS_PER_DAY)
    }

    pub fn seconds_to_days(seconds: u64) -> u64 {
        seconds / SECONDS_PER_DAY
    }

    pub fn format_duration(seconds: u64) -> (u64, u64, u64, u64) {
        let days = seconds / SECONDS_PER_DAY;
        let remaining = seconds % SECONDS_PER_DAY;
        let hours = remaining / SECONDS_PER_HOUR;
        let remaining = remaining % SECONDS_PER_HOUR;
        let minutes = remaining / SECONDS_PER_MINUTE;
        let secs = remaining % SECONDS_PER_MINUTE;
        
        (days, hours, minutes, secs)
    }

    pub fn is_expired(timestamp: u64, current_time: u64, max_age: u64) -> bool {
        current_time.saturating_sub(timestamp) > max_age
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_days_to_seconds() {
            assert_eq!(days_to_seconds(1), 86400);
        }

        #[test]
        fn test_seconds_to_days() {
            assert_eq!(seconds_to_days(172800), 2);
        }

        #[test]
        fn test_format_duration() {
            let (d, h, m, s) = format_duration(90061);
            assert_eq!(d, 1);
            assert_eq!(h, 1);
            assert_eq!(m, 1);
            assert_eq!(s, 1);
        }

        #[test]
        fn test_is_expired() {
            assert!(is_expired(1000, 2000, 500));
            assert!(!is_expired(1800, 2000, 500));
        }
    }
}

pub mod encoding {
    use super::*;

    pub fn hex_encode(bytes: &[u8]) -> String {
        let mut hex = String::new();
        
        for &byte in bytes {
            hex.push(nibble_to_char((byte >> 4) & 0x0f));
            hex.push(nibble_to_char(byte & 0x0f));
        }
        
        hex
    }

    pub fn hex_decode(hex: &str) -> Result<Vec<u8>, &'static str> {
        if hex.len() % 2 != 0 {
            return Err("Invalid hex length");
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

    fn char_to_nibble(c: char) -> Result<u8, &'static str> {
        match c {
            '0'..='9' => Ok(c as u8 - b'0'),
            'a'..='f' => Ok(c as u8 - b'a' + 10),
            'A'..='F' => Ok(c as u8 - b'A' + 10),
            _ => Err("Invalid hex character"),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_hex_encode() {
            assert_eq!(hex_encode(&[0xde, 0xad, 0xbe, 0xef]), "deadbeef");
        }

        #[test]
        fn test_hex_decode() {
            let result = hex_decode("deadbeef").unwrap();
            assert_eq!(result, vec![0xde, 0xad, 0xbe, 0xef]);
        }

        #[test]
        fn test_hex_roundtrip() {
            let original = b"hello world";
            let encoded = hex_encode(original);
            let decoded = hex_decode(&encoded).unwrap();
            assert_eq!(decoded, original);
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod integration_tests {
    use super::*;

    #[test]
    fn test_full_workflow() {
        let raw_score = 75.5;
        let config = score_normalizer::NormalizationConfig::default();
        let normalized = score_normalizer::normalize_score(raw_score, &config);
        
        assert!(normalized >= 0.0 && normalized <= 1.0);
        
        let denormalized = score_normalizer::denormalize_score(normalized, &config);
        assert!((denormalized - raw_score).abs() < 0.01);
    }

    #[test]
    fn test_data_pipeline() {
        let input = "  Hello World  ";
        let cleaned = data_cleaner::normalize_text(input);
        assert_eq!(cleaned, "hello world");
        
        let data = cleaned.as_bytes();
        let hash = crypto_utils::simple_hash(data);
        assert_eq!(hash.len(), 32);
        
        let encoded = encoding::hex_encode(&hash);
        assert_eq!(encoded.len(), 64);
    }
}
