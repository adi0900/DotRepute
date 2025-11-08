//! DotRepute Core - Reputation Scoring and Data Processing Library
//!
//! A standalone, reusable crate for decentralized reputation systems on Polkadot/Substrate.
//! 
//! # Features
//! 
//! - **Score Calculation**: Advanced reputation scoring algorithms
//! - **Data Validation**: Comprehensive data cleaning and validation
//! - **Identity Management**: Account validation and verification
//! - **Cryptographic Utilities**: Hashing, encoding, and checksums
//! - **WASM Compatible**: Compiles to WebAssembly for browser/on-chain use
//! - **no_std Support**: Works in resource-constrained environments
//! 
//! # Quick Start
//! 
//! ```rust
//! use dotrepute_core::scoring::{ScoreCalculator, MetricData};
//! 
//! let calculator = ScoreCalculator::new();
//! let data = MetricData {
//!     governance_votes: 50,
//!     staking_amount: 1000000,
//!     identity_verified: true,
//!     community_posts: 100,
//! };
//! 
//! let score = calculator.calculate(&data);
//! println!("Reputation Score: {}", score.total);
//! ```
//! 
//! # Modules
//! 
//! - [`scoring`] - Core reputation scoring algorithms
//! - [`validation`] - Data validation and cleaning utilities
//! - [`identity`] - Identity parsing and verification
//! - [`crypto`] - Cryptographic utilities
//! - [`encoding`] - Data encoding/decoding helpers
//! - [`math`] - Mathematical helper functions
//! 
//! # Feature Flags
//! 
//! - `std` (default): Enable standard library support
//! - `wasm`: Enable WebAssembly bindings
//! - `substrate`: Enable Substrate/Polkadot integration
//! - `full`: Enable all features

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{vec::Vec, string::String};

#[cfg(feature = "std")]
use std::{vec::Vec, string::String};

/// Core reputation scoring algorithms and types
pub mod scoring;

/// Data validation and cleaning utilities
pub mod validation;

/// Identity parsing and verification
pub mod identity;

/// Cryptographic utilities for hashing and encoding
pub mod crypto;

/// Data encoding and decoding helpers
pub mod encoding;

/// Mathematical helper functions
pub mod math;

/// Time-related utilities
pub mod time;

#[cfg(feature = "wasm")]
/// WebAssembly bindings for JavaScript integration
pub mod wasm;

/// Common types and traits used across modules
pub mod types {
    use super::*;
    use serde::{Deserialize, Serialize};
    use scale::{Decode, Encode};

    /// Result type used throughout the library
    pub type Result<T> = core::result::Result<T, Error>;

    /// Common error types
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, Serialize, Deserialize)]
    #[cfg_attr(feature = "substrate", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Invalid input data
        InvalidInput,
        /// Validation failed
        ValidationFailed,
        /// Score calculation error
        CalculationError,
        /// Encoding/decoding error
        EncodingError,
        /// Cryptographic operation failed
        CryptoError,
        /// Out of range value
        OutOfRange,
        /// Division by zero
        DivisionByZero,
    }

    impl core::fmt::Display for Error {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Error::InvalidInput => write!(f, "Invalid input data"),
                Error::ValidationFailed => write!(f, "Validation failed"),
                Error::CalculationError => write!(f, "Score calculation error"),
                Error::EncodingError => write!(f, "Encoding/decoding error"),
                Error::CryptoError => write!(f, "Cryptographic operation failed"),
                Error::OutOfRange => write!(f, "Value out of range"),
                Error::DivisionByZero => write!(f, "Division by zero"),
            }
        }
    }

    #[cfg(feature = "std")]
    impl std::error::Error for Error {}
}

/// Re-export commonly used types
pub use types::{Error, Result};

/// Library version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library name
pub const NAME: &str = env!("CARGO_PKG_NAME");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert_eq!(NAME, "dotrepute-core");
    }

    #[test]
    fn test_error_display() {
        let err = Error::InvalidInput;
        assert_eq!(format!("{}", err), "Invalid input data");
    }
}
