//! Mock Data Generation Module for Fully Testable Modules
//!
//! Provides comprehensive mock data generation for testing blockchain reputation systems
//! with realistic scenarios, edge cases, and error conditions.

use serde::{Deserialize, Serialize};
use scale::{Decode, Encode};

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{vec::Vec, string::String, collections::BTreeMap as HashMap};

#[cfg(feature = "std")]
use std::{vec::Vec, string::String, collections::HashMap};

/// Mock account data for testing
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, Serialize, Deserialize)]
pub struct MockAccount {
    /// Account identifier
    pub id: u32,
    
    /// Account address/hash
    pub address: String,
    
    /// Account name
    pub name: String,
    
    /// Current reputation score
    pub reputation_score: u64,
    
    /// Account creation timestamp
    pub created_at: u64,
    
    /// Account status
    pub is_active: bool,
}

impl MockAccount {
    /// Create a new mock account
    pub fn new(id: u32, address: &str, name: &str) -> Self {
        Self {
            id,
            address: address.to_string(),
            name: name.to_string(),
            reputation_score: 50,
            created_at: 1609459200, // 2021-01-01 00:00:00 UTC
            is_active: true,
        }
    }
    
    /// Create a high reputation account
    pub fn high_reputation(id: u32, address: &str, name: &str) -> Self {
        Self {
            reputation_score: 95,
            ..Self::new(id, address, name)
        }
    }
    
    /// Create a low reputation account
    pub fn low_reputation(id: u32, address: &str, name: &str) -> Self {
        Self {
            reputation_score: 15,
            ..Self::new(id, address, name)
        }
    }
    
    /// Create an inactive account
    pub fn inactive(id: u32, address: &str, name: &str) -> Self {
        Self {
            is_active: false,
            ..Self::new(id, address, name)
        }
    }
    
    /// Create an account with specific creation time
    pub fn with_creation_time(id: u32, address: &str, name: &str, created_at: u64) -> Self {
        Self {
            created_at,
            ..Self::new(id, address, name)
        }
    }
}

/// Mock staking data for testing
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, Serialize, Deserialize)]
pub struct MockStakeData {
    /// Staker account ID
    pub account_id: u32,
    
    /// Amount staked
    pub amount: u64,
    
    /// Staking duration in seconds
    pub duration: u64,
    
    /// Start timestamp
    pub start_time: u64,
    
    /// Whether stake is active
    pub is_active: bool,
    
    /// Number of rewards claimed
    pub rewards_claimed: u32,
}

impl MockStakeData {
    /// Create valid stake data
    pub fn valid(account_id: u32) -> Self {
        Self {
            account_id,
            amount: 1_000_000_000_000, // 1 million tokens
            duration: 2_592_000, // 30 days
            start_time: 1640995200, // 2022-01-01 00:00:00 UTC
            is_active: true,
            rewards_claimed: 0,
        }
    }
    
    /// Create large stake data
    pub fn large_stake(account_id: u32) -> Self {
        Self {
            amount: 10_000_000_000_000, // 10 million tokens
            ..Self::valid(account_id)
        }
    }
    
    /// Create small stake data
    pub fn small_stake(account_id: u32) -> Self {
        Self {
            amount: 100_000_000, // 100 tokens
            ..Self::valid(account_id)
        }
    }
    
    /// Create long duration stake
    pub fn long_duration(account_id: u32) -> Self {
        Self {
            duration: 31_536_000, // 365 days
            ..Self::valid(account_id)
        }
    }
    
    /// Create short duration stake
    pub fn short_duration(account_id: u32) -> Self {
        Self {
            duration: 86_400, // 1 day
            ..Self::valid(account_id)
        }
    }
    
    /// Create inactive stake
    pub fn inactive(account_id: u32) -> Self {
        Self {
            is_active: false,
            ..Self::valid(account_id)
        }
    }
    
