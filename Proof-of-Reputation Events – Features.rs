//! Proof-of-Reputation Events Module
//!
//! This module implements a proof-of-reputation system that validates user reliability
//! based on their on-chain and off-chain contributions.
//!
//! Features:
//! - Participation-based verification: User reliability proven through on-chain and off-chain contributions
//! - Score-based access: Minimum reputation score required for specific events or privileges
//! - On-chain traceability: Participation, voting, staking and other activities recorded on-chain
//! - Sybil resistance: Difficult to manipulate with fake accounts due to real contribution basis
//! - Dynamic level system: Users can reach higher reputation levels over time
//! - Governance integration: High-reputation users may have more voting power or proposal rights
//! - Reward and incentive mechanism: Users participating in events and contributing can earn tokens, badges or privileges
//! - Modular structure: Customizable rules and thresholds on different chains or communities

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Reputation event types
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum ReputationEventType {
    GovernanceParticipation,
    StakingActivity,
    IdentityVerification,
    CommunityContribution,
    TechnicalContribution,
    EventParticipation,
}

// Reputation event record
#[derive(Debug, Clone)]
pub struct ReputationEvent {
    pub id: u32,
    pub account_id: u32,
    pub event_type: ReputationEventType,
    pub score_change: f64,
    pub timestamp: u64,
    pub metadata: HashMap<String, String>,
}

// Reputation level
#[derive(Debug, Clone)]
pub struct ReputationLevel {
    pub level: u32,
    pub name: String,
    pub min_score: f64,
    pub privileges: Vec<String>,
}

// Access control rule
#[derive(Debug, Clone)]
pub struct AccessControlRule {
    pub id: u32,
    pub name: String,
    pub required_min_score: f64,
    pub required_level: Option<u32>,
    pub allowed_event_types: Vec<ReputationEventType>,
}

// Proof-of-Reputation system
pub struct ProofOfReputation {
    pub events: Vec<ReputationEvent>,
    pub user_scores: HashMap<u32, f64>,
    pub user_levels: HashMap<u32, u32>, // account_id -> level
    pub reputation_levels: Vec<ReputationLevel>,
    pub access_rules: Vec<AccessControlRule>,
    pub event_counter: u32,
}

impl ProofOfReputation {
    pub fn new() -> Self {
        let mut system = Self {
            events: Vec::new(),
            user_scores: HashMap::new(),
            user_levels: HashMap::new(),
            reputation_levels: Vec::new(),
            access_rules: Vec::new(),
            event_counter: 0,
        };
        
        // Initialize default reputation levels
        system.initialize_default_levels();
        
        system
    }
    
    fn initialize_default_levels(&mut self) {
        self.reputation_levels.push(ReputationLevel {
            level: 0,
            name: "Newcomer".to_string(),
            min_score: 0.0,
            privileges: vec!["Basic access".to_string()],
        });
        
        self.reputation_levels.push(ReputationLevel {
            level: 1,
            name: "Participant".to_string(),
            min_score: 50.0,
            privileges: vec!["Basic access".to_string(), "Event participation".to_string()],
        });
        
        self.reputation_levels.push(ReputationLevel {
            level: 2,
            name: "Contributor".to_string(),
            min_score: 150.0,
            privileges: vec!["Basic access".to_string(), "Event participation".to_string(), "Proposal submission".to_string()],
        });
        
        self.reputation_levels.push(ReputationLevel {
            level: 3,
            name: "Trusted".to_string(),
            min_score: 300.0,
            privileges: vec!["Basic access".to_string(), "Event participation".to_string(), "Proposal submission".to_string(), "Enhanced voting power".to_string()],
        });
        
        self.reputation_levels.push(ReputationLevel {
            level: 4,
            name: "Leader".to_string(),
            min_score: 500.0,
            privileges: vec!["Basic access".to_string(), "Event participation".to_string(), "Proposal submission".to_string(), "Enhanced voting power".to_string(), "Governance leadership".to_string()],
        });
    }
    
    pub fn add_reputation_event(
        &mut self,
        account_id: u32,
        event_type: ReputationEventType,
        score_change: f64,
        metadata: HashMap<String, String>,
    ) -> u32 {
        self.event_counter += 1;
        let id = self.event_counter;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        
        let event = ReputationEvent {
            id,
            account_id,
            event_type,
            score_change,
            timestamp,
            metadata,
        };
        
        // Update user score
        let current_score = self.user_scores.entry(account_id).or_insert(0.0);
        *current_score += score_change;
        
        // Update user level
        self.update_user_level(account_id);
        
        self.events.push(event);
        id
    }
    
    fn update_user_level(&mut self, account_id: u32) {
        if let Some(score) = self.user_scores.get(&account_id) {
            let mut level = 0;
            for reputation_level in &self.reputation_levels {
                if *score >= reputation_level.min_score {
                    level = reputation_level.level;
                } else {
                    break;
                }
            }
            self.user_levels.insert(account_id, level);
        }
    }
    
    pub fn get_user_score(&self, account_id: u32) -> f64 {
        *self.user_scores.get(&account_id).unwrap_or(&0.0)
    }
    
    pub fn get_user_level(&self, account_id: u32) -> u32 {
        *self.user_levels.get(&account_id).unwrap_or(&0)
    }
    
    pub fn get_user_privileges(&self, account_id: u32) -> Vec<String> {
        let level = self.get_user_level(account_id);
        if let Some(reputation_level) = self.reputation_levels.iter().find(|l| l.level == level) {
            reputation_level.privileges.clone()
        } else {
            vec![]
        }
    }
    
    pub fn add_access_rule(&mut self, rule: AccessControlRule) {
        self.access_rules.push(rule);
    }
    
