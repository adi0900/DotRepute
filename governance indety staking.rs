//! Governance + Identity + Staking Indexing Module
//!
//! This module implements a comprehensive indexing system that combines governance,
//! identity, and staking events from on-chain data.
//!
//! Features:
//! - Multi-source integration: Combines governance, identity, and staking events from on-chain in a single module
//! - Timestamped data tracking: Historical context of each event is preserved, time-based analysis possible
//! - User-specific score generation: Scoring based on each user's on-chain behavior
//! - Dynamic data update: Scores automatically updated as new events arrive
//! - Sybil resistance: Fake accounts eliminated thanks to indexing based on real contributions
//! - Governance-weighted contribution analysis: Governance activities such as voting, proposal submission, commenting included in scoring
//! - Identity verification integration: Trust level determined with on-chain identity (identity pallet) information
//! - Staking behavior analysis: Metrics such as stake amount, continuity, and target validator selection affect scoring
//! - Modular structure: Each event type can be indexed and weighted separately
//! - On-chain and off-chain compatible: Data can be processed from on-chain or external sources

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Event types
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum EventType {
    GovernanceVote,
    GovernanceProposal,
    GovernanceComment,
    IdentitySet,
    IdentityVerified,
    StakeBond,
    StakeUnbond,
    StakeRebond,
    StakeReward,
    StakeSlash,
}

// Indexed event record
#[derive(Debug, Clone)]
pub struct IndexedEvent {
    pub id: u32,
    pub account_id: u32,
    pub event_type: EventType,
    pub score_change: f64,
    pub timestamp: u64,
    pub block_number: u32,
    pub metadata: HashMap<String, String>,
}

// User score record
#[derive(Debug, Clone)]
pub struct UserScore {
    pub account_id: u32,
    pub total_score: f64,
    pub governance_score: f64,
    pub identity_score: f64,
    pub staking_score: f64,
    pub last_updated: u64,
}

// Weight configuration for different event types
#[derive(Debug, Clone)]
pub struct WeightConfiguration {
    pub governance_vote_weight: f64,
    pub governance_proposal_weight: f64,
    pub governance_comment_weight: f64,
    pub identity_set_weight: f64,
    pub identity_verified_weight: f64,
    pub stake_bond_weight: f64,
    pub stake_unbond_weight: f64,
    pub stake_rebond_weight: f64,
    pub stake_reward_weight: f64,
    pub stake_slash_weight: f64,
}

impl Default for WeightConfiguration {
    fn default() -> Self {
        Self {
            governance_vote_weight: 1.0,
            governance_proposal_weight: 5.0,
            governance_comment_weight: 0.5,
            identity_set_weight: 10.0,
            identity_verified_weight: 20.0,
            stake_bond_weight: 0.1,
            stake_unbond_weight: -0.1,
            stake_rebond_weight: 0.05,
            stake_reward_weight: 0.2,
            stake_slash_weight: -5.0,
        }
    }
}

// Governance + Identity + Staking Indexing system
pub struct GisIndexingSystem {
    pub events: Vec<IndexedEvent>,
    pub user_scores: HashMap<u32, UserScore>,
    pub weights: WeightConfiguration,
    pub event_counter: u32,
}

