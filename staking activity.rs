use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Staking statuses
#[derive(Debug, Clone, PartialEq)]
pub enum StakingStatus {
    Active,      // Active staking
    Inactive,    // Inactive staking
    Unbonding,   // In unbonding process
    Chilled,     // Chilled/frozen
}

// Slash types
#[derive(Debug, Clone, PartialEq)]
pub enum SlashType {
    Offence,     // Offence committed
    Misbehavior, // Misbehavior
    Liveness,    // Liveness fault
}

// Reward types
#[derive(Debug, Clone, PartialEq)]
pub enum RewardType {
    Staking,     // Staking reward
    Validator,   // Validator reward
    Nomination,  // Nomination reward
}

// Staking activity record
#[derive(Debug, Clone)]
pub struct StakingActivity {
    pub account_id: u32,           // Account ID
    pub status: StakingStatus,     // Staking status
    pub staked_amount: u128,       // Staked DOT amount
    pub start_time: u64,           // Staking start time
    pub validators: Vec<u32>,       // Selected validators
    pub rewards: Vec<RewardRecord>, // Reward history
    pub unbonding_info: Vec<UnbondingRecord>, // Unbonding status
    pub slashes: Vec<SlashRecord>, // Slashing history
    pub nominator_activities: Vec<NominatorActivity>, // Nominator activity
    pub active_sessions: u32,      // Active session count
    pub total_sessions: u32,       // Total session count
    pub last_active_time: u64,     // Last activity time
}

// Reward record
#[derive(Debug, Clone)]
pub struct RewardRecord {
    pub amount: u128,              // Reward amount
    pub reward_type: RewardType,   // Reward type
    pub timestamp: u64,            // Timestamp
    pub session_index: u32,        // Session index
    pub validator_id: Option<u32>, // Validator ID (if any)
}

// Unbonding record
#[derive(Debug, Clone)]
pub struct UnbondingRecord {
    pub amount: u128,              // Unbonding amount
    pub start_time: u64,           // Start time
    pub unlock_time: u64,          // Unlock time
    pub completed: bool,           // Completed status
}

// Slash record
#[derive(Debug, Clone)]
pub struct SlashRecord {
    pub amount: u128,              // Slashed amount
    pub slash_type: SlashType,     // Slash type
    pub timestamp: u64,            // Timestamp
    pub session_index: u32,        // Session index
    pub reason: String,            // Reason
}

// Nominator activity
#[derive(Debug, Clone)]
pub struct NominatorActivity {
    pub nominator_id: u32,         // Nominator ID
    pub nominated_amount: u128,    // Nominated amount
    pub timestamp: u64,            // Timestamp
    pub active: bool,              // Active status
}

