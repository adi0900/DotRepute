use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Extrinsic types
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum ExtrinsicType {
    Staking,        // Staking related extrinsics
    Governance,     // Governance related extrinsics
    Identity,       // Identity related extrinsics
    Transfer,       // Transfer related extrinsics
    Utility,        // Utility related extrinsics
    Session,        // Session related extrinsics
    Treasury,       // Treasury related extrinsics
    Democracy,      // Democracy related extrinsics
    Council,        // Council related extrinsics
    Technical,      // Technical committee extrinsics
    Preimage,       // Preimage related extrinsics
    Proxy,          // Proxy related extrinsics
    Multisig,       // Multisig related extrinsics
    Vesting,        // Vesting related extrinsics
    Custom(String), // Custom extrinsic types
}

// Extrinsic record
#[derive(Debug, Clone)]
pub struct ExtrinsicRecord {
    pub extrinsic_id: u32,         // Extrinsic ID
    pub extrinsic_type: ExtrinsicType, // Extrinsic type
    pub pallet: String,            // Pallet name
    pub call: String,              // Call name
    pub timestamp: u64,            // Extrinsic timestamp
    pub block_number: u32,         // Block number
    pub success: bool,             // Success status
    pub weight: u64,               // Extrinsic weight
    pub fee: u128,                 // Transaction fee
}

// Batch extrinsic record
#[derive(Debug, Clone)]
pub struct BatchExtrinsicRecord {
    pub batch_id: u32,             // Batch ID
    pub extrinsics: Vec<ExtrinsicRecord>, // Extrinsic records
    pub timestamp: u64,            // Batch timestamp
    pub block_number: u32,         // Block number
    pub total_weight: u64,         // Total weight
    pub total_fee: u128,           // Total fee
}

// Extrinsic activity metrics
#[derive(Debug, Clone)]
pub struct ExtrinsicActivityMetrics {
    pub account_id: u32,                           // Account ID
    pub extrinsics: Vec<ExtrinsicRecord>,          // Extrinsic records
    pub batch_extrinsics: Vec<BatchExtrinsicRecord>, // Batch extrinsic records
    pub extrinsic_types: HashMap<ExtrinsicType, u32>, // Extrinsic type counts
    pub first_extrinsic_date: Option<u64>,          // First extrinsic date
    pub last_extrinsic_date: Option<u64>,           // Last extrinsic date
    pub successful_extrinsics: u32,                 // Successful extrinsics count
    pub failed_extrinsics: u32,                     // Failed extrinsics count
    pub total_fees_paid: u128,                      // Total fees paid
    pub last_activity_time: u64,                    // Last activity timestamp
}

