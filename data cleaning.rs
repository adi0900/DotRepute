//! Data Cleaning Engine Module
//!
//! This module implements a comprehensive data cleaning system with validation,
//! type checking, missing data handling, inconsistency resolution, and anomaly detection.
//!
//! Features:
//! - Validation and type checking: Data type verification (date, number, text, etc.)
//! - Missing data management: Fill with average, median, mode, or default values
//! - Inconsistency resolution: Handle duplicates, case normalization, formatting
//! - Format conversions: Standardize dates, phone numbers, ID numbers
//! - Anomaly detection: Identify outliers and logical contradictions
//! - Post-cleaning reporting: Statistics on cleaned, deleted, corrected records

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Data types
#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Text,
    Number,
    Date,
    Email,
    Phone,
    IdNumber,
    Boolean,
}

// Validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
    
    pub fn add_error(&mut self, error: String) {
        self.is_valid = false;
        self.errors.push(error);
    }
    
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
}

// Data record
#[derive(Debug, Clone)]
pub struct DataRecord {
    pub id: u32,
    pub fields: HashMap<String, String>,
    pub metadata: HashMap<String, String>,
}

// Cleaning statistics
#[derive(Debug, Clone)]
pub struct CleaningStats {
    pub total_records: u32,
    pub cleaned_records: u32,
    pub deleted_records: u32,
    pub corrected_fields: u32,
    pub filled_fields: u32,
    pub duplicate_records: u32,
    pub anomalies_detected: u32,
    pub field_stats: HashMap<String, FieldStats>,
}

impl CleaningStats {
    pub fn new() -> Self {
        Self {
            total_records: 0,
            cleaned_records: 0,
            deleted_records: 0,
            corrected_fields: 0,
            filled_fields: 0,
            duplicate_records: 0,
            anomalies_detected: 0,
            field_stats: HashMap::new(),
        }
    }
}

// Field statistics
#[derive(Debug, Clone)]
pub struct FieldStats {
    pub field_name: String,
    pub total_values: u32,
    pub missing_values: u32,
    pub corrected_values: u32,
    pub filled_values: u32,
    pub invalid_values: u32,
}

impl FieldStats {
    pub fn new(field_name: String) -> Self {
        Self {
            field_name,
            total_values: 0,
            missing_values: 0,
            corrected_values: 0,
            filled_values: 0,
            invalid_values: 0,
        }
    }
}

// Data cleaning engine
pub struct DataCleaningEngine {
    pub records: Vec<DataRecord>,
    pub field_types: HashMap<String, DataType>,
    pub stats: CleaningStats,
}

