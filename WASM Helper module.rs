//! WASM Optimized Helper Modules
//!
//! High-performance utility modules optimized for WebAssembly deployment.
//!
//! Features:
//! - High performance: Compiled from Rust to run at near-native speed
//! - Modular and lightweight: Small, reusable functions with minimal dependencies
//! - Cross-platform compatibility: Works in browsers, Node.js, and edge environments
//! - Memory safety and isolation: Sandboxed execution with compile-time memory guarantees
//! - Data processing: Fast parsing of JSON, CSV, XML and other formats
//! - Frontend integration: Directly callable from web interfaces

#![cfg_attr(not(feature = "std"), no_std)]

use core::str;

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec, format};

// JSON parser helper
pub struct JsonParser;

impl JsonParser {
    pub fn parse_number(input: &str) -> Result<f64, &'static str> {
        input.trim().parse::<f64>().map_err(|_| "Invalid number")
    }
    
    pub fn parse_boolean(input: &str) -> Result<bool, &'static str> {
        match input.trim().to_lowercase().as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err("Invalid boolean"),
        }
    }
    
    pub fn escape_string(input: &str) -> String {
        let mut result = String::new();
        for ch in input.chars() {
            match ch {
                '"' => result.push_str("\\\""),
                '\\' => result.push_str("\\\\"),
                '\n' => result.push_str("\\n"),
                '\r' => result.push_str("\\r"),
                '\t' => result.push_str("\\t"),
                _ => result.push(ch),
            }
        }
        result
    }
}

// CSV parser helper
pub struct CsvParser;

impl CsvParser {
    pub fn parse_line(line: &str, delimiter: char) -> Vec<String> {
        let mut fields = Vec::new();
        let mut current_field = String::new();
        let mut in_quotes = false;
        
        for ch in line.chars() {
            match ch {
                '"' => in_quotes = !in_quotes,
                c if c == delimiter && !in_quotes => {
                    fields.push(current_field.clone());
                    current_field.clear();
                },
                c => current_field.push(c),
            }
        }
        
        fields.push(current_field);
        fields
    }
    
    pub fn count_fields(line: &str, delimiter: char) -> usize {
        Self::parse_line(line, delimiter).len()
    }
}

// Text processing helpers
pub struct TextProcessor;

impl TextProcessor {
    pub fn normalize_whitespace(input: &str) -> String {
        input.split_whitespace().collect::<Vec<&str>>().join(" ")
    }
    
    pub fn to_uppercase(input: &str) -> String {
        input.to_uppercase()
    }
    
    pub fn to_lowercase(input: &str) -> String {
        input.to_lowercase()
    }
    
    pub fn trim(input: &str) -> String {
        input.trim().to_string()
    }
    
    pub fn count_words(input: &str) -> usize {
        input.split_whitespace().count()
    }
    
    pub fn count_chars(input: &str) -> usize {
        input.chars().count()
    }
}

// Cryptographic helpers
pub struct CryptoHelper;

impl CryptoHelper {
    pub fn simple_hash(input: &str) -> u64 {
        let mut hash: u64 = 5381;
        for byte in input.bytes() {
            hash = ((hash << 5).wrapping_add(hash)).wrapping_add(byte as u64);
        }
        hash
    }
    
    pub fn xor_encrypt(data: &[u8], key: u8) -> Vec<u8> {
        data.iter().map(|&b| b ^ key).collect()
    }
    
    pub fn xor_decrypt(data: &[u8], key: u8) -> Vec<u8> {
        Self::xor_encrypt(data, key)
    }
}

// Numerical computation helpers
pub struct MathHelper;

impl MathHelper {
    pub fn mean(values: &[f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }
        values.iter().sum::<f64>() / values.len() as f64
    }
    
    pub fn median(values: &mut [f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = values.len() / 2;
        if values.len() % 2 == 0 {
            (values[mid - 1] + values[mid]) / 2.0
        } else {
            values[mid]
        }
    }
    
    pub fn std_dev(values: &[f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }
        let mean = Self::mean(values);
        let variance = values.iter()
            .map(|&v| (v - mean).powi(2))
            .sum::<f64>() / values.len() as f64;
        variance.sqrt()
    }
    
    pub fn min(values: &[f64]) -> f64 {
        values.iter().fold(f64::INFINITY, |a, &b| a.min(b))
    }
    
    pub fn max(values: &[f64]) -> f64 {
        values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
    }
}

// Data validation helpers
pub struct Validator;

impl Validator {
    pub fn is_email(input: &str) -> bool {
        input.contains('@') && input.contains('.') && input.len() > 5
    }
    
    pub fn is_url(input: &str) -> bool {
        input.starts_with("http://") || input.starts_with("https://")
    }
    
    pub fn is_numeric(input: &str) -> bool {
        input.parse::<f64>().is_ok()
    }
    
