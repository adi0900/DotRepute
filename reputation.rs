//!
//! This module implements a reputation system that calculates reputation scores
//! based on both on-chain and off-chain data.
//!
//! Features:
//! - Calculates reputation based on on-chain and off-chain data
//! - Reflects user contributions, participation, and reliability
//! - Modular structure, extensible with different metrics
//! - Generates unique scores for each user
//! - Updateable and evolving over time
//! - Collects data from governance, staking, identity and other areas
//! - Transparent and verifiable calculation logic
//! - Visualizable in UI/UX (badges, levels, charts)
//! - Integrates with other modules (e.g. reward systems)
//! - Can be calculated on-chain or off-chain

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Reputation categories
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum ReputationCategory {
    Governance,
    Staking,
    Identity,
    Community,
    Technical,
}

// Reputation metric trait for extensibility
pub trait ReputationMetric {
    fn calculate_score(&self, account_id: u32) -> f64;
    fn category(&self) -> ReputationCategory;
    fn name(&self) -> &'static str;
}

// Governance participation metric
pub struct GovernanceParticipationMetric {
    pub referendum_votes: Vec<(u32, bool, u64)>, // (referendum_id, voted, timestamp)
    pub proposals_seconded: Vec<(u32, u128, u64)>, // (proposal_id, deposit, timestamp)
    pub open_gov_participation: Vec<(u32, String, u64)>, // (track_id, activity_type, timestamp)
}

impl GovernanceParticipationMetric {
    pub fn new() -> Self {
        Self {
            referendum_votes: Vec::new(),
            proposals_seconded: Vec::new(),
            open_gov_participation: Vec::new(),
        }
    }
    
    pub fn add_referendum_vote(&mut self, referendum_id: u32, voted: bool, timestamp: u64) {
        self.referendum_votes.push((referendum_id, voted, timestamp));
    }
    
    pub fn add_proposal_second(&mut self, proposal_id: u32, deposit: u128, timestamp: u64) {
        self.proposals_seconded.push((proposal_id, deposit, timestamp));
    }
    
    pub fn add_open_gov_participation(&mut self, track_id: u32, activity_type: String, timestamp: u64) {
        self.open_gov_participation.push((track_id, activity_type, timestamp));
    }
}

impl ReputationMetric for GovernanceParticipationMetric {
    fn calculate_score(&self, _account_id: u32) -> f64 {
        let mut score = 0.0;
        
        // Score for referendum votes
        score += self.referendum_votes.len() as f64 * 2.0;
        
        // Score for proposals seconded
        score += self.proposals_seconded.len() as f64 * 3.0;
        
        // Score for open gov participation
        score += self.open_gov_participation.len() as f64 * 1.5;
        
        score
    }
    
    fn category(&self) -> ReputationCategory {
        ReputationCategory::Governance
    }
    
    fn name(&self) -> &'static str {
        "GovernanceParticipation"
    }
}

// Staking activity metric
pub struct StakingActivityMetric {
    pub stakes: Vec<(u128, u64)>, // (amount, timestamp)
    pub rewards: Vec<(u128, u64)>, // (amount, timestamp)
    pub slashes: Vec<(u128, u64)>, // (amount, timestamp)
}

impl StakingActivityMetric {
    pub fn new() -> Self {
        Self {
            stakes: Vec::new(),
            rewards: Vec::new(),
            slashes: Vec::new(),
        }
    }
    
    pub fn add_stake(&mut self, amount: u128, timestamp: u64) {
        self.stakes.push((amount, timestamp));
    }
    
    pub fn add_reward(&mut self, amount: u128, timestamp: u64) {
        self.rewards.push((amount, timestamp));
    }
    
    pub fn add_slash(&mut self, amount: u128, timestamp: u64) {
        self.slashes.push((amount, timestamp));
    }
}

impl ReputationMetric for StakingActivityMetric {
    fn calculate_score(&self, _account_id: u32) -> f64 {
        let mut score = 0.0;
        
        // Positive score for staking
        let total_staked: u128 = self.stakes.iter().map(|(amount, _)| amount).sum();
        score += (total_staked as f64) / 1000.0; // Normalize by 1000
        
        // Positive score for rewards
        let total_rewards: u128 = self.rewards.iter().map(|(amount, _)| amount).sum();
        score += (total_rewards as f64) / 100.0; // Normalize by 100
        
        // Negative score for slashes
        let total_slashed: u128 = self.slashes.iter().map(|(amount, _)| amount).sum();
        score -= (total_slashed as f64) / 50.0; // Normalize by 50
        
        score.max(0.0) // Ensure non-negative score
    }
    
    fn category(&self) -> ReputationCategory {
        ReputationCategory::Staking
    }
    
    fn name(&self) -> &'static str {
        "StakingActivity"
    }
}

