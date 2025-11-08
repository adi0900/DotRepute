/! WASM Library - Public API for JavaScript Integration
//! 
//! Exports high-performance utility functions for browser and serverless environments.
//! Compiled to WebAssembly for near-native performance in web applications.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(not(feature = "std"))]
use alloc::{vec::Vec, string::String};

#[cfg(feature = "std")]
use std::{vec::Vec, string::String};

pub mod score_normalizer;
pub mod data_cleaner;
pub mod identity_parser;
pub mod crypto_utils;
pub mod math_helpers;
pub mod time_utils;
pub mod encoding;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn normalize_score_js(value: f64, min: f64, max: f64) -> f64 {
    let config = score_normalizer::NormalizationConfig {
        min_value: min,
        max_value: max,
        scale_factor: 1.0,
    };
    score_normalizer::normalize_score(value, &config)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn clean_text_js(input: String) -> String {
    data_cleaner::normalize_text(&input)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn validate_account_js(account_id: String) -> bool {
    identity_parser::validate_identity_format(&account_id)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn hash_data_js(input: Vec<u8>) -> Vec<u8> {
    crypto_utils::simple_hash(&input).to_vec()
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn calculate_pow_js(base: u32, exp: u32) -> u32 {
    math_helpers::fast_pow(base as u64, exp) as u32
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn days_to_seconds_js(days: u32) -> u32 {
    time_utils::days_to_seconds(days as u64) as u32
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn hex_encode_js(bytes: Vec<u8>) -> String {
    encoding::hex_encode(&bytes)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn hex_decode_js(hex: String) -> Result<Vec<u8>, JsValue> {
    encoding::hex_decode(&hex)
        .map_err(|e| JsValue::from_str(e))
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn calculate_checksum_js(data: Vec<u8>) -> u32 {
    crypto_utils::checksum(&data)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn sqrt_js(n: u32) -> u32 {
    math_helpers::integer_sqrt(n as u64) as u32
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn gcd_js(a: u32, b: u32) -> u32 {
    math_helpers::gcd(a as u64, b as u64) as u32
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct WasmScoreNormalizer {
    min_value: f64,
    max_value: f64,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl WasmScoreNormalizer {
    #[wasm_bindgen(constructor)]
    pub fn new(min: f64, max: f64) -> Self {
        Self {
            min_value: min,
            max_value: max,
        }
    }

    pub fn normalize(&self, value: f64) -> f64 {
        let config = score_normalizer::NormalizationConfig {
            min_value: self.min_value,
            max_value: self.max_value,
            scale_factor: 1.0,
        };
        score_normalizer::normalize_score(value, &config)
    }

    pub fn denormalize(&self, value: f64) -> f64 {
        let config = score_normalizer::NormalizationConfig {
            min_value: self.min_value,
            max_value: self.max_value,
            scale_factor: 1.0,
        };
        score_normalizer::denormalize_score(value, &config)
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct WasmCryptoHelper;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl WasmCryptoHelper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self
    }

    pub fn hash(&self, data: Vec<u8>) -> Vec<u8> {
        crypto_utils::simple_hash(&data).to_vec()
    }

    pub fn encrypt(&self, data: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
        crypto_utils::xor_encrypt(&data, &key)
    }

    pub fn decrypt(&self, data: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
        crypto_utils::xor_decrypt(&data, &key)
    }

    pub fn checksum(&self, data: Vec<u8>) -> u32 {
        crypto_utils::checksum(&data)
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    #[test]
    fn test_library_integration() {
        let config = score_normalizer::NormalizationConfig::default();
        let normalized = score_normalizer::normalize_score(50.0, &config);
        assert_eq!(normalized, 0.5);
    }

    #[test]
    fn test_data_cleaning() {
        let cleaned = data_cleaner::normalize_text("  TEST  ");
        assert_eq!(cleaned, "test");
    }

    #[test]
    fn test_crypto_hash() {
        let data = b"test data";
        let hash = crypto_utils::simple_hash(data);
        assert_eq!(hash.len(), 32);
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_wasm_normalize() {
        let result = normalize_score_js(50.0, 0.0, 100.0);
        assert_eq!(result, 0.5);
    }

    #[wasm_bindgen_test]
    fn test_wasm_clean_text() {
        let result = clean_text_js("  Hello  ".to_string());
        assert_eq!(result, "hello");
    }

    #[wasm_bindgen_test]
    fn test_wasm_hex_encode() {
        let data = vec![0xde, 0xad, 0xbe, 0xef];
        let result = hex_encode_js(data);
        assert_eq!(result, "deadbeef");
    }
}
