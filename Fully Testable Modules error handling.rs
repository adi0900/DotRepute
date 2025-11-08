//! Error Handling Module for Fully Testable Modules
//!
//! Provides comprehensive error types, handling mechanisms, and recovery strategies
//! for blockchain reputation systems with extensive test coverage.

use serde::{Deserialize, Serialize};
use scale::{Decode, Encode};

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{vec::Vec, string::String, boxed::Box};

#[cfg(feature = "std")]
use std::{vec::Vec, string::String, boxed::Box};

/// Comprehensive error types for the reputation system
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, Serialize, Deserialize)]
pub enum ReputationError {
    /// Invalid input data
    InvalidInput(String),
    
    /// Data validation failed
    ValidationError(String),
    
    /// Score calculation error
    CalculationError(String),
    
    /// Storage access error
    StorageError(String),
    
    /// Network communication error
    NetworkError(String),
    
    /// Cryptographic operation failed
    CryptoError(String),
    
    /// Serialization/deserialization error
    SerializationError(String),
    
    /// Insufficient permissions
    PermissionDenied(String),
    
    /// Resource not found
    NotFound(String),
    
    /// Operation timeout
    Timeout(String),
    
    /// Division by zero
    DivisionByZero,
    
    /// Arithmetic overflow
    Overflow,
    
    /// Invalid state transition
    InvalidState(String),
    
    /// External service error
    ExternalServiceError(String),
}

impl ReputationError {
    /// Create a new InvalidInput error
    pub fn invalid_input(msg: &str) -> Self {
        Self::InvalidInput(msg.to_string())
    }
    
    /// Create a new ValidationError error
    pub fn validation_error(msg: &str) -> Self {
        Self::ValidationError(msg.to_string())
    }
    
    /// Create a new CalculationError error
    pub fn calculation_error(msg: &str) -> Self {
        Self::CalculationError(msg.to_string())
    }
    
    /// Create a new StorageError error
    pub fn storage_error(msg: &str) -> Self {
        Self::StorageError(msg.to_string())
    }
    
    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            ReputationError::InvalidInput(_) => true,
            ReputationError::ValidationError(_) => true,
            ReputationError::CalculationError(_) => true,
            ReputationError::StorageError(_) => true,
            ReputationError::NetworkError(_) => true,
            ReputationError::CryptoError(_) => false,
            ReputationError::SerializationError(_) => true,
            ReputationError::PermissionDenied(_) => false,
            ReputationError::NotFound(_) => false,
            ReputationError::Timeout(_) => true,
            ReputationError::DivisionByZero => false,
            ReputationError::Overflow => false,
            ReputationError::InvalidState(_) => false,
            ReputationError::ExternalServiceError(_) => true,
        }
    }
    
    /// Get error category for logging
    pub fn category(&self) -> &'static str {
        match self {
            ReputationError::InvalidInput(_) => "INPUT",
            ReputationError::ValidationError(_) => "VALIDATION",
            ReputationError::CalculationError(_) => "CALCULATION",
            ReputationError::StorageError(_) => "STORAGE",
            ReputationError::NetworkError(_) => "NETWORK",
            ReputationError::CryptoError(_) => "CRYPTO",
            ReputationError::SerializationError(_) => "SERIALIZATION",
            ReputationError::PermissionDenied(_) => "PERMISSION",
            ReputationError::NotFound(_) => "NOT_FOUND",
            ReputationError::Timeout(_) => "TIMEOUT",
            ReputationError::DivisionByZero => "ARITHMETIC",
            ReputationError::Overflow => "ARITHMETIC",
            ReputationError::InvalidState(_) => "STATE",
            ReputationError::ExternalServiceError(_) => "EXTERNAL",
        }
    }
}

/// Result type alias for reputation operations
pub type Result<T> = core::result::Result<T, ReputationError>;

/// Error context for debugging
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, Serialize, Deserialize)]
pub struct ErrorContext {
    /// Operation that failed
    pub operation: String,
    
    /// Module where error occurred
    pub module: String,
    
    /// Timestamp of error
    pub timestamp: u64,
    
    /// Additional context data
    pub context_data: Vec<(String, String)>,
}