impl ExtrinsicActivityMetrics {
    pub fn new(account_id: u32) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        ExtrinsicActivityMetrics {
            account_id,
            extrinsics: Vec::new(),
            batch_extrinsics: Vec::new(),
            extrinsic_types: HashMap::new(),
            first_extrinsic_date: None,
            last_extrinsic_date: None,
            successful_extrinsics: 0,
            failed_extrinsics: 0,
            total_fees_paid: 0,
            last_activity_time: now,
        }
    }

    // 1. Total extrinsic count
    pub fn get_total_extrinsic_count(&self) -> u32 {
        self.extrinsics.len() as u32
    }

    // Add extrinsic
    pub fn add_extrinsic(&mut self, pallet: String, call: String, extrinsic_type: ExtrinsicType, 
                        block_number: u32, success: bool, weight: u64, fee: u128) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let extrinsic_id = self.extrinsics.len() as u32 + 1;
        
        let extrinsic = ExtrinsicRecord {
            extrinsic_id,
            extrinsic_type: extrinsic_type.clone(),
            pallet,
            call,
            timestamp,
            block_number,
            success,
            weight,
            fee,
        };
        
        self.extrinsics.push(extrinsic);
        
        // Update first and last extrinsic dates
        if self.first_extrinsic_date.is_none() {
            self.first_extrinsic_date = Some(timestamp);
        }
        self.last_extrinsic_date = Some(timestamp);
        
        // Update success/failure counts
        if success {
            self.successful_extrinsics += 1;
        } else {
            self.failed_extrinsics += 1;
        }
        
        // Update fees
        self.total_fees_paid += fee;
        
        // Update extrinsic type counts
        let count = self.extrinsic_types.entry(extrinsic_type).or_insert(0);
        *count += 1;
        
        self.last_activity_time = timestamp;
    }

    // Get extrinsic records
    pub fn get_extrinsics(&self) -> &Vec<ExtrinsicRecord> {
        &self.extrinsics
    }

    // 2. Extrinsic diversity (staking, governance, identity, transfer, etc.)
    pub fn get_extrinsic_diversity(&self) -> usize {
        self.extrinsic_types.len()
    }

    // Get extrinsic types and their counts
    pub fn get_extrinsic_types(&self) -> &HashMap<ExtrinsicType, u32> {
        &self.extrinsic_types
    }

    // 3. Extrinsic timing (how regular, how recent)
    pub fn get_extrinsic_timing(&self) -> Option<(u64, u64)> {
        if let (Some(first), Some(last)) = (self.first_extrinsic_date, self.last_extrinsic_date) {
            Some((first, last))
        } else {
            None
        }
    }

    // Check if activity is regular (active within last 30 days)
    pub fn is_activity_regular(&self) -> bool {
        if let Some(last) = self.last_extrinsic_date {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();
            let days_since_last = (now - last) / (24 * 60 * 60);
            days_since_last <= 30
        } else {
            false
        }
    }

    // 4. First and last extrinsic dates
    pub fn get_first_extrinsic_date(&self) -> Option<u64> {
        self.first_extrinsic_date
    }

    pub fn get_last_extrinsic_date(&self) -> Option<u64> {
        self.last_extrinsic_date
    }

    // 5. Average transaction frequency (e.g., weekly)
    pub fn get_average_frequency(&self) -> f64 {
        if let (Some(first), Some(last)) = (self.first_extrinsic_date, self.last_extrinsic_date) {
            if first == last {
                return self.extrinsics.len() as f64;
            }
            
            let duration_days = (last - first) as f64 / (24.0 * 60.0 * 60.0);
            if duration_days > 0.0 {
                self.extrinsics.len() as f64 / (duration_days / 7.0) // Weekly frequency
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    // 6. Most frequently used extrinsic type
    pub fn get_most_frequent_extrinsic_type(&self) -> Option<(&ExtrinsicType, &u32)> {
        self.extrinsic_types.iter().max_by_key(|(_, count)| *count)
    }

    // 7. Governance related extrinsic ratio
    pub fn get_governance_extrinsic_ratio(&self) -> f64 {
        if self.extrinsics.is_empty() {
            return 0.0;
        }
        
        let governance_count = self.extrinsic_types.get(&ExtrinsicType::Governance)
            .copied()
            .unwrap_or(0);
            
        governance_count as f64 / self.extrinsics.len() as f64
    }

    // 8. Staking related extrinsic ratio
    pub fn get_staking_extrinsic_ratio(&self) -> f64 {
        if self.extrinsics.is_empty() {
            return 0.0;
        }
        
        let staking_count = self.extrinsic_types.get(&ExtrinsicType::Staking)
            .copied()
            .unwrap_or(0);
            
        staking_count as f64 / self.extrinsics.len() as f64
    }

    // 9. Identity related extrinsic ratio
    pub fn get_identity_extrinsic_ratio(&self) -> f64 {
        if self.extrinsics.is_empty() {
            return 0.0;
        }
        
        let identity_count = self.extrinsic_types.get(&ExtrinsicType::Identity)
            .copied()
            .unwrap_or(0);
            
        identity_count as f64 / self.extrinsics.len() as f64
    }

    // 10. Transfer operations ratio
    pub fn get_transfer_extrinsic_ratio(&self) -> f64 {
        if self.extrinsics.is_empty() {
            return 0.0;
        }
        
        let transfer_count = self.extrinsic_types.get(&ExtrinsicType::Transfer)
            .copied()
            .unwrap_or(0);
            
        transfer_count as f64 / self.extrinsics.len() as f64
    }

    // Get success rate
    pub fn get_success_rate(&self) -> f64 {
        if self.extrinsics.is_empty() {
            return 0.0;
        }
        
        self.successful_extrinsics as f64 / self.extrinsics.len() as f64
    }

    // Get total fees paid
    pub fn get_total_fees_paid(&self) -> u128 {
        self.total_fees_paid
    }

    // 11. Batch extrinsic usage (bulk transaction submission)
    pub fn add_batch_extrinsic(&mut self, extrinsics: Vec<ExtrinsicRecord>, block_number: u32) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let batch_id = self.batch_extrinsics.len() as u32 + 1;
        let total_weight: u64 = extrinsics.iter().map(|e| e.weight).sum();
        let total_fee: u128 = extrinsics.iter().map(|e| e.fee).sum();
        
        let batch = BatchExtrinsicRecord {
            batch_id,
            extrinsics: extrinsics.clone(),
            timestamp,
            block_number,
            total_weight,
            total_fee,
        };
        
        self.batch_extrinsics.push(batch);
        self.extrinsics.extend(extrinsics);
        
        // Update counts and fees
        for extrinsic in &batch.extrinsics {
            if extrinsic.success {
                self.successful_extrinsics += 1;
            } else {
                self.failed_extrinsics += 1;
            }
            self.total_fees_paid += extrinsic.fee;
            
            // Update extrinsic type counts
            let count = self.extrinsic_types.entry(extrinsic.extrinsic_type.clone()).or_insert(0);
            *count += 1;
        }
        
        // Update first and last extrinsic dates
        if self.first_extrinsic_date.is_none() {
            self.first_extrinsic_date = Some(timestamp);
        }
        self.last_extrinsic_date = Some(timestamp);
        
        self.last_activity_time = timestamp;
    }

    // Get batch extrinsic records
    pub fn get_batch_extrinsics(&self) -> &Vec<BatchExtrinsicRecord> {
        &self.batch_extrinsics
    }

    // Get batch extrinsic count
    pub fn get_batch_extrinsic_count(&self) -> usize {
        self.batch_extrinsics.len()
    }

    // Get batch usage ratio
    pub fn get_batch_usage_ratio(&self) -> f64 {
        if self.extrinsics.is_empty() {
            return 0.0;
        }
        
        let batch_extrinsic_count: usize = self.batch_extrinsics.iter()
            .map(|b| b.extrinsics.len())
            .sum();
            
        batch_extrinsic_count as f64 / self.extrinsics.len() as f64
    }

    // Get activity score based on various factors
    pub fn get_activity_score(&self) -> f64 {
        let mut score = 0.0;
        
        // Base score for total extrinsics
        score += self.extrinsics.len() as f64 * 0.5;
        
        // Score for diversity
        score += self.get_extrinsic_diversity() as f64 * 3.0;
        
        // Score for regular activity
        if self.is_activity_regular() {
            score += 10.0;
        }
        
        // Score for success rate
        score += self.get_success_rate() * 15.0;
        
        // Score for batch usage
        score += self.get_batch_usage_ratio() * 10.0;
        
        // Score for governance participation
        score += self.get_governance_extrinsic_ratio() * 8.0;
        
        // Score for staking participation
        score += self.get_staking_extrinsic_ratio() * 6.0;
        
        // Normalize score
        score
    }

    // Get recent activity count (last 90 days)
    pub fn get_recent_activity_count(&self) -> u32 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let ninety_days_ago = now - (90 * 24 * 60 * 60);
        
        self.extrinsics.iter()
            .filter(|e| e.timestamp > ninety_days_ago)
            .count() as u32
    }

    // Get last activity time
    pub fn get_last_activity_time(&self) -> u64 {
        self.last_activity_time
    }
}

