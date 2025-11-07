use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Staking operation types
#[derive(Debug, Clone, PartialEq)]
pub enum StakingOperation {
    Bond,          // Stake initiation (Bonded)
    Unbond,        // Stake unbonding (Unbonded)
    Rebond,        // Rebonding/Restaking
    Withdraw,      // Withdrawal
    Nominate,      // Validator nomination
    ClaimReward,   // Reward claiming
    Slash,         // Slashing event
}

// Staking activity record
#[derive(Debug, Clone)]
pub struct StakingActivityRecord {
    pub operation_type: StakingOperation, // Operation type
    pub amount: Option<u128>,             // Amount (if applicable)
    pub validator_ids: Option<Vec<u32>>,  // Validator IDs (if applicable)
    pub timestamp: u64,                   // Operation timestamp
    pub block_number: u32,                // Block number
    pub extrinsic_hash: String,           // Extrinsic hash
}

// Validator information
#[derive(Debug, Clone)]
pub struct ValidatorInfo {
    pub validator_id: u32,                // Validator ID
    pub is_slashed: bool,                 // Slashing status
    pub slash_amount: u128,               // Slashing amount
    pub commission: f64,                  // Commission rate
    pub is_active: bool,                  // Active status
}

// Staking activity metrics
#[derive(Debug, Clone)]
pub struct StakingActivityMetrics {
    pub account_id: u32,                             // Account ID
    pub staking_activities: Vec<StakingActivityRecord>, // All staking operations
    pub validators: HashMap<u32, ValidatorInfo>,      // Validator information
    pub first_stake_date: Option<u64>,                // First stake date
    pub total_staked_amount: u128,                    // Total staked amount
    pub current_staked_amount: u128,                  // Current staked amount
    pub total_unbonded_amount: u128,                  // Total unbonded amount
    pub total_withdrawn_amount: u128,                 // Total withdrawn amount
    pub total_rewards_claimed: u128,                  // Total claimed rewards
    pub slashing_events: u32,                         // Slashing events count
    pub total_staking_extrinsics: u32,                // Total staking extrinsics count
    pub last_activity_time: u64,                      // Last activity timestamp
}