    /// Create stake with claimed rewards
    pub fn with_rewards(account_id: u32, rewards: u32) -> Self {
        Self {
            rewards_claimed: rewards,
            ..Self::valid(account_id)
        }
    }
}

/// Mock governance activity data
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, Serialize, Deserialize)]
pub struct MockGovernanceActivity {
    /// Account ID
    pub account_id: u32,
    
    /// Number of votes cast
    pub votes_count: u32,
    
    /// Number of proposals submitted
    pub proposals_count: u32,
    
    /// Number of proposals approved
    pub proposals_approved: u32,
    
    /// Last activity timestamp
    pub last_activity: u64,
    
    /// Total voting power used
    pub voting_power_used: u64,
}

impl MockGovernanceActivity {
    /// Create active governance participant
    pub fn active_participant(account_id: u32) -> Self {
        Self {
            account_id,
            votes_count: 50,
            proposals_count: 5,
            proposals_approved: 3,
            last_activity: 1672531200, // 2023-01-01 00:00:00 UTC
            voting_power_used: 5000,
        }
    }
    
    /// Create high activity participant
    pub fn high_activity(account_id: u32) -> Self {
        Self {
            votes_count: 500,
            proposals_count: 50,
            proposals_approved: 30,
            voting_power_used: 50000,
            ..Self::active_participant(account_id)
        }
    }
    
    /// Create low activity participant
    pub fn low_activity(account_id: u32) -> Self {
        Self {
            votes_count: 5,
            proposals_count: 1,
            proposals_approved: 0,
            voting_power_used: 500,
            ..Self::active_participant(account_id)
        }
    }
    
    /// Create inactive participant
    pub fn inactive_participant(account_id: u32) -> Self {
        Self {
            votes_count: 0,
            proposals_count: 0,
            proposals_approved: 0,
            voting_power_used: 0,
            ..Self::active_participant(account_id)
        }
    }
    
    /// Create participant with recent activity
    pub fn recent_activity(account_id: u32) -> Self {
        Self {
            last_activity: 1699430400, // Recent timestamp
            ..Self::active_participant(account_id)
        }
    }
}

/// Mock identity verification data
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, Serialize, Deserialize)]
pub struct MockIdentityData {
    /// Account ID
    pub account_id: u32,
    
    /// Whether identity is verified
    pub is_verified: bool,
    
    /// Number of judgements received
    pub judgements_count: u32,
    
    /// Identity confidence score (0-100)
    pub confidence_score: u32,
    
    /// Verification timestamp
    pub verified_at: u64,
    
    /// Identity type
    pub identity_type: IdentityType,
}

/// Identity verification types
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, Serialize, Deserialize)]
pub enum IdentityType {
    /// Basic wallet verification
    Wallet,
    
    /// Email verification
    Email,
    
    /// Phone verification
    Phone,
    
    /// Government ID verification
    GovernmentId,
    
    /// Biometric verification
    Biometric,
    
    /// Social media verification
    SocialMedia,
}

impl MockIdentityData {
    /// Create verified identity
    pub fn verified(account_id: u32) -> Self {
        Self {
            account_id,
            is_verified: true,
            judgements_count: 3,
            confidence_score: 90,
            verified_at: 1640995200,
            identity_type: IdentityType::GovernmentId,
        }
    }
    
    /// Create unverified identity
    pub fn unverified(account_id: u32) -> Self {
        Self {
            account_id,
            is_verified: false,
            judgements_count: 0,
            confidence_score: 0,
            verified_at: 0,
            identity_type: IdentityType::Wallet,
        }
    }
    
    /// Create high confidence identity
    pub fn high_confidence(account_id: u32) -> Self {
        Self {
            confidence_score: 98,
            judgements_count: 5,
            ..Self::verified(account_id)
        }
    }
    
    /// Create low confidence identity
    pub fn low_confidence(account_id: u32) -> Self {
        Self {
            confidence_score: 30,
            judgements_count: 1,
            ..Self::verified(account_id)
        }
    }
    