// Identity verification metric
pub struct IdentityVerificationMetric {
    pub is_identity_set: bool,
    pub judgements: Vec<String>,
    pub last_updated: u64,
}

impl IdentityVerificationMetric {
    pub fn new() -> Self {
        Self {
            is_identity_set: false,
            judgements: Vec::new(),
            last_updated: 0,
        }
    }
    
    pub fn set_identity(&mut self, judgements: Vec<String>) {
        self.is_identity_set = true;
        self.judgements = judgements;
        self.last_updated = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
    }
    
    pub fn clear_identity(&mut self) {
        self.is_identity_set = false;
        self.judgements.clear();
        self.last_updated = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
    }
}

impl ReputationMetric for IdentityVerificationMetric {
    fn calculate_score(&self, _account_id: u32) -> f64 {
        if !self.is_identity_set {
            return 0.0;
        }
        
        let mut score = 10.0; // Base score for having identity set
        
        // Additional score based on judgements
        for judgement in &self.judgements {
            match judgement.as_str() {
                "Reasonable" => score += 5.0,
                "KnownGood" => score += 10.0,
                "OutOfDate" => score -= 2.0,
                "LowQuality" => score -= 5.0,
                "Erroneous" => score -= 10.0,
                _ => score += 1.0, // Unknown judgement, small positive score
            }
        }
        
        score.max(0.0) // Ensure non-negative score
    }
    
    fn category(&self) -> ReputationCategory {
        ReputationCategory::Identity
    }
    
    fn name(&self) -> &'static str {
        "IdentityVerification"
    }
}

// Community engagement metric
pub struct CommunityEngagementMetric {
    pub forum_posts: Vec<(String, u64)>, // (post_type, timestamp)
    pub social_media_activity: Vec<(String, f64, u64)>, // (platform, engagement_score, timestamp)
    pub local_meetups: Vec<(String, u64)>, // (event_type, timestamp)
}

impl CommunityEngagementMetric {
    pub fn new() -> Self {
        Self {
            forum_posts: Vec::new(),
            social_media_activity: Vec::new(),
            local_meetups: Vec::new(),
        }
    }
    
    pub fn add_forum_post(&mut self, post_type: String, timestamp: u64) {
        self.forum_posts.push((post_type, timestamp));
    }
    
    pub fn add_social_media_activity(&mut self, platform: String, engagement_score: f64, timestamp: u64) {
        self.social_media_activity.push((platform, engagement_score, timestamp));
    }
    
    pub fn add_local_meetup(&mut self, event_type: String, timestamp: u64) {
        self.local_meetups.push((event_type, timestamp));
    }
}

impl ReputationMetric for CommunityEngagementMetric {
    fn calculate_score(&self, _account_id: u32) -> f64 {
        let mut score = 0.0;
        
        // Score for forum posts
        score += self.forum_posts.len() as f64 * 1.0;
        
        // Score for social media activity
        let total_engagement: f64 = self.social_media_activity.iter().map(|(_, engagement, _)| engagement).sum();
        score += total_engagement * 0.5;
        
        // Score for local meetups
        score += self.local_meetups.len() as f64 * 3.0;
        
        score
    }
    
    fn category(&self) -> ReputationCategory {
        ReputationCategory::Community
    }
    
    fn name(&self) -> &'static str {
        "CommunityEngagement"
    }
}

// Reputation score structure
#[derive(Debug, Clone)]
pub struct ReputationScore {
    pub account_id: u32,
    pub total_score: f64,
    pub category_scores: HashMap<ReputationCategory, f64>,
    pub metric_scores: HashMap<String, f64>,
    pub last_updated: u64,
}

impl ReputationScore {
    pub fn new(account_id: u32) -> Self {
        Self {
            account_id,
            total_score: 0.0,
            category_scores: HashMap::new(),
            metric_scores: HashMap::new(),
            last_updated: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
        }
    }
}

// Reputation calculator
pub struct ReputationCalculator {
    pub metrics: HashMap<String, Box<dyn ReputationMetric>>,
    pub weights: HashMap<ReputationCategory, f64>,
    pub scores: HashMap<u32, ReputationScore>,
}

impl ReputationCalculator {
    pub fn new() -> Self {
        let mut weights = HashMap::new();
        weights.insert(ReputationCategory::Governance, 0.3);
        weights.insert(ReputationCategory::Staking, 0.25);
        weights.insert(ReputationCategory::Identity, 0.2);
        weights.insert(ReputationCategory::Community, 0.15);
        weights.insert(ReputationCategory::Technical, 0.1);
        
        Self {
            metrics: HashMap::new(),
            weights,
            scores: HashMap::new(),
        }
    }
    
    pub fn add_metric(&mut self, name: String, metric: Box<dyn ReputationMetric>) {
        self.metrics.insert(name, metric);
    }
    