impl StakingActivityMetrics {
    pub fn new(account_id: u32) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        StakingActivityMetrics {
            account_id,
            staking_activities: Vec::new(),
            validators: HashMap::new(),
            first_stake_date: None,
            total_staked_amount: 0,
            current_staked_amount: 0,
            total_unbonded_amount: 0,
            total_withdrawn_amount: 0,
            total_rewards_claimed: 0,
            slashing_events: 0,
            total_staking_extrinsics: 0,
            last_activity_time: now,
        }
    }

    // 1. Stake Initiation (Bonded)
    pub fn start_staking(&mut self, amount: u128, block_number: u32, extrinsic_hash: String) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        // Set first stake date
        if self.first_stake_date.is_none() {
            self.first_stake_date = Some(timestamp);
        }
        
        let activity = StakingActivityRecord {
            operation_type: StakingOperation::Bond,
            amount: Some(amount),
            validator_ids: None,
            timestamp,
            block_number,
            extrinsic_hash,
        };
        
        self.staking_activities.push(activity);
        self.total_staked_amount += amount;
        self.current_staked_amount += amount;
        self.total_staking_extrinsics += 1;
        self.last_activity_time = timestamp;
    }

    pub fn is_staking_started(&self) -> bool {
        self.first_stake_date.is_some()
    }

    pub fn get_first_stake_date(&self) -> Option<u64> {
        self.first_stake_date
    }

    // Active duration calculation (in days)
    pub fn get_active_duration_days(&self) -> Option<u64> {
        if let Some(first_stake) = self.first_stake_date {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();
            let duration_seconds = now - first_stake;
            Some(duration_seconds / (24 * 60 * 60)) // Convert seconds to days
        } else {
            None
        }
    }

    // 2. Stake Amount (Bonded Amount)
    pub fn get_current_staked_amount(&self) -> u128 {
        self.current_staked_amount
    }

    pub fn get_total_staked_amount(&self) -> u128 {
        self.total_staked_amount
    }

    // Logarithmic weighting
    pub fn get_logarithmic_stake_weight(&self) -> f64 {
        if self.current_staked_amount == 0 {
            0.0
        } else {
            (self.current_staked_amount as f64).ln() / 10.0 // Logarithmic scaling
        }
    }

    // Threshold-based weighting
    pub fn get_threshold_stake_weight(&self) -> f64 {
        let amount = self.current_staked_amount;
        match amount {
            0..=1000 => 0.1,        // 0-1000 DOT: Low weight
            1001..=10000 => 0.5,    // 1001-10000 DOT: Medium weight
            10001..=100000 => 1.0,  // 10001-100000 DOT: High weight
            _ => 1.5,               // 100000+ DOT: Very high weight
        }
    }

    // 3. Unbonding Initiation (Unbonded)
    pub fn start_unbonding(&mut self, amount: u128, block_number: u32, extrinsic_hash: String) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let activity = StakingActivityRecord {
            operation_type: StakingOperation::Unbond,
            amount: Some(amount),
            validator_ids: None,
            timestamp,
            block_number,
            extrinsic_hash,
        };
        
        self.staking_activities.push(activity);
        self.current_staked_amount = self.current_staked_amount.saturating_sub(amount);
        self.total_unbonded_amount += amount;
        self.total_staking_extrinsics += 1;
        self.last_activity_time = timestamp;
    }

    pub fn is_unbonding_started(&self) -> bool {
        self.staking_activities.iter().any(|a| a.operation_type == StakingOperation::Unbond)
    }

    // Stake duration analysis
    pub fn get_average_stake_duration(&self) -> Option<u64> {
        if self.first_stake_date.is_none() || self.current_staked_amount == 0 {
            return None;
        }
        
        let first_stake = self.first_stake_date.unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        Some(now - first_stake)
    }

    // 4. Rebonding / Restaking
    pub fn rebond_staking(&mut self, amount: u128, block_number: u32, extrinsic_hash: String) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let activity = StakingActivityRecord {
            operation_type: StakingOperation::Rebond,
            amount: Some(amount),
            validator_ids: None,
            timestamp,
            block_number,
            extrinsic_hash,
        };
        
        self.staking_activities.push(activity);
        self.current_staked_amount += amount;
        self.total_staking_extrinsics += 1;
        self.last_activity_time = timestamp;
    }

    pub fn get_rebonding_count(&self) -> usize {
        self.staking_activities.iter().filter(|a| a.operation_type == StakingOperation::Rebond).count()
    }

    // 5. Withdraw (Withdrawal)
    pub fn withdraw_stake(&mut self, amount: u128, block_number: u32, extrinsic_hash: String) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let activity = StakingActivityRecord {
            operation_type: StakingOperation::Withdraw,
            amount: Some(amount),
            validator_ids: None,
            timestamp,
            block_number,
            extrinsic_hash,
        };
        
        self.staking_activities.push(activity);
        self.total_withdrawn_amount += amount;
        self.total_staking_extrinsics += 1;
        self.last_activity_time = timestamp;
    }

    pub fn get_withdrawn_amount(&self) -> u128 {
        self.total_withdrawn_amount
    }

    // 6. Validator Change / Nominate
    pub fn nominate_validators(&mut self, validator_ids: Vec<u32>, block_number: u32, extrinsic_hash: String) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let activity = StakingActivityRecord {
            operation_type: StakingOperation::Nominate,
            amount: None,
            validator_ids: Some(validator_ids.clone()),
            timestamp,
            block_number,
            extrinsic_hash,
        };
        
        self.staking_activities.push(activity);
        self.total_staking_extrinsics += 1;
        self.last_activity_time = timestamp;
    }

    pub fn get_nominated_validators(&self) -> Vec<u32> {
        let mut validators = Vec::new();
        for activity in &self.staking_activities {
            if activity.operation_type == StakingOperation::Nominate {
                if let Some(ref validator_ids) = activity.validator_ids {
                    validators.extend(validator_ids);
                }
            }
        }
        validators.sort();
        validators.dedup();
        validators
    }

    // Diversity score (number of nominated validators)
    pub fn get_validator_diversity_score(&self) -> f64 {
        let nominated_validators = self.get_nominated_validators();
        nominated_validators.len() as f64 / 10.0 // Full score up to 10 validators
    }

    // 7. Slashing Events
    pub fn add_validator_slash(&mut self, validator_id: u32, slash_amount: u128) {
        // Update validator information
        if let Some(validator) = self.validators.get_mut(&validator_id) {
            validator.is_slashed = true;
            validator.slash_amount += slash_amount;
        } else {
            let validator = ValidatorInfo {
                validator_id,
                is_slashed: true,
                slash_amount,
                commission: 0.0,
                is_active: true,
            };
            self.validators.insert(validator_id, validator);
        }
        
        self.slashing_events += 1;
        self.total_staking_extrinsics += 1;
    }

    pub fn get_slashing_events(&self) -> u32 {
        self.slashing_events
    }

    pub fn get_slashed_validators(&self) -> Vec<u32> {
        self.validators.iter()
            .filter(|(_, v)| v.is_slashed)
            .map(|(id, _)| *id)
            .collect()
    }

    // Risky validator selection penalty score
    pub fn get_slashing_penalty_score(&self) -> f64 {
        -(self.slashing_events as f64 * 2.0) // -2 points for each slashing event
    }

    // 8. Reward Claim (Reward Withdrawal)
    pub fn claim_reward(&mut self, amount: u128, block_number: u32, extrinsic_hash: String) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let activity = StakingActivityRecord {
            operation_type: StakingOperation::ClaimReward,
            amount: Some(amount),
            validator_ids: None,
            timestamp,
            block_number,
            extrinsic_hash,
        };
        
        self.staking_activities.push(activity);
        self.total_rewards_claimed += amount;
        self.total_staking_extrinsics += 1;
        self.last_activity_time = timestamp;
    }

    pub fn get_claimed_rewards(&self) -> u128 {
        self.total_rewards_claimed
    }

    // Regular reward claim analysis
    pub fn get_reward_claim_frequency(&self) -> f64 {
        if self.total_rewards_claimed == 0 {
            return 0.0;
        }
        
        let reward_activities = self.staking_activities.iter()
            .filter(|a| a.operation_type == StakingOperation::ClaimReward)
            .count();
            
        reward_activities as f64 / self.staking_activities.len() as f64
    }

    // 9. Staking Extrinsic Count
    pub fn get_total_staking_extrinsics(&self) -> u32 {
        self.total_staking_extrinsics
    }

    pub fn get_staking_activities(&self) -> &Vec<StakingActivityRecord> {
        &self.staking_activities
    }

    // User's technical activity on-chain
    pub fn get_technical_activity_score(&self) -> f64 {
        self.total_staking_extrinsics as f64 * 0.5 // 0.5 points per extrinsic
    }

    // Overall staking activity score
    pub fn get_overall_staking_score(&self) -> f64 {
        let mut score = 0.0;
        
        // Stake amount score
        score += self.get_logarithmic_stake_weight() * 10.0;
        
        // Active duration score
        if let Some(duration_days) = self.get_active_duration_days() {
            score += (duration_days as f64).min(365.0) / 36.5; // Max 10 points (1 year)
        }
        
        // Rebonding behavior score
        score += self.get_rebonding_count() as f64 * 1.0;
        
        // Validator diversity score
        score += self.get_validator_diversity_score() * 5.0;
        
        // Reward claim score
        score += self.get_reward_claim_frequency() * 10.0;
        
        // Technical activity score
        score += self.get_technical_activity_score();
        
        // Slashing penalty score
        score += self.get_slashing_penalty_score();
        
        score.max(0.0) // Ensure non-negative score
    }

    // Add validator information
    pub fn add_validator_info(&mut self, validator_id: u32, commission: f64, is_active: bool) {
        let validator = ValidatorInfo {
            validator_id,
            is_slashed: false,
            slash_amount: 0,
            commission,
            is_active,
        };
        self.validators.insert(validator_id, validator);
    }

    // Safe validator selection check
    pub fn get_safe_validator_selection_score(&self) -> f64 {
        let nominated_validators: Vec<u32> = self.get_nominated_validators();
        if nominated_validators.is_empty() {
            return 0.0;
        }
        
        let safe_validators = nominated_validators.iter()
            .filter(|&id| {
                if let Some(validator) = self.validators.get(id) {
                    !validator.is_slashed && validator.is_active && validator.commission < 0.2 // Less than 20% commission
                } else {
                    false
                }
            })
            .count();
            
        (safe_validators as f64 / nominated_validators.len() as f64) * 10.0 // Max 10 points
    }
}

