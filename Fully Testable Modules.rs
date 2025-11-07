use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Mockable time provider trait for deterministic testing
pub trait TimeProvider {
    fn now(&self) -> u64;
}

// Real time provider
pub struct RealTimeProvider;

impl TimeProvider for RealTimeProvider {
    fn now(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }
}

// Mock time provider for testing
#[cfg(test)]
pub struct MockTimeProvider {
    pub current_time: u64,
}

#[cfg(test)]
impl TimeProvider for MockTimeProvider {
    fn now(&self) -> u64 {
        self.current_time
    }
}

// Activity types
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum ActivityType {
    Voting,
    Proposal,
    Staking,
    Governance,
    Community,
}

// Activity record
#[derive(Debug, Clone)]
pub struct ActivityRecord {
    pub id: u32,
    pub account_id: u32,
    pub activity_type: ActivityType,
    pub timestamp: u64,
    pub success: bool,
    pub metadata: HashMap<String, String>,
}

// Testable module
pub struct FullyTestableModule<T: TimeProvider> {
    pub activities: Vec<ActivityRecord>,
    pub account_activities: HashMap<u32, Vec<u32>>, // account_id -> activity_ids
    pub time_provider: T,
}

impl<T: TimeProvider> FullyTestableModule<T> {
    pub fn new(time_provider: T) -> Self {
        FullyTestableModule {
            activities: Vec::new(),
            account_activities: HashMap::new(),
            time_provider,
        }
    }

    // Add activity - fully testable with deterministic time
    pub fn add_activity(
        &mut self,
        account_id: u32,
        activity_type: ActivityType,
        success: bool,
        metadata: HashMap<String, String>,
    ) -> u32 {
        let timestamp = self.time_provider.now();
        let id = self.activities.len() as u32 + 1;
        
        let activity = ActivityRecord {
            id,
            account_id,
            activity_type,
            timestamp,
            success,
            metadata,
        };
        
        self.activities.push(activity);
        
        // Index by account
        self.account_activities
            .entry(account_id)
            .or_insert_with(Vec::new)
            .push(id);
            
        id
    }

    // Get activities for an account
    pub fn get_account_activities(&self, account_id: u32) -> Vec<&ActivityRecord> {
        if let Some(activity_ids) = self.account_activities.get(&account_id) {
            activity_ids
                .iter()
                .filter_map(|&id| self.activities.get((id - 1) as usize))
                .collect()
        } else {
            Vec::new()
        }
    }

    // Get activity by ID
    pub fn get_activity(&self, id: u32) -> Option<&ActivityRecord> {
        self.activities.get((id - 1) as usize)
    }

    // Get all activities
    pub fn get_all_activities(&self) -> &Vec<ActivityRecord> {
        &self.activities
    }

    // Get activity count
    pub fn get_activity_count(&self) -> usize {
        self.activities.len()
    }

    // Get successful activities count
    pub fn get_successful_activities_count(&self) -> usize {
        self.activities.iter().filter(|a| a.success).count()
    }

    // Get failed activities count
    pub fn get_failed_activities_count(&self) -> usize {
        self.activities.iter().filter(|a| !a.success).count()
    }

    // Get activities by type
    pub fn get_activities_by_type(&self, activity_type: ActivityType) -> Vec<&ActivityRecord> {
        self.activities
            .iter()
            .filter(|a| a.activity_type == activity_type)
            .collect()
    }

    // Get recent activities (last 30 days)
    pub fn get_recent_activities(&self, days: u64) -> Vec<&ActivityRecord> {
        let now = self.time_provider.now();
        let cutoff = now - (days * 24 * 60 * 60);
        
        self.activities
            .iter()
            .filter(|a| a.timestamp > cutoff)
            .collect()
    }
}