impl StakingActivity {
    pub fn new(account_id: u32) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        StakingActivity {
            account_id,
            status: StakingStatus::Inactive,
            staked_amount: 0,
            start_time: now,
            validators: Vec::new(),
            rewards: Vec::new(),
            unbonding_info: Vec::new(),
            slashes: Vec::new(),
            nominator_activities: Vec::new(),
            active_sessions: 0,
            total_sessions: 0,
            last_active_time: now,
        }
    }

    // Staking Status
    pub fn get_staking_status(&self) -> &StakingStatus {
        &self.status
    }

    pub fn set_staking_status(&mut self, status: StakingStatus) {
        self.status = status;
        if status == StakingStatus::Active {
            self.last_active_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();
            self.active_sessions += 1;
        }
        self.total_sessions += 1;
    }

    // Staked DOT Amount
    pub fn get_staked_amount(&self) -> u128 {
        self.staked_amount
    }

    pub fn stake_amount(&mut self, amount: u128) {
        self.staked_amount += amount;
        if self.status != StakingStatus::Active {
            self.set_staking_status(StakingStatus::Active);
        }
    }

    pub fn unstake_amount(&mut self, amount: u128) {
        if self.staked_amount >= amount {
            self.staked_amount -= amount;
        } else {
            self.staked_amount = 0;
        }
        
        if self.staked_amount == 0 && self.status == StakingStatus::Active {
            self.set_staking_status(StakingStatus::Inactive);
        }
    }

    // Staking Duration
    pub fn staking_duration(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        now - self.start_time
    }

    // Validator Selection
    pub fn select_validators(&mut self, validators: Vec<u32>) {
        self.validators = validators;
    }

    pub fn get_selected_validators(&self) -> &Vec<u32> {
        &self.validators
    }

    pub fn add_validator(&mut self, validator_id: u32) {
        if !self.validators.contains(&validator_id) {
            self.validators.push(validator_id);
        }
    }

    pub fn remove_validator(&mut self, validator_id: u32) {
        self.validators.retain(|&id| id != validator_id);
    }

    // Reward History
    pub fn add_reward(&mut self, amount: u128, reward_type: RewardType, session_index: u32, validator_id: Option<u32>) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let reward = RewardRecord {
            amount,
            reward_type,
            timestamp,
            session_index,
            validator_id,
        };
        
        self.rewards.push(reward);
    }

    pub fn get_reward_history(&self) -> &Vec<RewardRecord> {
        &self.rewards
    }

    pub fn total_rewards(&self) -> u128 {
        self.rewards.iter().map(|r| r.amount).sum()
    }

    // Unbonding Status
    pub fn start_unbonding(&mut self, amount: u128, unlock_duration: u64) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let unbonding = UnbondingRecord {
            amount,
            start_time: now,
            unlock_time: now + unlock_duration,
            completed: false,
        };
        
        self.unbonding_info.push(unbonding);
        self.set_staking_status(StakingStatus::Unbonding);
    }

    pub fn get_unbonding_status(&self) -> &Vec<UnbondingRecord> {
        &self.unbonding_info
    }

    pub fn complete_unbonding(&mut self, index: usize) {
        if let Some(unbonding) = self.unbonding_info.get_mut(index) {
            unbonding.completed = true;
        }
    }

    // Slashing History
    pub fn add_slash(&mut self, amount: u128, slash_type: SlashType, session_index: u32, reason: String) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let slash = SlashRecord {
            amount,
            slash_type,
            timestamp,
            session_index,
            reason,
        };
        
        self.slashes.push(slash);
        
        // Deduct slashed amount from staked amount
        if self.staked_amount >= amount {
            self.staked_amount -= amount;
        } else {
            self.staked_amount = 0;
        }
    }

    pub fn get_slash_history(&self) -> &Vec<SlashRecord> {
        &self.slashes
    }

    pub fn total_slashes(&self) -> u128 {
        self.slashes.iter().map(|s| s.amount).sum()
    }

    // Nominator Activity
    pub fn add_nominator_activity(&mut self, nominator_id: u32, nominated_amount: u128) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let activity = NominatorActivity {
            nominator_id,
            nominated_amount,
            timestamp,
            active: true,
        };
        
        self.nominator_activities.push(activity);
    }

    pub fn get_nominator_activities(&self) -> &Vec<NominatorActivity> {
        &self.nominator_activities
    }

    pub fn total_nominated_amount(&self) -> u128 {
        self.nominator_activities.iter().map(|n| n.nominated_amount).sum()
    }

    // Activity Frequency
    pub fn activity_frequency(&self) -> f64 {
        if self.total_sessions == 0 {
            return 0.0;
        }
        self.active_sessions as f64 / self.total_sessions as f64
    }

    pub fn get_active_sessions(&self) -> u32 {
        self.active_sessions
    }

    pub fn get_total_sessions(&self) -> u32 {
        self.total_sessions
    }

    // Re-nomination
    pub fn re_nominate(&mut self) {
        // Make all nominator activities active
        for activity in &mut self.nominator_activities {
            activity.active = true;
        }
        
        // Set staking status to active
        if self.status != StakingStatus::Active {
            self.set_staking_status(StakingStatus::Active);
        }
        
        self.last_active_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
    }

    pub fn chill_staking(&mut self) {
        self.status = StakingStatus::Chilled;
        // Make all nominator activities inactive
        for activity in &mut self.nominator_activities {
            activity.active = false;
        }
    }
}

// Validator information
#[derive(Debug, Clone)]
pub struct Validator {
    pub id: u32,
    pub commission: f64,           // Commission rate
    pub total_stake: u128,         // Total stake
    pub nominators: Vec<u32>,      // Nominators
    pub is_active: bool,           // Active status
    pub slashed: bool,             // Slashed status
}

impl Validator {
    pub fn new(id: u32, commission: f64) -> Self {
        Validator {
            id,
            commission,
            total_stake: 0,
            nominators: Vec::new(),
            is_active: false,
            slashed: false,
        }
    }

    pub fn add_stake(&mut self, amount: u128) {
        self.total_stake += amount;
    }

    pub fn add_nominator(&mut self, nominator_id: u32) {
        if !self.nominators.contains(&nominator_id) {
            self.nominators.push(nominator_id);
        }
    }
}

// Staking manager
pub struct StakingManager {
    pub staking_activities: HashMap<u32, StakingActivity>,
    pub validators: HashMap<u32, Validator>,
}

impl StakingManager {
    pub fn new() -> Self {
        StakingManager {
            staking_activities: HashMap::new(),
            validators: HashMap::new(),
        }
    }

    pub fn create_staking_activity(&mut self, account_id: u32) -> &StakingActivity {
        let activity = StakingActivity::new(account_id);
        self.staking_activities.insert(account_id, activity);
        self.staking_activities.get(&account_id).unwrap()
    }

    pub fn get_staking_activity(&self, account_id: u32) -> Option<&StakingActivity> {
        self.staking_activities.get(&account_id)
    }

    pub fn create_validator(&mut self, id: u32, commission: f64) -> &Validator {
        let validator = Validator::new(id, commission);
        self.validators.insert(id, validator);
        self.validators.get(&id).unwrap()
    }