impl ErrorContext {
    /// Create new error context
    pub fn new(operation: &str, module: &str) -> Self {
        Self {
            operation: operation.to_string(),
            module: module.to_string(),
            timestamp: 0, // In real implementation, this would be actual timestamp
            context_data: Vec::new(),
        }
    }
    
    /// Add context data
    pub fn add_context(&mut self, key: &str, value: &str) {
        self.context_data.push((key.to_string(), value.to_string()));
    }
    
    /// Get context as formatted string
    pub fn format_context(&self) -> String {
        let mut context = format!("Operation: {}, Module: {}", self.operation, self.module);
        
        for (key, value) in &self.context_data {
            context.push_str(&format!(", {}: {}", key, value));
        }
        
        context
    }
}

/// Error recovery strategy
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, Serialize, Deserialize)]
pub enum RecoveryStrategy {
    /// Retry operation with exponential backoff
    RetryWithBackoff(u32),
    
    /// Use fallback data or service
    UseFallback,
    
    /// Skip operation and continue
    SkipOperation,
    
    /// Return default value
    ReturnDefault,
    
    /// Fail gracefully with user notification
    FailGracefully(String),
}

/// Error handler for reputation system
pub struct ErrorHandler;

impl ErrorHandler {
    /// Handle error with recovery strategy
    pub fn handle_error(error: &ReputationError, context: &ErrorContext) -> RecoveryStrategy {
        match error {
            ReputationError::InvalidInput(_) => {
                RecoveryStrategy::FailGracefully("Invalid input provided".to_string())
            },
            
            ReputationError::ValidationError(_) => {
                RecoveryStrategy::FailGracefully("Data validation failed".to_string())
            },
            
            ReputationError::CalculationError(_) => {
                RecoveryStrategy::ReturnDefault
            },
            
            ReputationError::StorageError(_) => {
                RecoveryStrategy::RetryWithBackoff(3)
            },
            
            ReputationError::NetworkError(_) => {
                RecoveryStrategy::RetryWithBackoff(5)
            },
            
            ReputationError::Timeout(_) => {
                RecoveryStrategy::RetryWithBackoff(2)
            },
            
            ReputationError::ExternalServiceError(_) => {
                RecoveryStrategy::UseFallback
            },
            
            // Non-recoverable errors
            _ => {
                RecoveryStrategy::FailGracefully("Critical error occurred".to_string())
            }
        }
    }
    
    /// Log error with context
    pub fn log_error(error: &ReputationError, context: &ErrorContext) {
        let log_message = format!(
            "[{}] {} - {} - Context: {}",
            error.category(),
            error,
            context.operation,
            context.format_context()
        );
        
        // In a real implementation, this would log to a file or service
        #[cfg(feature = "std")]
        eprintln!("ERROR: {}", log_message);
    }
    
    /// Convert error to user-friendly message
    pub fn user_message(error: &ReputationError) -> String {
        match error {
            ReputationError::InvalidInput(_) => "Please check your input and try again.".to_string(),
            ReputationError::ValidationError(_) => "Data validation failed. Please verify your data.".to_string(),
            ReputationError::CalculationError(_) => "Score calculation failed. Using default values.".to_string(),
            ReputationError::StorageError(_) => "Storage access error. Retrying...".to_string(),
            ReputationError::NetworkError(_) => "Network error. Please check your connection.".to_string(),
            ReputationError::Timeout(_) => "Operation timed out. Please try again.".to_string(),
            ReputationError::ExternalServiceError(_) => "External service unavailable. Using backup.".to_string(),
            _ => "An unexpected error occurred. Please contact support.".to_string(),
        }
    }
}

/// Mock error data for testing
pub struct MockErrorData;

impl MockErrorData {
    /// Generate invalid input error
    pub fn invalid_input() -> ReputationError {
        ReputationError::invalid_input("Test invalid input")
    }
    
    /// Generate validation error
    pub fn validation_error() -> ReputationError {
        ReputationError::validation_error("Test validation error")
    }
    
    /// Generate calculation error
    pub fn calculation_error() -> ReputationError {
        ReputationError::calculation_error("Test calculation error")
    }
    
    /// Generate storage error
    pub fn storage_error() -> ReputationError {
        ReputationError::storage_error("Test storage error")
    }
    
    /// Generate network error
    pub fn network_error() -> ReputationError {
        ReputationError::NetworkError("Test network error".to_string())
    }
    