// Convenience type for production use
pub type ProductionTestableModule = FullyTestableModule<RealTimeProvider>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_creation() {
        let time_provider = MockTimeProvider { current_time: 1000 };
        let module = FullyTestableModule::new(time_provider);
        
        assert_eq!(module.get_activity_count(), 0);
        assert_eq!(module.get_successful_activities_count(), 0);
        assert_eq!(module.get_failed_activities_count(), 0);
    }

    #[test]
    fn test_add_activity() {
        let time_provider = MockTimeProvider { current_time: 1000 };
        let mut module = FullyTestableModule::new(time_provider);
        
        let mut metadata = HashMap::new();
        metadata.insert("test".to_string(), "value".to_string());
        
        let activity_id = module.add_activity(
            1,
            ActivityType::Voting,
            true,
            metadata.clone(),
        );
        
        assert_eq!(activity_id, 1);
        assert_eq!(module.get_activity_count(), 1);
        assert_eq!(module.get_successful_activities_count(), 1);
        assert_eq!(module.get_failed_activities_count(), 0);
        
        let activity = module.get_activity(1).unwrap();
        assert_eq!(activity.id, 1);
        assert_eq!(activity.account_id, 1);
        assert_eq!(activity.activity_type, ActivityType::Voting);
        assert_eq!(activity.timestamp, 1000);
        assert_eq!(activity.success, true);
        assert_eq!(activity.metadata, metadata);
    }

    #[test]
    fn test_get_account_activities() {
        let time_provider = MockTimeProvider { current_time: 1000 };
        let mut module = FullyTestableModule::new(time_provider);
        
        // Add activities for different accounts
        module.add_activity(1, ActivityType::Voting, true, HashMap::new());
        module.add_activity(2, ActivityType::Proposal, false, HashMap::new());
        module.add_activity(1, ActivityType::Staking, true, HashMap::new());
        
        // Check account 1 activities
        let account1_activities = module.get_account_activities(1);
        assert_eq!(account1_activities.len(), 2);
        assert_eq!(account1_activities[0].account_id, 1);
        assert_eq!(account1_activities[1].account_id, 1);
        
        // Check account 2 activities
        let account2_activities = module.get_account_activities(2);
        assert_eq!(account2_activities.len(), 1);
        assert_eq!(account2_activities[0].account_id, 2);
        
        // Check non-existent account
        let empty_activities = module.get_account_activities(3);
        assert_eq!(empty_activities.len(), 0);
    }

    #[test]
    fn test_get_activities_by_type() {
        let time_provider = MockTimeProvider { current_time: 1000 };
        let mut module = FullyTestableModule::new(time_provider);
        
        // Add different types of activities
        module.add_activity(1, ActivityType::Voting, true, HashMap::new());
        module.add_activity(2, ActivityType::Proposal, false, HashMap::new());
        module.add_activity(3, ActivityType::Voting, true, HashMap::new());
        module.add_activity(4, ActivityType::Staking, false, HashMap::new());
        
        // Check voting activities
        let voting_activities = module.get_activities_by_type(ActivityType::Voting);
        assert_eq!(voting_activities.len(), 2);
        
        // Check proposal activities
        let proposal_activities = module.get_activities_by_type(ActivityType::Proposal);
        assert_eq!(proposal_activities.len(), 1);
        
        // Check staking activities
        let staking_activities = module.get_activities_by_type(ActivityType::Staking);
        assert_eq!(staking_activities.len(), 1);
        
        // Check community activities (none added)
        let community_activities = module.get_activities_by_type(ActivityType::Community);
        assert_eq!(community_activities.len(), 0);
    }

    #[test]
    fn test_get_recent_activities() {
        let time_provider = MockTimeProvider { current_time: 1000000 };
        let mut module = FullyTestableModule::new(time_provider);
        
        // Add activities at different times
        module.time_provider.current_time = 1000000; // 1M seconds
        module.add_activity(1, ActivityType::Voting, true, HashMap::new());
        
        module.time_provider.current_time = 900000; // 900K seconds (100K seconds ago)
        module.add_activity(2, ActivityType::Proposal, false, HashMap::new());
        
        module.time_provider.current_time = 800000; // 800K seconds (200K seconds ago)
        module.add_activity(3, ActivityType::Staking, true, HashMap::new());
        
        // Get activities from last 2 days (172800 seconds)
        module.time_provider.current_time = 1000000;
        let recent_activities = module.get_recent_activities(2);
        assert_eq!(recent_activities.len(), 2); // Only first two should be recent
        
        // Get activities from last 5 days (432000 seconds)
        let recent_activities = module.get_recent_activities(5);
        assert_eq!(recent_activities.len(), 3); // All should be recent
    }

    #[test]
    fn test_deterministic_behavior() {
        // Test that the module behaves deterministically with mock time
        let time_provider1 = MockTimeProvider { current_time: 5000 };
        let mut module1 = FullyTestableModule::new(time_provider1);
        
        let time_provider2 = MockTimeProvider { current_time: 5000 };
        let mut module2 = FullyTestableModule::new(time_provider2);
        
        // Add identical activities to both modules
        let mut metadata1 = HashMap::new();
        metadata1.insert("key1".to_string(), "value1".to_string());
        
        let mut metadata2 = HashMap::new();
        metadata2.insert("key1".to_string(), "value1".to_string());
        
        module1.add_activity(1, ActivityType::Voting, true, metadata1);
        module2.add_activity(1, ActivityType::Voting, true, metadata2);
        
        // Both modules should have identical state
        assert_eq!(module1.get_activity_count(), module2.get_activity_count());
        assert_eq!(module1.get_successful_activities_count(), module2.get_successful_activities_count());
        
        let activity1 = module1.get_activity(1).unwrap();
        let activity2 = module2.get_activity(1).unwrap();
        
        assert_eq!(activity1.id, activity2.id);
        assert_eq!(activity1.account_id, activity2.account_id);
        assert_eq!(activity1.activity_type, activity2.activity_type);
        assert_eq!(activity1.timestamp, activity2.timestamp);
        assert_eq!(activity1.success, activity2.success);
        assert_eq!(activity1.metadata, activity2.metadata);
    }

    #[test]
    fn test_error_scenarios() {
        let time_provider = MockTimeProvider { current_time: 1000 };
        let module = FullyTestableModule::new(time_provider);
        
        // Test getting non-existent activity
        assert!(module.get_activity(999).is_none());
        
        // Test getting activities for non-existent account
        assert_eq!(module.get_account_activities(999).len(), 0);
    }
}