    pub fn get_validator(&self, id: u32) -> Option<&Validator> {
        self.validators.get(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staking_activity_creation() {
        let mut manager = StakingManager::new();
        let activity = manager.create_staking_activity(1);
        
        assert_eq!(activity.account_id, 1);
        assert_eq!(activity.status, StakingStatus::Inactive);
        assert_eq!(activity.staked_amount, 0);
    }

    #[test]
    fn test_staking_amount() {
        let mut manager = StakingManager::new();
        let activity = manager.create_staking_activity(1);
        let activity = manager.staking_activities.get_mut(&1).unwrap();
        
        activity.stake_amount(1000);
        assert_eq!(activity.get_staked_amount(), 1000);
        assert_eq!(activity.get_staking_status(), &StakingStatus::Active);
        
        activity.unstake_amount(500);
        assert_eq!(activity.get_staked_amount(), 500);
    }

    #[test]
    fn test_validator_selection() {
        let mut manager = StakingManager::new();
        let activity = manager.create_staking_activity(1);
        let activity = manager.staking_activities.get_mut(&1).unwrap();
        
        activity.select_validators(vec![100, 200, 300]);
        assert_eq!(activity.get_selected_validators().len(), 3);
        
        activity.add_validator(400);
        assert_eq!(activity.get_selected_validators().len(), 4);
        
        activity.remove_validator(200);
        assert_eq!(activity.get_selected_validators().len(), 3);
    }

    #[test]
    fn test_rewards() {
        let mut manager = StakingManager::new();
        let activity = manager.create_staking_activity(1);
        let activity = manager.staking_activities.get_mut(&1).unwrap();
        
        activity.add_reward(100, RewardType::Staking, 1, Some(100));
        activity.add_reward(200, RewardType::Validator, 2, None);
        
        assert_eq!(activity.get_reward_history().len(), 2);
        assert_eq!(activity.total_rewards(), 300);
    }

    #[test]
    fn test_unbonding() {
        let mut manager = StakingManager::new();
        let activity = manager.create_staking_activity(1);
        let activity = manager.staking_activities.get_mut(&1).unwrap();
        
        activity.stake_amount(1000);
        activity.start_unbonding(500, 86400); // 1 day
        
        assert_eq!(activity.get_unbonding_status().len(), 1);
        assert_eq!(activity.get_staking_status(), &StakingStatus::Unbonding);
    }

    #[test]
    fn test_slashing() {
        let mut manager = StakingManager::new();
        let activity = manager.create_staking_activity(1);
        let activity = manager.staking_activities.get_mut(&1).unwrap();
        
        activity.stake_amount(1000);
        activity.add_slash(100, SlashType::Offence, 1, "Misbehavior".to_string());
        
        assert_eq!(activity.get_slash_history().len(), 1);
        assert_eq!(activity.get_staked_amount(), 900);
    }

    #[test]
    fn test_nominator_activity() {
        let mut manager = StakingManager::new();
        let activity = manager.create_staking_activity(1);
        let activity = manager.staking_activities.get_mut(&1).unwrap();
        
        activity.add_nominator_activity(100, 500);
        activity.add_nominator_activity(200, 300);
        
        assert_eq!(activity.get_nominator_activities().len(), 2);
        assert_eq!(activity.total_nominated_amount(), 800);
    }

    #[test]
    fn test_activity_frequency() {
        let mut manager = StakingManager::new();
        let activity = manager.create_staking_activity(1);
        let activity = manager.staking_activities.get_mut(&1).unwrap();
        
        activity.set_staking_status(StakingStatus::Active);
        activity.set_staking_status(StakingStatus::Inactive);
        activity.set_staking_status(StakingStatus::Active);
        
        assert_eq!(activity.get_active_sessions(), 2);
        assert_eq!(activity.get_total_sessions(), 3);
        assert_eq!(activity.activity_frequency(), 2.0 / 3.0);
    }

    #[test]
    fn test_re_nominate() {
        let mut manager = StakingManager::new();
        let activity = manager.create_staking_activity(1);
        let activity = manager.staking_activities.get_mut(&1).unwrap();
        
        activity.add_nominator_activity(100, 500);
        activity.chill_staking();
        activity.re_nominate();
        
        assert_eq!(activity.get_staking_status(), &StakingStatus::Active);
    }

    #[test]
    fn test_staking_duration() {
        let mut manager = StakingManager::new();
        let activity = manager.create_staking_activity(1);
        let duration = activity.staking_duration();
        
        assert!(duration >= 0);
    }

    #[test]
    fn test_validator_creation() {
        let mut manager = StakingManager::new();
        let validator = manager.create_validator(1, 0.1);
        
        assert_eq!(validator.id, 1);
        assert_eq!(validator.commission, 0.1);
        assert_eq!(validator.total_stake, 0);
    }
}