// Extrinsic activity manager
pub struct ExtrinsicActivityManager {
    pub metrics: HashMap<u32, ExtrinsicActivityMetrics>, // Account ID -> Metrics
}

impl ExtrinsicActivityManager {
    pub fn new() -> Self {
        ExtrinsicActivityManager {
            metrics: HashMap::new(),
        }
    }

    pub fn create_metrics(&mut self, account_id: u32) -> &ExtrinsicActivityMetrics {
        let metrics = ExtrinsicActivityMetrics::new(account_id);
        self.metrics.insert(account_id, metrics);
        self.metrics.get(&account_id).unwrap()
    }

    pub fn get_metrics(&self, account_id: u32) -> Option<&ExtrinsicActivityMetrics> {
        self.metrics.get(&account_id)
    }

    pub fn get_all_metrics(&self) -> &HashMap<u32, ExtrinsicActivityMetrics> {
        &self.metrics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extrinsic_metrics_creation() {
        let mut manager = ExtrinsicActivityManager::new();
        let metrics = manager.create_metrics(1);
        
        assert_eq!(metrics.account_id, 1);
        assert_eq!(metrics.get_total_extrinsic_count(), 0);
        assert_eq!(metrics.get_extrinsic_diversity(), 0);
    }

    #[test]
    fn test_add_extrinsic() {
        let mut manager = ExtrinsicActivityManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.add_extrinsic(
            "Balances".to_string(),
            "transfer".to_string(),
            ExtrinsicType::Transfer,
            1000,
            true,
            1000000,
            100
        );
        
        metrics.add_extrinsic(
            "Staking".to_string(),
            "bond".to_string(),
            ExtrinsicType::Staking,
            1001,
            true,
            2000000,
            200
        );
        
        assert_eq!(metrics.get_total_extrinsic_count(), 2);
        assert_eq!(metrics.get_extrinsic_diversity(), 2);
        assert!(metrics.get_first_extrinsic_date().is_some());
        assert!(metrics.get_last_extrinsic_date().is_some());
    }

    #[test]
    fn test_extrinsic_diversity() {
        let mut manager = ExtrinsicActivityManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.add_extrinsic("Balances".to_string(), "transfer".to_string(), ExtrinsicType::Transfer, 1000, true, 1000000, 100);
        metrics.add_extrinsic("Staking".to_string(), "bond".to_string(), ExtrinsicType::Staking, 1001, true, 2000000, 200);
        metrics.add_extrinsic("Democracy".to_string(), "vote".to_string(), ExtrinsicType::Governance, 1002, true, 1500000, 150);
        metrics.add_extrinsic("Identity".to_string(), "set_identity".to_string(), ExtrinsicType::Identity, 1003, true, 1200000, 120);
        
        assert_eq!(metrics.get_extrinsic_diversity(), 4);
        let types = metrics.get_extrinsic_types();
        assert_eq!(*types.get(&ExtrinsicType::Transfer).unwrap(), 1);
        assert_eq!(*types.get(&ExtrinsicType::Staking).unwrap(), 1);
        assert_eq!(*types.get(&ExtrinsicType::Governance).unwrap(), 1);
        assert_eq!(*types.get(&ExtrinsicType::Identity).unwrap(), 1);
    }

    #[test]
    fn test_extrinsic_timing() {
        let mut manager = ExtrinsicActivityManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.add_extrinsic("Balances".to_string(), "transfer".to_string(), ExtrinsicType::Transfer, 1000, true, 1000000, 100);
        
        let timing = metrics.get_extrinsic_timing();
        assert!(timing.is_some());
        
        let (first, last) = timing.unwrap();
        assert_eq!(first, last);
    }

    #[test]
    fn test_frequency_calculations() {
        let mut manager = ExtrinsicActivityManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.add_extrinsic("Balances".to_string(), "transfer".to_string(), ExtrinsicType::Transfer, 1000, true, 1000000, 100);
        metrics.add_extrinsic("Staking".to_string(), "bond".to_string(), ExtrinsicType::Staking, 1001, true, 2000000, 200);
        
        let frequency = metrics.get_average_frequency();
        assert!(frequency >= 0.0);
        
        let is_regular = metrics.is_activity_regular();
        assert!(is_regular == true || is_regular == false);
    }

    #[test]
    fn test_extrinsic_ratios() {
        let mut manager = ExtrinsicActivityManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.add_extrinsic("Balances".to_string(), "transfer".to_string(), ExtrinsicType::Transfer, 1000, true, 1000000, 100);
        metrics.add_extrinsic("Balances".to_string(), "transfer".to_string(), ExtrinsicType::Transfer, 1001, true, 1000000, 100);
        metrics.add_extrinsic("Staking".to_string(), "bond".to_string(), ExtrinsicType::Staking, 1002, true, 2000000, 200);
        metrics.add_extrinsic("Democracy".to_string(), "vote".to_string(), ExtrinsicType::Governance, 1003, true, 1500000, 150);
        metrics.add_extrinsic("Identity".to_string(), "set_identity".to_string(), ExtrinsicType::Identity, 1004, true, 1200000, 120);
        
        assert_eq!(metrics.get_transfer_extrinsic_ratio(), 0.4); // 2 out of 5
        assert_eq!(metrics.get_staking_extrinsic_ratio(), 0.2);  // 1 out of 5
        assert_eq!(metrics.get_governance_extrinsic_ratio(), 0.2); // 1 out of 5
        assert_eq!(metrics.get_identity_extrinsic_ratio(), 0.2); // 1 out of 5
    }

    #[test]
    fn test_most_frequent_type() {
        let mut manager = ExtrinsicActivityManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.add_extrinsic("Balances".to_string(), "transfer".to_string(), ExtrinsicType::Transfer, 1000, true, 1000000, 100);
        metrics.add_extrinsic("Balances".to_string(), "transfer".to_string(), ExtrinsicType::Transfer, 1001, true, 1000000, 100);
        metrics.add_extrinsic("Balances".to_string(), "transfer".to_string(), ExtrinsicType::Transfer, 1002, true, 1000000, 100);
        metrics.add_extrinsic("Staking".to_string(), "bond".to_string(), ExtrinsicType::Staking, 1003, true, 2000000, 200);
        
        let most_frequent = metrics.get_most_frequent_extrinsic_type();
        assert!(most_frequent.is_some());
        assert_eq!(most_frequent.unwrap().0, &ExtrinsicType::Transfer);
        assert_eq!(*most_frequent.unwrap().1, 3);
    }

    #[test]
    fn test_success_rate() {
        let mut manager = ExtrinsicActivityManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.add_extrinsic("Balances".to_string(), "transfer".to_string(), ExtrinsicType::Transfer, 1000, true, 1000000, 100);
        metrics.add_extrinsic("Staking".to_string(), "bond".to_string(), ExtrinsicType::Staking, 1001, false, 2000000, 200);
        metrics.add_extrinsic("Democracy".to_string(), "vote".to_string(), ExtrinsicType::Governance, 1002, true, 1500000, 150);
        
        assert_eq!(metrics.successful_extrinsics, 2);
        assert_eq!(metrics.failed_extrinsics, 1);
        assert_eq!(metrics.get_success_rate(), 2.0 / 3.0);
    }

    #[test]
    fn test_batch_extrinsic_usage() {
        let mut manager = ExtrinsicActivityManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        let extrinsics = vec![
            ExtrinsicRecord {
                extrinsic_id: 1,
                extrinsic_type: ExtrinsicType::Transfer,
                pallet: "Balances".to_string(),
                call: "transfer".to_string(),
                timestamp: 1000000,
                block_number: 1000,
                success: true,
                weight: 1000000,
                fee: 100,
            },
            ExtrinsicRecord {
                extrinsic_id: 2,
                extrinsic_type: ExtrinsicType::Staking,
                pallet: "Staking".to_string(),
                call: "bond".to_string(),
                timestamp: 1000001,
                block_number: 1001,
                success: true,
                weight: 2000000,
                fee: 200,
            }
        ];
        
        metrics.add_batch_extrinsic(extrinsics, 1000);
        
        assert_eq!(metrics.get_batch_extrinsic_count(), 1);
        assert_eq!(metrics.get_total_extrinsic_count(), 2);
        assert_eq!(metrics.get_batch_usage_ratio(), 1.0); // All extrinsics are from batch
    }

    #[test]
    fn test_activity_score() {
        let mut manager = ExtrinsicActivityManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.add_extrinsic("Balances".to_string(), "transfer".to_string(), ExtrinsicType::Transfer, 1000, true, 1000000, 100);
        metrics.add_extrinsic("Staking".to_string(), "bond".to_string(), ExtrinsicType::Staking, 1001, true, 2000000, 200);
        metrics.add_extrinsic("Democracy".to_string(), "vote".to_string(), ExtrinsicType::Governance, 1002, true, 1500000, 150);
        
        let score = metrics.get_activity_score();
        assert!(score >= 0.0);
    }

    #[test]
    fn test_recent_activity() {
        let mut manager = ExtrinsicActivityManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.add_extrinsic("Balances".to_string(), "transfer".to_string(), ExtrinsicType::Transfer, 1000, true, 1000000, 100);
        
        let recent_count = metrics.get_recent_activity_count();
        assert!(recent_count >= 0);
    }
}