    /// Generate timeout error
    pub fn timeout_error() -> ReputationError {
        ReputationError::Timeout("Test timeout".to_string())
    }
    
    /// Generate division by zero error
    pub fn division_by_zero() -> ReputationError {
        ReputationError::DivisionByZero
    }
    
    /// Generate overflow error
    pub fn overflow_error() -> ReputationError {
        ReputationError::Overflow
    }
}

/// Error recovery simulator for testing
pub struct ErrorRecoverySimulator;

impl ErrorRecoverySimulator {
    /// Simulate error handling and recovery
    pub fn simulate_recovery(error: &ReputationError) -> (RecoveryStrategy, String) {
        let context = ErrorContext::new("test_operation", "test_module");
        let strategy = ErrorHandler::handle_error(error, &context);
        let user_message = ErrorHandler::user_message(error);
        
        (strategy, user_message)
    }
    
    /// Test all error types with recovery
    pub fn test_all_errors() -> Vec<(ReputationError, RecoveryStrategy, String)> {
        let errors = vec![
            MockErrorData::invalid_input(),
            MockErrorData::validation_error(),
            MockErrorData::calculation_error(),
            MockErrorData::storage_error(),
            MockErrorData::network_error(),
            MockErrorData::timeout_error(),
            MockErrorData::division_by_zero(),
            MockErrorData::overflow_error(),
        ];
        
        errors.into_iter()
            .map(|error| {
                let (strategy, message) = Self::simulate_recovery(&error);
                (error, strategy, message)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = ReputationError::invalid_input("test input");
        assert_eq!(error.category(), "INPUT");
        
        let error = ReputationError::validation_error("test validation");
        assert_eq!(error.category(), "VALIDATION");
        
        let error = ReputationError::calculation_error("test calculation");
        assert_eq!(error.category(), "CALCULATION");
    }

    #[test]
    fn test_error_recoverability() {
        let recoverable_error = ReputationError::invalid_input("test");
        assert!(recoverable_error.is_recoverable());
        
        let non_recoverable_error = ReputationError::DivisionByZero;
        assert!(!non_recoverable_error.is_recoverable());
        
        let recoverable_error = ReputationError::StorageError("test".to_string());
        assert!(recoverable_error.is_recoverable());
    }

    #[test]
    fn test_error_context() {
        let mut context = ErrorContext::new("test_operation", "test_module");
        context.add_context("param1", "value1");
        context.add_context("param2", "value2");
        
        let formatted = context.format_context();
        assert!(formatted.contains("test_operation"));
        assert!(formatted.contains("test_module"));
        assert!(formatted.contains("param1"));
        assert!(formatted.contains("value1"));
    }

    #[test]
    fn test_error_handler_recovery() {
        let error = MockErrorData::storage_error();
        let context = ErrorContext::new("test", "test");
        let strategy = ErrorHandler::handle_error(&error, &context);
        
        match strategy {
            RecoveryStrategy::RetryWithBackoff(count) => assert_eq!(count, 3),
            _ => panic!("Expected retry strategy"),
        }
        
        let error = MockErrorData::network_error();
        let strategy = ErrorHandler::handle_error(&error, &context);
        
        match strategy {
            RecoveryStrategy::RetryWithBackoff(count) => assert_eq!(count, 5),
            _ => panic!("Expected retry strategy"),
        }
    }

    #[test]
    fn test_user_messages() {
        let error = MockErrorData::invalid_input();
        let message = ErrorHandler::user_message(&error);
        assert!(!message.is_empty());
        assert!(message.contains("input"));
        
        let error = MockErrorData::validation_error();
        let message = ErrorHandler::user_message(&error);
        assert!(message.contains("validation"));
        
        let error = MockErrorData::division_by_zero();
        let message = ErrorHandler::user_message(&error);
        assert!(message.contains("unexpected"));
    }

    #[test]
    fn test_mock_error_data() {
        assert!(matches!(MockErrorData::invalid_input(), ReputationError::InvalidInput(_)));
        assert!(matches!(MockErrorData::validation_error(), ReputationError::ValidationError(_)));
        assert!(matches!(MockErrorData::calculation_error(), ReputationError::CalculationError(_)));
        assert!(matches!(MockErrorData::storage_error(), ReputationError::StorageError(_)));
        assert!(matches!(MockErrorData::network_error(), ReputationError::NetworkError(_)));
        assert!(matches!(MockErrorData::timeout_error(), ReputationError::Timeout(_)));
        assert_eq!(MockErrorData::division_by_zero(), ReputationError::DivisionByZero);
        assert_eq!(MockErrorData::overflow_error(), ReputationError::Overflow);
    }

    #[test]
    fn test_error_recovery_simulator() {
        let error = MockErrorData::storage_error();
        let (strategy, message) = ErrorRecoverySimulator::simulate_recovery(&error);
        
        match strategy {
            RecoveryStrategy::RetryWithBackoff(count) => assert_eq!(count, 3),
            _ => panic!("Expected retry strategy"),
        }
        
        assert!(!message.is_empty());
    }

    #[test]
    fn test_all_errors_simulation() {
        let results = ErrorRecoverySimulator::test_all_errors();
        assert_eq!(results.len(), 8); // We have 8 different error types
        
        for (error, strategy, message) in results {
            assert!(!matches!(error, ReputationError::PermissionDenied(_))); // We didn't create this one
            assert!(!message.is_empty());
            
            // Verify strategies make sense for error types
            match error {
                ReputationError::InvalidInput(_) => {
                    assert!(matches!(strategy, RecoveryStrategy::FailGracefully(_)));
                },
                ReputationError::StorageError(_) => {
                    assert!(matches!(strategy, RecoveryStrategy::RetryWithBackoff(_)));
                },
                _ => {} // Other strategies are acceptable
            }
        }
    }

    #[test]
    fn test_result_type_alias() {
        fn test_function() -> Result<u32> {
            Ok(42)
        }
        
        let result = test_function();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        
        fn error_function() -> Result<u32> {
            Err(ReputationError::invalid_input("test"))
        }
        
        let result = error_function();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ReputationError::InvalidInput(_)));
    }