    /// Create recently verified identity
    pub fn recent_verification(account_id: u32) -> Self {
        Self {
            verified_at: 1699430400,
            ..Self::verified(account_id)
        }
    }
}

/// Mock community engagement data
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, Serialize, Deserialize)]
pub struct MockCommunityData {
    /// Account ID
    pub account_id: u32,
    
    /// Number of posts created
    pub posts_count: u32,
    
    /// Number of comments made
    pub comments_count: u32,
    
    /// Number of upvotes received
    pub upvotes_received: u32,
    
    /// Number of downvotes received
    pub downvotes_received: u32,
    
    /// Community reputation score
    pub community_score: u32,
    
    /// Last activity timestamp
    pub last_activity: u64,
}

impl MockCommunityData {
    /// Create active community member
    pub fn active_member(account_id: u32) -> Self {
        Self {
            account_id,
            posts_count: 100,
            comments_count: 500,
            upvotes_received: 2500,
            downvotes_received: 50,
            community_score: 85,
            last_activity: 1699430400,
        }
    }
    
    /// Create high engagement member
    pub fn high_engagement(account_id: u32) -> Self {
        Self {
            posts_count: 500,
            comments_count: 2500,
            upvotes_received: 15000,
            downvotes_received: 100,
            community_score: 95,
            ..Self::active_member(account_id)
        }
    }
    
    /// Create low engagement member
    pub fn low_engagement(account_id: u32) -> Self {
        Self {
            posts_count: 5,
            comments_count: 25,
            upvotes_received: 50,
            downvotes_received: 10,
            community_score: 45,
            ..Self::active_member(account_id)
        }
    }
    
    /// Create inactive member
    pub fn inactive_member(account_id: u32) -> Self {
        Self {
            posts_count: 0,
            comments_count: 0,
            upvotes_received: 0,
            downvotes_received: 0,
            community_score: 0,
            last_activity: 0,
            ..Self::active_member(account_id)
        }
    }
}

/// Mock data generator for comprehensive testing
pub struct MockDataGenerator;

impl MockDataGenerator {
    /// Generate a set of mock accounts
    pub fn generate_accounts(count: usize) -> Vec<MockAccount> {
        let mut accounts = Vec::new();
        
        for i in 0..count {
            let account = match i % 4 {
                0 => MockAccount::high_reputation(i as u32, &format!("5G{}", i), &format!("User{}", i)),
                1 => MockAccount::low_reputation(i as u32, &format!("5F{}", i), &format!("User{}", i)),
                2 => MockAccount::inactive(i as u32, &format!("5E{}", i), &format!("User{}", i)),
                _ => MockAccount::new(i as u32, &format!("5D{}", i), &format!("User{}", i)),
            };
            
            accounts.push(account);
        }
        
        accounts
    }
    
    /// Generate mock staking data
    pub fn generate_stake_data(account_ids: &[u32]) -> Vec<MockStakeData> {
        let mut stakes = Vec::new();
        
        for (i, &account_id) in account_ids.iter().enumerate() {
            let stake = match i % 5 {
                0 => MockStakeData::large_stake(account_id),
                1 => MockStakeData::small_stake(account_id),
                2 => MockStakeData::long_duration(account_id),
                3 => MockStakeData::short_duration(account_id),
                _ => MockStakeData::valid(account_id),
            };
            
            stakes.push(stake);
        }
        
        stakes
    }
    
    /// Generate mock governance activity
    pub fn generate_governance_activity(account_ids: &[u32]) -> Vec<MockGovernanceActivity> {
        let mut activities = Vec::new();
        
        for (i, &account_id) in account_ids.iter().enumerate() {
            let activity = match i % 4 {
                0 => MockGovernanceActivity::high_activity(account_id),
                1 => MockGovernanceActivity::low_activity(account_id),
                2 => MockGovernanceActivity::inactive_participant(account_id),
                _ => MockGovernanceActivity::active_participant(account_id),
            };
            
            activities.push(activity);
        }
        
        activities
    }
    
