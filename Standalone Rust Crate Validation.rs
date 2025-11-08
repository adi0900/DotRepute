//! Data validation and cleaning utilities
//!
//! Provides comprehensive validation and cleaning functions for reputation data.

use crate::{Error, Result};

#[cfg(not(feature = "std"))]
use alloc::{vec::Vec, string::String};

/// Validate numeric value is within range
pub fn validate_range(value: u64, min: u64, max: u64) -> Result<()> {
    if value < min || value > max {
        return Err(Error::OutOfRange);
    }
    Ok(())
}

/// Clean and normalize text
pub fn normalize_text(input: &str) -> String {
    input.trim().to_lowercase()
}

/// Remove duplicates from a vector
pub fn remove_duplicates<T: PartialEq + Clone>(items: &[T]) -> Vec<T> {
    let mut result = Vec::new();
    for item in items {
        if !result.contains(item) {
            result.push(item.clone());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_range() {
        assert!(validate_range(50, 0, 100).is_ok());
        assert!(validate_range(150, 0, 100).is_err());
    }

    #[test]
    fn test_normalize_text() {
        assert_eq!(normalize_text("  Hello World  "), "hello world");
    }
}