    #[test]
    fn test_error_equality() {
        let error1 = ReputationError::invalid_input("same message");
        let error2 = ReputationError::invalid_input("same message");
        let error3 = ReputationError::invalid_input("different message");
        
        assert_eq!(error1, error2);
        assert_ne!(error1, error3);
        
        let error4 = ReputationError::DivisionByZero;
        let error5 = ReputationError::DivisionByZero;
        let error6 = ReputationError::Overflow;
        
        assert_eq!(error4, error5);
        assert_ne!(error4, error6);
    }

    #[test]
    fn test_recovery_strategy_variants() {
        let strategies = vec![
            RecoveryStrategy::RetryWithBackoff(3),
            RecoveryStrategy::UseFallback,
            RecoveryStrategy::SkipOperation,
            RecoveryStrategy::ReturnDefault,
            RecoveryStrategy::FailGracefully("test".to_string()),
        ];
        
        assert_eq!(strategies.len(), 5);
        
        match &strategies[0] {
            RecoveryStrategy::RetryWithBackoff(count) => assert_eq!(*count, 3),
            _ => panic!("Expected retry strategy"),
        }
        
        match &strategies[4] {
            RecoveryStrategy::FailGracefully(msg) => assert_eq!(msg, "test"),
            _ => panic!("Expected fail gracefully strategy"),
        }
    }