impl DataCleaningEngine {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            field_types: HashMap::new(),
            stats: CleaningStats::new(),
        }
    }
    
    pub fn add_record(&mut self, record: DataRecord) {
        self.records.push(record);
        self.stats.total_records += 1;
    }
    
    pub fn set_field_type(&mut self, field_name: String, data_type: DataType) {
        self.field_types.insert(field_name, data_type);
    }
    
    // Validation and type checking
    pub fn validate_field(&self, field_name: &str, value: &str) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        if let Some(data_type) = self.field_types.get(field_name) {
            match data_type {
                DataType::Email => {
                    if !self.is_valid_email(value) {
                        result.add_error(format!("Invalid email format: {}", value));
                    }
                },
                DataType::Phone => {
                    if !self.is_valid_phone(value) {
                        result.add_error(format!("Invalid phone format: {}", value));
                    }
                },
                DataType::IdNumber => {
                    if !self.is_valid_id_number(value) {
                        result.add_error(format!("Invalid ID number: {}", value));
                    }
                },
                DataType::Number => {
                    if value.parse::<f64>().is_err() {
                        result.add_error(format!("Invalid number format: {}", value));
                    }
                },
                DataType::Date => {
                    if !self.is_valid_date(value) {
                        result.add_error(format!("Invalid date format: {}", value));
                    }
                },
                DataType::Boolean => {
                    if !["true", "false", "0", "1", "yes", "no"].contains(&value.to_lowercase().as_str()) {
                        result.add_error(format!("Invalid boolean value: {}", value));
                    }
                },
                _ => {},
            }
        }
        
        result
    }
    
    fn is_valid_email(&self, email: &str) -> bool {
        email.contains('@') && email.contains('.') && email.len() > 5
    }
    
    fn is_valid_phone(&self, phone: &str) -> bool {
        let digits: String = phone.chars().filter(|c| c.is_digit(10)).collect();
        digits.len() >= 10 && digits.len() <= 15
    }
    
    fn is_valid_id_number(&self, id: &str) -> bool {
        let digits: String = id.chars().filter(|c| c.is_digit(10)).collect();
        // Support various national ID formats (7-20 digits)
        digits.len() >= 7 && digits.len() <= 20
    }
    
    fn is_valid_date(&self, date: &str) -> bool {
        // Simple date validation (supports multiple formats)
        let parts: Vec<&str> = date.split(&['-', '/', '.'][..]).collect();
        parts.len() == 3 && parts.iter().all(|p| p.parse::<u32>().is_ok())
    }
    
    // Missing data management
    pub fn fill_missing_values(&mut self, field_name: &str, strategy: FillStrategy) {
        let values: Vec<f64> = self.records
            .iter()
            .filter_map(|r| r.fields.get(field_name))
            .filter(|v| !v.is_empty())
            .filter_map(|v| v.parse::<f64>().ok())
            .collect();
        
        if values.is_empty() {
            return;
        }
        
        let fill_value = match strategy {
            FillStrategy::Mean => {
                let sum: f64 = values.iter().sum();
                sum / values.len() as f64
            },
            FillStrategy::Median => {
                let mut sorted = values.clone();
                sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
                if sorted.len() % 2 == 0 {
                    (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2.0
                } else {
                    sorted[sorted.len() / 2]
                }
            },
            FillStrategy::Mode => {
                let mut counts: HashMap<String, u32> = HashMap::new();
                for record in &self.records {
                    if let Some(value) = record.fields.get(field_name) {
                        if !value.is_empty() {
                            *counts.entry(value.clone()).or_insert(0) += 1;
                        }
                    }
                }
                
                if let Some((mode_value, _)) = counts.iter().max_by_key(|(_, count)| *count) {
                    if let Ok(val) = mode_value.parse::<f64>() {
                        val
                    } else {
                        return;
                    }
                } else {
                    return;
                }
            },
            FillStrategy::Default(val) => val,
        };
        
        for record in &mut self.records {
            if let Some(value) = record.fields.get_mut(field_name) {
                if value.is_empty() {
                    *value = fill_value.to_string();
                    self.stats.filled_fields += 1;
                }
            }
        }
    }
    
    // Inconsistency resolution
    pub fn normalize_text_field(&mut self, field_name: &str) {
        for record in &mut self.records {
            if let Some(value) = record.fields.get_mut(field_name) {
                let normalized = self.normalize_text(value);
                if *value != normalized {
                    *value = normalized;
                    self.stats.corrected_fields += 1;
                }
            }
        }
    }
    
    fn normalize_text(&self, text: &str) -> String {
        // Trim whitespace
        let mut normalized = text.trim().to_string();
        
        // Remove multiple spaces
        while normalized.contains("  ") {
            normalized = normalized.replace("  ", " ");
        }
        
        // Capitalize first letter of each word
        normalized = normalized
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<String>>()
            .join(" ");
        
        normalized
    }
    
    // Remove duplicates
    pub fn remove_duplicates(&mut self, key_fields: Vec<String>) {
        let mut seen: HashMap<String, u32> = HashMap::new();
        let mut to_remove: Vec<usize> = Vec::new();
        
        for (index, record) in self.records.iter().enumerate() {
            let key: String = key_fields
                .iter()
                .filter_map(|field| record.fields.get(field))
                .cloned()
                .collect::<Vec<String>>()
                .join("|");
            
            if let Some(_) = seen.get(&key) {
                to_remove.push(index);
                self.stats.duplicate_records += 1;
            } else {
                seen.insert(key, 1);
            }
        }
        
        // Remove duplicates in reverse order to maintain indices
        for index in to_remove.iter().rev() {
            self.records.remove(*index);
            self.stats.deleted_records += 1;
        }
    }
    
    // Format conversions
    pub fn standardize_date_format(&mut self, field_name: &str) {
        for record in &mut self.records {
            if let Some(value) = record.fields.get_mut(field_name) {
                if let Some(standardized) = self.convert_to_iso_date(value) {
                    if *value != standardized {
                        *value = standardized;
                        self.stats.corrected_fields += 1;
                    }
                }
            }
        }
    }
    
    fn convert_to_iso_date(&self, date: &str) -> Option<String> {
        let parts: Vec<&str> = date.split(&['-', '/', '.'][..]).collect();
        
        if parts.len() != 3 {
            return None;
        }
        
        let (year, month, day) = if parts[0].len() == 4 {
            // Already in YYYY-MM-DD format
            (parts[0], parts[1], parts[2])
        } else if parts[2].len() == 4 {
            // DD-MM-YYYY format
            (parts[2], parts[1], parts[0])
        } else {
            return None;
        };
        
        Some(format!("{}-{:02}-{:02}", 
            year, 
            month.parse::<u32>().ok()?, 
            day.parse::<u32>().ok()?))
    }
    
    pub fn standardize_phone_format(&mut self, field_name: &str) {
        for record in &mut self.records {
            if let Some(value) = record.fields.get_mut(field_name) {
                let standardized = self.format_phone(value);
                if *value != standardized {
                    *value = standardized;
                    self.stats.corrected_fields += 1;
                }
            }
        }
    }
    
    fn format_phone(&self, phone: &str) -> String {
        let digits: String = phone.chars().filter(|c| c.is_digit(10)).collect();
        
        if digits.len() == 10 {
            format!("0{} {} {} {}", 
                &digits[0..3], 
                &digits[3..6], 
                &digits[6..8], 
                &digits[8..10])
        } else if digits.len() == 11 && digits.starts_with('0') {
            format!("{} {} {} {}", 
                &digits[0..4], 
                &digits[4..7], 
                &digits[7..9], 
                &digits[9..11])
        } else {
            phone.to_string()
        }
    }
    
    // Anomaly detection
    pub fn detect_anomalies(&mut self, field_name: &str, min_value: f64, max_value: f64) -> Vec<u32> {
        let mut anomalies = Vec::new();
        
        for record in &self.records {
            if let Some(value) = record.fields.get(field_name) {
                if let Ok(num) = value.parse::<f64>() {
                    if num < min_value || num > max_value {
                        anomalies.push(record.id);
                        self.stats.anomalies_detected += 1;
                    }
                }
            }
        }
        
        anomalies
    }
    
    pub fn detect_logical_contradictions(&mut self) -> Vec<String> {
        let mut contradictions = Vec::new();
        
        for record in &self.records {
            // Check birth date vs registration date
            if let (Some(birth_date), Some(reg_date)) = 
                (record.fields.get("birth_date"), record.fields.get("registration_date")) {
                if birth_date > reg_date {
                    contradictions.push(format!(
                        "Record {}: Birth date ({}) is after registration date ({})",
                        record.id, birth_date, reg_date
                    ));
                    self.stats.anomalies_detected += 1;
                }
            }
            
            // Check age consistency
            if let Some(age) = record.fields.get("age") {
                if let Ok(age_val) = age.parse::<u32>() {
                    if age_val > 120 {
                        contradictions.push(format!(
                            "Record {}: Age ({}) exceeds 120",
                            record.id, age_val
                        ));
                        self.stats.anomalies_detected += 1;
                    }
                }
            }
        }
        
        contradictions
    }
    
    // Cleaning report
    pub fn generate_report(&self) -> CleaningReport {
        CleaningReport {
            stats: self.stats.clone(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
        }
    }
    
    pub fn clean_all(&mut self) {
        self.stats.cleaned_records = self.records.len() as u32;
    }
}

// Fill strategy for missing values
#[derive(Debug, Clone)]
pub enum FillStrategy {
    Mean,
    Median,
    Mode,
    Default(f64),
}

// Cleaning report
#[derive(Debug, Clone)]
pub struct CleaningReport {
    pub stats: CleaningStats,
    pub timestamp: u64,
}

impl CleaningReport {
    pub fn to_string(&self) -> String {
        format!(
            "Data Cleaning Report\n\
             ==================\n\
             Total Records: {}\n\
             Cleaned Records: {}\n\
             Deleted Records: {}\n\
             Corrected Fields: {}\n\
             Filled Fields: {}\n\
             Duplicate Records: {}\n\
             Anomalies Detected: {}\n\
             Timestamp: {}",
            self.stats.total_records,
            self.stats.cleaned_records,
            self.stats.deleted_records,
            self.stats.corrected_fields,
            self.stats.filled_fields,
            self.stats.duplicate_records,
            self.stats.anomalies_detected,
            self.timestamp
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_validation() {
        let engine = DataCleaningEngine::new();
        
        assert!(engine.is_valid_email("test@example.com"));
        assert!(!engine.is_valid_email("invalid-email"));
        assert!(!engine.is_valid_email("test@"));
    }
    
    #[test]
    fn test_phone_validation() {
        let engine = DataCleaningEngine::new();
        
        assert!(engine.is_valid_phone("05551234567"));
        assert!(engine.is_valid_phone("0555 123 45 67"));
        assert!(!engine.is_valid_phone("123"));
    }
    
    #[test]
    fn test_id_number_validation() {
        let engine = DataCleaningEngine::new();
        
        // Turkish ID (11 digits)
        assert!(engine.is_valid_id_number("12345678901"));
        // US SSN (9 digits)
        assert!(engine.is_valid_id_number("123456789"));
        // UK National Insurance (various formats)
        assert!(engine.is_valid_id_number("1234567890"));
        // Too short
        assert!(!engine.is_valid_id_number("123"));
        // Too long
        assert!(!engine.is_valid_id_number("123456789012345678901"));
    }
    
    #[test]
    fn test_date_validation() {
        let engine = DataCleaningEngine::new();
        
        assert!(engine.is_valid_date("2023-12-25"));
        assert!(engine.is_valid_date("25/12/2023"));
        assert!(engine.is_valid_date("25.12.2023"));
        assert!(!engine.is_valid_date("invalid-date"));
    }
    
    #[test]
    fn test_text_normalization() {
        let engine = DataCleaningEngine::new();
        
        assert_eq!(engine.normalize_text("  ahmet   yılmaz  "), "Ahmet Yılmaz");
        assert_eq!(engine.normalize_text("AHMET YILMAZ"), "Ahmet Yılmaz");
    }
    
    #[test]
    fn test_date_format_conversion() {
        let engine = DataCleaningEngine::new();
        
        assert_eq!(engine.convert_to_iso_date("25-12-2023"), Some("2023-12-25".to_string()));
        assert_eq!(engine.convert_to_iso_date("2023-12-25"), Some("2023-12-25".to_string()));
    }
    
    #[test]
    fn test_phone_format() {
        let engine = DataCleaningEngine::new();
        
        assert_eq!(engine.format_phone("5551234567"), "0555 123 45 67");
        assert_eq!(engine.format_phone("05551234567"), "0555 123 45 67");
    }
    
    #[test]
    fn test_fill_missing_values() {
        let mut engine = DataCleaningEngine::new();
        
        let mut record1 = DataRecord {
            id: 1,
            fields: HashMap::new(),
            metadata: HashMap::new(),
        };
        record1.fields.insert("age".to_string(), "25".to_string());
        
        let mut record2 = DataRecord {
            id: 2,
            fields: HashMap::new(),
            metadata: HashMap::new(),
        };
        record2.fields.insert("age".to_string(), "30".to_string());
        
        let mut record3 = DataRecord {
            id: 3,
            fields: HashMap::new(),
            metadata: HashMap::new(),
        };
        record3.fields.insert("age".to_string(), "".to_string());
        
        engine.add_record(record1);
        engine.add_record(record2);
        engine.add_record(record3);
        
        engine.fill_missing_values("age", FillStrategy::Mean);
        
        assert_eq!(engine.stats.filled_fields, 1);
    }
    
    #[test]
    fn test_remove_duplicates() {
        let mut engine = DataCleaningEngine::new();
        
        let mut record1 = DataRecord {
            id: 1,
            fields: HashMap::new(),
            metadata: HashMap::new(),
        };
        record1.fields.insert("name".to_string(), "Ahmet".to_string());
        
        let mut record2 = DataRecord {
            id: 2,
            fields: HashMap::new(),
            metadata: HashMap::new(),
        };
        record2.fields.insert("name".to_string(), "Ahmet".to_string());
        
        engine.add_record(record1);
        engine.add_record(record2);
        
        engine.remove_duplicates(vec!["name".to_string()]);
        
        assert_eq!(engine.records.len(), 1);
        assert_eq!(engine.stats.duplicate_records, 1);
    }
    
    #[test]
    fn test_anomaly_detection() {
        let mut engine = DataCleaningEngine::new();
        
        let mut record1 = DataRecord {
            id: 1,
            fields: HashMap::new(),
            metadata: HashMap::new(),
        };
        record1.fields.insert("age".to_string(), "150".to_string());
        
        engine.add_record(record1);
        
        let anomalies = engine.detect_anomalies("age", 0.0, 120.0);
        
        assert_eq!(anomalies.len(), 1);
        assert_eq!(engine.stats.anomalies_detected, 1);
    }
}