    pub fn is_alphanumeric(input: &str) -> bool {
        input.chars().all(|c| c.is_alphanumeric())
    }
    
    pub fn has_min_length(input: &str, min: usize) -> bool {
        input.len() >= min
    }
    
    pub fn has_max_length(input: &str, max: usize) -> bool {
        input.len() <= max
    }
}

// Base64 encoding/decoding helpers
pub struct Base64Helper;

impl Base64Helper {
    const CHARS: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    pub fn encode(input: &[u8]) -> String {
        let mut result = String::new();
        let mut i = 0;
        
        while i < input.len() {
            let b1 = input[i];
            let b2 = if i + 1 < input.len() { input[i + 1] } else { 0 };
            let b3 = if i + 2 < input.len() { input[i + 2] } else { 0 };
            
            result.push(Self::CHARS[(b1 >> 2) as usize] as char);
            result.push(Self::CHARS[(((b1 & 0x03) << 4) | (b2 >> 4)) as usize] as char);
            
            if i + 1 < input.len() {
                result.push(Self::CHARS[(((b2 & 0x0f) << 2) | (b3 >> 6)) as usize] as char);
            } else {
                result.push('=');
            }
            
            if i + 2 < input.len() {
                result.push(Self::CHARS[(b3 & 0x3f) as usize] as char);
            } else {
                result.push('=');
            }
            
            i += 3;
        }
        
        result
    }
}

// Date/Time helpers
pub struct DateTimeHelper;

impl DateTimeHelper {
    pub fn is_leap_year(year: u32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
    
    pub fn days_in_month(year: u32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if Self::is_leap_year(year) { 29 } else { 28 },
            _ => 0,
        }
    }
    
    pub fn parse_iso_date(date: &str) -> Result<(u32, u32, u32), &'static str> {
        let parts: Vec<&str> = date.split('-').collect();
        if parts.len() != 3 {
            return Err("Invalid date format");
        }
        
        let year = parts[0].parse::<u32>().map_err(|_| "Invalid year")?;
        let month = parts[1].parse::<u32>().map_err(|_| "Invalid month")?;
        let day = parts[2].parse::<u32>().map_err(|_| "Invalid day")?;
        
        if month < 1 || month > 12 {
            return Err("Invalid month");
        }
        
        if day < 1 || day > Self::days_in_month(year, month) {
            return Err("Invalid day");
        }
        
        Ok((year, month, day))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_parser() {
        assert_eq!(JsonParser::parse_number("123.45"), Ok(123.45));
        assert_eq!(JsonParser::parse_boolean("true"), Ok(true));
        assert_eq!(JsonParser::escape_string("Hello \"World\""), "Hello \\\"World\\\"");
    }
    
    #[test]
    fn test_csv_parser() {
        let fields = CsvParser::parse_line("a,b,c", ',');
        assert_eq!(fields, vec!["a", "b", "c"]);
        assert_eq!(CsvParser::count_fields("1,2,3,4", ','), 4);
    }
    
    #[test]
    fn test_text_processor() {
        assert_eq!(TextProcessor::normalize_whitespace("  hello   world  "), "hello world");
        assert_eq!(TextProcessor::count_words("hello world test"), 3);
        assert_eq!(TextProcessor::count_chars("hello"), 5);
    }
    
    #[test]
    fn test_crypto_helper() {
        let hash1 = CryptoHelper::simple_hash("test");
        let hash2 = CryptoHelper::simple_hash("test");
        assert_eq!(hash1, hash2);
        
        let data = b"secret";
        let encrypted = CryptoHelper::xor_encrypt(data, 42);
        let decrypted = CryptoHelper::xor_decrypt(&encrypted, 42);
        assert_eq!(data, &decrypted[..]);
    }
    
    #[test]
    fn test_math_helper() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(MathHelper::mean(&values), 3.0);
        assert_eq!(MathHelper::min(&values), 1.0);
        assert_eq!(MathHelper::max(&values), 5.0);
    }
    
    #[test]
    fn test_validator() {
        assert!(Validator::is_email("test@example.com"));
        assert!(!Validator::is_email("invalid"));
        assert!(Validator::is_url("https://example.com"));
        assert!(Validator::is_numeric("123.45"));
        assert!(Validator::is_alphanumeric("abc123"));
    }
    
    #[test]
    fn test_base64() {
        let encoded = Base64Helper::encode(b"hello");
        assert_eq!(encoded, "aGVsbG8=");
    }
    
    #[test]
    fn test_datetime_helper() {
        assert!(DateTimeHelper::is_leap_year(2020));
        assert!(!DateTimeHelper::is_leap_year(2021));
        assert_eq!(DateTimeHelper::days_in_month(2020, 2), 29);
        assert_eq!(DateTimeHelper::days_in_month(2021, 2), 28);
        assert_eq!(DateTimeHelper::parse_iso_date("2023-12-25"), Ok((2023, 12, 25)));
    }
}