    #[test]
    fn test_edge_cases() {
        // Test error with empty string
        let error = ReputationError::invalid_input("");
        assert_eq!(error.category(), "INPUT");
        
        // Test context with empty strings
        let context = ErrorContext::new("", "");
        assert_eq!(context.operation, "");
        assert_eq!(context.module, "");
        
        // Test formatting empty context
        let formatted = context.format_context();
        assert!(formatted.contains("Operation: "));
        assert!(formatted.contains("Module: "));
        
        // Test error handler with empty context
        let strategy = ErrorHandler::handle_error(&error, &context);
        match strategy {
            RecoveryStrategy::FailGracefully(_) => {}, // Expected
            _ => panic!("Unexpected strategy"),
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod integration_tests {
    use super::*;

    #[test]
    fn integration_test_error_handling_workflow() {
        // Test complete error handling workflow
        let errors_to_test = vec![
            MockErrorData::invalid_input(),
            MockErrorData::storage_error(),
            MockErrorData::network_error(),
            MockErrorData::division_by_zero(),
        ];
        
        for error in errors_to_test {
            // Create context
            let mut context = ErrorContext::new("integration_test", "error_handling");
            context.add_context("test_id", "workflow_test");
            
            // Handle error
            let strategy = ErrorHandler::handle_error(&error, &context);
            
            // Log error
            ErrorHandler::log_error(&error, &context);
            
            // Generate user message
            let user_message = ErrorHandler::user_message(&error);
            
            // Verify all steps worked
            assert!(!matches!(strategy, RecoveryStrategy::RetryWithBackoff(0))); // Should have valid retry count if retry strategy
            assert!(!user_message.is_empty());
            
            // Test specific behaviors
            match error {
                ReputationError::InvalidInput(_) => {
                    assert!(matches!(strategy, RecoveryStrategy::FailGracefully(_)));
                    assert!(user_message.contains("input"));
                },
                ReputationError::StorageError(_) => {
                    assert!(matches!(strategy, RecoveryStrategy::RetryWithBackoff(_)));
                    assert!(user_message.contains("storage"));
                },
                ReputationError::NetworkError(_) => {
                    assert!(matches!(strategy, RecoveryStrategy::RetryWithBackoff(_)));
                    assert!(user_message.contains("network"));
                },
                ReputationError::DivisionByZero => {
                    assert!(matches!(strategy, RecoveryStrategy::FailGracefully(_)));
                    assert!(user_message.contains("unexpected"));
                },
                _ => {} // Other cases are fine
            }
        }
    }

    #[test]
    fn integration_test_recovery_simulation() {
        // Test error recovery simulation with all error types
        let simulation_results = ErrorRecoverySimulator::test_all_errors();
        
        // Verify we got results for all error types
        assert_eq!(simulation_results.len(), 8);
        
        // Check that each result has valid components
        for (error, strategy, message) in simulation_results {
            // All errors should be valid variants
            match error {
                ReputationError::InvalidInput(_) | 
                ReputationError::ValidationError(_) | 
                ReputationError::CalculationError(_) | 
                ReputationError::StorageError(_) | 
                ReputationError::NetworkError(_) | 
                ReputationError::Timeout(_) | 
                ReputationError::DivisionByZero | 
                ReputationError::Overflow => {
                    // Valid error type
                },
                _ => panic!("Unexpected error type in simulation"),
            }
            
            // All strategies should be valid variants
            match strategy {
                RecoveryStrategy::RetryWithBackoff(_) | 
                RecoveryStrategy::UseFallback | 
                RecoveryStrategy::SkipOperation | 
                RecoveryStrategy::ReturnDefault | 
                RecoveryStrategy::FailGracefully(_) => {
                    // Valid strategy
                }
            }
            
            // All messages should be non-empty
            assert!(!message.is_empty());
        }
    }

    #[test]
    fn integration_test_error_context_enhancement() {
        // Test enhanced error context functionality
        let mut context = ErrorContext::new("complex_operation", "enhanced_module");
        
        // Add multiple context items
        context.add_context("user_id", "12345");
        context.add_context("session_id", "abcde-12345");
        context.add_context("request_id", "req-001");
        context.add_context("timestamp", "1234567890");
        
        // Format context
        let formatted = context.format_context();
        
        // Verify all context items are included
        assert!(formatted.contains("complex_operation"));
        assert!(formatted.contains("enhanced_module"));
        assert!(formatted.contains("user_id"));
        assert!(formatted.contains("12345"));
        assert!(formatted.contains("session_id"));
        assert!(formatted.contains("abcde-12345"));
        assert!(formatted.contains("request_id"));
        assert!(formatted.contains("req-001"));
        assert!(formatted.contains("timestamp"));
        assert!(formatted.contains("1234567890"));
        
        // Test with error handling
        let error = MockErrorData::validation_error();
        let strategy = ErrorHandler::handle_error(&error, &context);
        let user_message = ErrorHandler::user_message(&error);
        
        // Verify error handling still works with enhanced context
        assert!(matches!(strategy, RecoveryStrategy::FailGracefully(_)));
        assert!(!user_message.is_empty());
        assert!(user_message.contains("validation"));
    }
}