// Staking metrics manager
pub struct StakingMetricsManager {
    pub metrics: HashMap<u32, StakingActivityMetrics>, // Account ID -> Metrics
}

impl StakingMetricsManager {
    pub fn new() -> Self {
        StakingMetricsManager {
            metrics: HashMap::new(),
        }
    }

    pub fn create_metrics(&mut self, account_id: u32) -> &StakingActivityMetrics {
        let metrics = StakingActivityMetrics::new(account_id);
        self.metrics.insert(account_id, metrics);
        self.metrics.get(&account_id).unwrap()
    }

    pub fn get_metrics(&self, account_id: u32) -> Option<&StakingActivityMetrics> {
        self.metrics.get(&account_id)
    }

    pub fn get_all_metrics(&self) -> &HashMap<u32, StakingActivityMetrics> {
        &self.metrics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staking_metrics_creation() {
        let mut manager = StakingMetricsManager::new();
        let metrics = manager.create_metrics(1);
        
        assert_eq!(metrics.account_id, 1);
        assert_eq!(metrics.is_staking_started(), false);
        assert_eq!(metrics.get_current_staked_amount(), 0);
    }

    #[test]
    fn test_staking_start() {
        let mut manager = StakingMetricsManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.start_staking(1000, 1000, "0x123456".to_string());
        
        assert_eq!(metrics.is_staking_started(), true);
        assert_eq!(metrics.get_current_staked_amount(), 1000);
        assert_eq!(metrics.get_total_staked_amount(), 1000);
    }

    #[test]
    fn test_active_duration() {
        let mut manager = StakingMetricsManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.start_staking(1000, 1000, "0x123456".to_string());
        
        // Active duration check (approximate value)
        if let Some(duration) = metrics.get_active_duration_days() {
            assert!(duration >= 0);
        }
    }

    #[test]
    fn test_stake_weighting() {
        let mut manager = StakingMetricsManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.start_staking(10000, 1000, "0x123456".to_string());
        
        let log_weight = metrics.get_logarithmic_stake_weight();
        let threshold_weight = metrics.get_threshold_stake_weight();
        
        assert!(log_weight > 0.0);
        assert!(threshold_weight > 0.0);
    }

    #[test]
    fn test_unbonding() {
        let mut manager = StakingMetricsManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.start_staking(1000, 1000, "0x123456".to_string());
        metrics.start_unbonding(500, 1001, "0x789012".to_string());
        
        assert_eq!(metrics.get_current_staked_amount(), 500);
        assert_eq!(metrics.get_total_unbonded_amount(), 500);
        assert_eq!(metrics.is_unbonding_started(), true);
    }

    #[test]
    fn test_rebonding() {
        let mut manager = StakingMetricsManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.start_staking(1000, 1000, "0x123456".to_string());
        metrics.start_unbonding(500, 1001, "0x789012".to_string());
        metrics.rebond_staking(300, 1002, "0x345678".to_string());
        
        assert_eq!(metrics.get_current_staked_amount(), 800);
        assert_eq!(metrics.get_rebonding_count(), 1);
    }

    #[test]
    fn test_withdraw() {
        let mut manager = StakingMetricsManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.start_staking(1000, 1000, "0x123456".to_string());
        metrics.start_unbonding(1000, 1001, "0x789012".to_string());
        metrics.withdraw_stake(1000, 1002, "0x345678".to_string());
        
        assert_eq!(metrics.get_withdrawn_amount(), 1000);
    }

    #[test]
    fn test_validator_nomination() {
        let mut manager = StakingMetricsManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.nominate_validators(vec![100, 200, 300], 1000, "0x123456".to_string());
        
        let nominated = metrics.get_nominated_validators();
        assert_eq!(nominated.len(), 3);
        assert!(nominated.contains(&100));
        assert!(nominated.contains(&200));
        assert!(nominated.contains(&300));
    }

    #[test]
    fn test_slashing_events() {
        let mut manager = StakingMetricsManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.add_validator_info(100, 0.1, true);
        metrics.nominate_validators(vec![100], 1000, "0x123456".to_string());
        metrics.add_validator_slash(100, 100);
        
        assert_eq!(metrics.get_slashing_events(), 1);
        assert_eq!(metrics.get_slashed_validators().len(), 1);
    }

    #[test]
    fn test_reward_claim() {
        let mut manager = StakingMetricsManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.claim_reward(50, 1000, "0x123456".to_string());
        metrics.claim_reward(75, 1001, "0x789012".to_string());
        
        assert_eq!(metrics.get_claimed_rewards(), 125);
    }

    #[test]
    fn test_staking_extrinsics() {
        let mut manager = StakingMetricsManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.start_staking(1000, 1000, "0x123456".to_string());
        metrics.nominate_validators(vec![100, 200], 1001, "0x789012".to_string());
        metrics.claim_reward(50, 1002, "0x345678".to_string());
        
        assert_eq!(metrics.get_total_staking_extrinsics(), 3);
    }

    #[test]
    fn test_overall_score() {
        let mut manager = StakingMetricsManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.start_staking(10000, 1000, "0x123456".to_string());
        metrics.nominate_validators(vec![100, 200], 1001, "0x789012".to_string());
        metrics.add_validator_info(100, 0.1, true);
        metrics.add_validator_info(200, 0.15, true);
        metrics.claim_reward(100, 1002, "0x345678".to_string());
        
        let score = metrics.get_overall_staking_score();
        let safe_validator_score = metrics.get_safe_validator_selection_score();
        
        assert!(score >= 0.0);
        assert!(safe_validator_score >= 0.0);
    }
}