    /// Generate mock identity data
    pub fn generate_identity_data(account_ids: &[u32]) -> Vec<MockIdentityData> {
        let mut identities = Vec::new();
        
        for (i, &account_id) in account_ids.iter().enumerate() {
            let identity = match i % 4 {
                0 => MockIdentityData::high_confidence(account_id),
                1 => MockIdentityData::low_confidence(account_id),
                2 => MockIdentityData::unverified(account_id),
                _ => MockIdentityData::verified(account_id),
            };
            
            identities.push(identity);
        }
        
        identities
    }
    
    /// Generate mock community data
    pub fn generate_community_data(account_ids: &[u32]) -> Vec<MockCommunityData> {
        let mut communities = Vec::new();
        
        for (i, &account_id) in account_ids.iter().enumerate() {
            let community = match i % 4 {
                0 => MockCommunityData::high_engagement(account_id),
                1 => MockCommunityData::low_engagement(account_id),
                2 => MockCommunityData::inactive_member(account_id),
                _ => MockCommunityData::active_member(account_id),
            };
            
            communities.push(community);
        }
        
        communities
    }
    
    /// Generate complete mock dataset
    pub fn generate_complete_dataset(accounts_count: usize) -> MockDataset {
        let accounts = Self::generate_accounts(accounts_count);
        let account_ids: Vec<u32> = accounts.iter().map(|a| a.id).collect();
        
        MockDataset {
            accounts,
            stake_data: Self::generate_stake_data(&account_ids),
            governance_activities: Self::generate_governance_activity(&account_ids),
            identity_data: Self::generate_identity_data(&account_ids),
            community_data: Self::generate_community_data(&account_ids),
        }
    }
}

/// Complete mock dataset for testing
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MockDataset {
    /// Mock accounts
    pub accounts: Vec<MockAccount>,
    
    /// Mock staking data
    pub stake_data: Vec<MockStakeData>,
    
    /// Mock governance activities
    pub governance_activities: Vec<MockGovernanceActivity>,
    
    /// Mock identity data
    pub identity_data: Vec<MockIdentityData>,
    
    /// Mock community data
    pub community_data: Vec<MockCommunityData>,
}

impl MockDataset {
    /// Get account by ID
    pub fn get_account(&self, id: u32) -> Option<&MockAccount> {
        self.accounts.iter().find(|a| a.id == id)
    }
    
    /// Get stake data by account ID
    pub fn get_stake_data(&self, account_id: u32) -> Option<&MockStakeData> {
        self.stake_data.iter().find(|s| s.account_id == account_id)
    }
    
    /// Get governance activity by account ID
    pub fn get_governance_activity(&self, account_id: u32) -> Option<&MockGovernanceActivity> {
        self.governance_activities.iter().find(|g| g.account_id == account_id)
    }
    
    /// Get identity data by account ID
    pub fn get_identity_data(&self, account_id: u32) -> Option<&MockIdentityData> {
        self.identity_data.iter().find(|i| i.account_id == account_id)
    }
    