    pub fn calculate_reputation(&mut self, account_id: u32) -> &ReputationScore {
        let mut category_scores: HashMap<ReputationCategory, f64> = HashMap::new();
        let mut metric_scores: HashMap<String, f64> = HashMap::new();
        
        // Calculate scores for each metric
        for (name, metric) in &self.metrics {
            let score = metric.calculate_score(account_id);
            metric_scores.insert(name.clone(), score);
            
            let category = metric.category();
            let category_score = category_scores.entry(category).or_insert(0.0);
            *category_score += score;
        }
        
        // Calculate total score using weights
        let mut total_score = 0.0;
        for (category, weight) in &self.weights {
            if let Some(category_score) = category_scores.get(category) {
                total_score += category_score * weight;
            }
        }
        
        // Create and store the reputation score
        let reputation_score = ReputationScore {
            account_id,
            total_score,
            category_scores,
            metric_scores,
            last_updated: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
        };
        
        self.scores.insert(account_id, reputation_score);
        self.scores.get(&account_id).unwrap()
    }
    
    pub fn get_reputation(&self, account_id: u32) -> Option<&ReputationScore> {
        self.scores.get(&account_id)
    }
    
    pub fn get_top_accounts(&self, limit: usize) -> Vec<(u32, f64)> {
        let mut accounts: Vec<(u32, f64)> = self.scores
            .iter()
            .map(|(account_id, score)| (*account_id, score.total_score))
            .collect();
        
        accounts.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        accounts.truncate(limit);
        
        accounts
    }
    
    pub fn update_weights(&mut self, weights: HashMap<ReputationCategory, f64>) {
        self.weights = weights;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_governance_participation_metric() {
        let mut metric = GovernanceParticipationMetric::new();
        
        // Add some governance activities
        metric.add_referendum_vote(1, true, 1000);
        metric.add_proposal_second(1, 100, 1001);
        metric.add_open_gov_participation(1, "Voting".to_string(), 1002);
        
        let score = metric.calculate_score(1);
        assert!(score > 0.0);
    }
    
    #[test]
    fn test_staking_activity_metric() {
        let mut metric = StakingActivityMetric::new();
        
        // Add staking activities
        metric.add_stake(1000, 1000);
        metric.add_reward(50, 1001);
        metric.add_slash(10, 1002);
        
        let score = metric.calculate_score(1);
        assert!(score > 0.0);
    }
    
    #[test]
    fn test_identity_verification_metric() {
        let mut metric = IdentityVerificationMetric::new();
        
        // Set identity with positive judgements
        metric.set_identity(vec!["Reasonable".to_string(), "KnownGood".to_string()]);
        
        let score = metric.calculate_score(1);
        assert!(score > 0.0);
        
        // Clear identity
        metric.clear_identity();
        let score = metric.calculate_score(1);
        assert_eq!(score, 0.0);
    }
    
    #[test]
    fn test_community_engagement_metric() {
        let mut metric = CommunityEngagementMetric::new();
        
        // Add community activities
        metric.add_forum_post("Post".to_string(), 1000);
        metric.add_social_media_activity("Twitter".to_string(), 5.0, 1001);
        metric.add_local_meetup("Meetup".to_string(), 1002);
        
        let score = metric.calculate_score(1);
        assert!(score > 0.0);
    }
    
    #[test]
    fn test_reputation_calculator() {
        let mut calculator = ReputationCalculator::new();
        
        // Add metrics
        calculator.add_metric(
            "Governance".to_string(),
            Box::new(GovernanceParticipationMetric::new())
        );
        calculator.add_metric(
            "Staking".to_string(),
            Box::new(StakingActivityMetric::new())
        );
        calculator.add_metric(
            "Identity".to_string(),
            Box::new(IdentityVerificationMetric::new())
        );
        calculator.add_metric(
            "Community".to_string(),
            Box::new(CommunityEngagementMetric::new())
        );
        
        // Calculate reputation for an account
        let reputation = calculator.calculate_reputation(1);
        assert_eq!(reputation.account_id, 1);
        assert_eq!(reputation.total_score, 0.0); // All metrics are empty
        
        // Add some data to metrics and recalculate
        if let Some(metric) = calculator.metrics.get_mut("Governance") {
            let governance_metric = metric.as_any_mut().downcast_mut::<GovernanceParticipationMetric>().unwrap();
            governance_metric.add_referendum_vote(1, true, 1000);
        }
        
        let reputation = calculator.calculate_reputation(1);
        assert!(reputation.total_score > 0.0);
    }
}

// Helper trait to allow downcasting
trait ReputationMetricAny: ReputationMetric {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: ReputationMetric + 'static> ReputationMetricAny for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl dyn ReputationMetric {
    fn as_any(&self) -> &dyn std::any::Any {
        self.as_ref()
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self.as_mut()
    }
}