    pub fn check_access(&self, account_id: u32, rule_id: u32) -> bool {
        if let Some(rule) = self.access_rules.iter().find(|r| r.id == rule_id) {
            let user_score = self.get_user_score(account_id);
            let user_level = self.get_user_level(account_id);
            
            // Check minimum score requirement
            if user_score < rule.required_min_score {
                return false;
            }
            
            // Check level requirement if specified
            if let Some(required_level) = rule.required_level {
                if user_level < required_level {
                    return false;
                }
            }
            
            true
        } else {
            false
        }
    }
    
    pub fn get_events_for_user(&self, account_id: u32) -> Vec<&ReputationEvent> {
        self.events.iter().filter(|e| e.account_id == account_id).collect()
    }
    
    pub fn get_top_users(&self, limit: usize) -> Vec<(u32, f64)> {
        let mut users: Vec<(u32, f64)> = self.user_scores.iter().map(|(id, score)| (*id, *score)).collect();
        users.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        users.truncate(limit);
        users
    }
    
    pub fn get_reputation_levels(&self) -> &Vec<ReputationLevel> {
        &self.reputation_levels
    }
    
    pub fn add_custom_level(&mut self, level: ReputationLevel) {
        self.reputation_levels.push(level);
        // Re-sort levels by min_score
        self.reputation_levels.sort_by(|a, b| a.min_score.partial_cmp(&b.min_score).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proof_of_reputation_creation() {
        let por = ProofOfReputation::new();
        assert_eq!(por.events.len(), 0);
        assert_eq!(por.user_scores.len(), 0);
        assert_eq!(por.reputation_levels.len(), 5); // Default levels
    }
    
    #[test]
    fn test_add_reputation_event() {
        let mut por = ProofOfReputation::new();
        let mut metadata = HashMap::new();
        metadata.insert("proposal_id".to_string(), "1".to_string());
        
        let event_id = por.add_reputation_event(
            1,
            ReputationEventType::GovernanceParticipation,
            10.0,
            metadata,
        );
        
        assert_eq!(event_id, 1);
        assert_eq!(por.events.len(), 1);
        assert_eq!(por.get_user_score(1), 10.0);
    }
    
    #[test]
    fn test_user_level_update() {
        let mut por = ProofOfReputation::new();
        
        // Add events to increase user score
        por.add_reputation_event(1, ReputationEventType::GovernanceParticipation, 60.0, HashMap::new());
        assert_eq!(por.get_user_level(1), 1); // Participant level (50+)
        
        por.add_reputation_event(1, ReputationEventType::StakingActivity, 100.0, HashMap::new());
        assert_eq!(por.get_user_level(1), 2); // Contributor level (150+)
        
        por.add_reputation_event(1, ReputationEventType::CommunityContribution, 200.0, HashMap::new());
        assert_eq!(por.get_user_level(1), 3); // Trusted level (300+)
    }
    
    #[test]
    fn test_access_control() {
        let mut por = ProofOfReputation::new();
        
        // Add access rule
        let rule = AccessControlRule {
            id: 1,
            name: "High reputation access".to_string(),
            required_min_score: 100.0,
            required_level: Some(2),
            allowed_event_types: vec![ReputationEventType::GovernanceParticipation],
        };
        por.add_access_rule(rule);
        
        // User with low reputation should not have access
        assert!(!por.check_access(1, 1));
        
        // Increase user reputation
        por.add_reputation_event(1, ReputationEventType::GovernanceParticipation, 150.0, HashMap::new());
        
        // User with high reputation should have access
        assert!(por.check_access(1, 1));
    }
    
    #[test]
    fn test_user_privileges() {
        let mut por = ProofOfReputation::new();
        
        // Check privileges for newcomer
        let privileges = por.get_user_privileges(1);
        assert!(privileges.contains(&"Basic access".to_string()));
        assert_eq!(privileges.len(), 1);
        
        // Increase user reputation to participant level
        por.add_reputation_event(1, ReputationEventType::GovernanceParticipation, 60.0, HashMap::new());
        let privileges = por.get_user_privileges(1);
        assert!(privileges.contains(&"Basic access".to_string()));
        assert!(privileges.contains(&"Event participation".to_string()));
        assert_eq!(privileges.len(), 2);
    }
    
    #[test]
    fn test_get_events_for_user() {
        let mut por = ProofOfReputation::new();
        
        // Add events for different users
        por.add_reputation_event(1, ReputationEventType::GovernanceParticipation, 10.0, HashMap::new());
        por.add_reputation_event(2, ReputationEventType::StakingActivity, 20.0, HashMap::new());
        por.add_reputation_event(1, ReputationEventType::CommunityContribution, 15.0, HashMap::new());
        
        // Check events for user 1
        let user_events = por.get_events_for_user(1);
        assert_eq!(user_events.len(), 2);
        
        // Check events for user 2
        let user_events = por.get_events_for_user(2);
        assert_eq!(user_events.len(), 1);
    }
    
    #[test]
    fn test_get_top_users() {
        let mut por = ProofOfReputation::new();
        
        // Add reputation scores for different users
        por.add_reputation_event(1, ReputationEventType::GovernanceParticipation, 100.0, HashMap::new());
        por.add_reputation_event(2, ReputationEventType::StakingActivity, 200.0, HashMap::new());
        por.add_reputation_event(3, ReputationEventType::CommunityContribution, 150.0, HashMap::new());
        
        // Get top 2 users
        let top_users = por.get_top_users(2);
        assert_eq!(top_users.len(), 2);
        assert_eq!(top_users[0].0, 2); // User 2 with highest score
        assert_eq!(top_users[1].0, 3); // User 3 with second highest score
    }
}