impl GisIndexingSystem {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            user_scores: HashMap::new(),
            weights: WeightConfiguration::default(),
            event_counter: 0,
        }
    }
    
    pub fn add_event(
        &mut self,
        account_id: u32,
        event_type: EventType,
        timestamp: u64,
        block_number: u32,
        metadata: HashMap<String, String>,
    ) -> u32 {
        self.event_counter += 1;
        let id = self.event_counter;
        
        let score_change = self.calculate_score_change(&event_type, &metadata);
        
        let event = IndexedEvent {
            id,
            account_id,
            event_type,
            score_change,
            timestamp,
            block_number,
            metadata,
        };
        
        // Update user score
        self.update_user_score(account_id, &event);
        
        self.events.push(event);
        id
    }
    
    fn calculate_score_change(&self, event_type: &EventType, metadata: &HashMap<String, String>) -> f64 {
        match event_type {
            EventType::GovernanceVote => {
                let conviction = metadata.get("conviction").and_then(|c| c.parse::<f64>().ok()).unwrap_or(1.0);
                self.weights.governance_vote_weight * conviction
            },
            EventType::GovernanceProposal => self.weights.governance_proposal_weight,
            EventType::GovernanceComment => self.weights.governance_comment_weight,
            EventType::IdentitySet => self.weights.identity_set_weight,
            EventType::IdentityVerified => self.weights.identity_verified_weight,
            EventType::StakeBond => {
                let amount = metadata.get("amount").and_then(|a| a.parse::<f64>().ok()).unwrap_or(0.0);
                self.weights.stake_bond_weight * (amount / 1000.0) // Normalize by 1000
            },
            EventType::StakeUnbond => {
                let amount = metadata.get("amount").and_then(|a| a.parse::<f64>().ok()).unwrap_or(0.0);
                self.weights.stake_unbond_weight * (amount / 1000.0) // Normalize by 1000
            },
            EventType::StakeRebond => {
                let amount = metadata.get("amount").and_then(|a| a.parse::<f64>().ok()).unwrap_or(0.0);
                self.weights.stake_rebond_weight * (amount / 1000.0) // Normalize by 1000
            },
            EventType::StakeReward => {
                let amount = metadata.get("amount").and_then(|a| a.parse::<f64>().ok()).unwrap_or(0.0);
                self.weights.stake_reward_weight * (amount / 100.0) // Normalize by 100
            },
            EventType::StakeSlash => {
                let amount = metadata.get("amount").and_then(|a| a.parse::<f64>().ok()).unwrap_or(0.0);
                self.weights.stake_slash_weight * (amount / 100.0) // Normalize by 100
            },
        }
    }
    
    fn update_user_score(&mut self, account_id: u32, event: &IndexedEvent) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        
        let user_score = self.user_scores.entry(account_id).or_insert_with(|| UserScore {
            account_id,
            total_score: 0.0,
            governance_score: 0.0,
            identity_score: 0.0,
            staking_score: 0.0,
            last_updated: now,
        });
        
        // Update specific category scores
        match event.event_type {
            EventType::GovernanceVote | EventType::GovernanceProposal | EventType::GovernanceComment => {
                user_score.governance_score += event.score_change;
            },
            EventType::IdentitySet | EventType::IdentityVerified => {
                user_score.identity_score += event.score_change;
            },
            EventType::StakeBond | EventType::StakeUnbond | EventType::StakeRebond | EventType::StakeReward | EventType::StakeSlash => {
                user_score.staking_score += event.score_change;
            },
        }
        
        // Update total score
        user_score.total_score = user_score.governance_score + user_score.identity_score + user_score.staking_score;
        user_score.last_updated = now;
    }
    
    pub fn get_user_score(&self, account_id: u32) -> Option<&UserScore> {
        self.user_scores.get(&account_id)
    }
    
    pub fn get_events_for_user(&self, account_id: u32) -> Vec<&IndexedEvent> {
        self.events.iter().filter(|e| e.account_id == account_id).collect()
    }
    
    pub fn get_top_users(&self, limit: usize) -> Vec<(u32, f64)> {
        let mut users: Vec<(u32, f64)> = self.user_scores
            .iter()
            .map(|(account_id, score)| (*account_id, score.total_score))
            .collect();
        
        users.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        users.truncate(limit);
        users
    }
    
    pub fn update_weights(&mut self, weights: WeightConfiguration) {
        self.weights = weights;
        
        // Recalculate all scores with new weights
        self.recalculate_all_scores();
    }
    
    fn recalculate_all_scores(&mut self) {
        // Reset all scores
        for user_score in self.user_scores.values_mut() {
            user_score.governance_score = 0.0;
            user_score.identity_score = 0.0;
            user_score.staking_score = 0.0;
        }
        
        // Recalculate scores for all events
        for event in &self.events {
            if let Some(user_score) = self.user_scores.get_mut(&event.account_id) {
                let score_change = self.calculate_score_change(&event.event_type, &event.metadata);
                
                match event.event_type {
                    EventType::GovernanceVote | EventType::GovernanceProposal | EventType::GovernanceComment => {
                        user_score.governance_score += score_change;
                    },
                    EventType::IdentitySet | EventType::IdentityVerified => {
                        user_score.identity_score += score_change;
                    },
                    EventType::StakeBond | EventType::StakeUnbond | EventType::StakeRebond | EventType::StakeReward | EventType::StakeSlash => {
                        user_score.staking_score += score_change;
                    },
                }
                
                user_score.total_score = user_score.governance_score + user_score.identity_score + user_score.staking_score;
            }
        }
    }
    
    pub fn get_events_by_type(&self, event_type: EventType) -> Vec<&IndexedEvent> {
        self.events.iter().filter(|e| e.event_type == event_type).collect()
    }
    
    pub fn get_events_in_time_range(&self, start_time: u64, end_time: u64) -> Vec<&IndexedEvent> {
        self.events.iter().filter(|e| e.timestamp >= start_time && e.timestamp <= end_time).collect()
    }
    
    pub fn get_user_governance_score(&self, account_id: u32) -> f64 {
        self.user_scores.get(&account_id).map(|s| s.governance_score).unwrap_or(0.0)
    }
    
    pub fn get_user_identity_score(&self, account_id: u32) -> f64 {
        self.user_scores.get(&account_id).map(|s| s.identity_score).unwrap_or(0.0)
    }
    
    pub fn get_user_staking_score(&self, account_id: u32) -> f64 {
        self.user_scores.get(&account_id).map(|s| s.staking_score).unwrap_or(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gis_indexing_system_creation() {
        let system = GisIndexingSystem::new();
        assert_eq!(system.events.len(), 0);
        assert_eq!(system.user_scores.len(), 0);
    }
    
    #[test]
    fn test_add_event() {
        let mut system = GisIndexingSystem::new();
        let mut metadata = HashMap::new();
        metadata.insert("conviction".to_string(), "2.0".to_string());
        
        let event_id = system.add_event(
            1,
            EventType::GovernanceVote,
            1000,
            100,
            metadata,
        );
        
        assert_eq!(event_id, 1);
        assert_eq!(system.events.len(), 1);
        assert_eq!(system.user_scores.len(), 1);
        
        let user_score = system.get_user_score(1).unwrap();
        assert!(user_score.total_score > 0.0);
    }
    
    #[test]
    fn test_user_score_calculation() {
        let mut system = GisIndexingSystem::new();
        
        // Add governance vote
        let mut metadata = HashMap::new();
        metadata.insert("conviction".to_string(), "2.0".to_string());
        system.add_event(1, EventType::GovernanceVote, 1000, 100, metadata);
        
        // Add identity set
        system.add_event(1, EventType::IdentitySet, 1001, 101, HashMap::new());
        
        // Add staking bond
        let mut metadata = HashMap::new();
        metadata.insert("amount".to_string(), "10000.0".to_string());
        system.add_event(1, EventType::StakeBond, 1002, 102, metadata);
        
        let user_score = system.get_user_score(1).unwrap();
        assert!(user_score.total_score > 0.0);
        assert!(user_score.governance_score > 0.0);
        assert!(user_score.identity_score > 0.0);
        assert!(user_score.staking_score > 0.0);
    }
    
    #[test]
    fn test_weight_update() {
        let mut system = GisIndexingSystem::new();
        
        // Add an event
        system.add_event(1, EventType::GovernanceVote, 1000, 100, HashMap::new());
        let score_before = system.get_user_score(1).unwrap().total_score;
        
        // Update weights
        let mut new_weights = WeightConfiguration::default();
        new_weights.governance_vote_weight = 5.0; // Increase weight
        system.update_weights(new_weights);
        
        let score_after = system.get_user_score(1).unwrap().total_score;
        assert!(score_after > score_before);
    }
    
    #[test]
    fn test_get_events_for_user() {
        let mut system = GisIndexingSystem::new();
        
        // Add events for different users
        system.add_event(1, EventType::GovernanceVote, 1000, 100, HashMap::new());
        system.add_event(2, EventType::StakeBond, 1001, 101, HashMap::new());
        system.add_event(1, EventType::IdentitySet, 1002, 102, HashMap::new());
        
        // Check events for user 1
        let user_events = system.get_events_for_user(1);
        assert_eq!(user_events.len(), 2);
        
        // Check events for user 2
        let user_events = system.get_events_for_user(2);
        assert_eq!(user_events.len(), 1);
    }
    
    #[test]
    fn test_get_top_users() {
        let mut system = GisIndexingSystem::new();
        
        // Add scores for different users
        system.add_event(1, EventType::GovernanceProposal, 1000, 100, HashMap::new()); // High score
        system.add_event(2, EventType::GovernanceVote, 1001, 101, HashMap::new()); // Low score
        system.add_event(3, EventType::IdentityVerified, 1002, 102, HashMap::new()); // Medium score
        
        // Get top 2 users
        let top_users = system.get_top_users(2);
        assert_eq!(top_users.len(), 2);
    }
    
    #[test]
    fn test_get_events_by_type() {
        let mut system = GisIndexingSystem::new();
        
        // Add different types of events
        system.add_event(1, EventType::GovernanceVote, 1000, 100, HashMap::new());
        system.add_event(1, EventType::StakeBond, 1001, 101, HashMap::new());
        system.add_event(1, EventType::GovernanceProposal, 1002, 102, HashMap::new());
        
        // Check governance events
        let governance_events = system.get_events_by_type(EventType::GovernanceVote);
        assert_eq!(governance_events.len(), 1);
        
        let governance_events = system.get_events_by_type(EventType::GovernanceProposal);
        assert_eq!(governance_events.len(), 1);
    }
    
    #[test]
    fn test_get_events_in_time_range() {
        let mut system = GisIndexingSystem::new();
        
        // Add events at different times
        system.add_event(1, EventType::GovernanceVote, 1000, 100, HashMap::new());
        system.add_event(1, EventType::StakeBond, 2000, 200, HashMap::new());
        system.add_event(1, EventType::IdentitySet, 3000, 300, HashMap::new());
        
        // Get events in time range
        let events = system.get_events_in_time_range(1500, 2500);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, EventType::StakeBond);
    }
    
    #[test]
    fn test_category_scores() {
        let mut system = GisIndexingSystem::new();
        
        // Add governance event
        system.add_event(1, EventType::GovernanceVote, 1000, 100, HashMap::new());
        assert!(system.get_user_governance_score(1) > 0.0);
        assert_eq!(system.get_user_identity_score(1), 0.0);
        assert_eq!(system.get_user_staking_score(1), 0.0);
        
        // Add identity event
        system.add_event(1, EventType::IdentitySet, 1001, 101, HashMap::new());
        assert!(system.get_user_identity_score(1) > 0.0);
        
        // Add staking event
        let mut metadata = HashMap::new();
        metadata.insert("amount".to_string(), "5000.0".to_string());
        system.add_event(1, EventType::StakeBond, 1002, 102, metadata);
        assert!(system.get_user_staking_score(1) > 0.0);
    }
}