    /// Get community data by account ID
    pub fn get_community_data(&self, account_id: u32) -> Option<&MockCommunityData> {
        self.community_data.iter().find(|c| c.account_id == account_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_account_creation() {
        let account = MockAccount::new(1, "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", "Alice");
        assert_eq!(account.id, 1);
        assert_eq!(account.name, "Alice");
        assert_eq!(account.reputation_score, 50);
        assert!(account.is_active);
    }

    #[test]
    fn test_specialized_account_types() {
        let high_rep = MockAccount::high_reputation(2, "5FH", "Bob");
        assert_eq!(high_rep.reputation_score, 95);
        
        let low_rep = MockAccount::low_reputation(3, "5FL", "Charlie");
        assert_eq!(low_rep.reputation_score, 15);
        
        let inactive = MockAccount::inactive(4, "5FI", "Dave");
        assert!(!inactive.is_active);
    }

    #[test]
    fn test_mock_stake_data() {
        let stake = MockStakeData::valid(1);
        assert_eq!(stake.account_id, 1);
        assert_eq!(stake.amount, 1_000_000_000_000);
        assert!(stake.is_active);
        
        let large = MockStakeData::large_stake(2);
        assert_eq!(large.amount, 10_000_000_000_000);
        
        let small = MockStakeData::small_stake(3);
        assert_eq!(small.amount, 100_000_000);
    }

    #[test]
    fn test_stake_variations() {
        let long = MockStakeData::long_duration(1);
        assert_eq!(long.duration, 31_536_000);
        
        let short = MockStakeData::short_duration(2);
        assert_eq!(short.duration, 86_400);
        
        let inactive = MockStakeData::inactive(3);
        assert!(!inactive.is_active);
        
        let rewarded = MockStakeData::with_rewards(4, 5);
        assert_eq!(rewarded.rewards_claimed, 5);
    }

    #[test]
    fn test_governance_activity() {
        let active = MockGovernanceActivity::active_participant(1);
        assert_eq!(active.account_id, 1);
        assert_eq!(active.votes_count, 50);
        assert_eq!(active.proposals_count, 5);
        
        let high = MockGovernanceActivity::high_activity(2);
        assert_eq!(high.votes_count, 500);
        
        let low = MockGovernanceActivity::low_activity(3);
        assert_eq!(low.votes_count, 5);
        
        let inactive = MockGovernanceActivity::inactive_participant(4);
        assert_eq!(inactive.votes_count, 0);
    }

    #[test]
    fn test_identity_data() {
        let verified = MockIdentityData::verified(1);
        assert!(verified.is_verified);
        assert_eq!(verified.confidence_score, 90);
        assert_eq!(verified.judgements_count, 3);
        
        let unverified = MockIdentityData::unverified(2);
        assert!(!unverified.is_verified);
        assert_eq!(unverified.confidence_score, 0);
        
        let high_conf = MockIdentityData::high_confidence(3);
        assert_eq!(high_conf.confidence_score, 98);
        
        let low_conf = MockIdentityData::low_confidence(4);
        assert_eq!(low_conf.confidence_score, 30);
    }

    #[test]
    fn test_community_data() {
        let active = MockCommunityData::active_member(1);
        assert_eq!(active.account_id, 1);
        assert_eq!(active.posts_count, 100);
        assert_eq!(active.community_score, 85);
        
        let high = MockCommunityData::high_engagement(2);
        assert_eq!(high.posts_count, 500);
        assert_eq!(high.community_score, 95);
        
        let low = MockCommunityData::low_engagement(3);
        assert_eq!(low.posts_count, 5);
        assert_eq!(low.community_score, 45);
        
        let inactive = MockCommunityData::inactive_member(4);
        assert_eq!(inactive.posts_count, 0);
        assert_eq!(inactive.community_score, 0);
    }

    #[test]
    fn test_mock_data_generator() {
        let accounts = MockDataGenerator::generate_accounts(10);
        assert_eq!(accounts.len(), 10);
        
        // Check that we have different types of accounts
        let high_rep_count = accounts.iter().filter(|a| a.reputation_score >= 90).count();
        let low_rep_count = accounts.iter().filter(|a| a.reputation_score <= 20).count();
        let inactive_count = accounts.iter().filter(|a| !a.is_active).count();
        
        assert!(high_rep_count > 0);
        assert!(low_rep_count > 0);
        assert!(inactive_count > 0);
    }

    #[test]
    fn test_stake_data_generation() {
        let account_ids = vec![1, 2, 3, 4, 5];
        let stakes = MockDataGenerator::generate_stake_data(&account_ids);
        assert_eq!(stakes.len(), 5);
        
        // Check that we have different stake types
        let large_count = stakes.iter().filter(|s| s.amount > 5_000_000_000_000).count();
        let small_count = stakes.iter().filter(|s| s.amount < 500_000_000).count();
        
        assert!(large_count > 0);
        assert!(small_count > 0);
    }

    #[test]
    fn test_governance_data_generation() {
        let account_ids = vec![1, 2, 3, 4];
        let activities = MockDataGenerator::generate_governance_activity(&account_ids);
        assert_eq!(activities.len(), 4);
        
        // Check that we have different activity levels
        let high_count = activities.iter().filter(|a| a.votes_count > 100).count();
        let low_count = activities.iter().filter(|a| a.votes_count < 10).count();
        let inactive_count = activities.iter().filter(|a| a.votes_count == 0).count();
        
        assert!(high_count > 0);
        assert!(low_count > 0);
        assert!(inactive_count > 0);
    }

    #[test]
    fn test_identity_data_generation() {
        let account_ids = vec![1, 2, 3, 4];
        let identities = MockDataGenerator::generate_identity_data(&account_ids);
        assert_eq!(identities.len(), 4);
        
        // Check that we have different identity confidence levels
        let high_conf_count = identities.iter().filter(|i| i.confidence_score > 90).count();
        let low_conf_count = identities.iter().filter(|i| i.confidence_score < 50).count();
        let unverified_count = identities.iter().filter(|i| !i.is_verified).count();
        
        assert!(high_conf_count > 0);
        assert!(low_conf_count > 0);
        assert!(unverified_count > 0);
    }

    #[test]
    fn test_community_data_generation() {
        let account_ids = vec![1, 2, 3, 4];
        let communities = MockDataGenerator::generate_community_data(&account_ids);
        assert_eq!(communities.len(), 4);
        
        // Check that we have different engagement levels
        let high_count = communities.iter().filter(|c| c.community_score > 90).count();
        let low_count = communities.iter().filter(|c| c.community_score < 50).count();
        let inactive_count = communities.iter().filter(|c| c.community_score == 0).count();
        
        assert!(high_count > 0);
        assert!(low_count > 0);
        assert!(inactive_count > 0);
    }

    #[test]
    fn test_complete_dataset_generation() {
        let dataset = MockDataGenerator::generate_complete_dataset(5);
        assert_eq!(dataset.accounts.len(), 5);
        assert_eq!(dataset.stake_data.len(), 5);
        assert_eq!(dataset.governance_activities.len(), 5);
        assert_eq!(dataset.identity_data.len(), 5);
        assert_eq!(dataset.community_data.len(), 5);
    }

    #[test]
    fn test_dataset_accessors() {
        let dataset = MockDataGenerator::generate_complete_dataset(3);
        
        // Test account access
        let account = dataset.get_account(0);
        assert!(account.is_some());
        assert_eq!(account.unwrap().id, 0);
        
        // Test stake data access
        let stake = dataset.get_stake_data(1);
        assert!(stake.is_some());
        assert_eq!(stake.unwrap().account_id, 1);
        
        // Test governance activity access
        let gov = dataset.get_governance_activity(2);
        assert!(gov.is_some());
        assert_eq!(gov.unwrap().account_id, 2);
        
        // Test identity data access
        let identity = dataset.get_identity_data(0);
        assert!(identity.is_some());
        assert_eq!(identity.unwrap().account_id, 0);
        
        // Test community data access
        let community = dataset.get_community_data(1);
        assert!(community.is_some());
        assert_eq!(community.unwrap().account_id, 1);
    }

    #[test]
    fn test_edge_cases() {
        // Test account with empty name
        let account = MockAccount::new(999, "5G", "");
        assert_eq!(account.name, "");
        
        // Test stake with zero amount
        let mut stake = MockStakeData::valid(999);
        stake.amount = 0;
        assert_eq!(stake.amount, 0);
        
        // Test governance with zero votes
        let mut gov = MockGovernanceActivity::active_participant(999);
        gov.votes_count = 0;
        assert_eq!(gov.votes_count, 0);
        
        // Test identity with zero confidence
        let mut identity = MockIdentityData::verified(999);
        identity.confidence_score = 0;
        assert_eq!(identity.confidence_score, 0);
        
        // Test community with zero posts
        let mut community = MockCommunityData::active_member(999);
        community.posts_count = 0;
        assert_eq!(community.posts_count, 0);
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod integration_tests {
    use super::*;

    #[test]
    fn integration_test_complete_mock_workflow() {
        // Generate a complete dataset
        let dataset = MockDataGenerator::generate_complete_dataset(10);
        
        // Verify dataset structure
        assert_eq!(dataset.accounts.len(), 10);
        assert_eq!(dataset.stake_data.len(), 10);
        assert_eq!(dataset.governance_activities.len(), 10);
        assert_eq!(dataset.identity_data.len(), 10);
        assert_eq!(dataset.community_data.len(), 10);
        
        // Verify cross-references
        for account in &dataset.accounts {
            // Every account should have corresponding data in other collections
            assert!(dataset.get_stake_data(account.id).is_some());
            assert!(dataset.get_governance_activity(account.id).is_some());
            assert!(dataset.get_identity_data(account.id).is_some());
            assert!(dataset.get_community_data(account.id).is_some());
        }
        
        // Test data variety
        let high_rep_accounts = dataset.accounts.iter().filter(|a| a.reputation_score >= 90).count();
        let low_rep_accounts = dataset.accounts.iter().filter(|a| a.reputation_score <= 20).count();
        let inactive_accounts = dataset.accounts.iter().filter(|a| !a.is_active).count();
        
        assert!(high_rep_accounts > 0);
        assert!(low_rep_accounts > 0);
        assert!(inactive_accounts > 0);
        
        // Test stake variety
        let large_stakes = dataset.stake_data.iter().filter(|s| s.amount > 5_000_000_000_000).count();
        let small_stakes = dataset.stake_data.iter().filter(|s| s.amount < 500_000_000).count();
        let inactive_stakes = dataset.stake_data.iter().filter(|s| !s.is_active).count();
        
        assert!(large_stakes > 0);
        assert!(small_stakes > 0);
        assert!(inactive_stakes > 0);
    }

    #[test]
    fn integration_test_data_consistency() {
        // Generate dataset
        let dataset = MockDataGenerator::generate_complete_dataset(5);
        
        // Verify all account IDs are consistent across collections
        let account_ids: Vec<u32> = dataset.accounts.iter().map(|a| a.id).collect();
        
        for stake in &dataset.stake_data {
            assert!(account_ids.contains(&stake.account_id));
        }
        
        for gov in &dataset.governance_activities {
            assert!(account_ids.contains(&gov.account_id));
        }
        
        for identity in &dataset.identity_data {
            assert!(account_ids.contains(&identity.account_id));
        }
        
        for community in &dataset.community_data {
            assert!(account_ids.contains(&community.account_id));
        }
    }

    #[test]
    fn integration_test_scenario_generation() {
        // Test generating specific scenarios
        let account_ids = vec![100, 200, 300, 400, 500];
        
        let stakes = MockDataGenerator::generate_stake_data(&account_ids);
        let gov_activities = MockDataGenerator::generate_governance_activity(&account_ids);
        let identities = MockDataGenerator::generate_identity_data(&account_ids);
        let communities = MockDataGenerator::generate_community_data(&account_ids);
        
        // Verify all generated data uses the provided account IDs
        for stake in &stakes {
            assert!(account_ids.contains(&stake.account_id));
        }
        
        for gov in &gov_activities {
            assert!(account_ids.contains(&gov.account_id));
        }
        
        for identity in &identities {
            assert!(account_ids.contains(&identity.account_id));
        }
        
        for community in &communities {
            assert!(account_ids.contains(&community.account_id));
        }
        
        // Verify we have the expected number of items
        assert_eq!(stakes.len(), 5);
        assert_eq!(gov_activities.len(), 5);
        assert_eq!(identities.len(), 5);
        assert_eq!(communities.len(), 5);
    }
}
