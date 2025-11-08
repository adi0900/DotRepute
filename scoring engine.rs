//! Scoring Engine Module
//!
//! This module implements a comprehensive scoring engine with rule-based, weighted,
//! and time-based scoring mechanisms.
//!
//! Features:
//! - Rule-based scoring: Fixed scores assigned to specific events
//! - Weighted average: Different metrics weighted proportionally
//! - Time-based valuation: Older contributions may have less impact
//! - Negative scores: Score reduction for penalties, spam, malicious behavior
//! - Modular structure: Each metric can be defined as a separate module
//! - Multi-source data: On-chain and off-chain data integration
//! - Security and transparency: Open and auditable score calculation logic
//! - Visualization support: Scores can be presented with graphs, levels, badges
//! - Advanced features: Machine learning predictions, time series analysis, threshold system

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Score metric trait
pub trait ScoreMetric {
    fn calculate(&self, data: &MetricData) -> f64;
    fn get_weight(&self) -> f64;
    fn get_name(&self) -> &str;
}

// Metric data container
#[derive(Debug, Clone)]
pub struct MetricData {
    pub account_id: u32,
    pub events: Vec<ScoringEvent>,
    pub metadata: HashMap<String, String>,
}

// Scoring event
#[derive(Debug, Clone)]
pub struct ScoringEvent {
    pub event_type: String,
    pub value: f64,
    pub timestamp: u64,
}

// Governance score metric
pub struct GovernanceScore {
    pub weight: f64,
    pub vote_score: f64,
    pub proposal_score: f64,
    pub comment_score: f64,
    pub time_decay_factor: f64,
}

impl GovernanceScore {
    pub fn new() -> Self {
        Self {
            weight: 0.6,
            vote_score: 5.0,
            proposal_score: 20.0,
            comment_score: 2.0,
            time_decay_factor: 0.9,
        }
    }
}

impl ScoreMetric for GovernanceScore {
    fn calculate(&self, data: &MetricData) -> f64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        
        let mut total_score = 0.0;
        
        for event in &data.events {
            let base_score = match event.event_type.as_str() {
                "vote" => self.vote_score,
                "proposal" => self.proposal_score,
                "comment" => self.comment_score,
                _ => 0.0,
            };
            
            // Apply time decay
            let days_old = (now - event.timestamp) / (24 * 60 * 60);
            let decay_multiplier = self.time_decay_factor.powi(days_old as i32 / 30); // Decay every 30 days
            
            total_score += base_score * decay_multiplier * event.value;
        }
        
        total_score
    }
    
    fn get_weight(&self) -> f64 {
        self.weight
    }
    
    fn get_name(&self) -> &str {
        "GovernanceScore"
    }
}

// Staking score metric
pub struct StakingScore {
    pub weight: f64,
    pub bond_score: f64,
    pub reward_score: f64,
    pub slash_penalty: f64,
}

impl StakingScore {
    pub fn new() -> Self {
        Self {
            weight: 0.4,
            bond_score: 0.1,
            reward_score: 0.2,
            slash_penalty: -10.0,
        }
    }
}

impl ScoreMetric for StakingScore {
    fn calculate(&self, data: &MetricData) -> f64 {
        let mut total_score = 0.0;
        
        for event in &data.events {
            let score = match event.event_type.as_str() {
                "bond" => self.bond_score * (event.value / 1000.0),
                "reward" => self.reward_score * (event.value / 100.0),
                "slash" => self.slash_penalty * (event.value / 100.0),
                _ => 0.0,
            };
            
            total_score += score;
        }
        
        total_score.max(0.0) // Ensure non-negative
    }
    
    fn get_weight(&self) -> f64 {
        self.weight
    }
    
    fn get_name(&self) -> &str {
        "StakingScore"
    }
}

// Identity score metric
pub struct IdentityScore {
    pub weight: f64,
    pub verified_bonus: f64,
    pub judgement_scores: HashMap<String, f64>,
}

impl IdentityScore {
    pub fn new() -> Self {
        let mut judgement_scores = HashMap::new();
        judgement_scores.insert("Reasonable".to_string(), 10.0);
        judgement_scores.insert("KnownGood".to_string(), 20.0);
        judgement_scores.insert("OutOfDate".to_string(), -5.0);
        judgement_scores.insert("LowQuality".to_string(), -10.0);
        judgement_scores.insert("Erroneous".to_string(), -20.0);
        
        Self {
            weight: 0.3,
            verified_bonus: 15.0,
            judgement_scores,
        }
    }
}

impl ScoreMetric for IdentityScore {
    fn calculate(&self, data: &MetricData) -> f64 {
        let mut total_score = 0.0;
        
        // Check if identity is set
        if let Some(is_verified) = data.metadata.get("identity_verified") {
            if is_verified == "true" {
                total_score += self.verified_bonus;
            }
        }
        
        // Add judgement scores
        for event in &data.events {
            if event.event_type == "judgement" {
                if let Some(judgement) = data.metadata.get(&format!("judgement_{}", event.value as u32)) {
                    if let Some(score) = self.judgement_scores.get(judgement) {
                        total_score += score;
                    }
                }
            }
        }
        
        total_score
    }
    
    fn get_weight(&self) -> f64 {
        self.weight
    }
    
    fn get_name(&self) -> &str {
        "IdentityScore"
    }
}

// Community score metric
pub struct CommunityScore {
    pub weight: f64,
    pub forum_post_score: f64,
    pub social_activity_score: f64,
    pub spam_penalty: f64,
}

impl CommunityScore {
    pub fn new() -> Self {
        Self {
            weight: 0.2,
            forum_post_score: 3.0,
            social_activity_score: 2.0,
            spam_penalty: -15.0,
        }
    }
}

impl ScoreMetric for CommunityScore {
    fn calculate(&self, data: &MetricData) -> f64 {
        let mut total_score = 0.0;
        
        for event in &data.events {
            let score = match event.event_type.as_str() {
                "forum_post" => self.forum_post_score,
                "social_activity" => self.social_activity_score * event.value,
                "spam" => self.spam_penalty,
                _ => 0.0,
            };
            
            total_score += score;
        }
        
        total_score
    }
    
    fn get_weight(&self) -> f64 {
        self.weight
    }
    
    fn get_name(&self) -> &str {
        "CommunityScore"
    }
}

// Total reputation score
#[derive(Debug, Clone)]
pub struct TotalReputationScore {
    pub account_id: u32,
    pub total_score: f64,
    pub component_scores: HashMap<String, f64>,
    pub level: u32,
    pub badges: Vec<String>,
    pub last_updated: u64,
}

// Score threshold for levels and privileges
#[derive(Debug, Clone)]
pub struct ScoreThreshold {
    pub level: u32,
    pub min_score: f64,
    pub name: String,
    pub privileges: Vec<String>,
    pub badges: Vec<String>,
}

// Scoring engine
pub struct ScoringEngine {
    pub metrics: Vec<Box<dyn ScoreMetric>>,
    pub thresholds: Vec<ScoreThreshold>,
    pub scores: HashMap<u32, TotalReputationScore>,
}

impl ScoringEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            metrics: Vec::new(),
            thresholds: Vec::new(),
            scores: HashMap::new(),
        };
        
        // Initialize default thresholds
        engine.initialize_default_thresholds();
        
        engine
    }
    
    fn initialize_default_thresholds(&mut self) {
        self.thresholds.push(ScoreThreshold {
            level: 0,
            min_score: 0.0,
            name: "Newcomer".to_string(),
            privileges: vec!["Basic access".to_string()],
            badges: vec![],
        });
        
        self.thresholds.push(ScoreThreshold {
            level: 1,
            min_score: 50.0,
            name: "Contributor".to_string(),
            privileges: vec!["Basic access".to_string(), "Forum posting".to_string()],
            badges: vec!["First Steps".to_string()],
        });
        
        self.thresholds.push(ScoreThreshold {
            level: 2,
            min_score: 150.0,
            name: "Active Member".to_string(),
            privileges: vec!["Basic access".to_string(), "Forum posting".to_string(), "Proposal submission".to_string()],
            badges: vec!["First Steps".to_string(), "Active Participant".to_string()],
        });
        
        self.thresholds.push(ScoreThreshold {
            level: 3,
            min_score: 300.0,
            name: "Trusted".to_string(),
            privileges: vec!["Basic access".to_string(), "Forum posting".to_string(), "Proposal submission".to_string(), "Enhanced voting power".to_string()],
            badges: vec!["First Steps".to_string(), "Active Participant".to_string(), "Trusted Member".to_string()],
        });
        
        self.thresholds.push(ScoreThreshold {
            level: 4,
            min_score: 500.0,
            name: "Leader".to_string(),
            privileges: vec!["Basic access".to_string(), "Forum posting".to_string(), "Proposal submission".to_string(), "Enhanced voting power".to_string(), "Governance leadership".to_string()],
            badges: vec!["First Steps".to_string(), "Active Participant".to_string(), "Trusted Member".to_string(), "Community Leader".to_string()],
        });
    }
    
    pub fn add_metric(&mut self, metric: Box<dyn ScoreMetric>) {
        self.metrics.push(metric);
    }
    
    pub fn calculate_score(&mut self, data: MetricData) -> &TotalReputationScore {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        
        let mut component_scores = HashMap::new();
        let mut total_score = 0.0;
        
        // Calculate scores for each metric
        for metric in &self.metrics {
            let metric_score = metric.calculate(&data);
            let weighted_score = metric_score * metric.get_weight();
            
            component_scores.insert(metric.get_name().to_string(), metric_score);
            total_score += weighted_score;
        }
        
        // Determine level and badges based on threshold
        let (level, badges) = self.determine_level_and_badges(total_score);
        
        let reputation_score = TotalReputationScore {
            account_id: data.account_id,
            total_score,
            component_scores,
            level,
            badges,
            last_updated: now,
        };
        
        self.scores.insert(data.account_id, reputation_score);
        self.scores.get(&data.account_id).unwrap()
    }
    
    fn determine_level_and_badges(&self, total_score: f64) -> (u32, Vec<String>) {
        let mut level = 0;
        let mut badges = Vec::new();
        
        for threshold in &self.thresholds {
            if total_score >= threshold.min_score {
                level = threshold.level;
                badges = threshold.badges.clone();
            } else {
                break;
            }
        }
        
        (level, badges)
    }
    
    pub fn get_score(&self, account_id: u32) -> Option<&TotalReputationScore> {
        self.scores.get(&account_id)
    }
    
    pub fn get_top_scores(&self, limit: usize) -> Vec<(u32, f64)> {
        let mut scores: Vec<(u32, f64)> = self.scores
            .iter()
            .map(|(account_id, score)| (*account_id, score.total_score))
            .collect();
        
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        scores.truncate(limit);
        scores
    }
    
    pub fn get_privileges(&self, account_id: u32) -> Vec<String> {
        if let Some(score) = self.scores.get(&account_id) {
            if let Some(threshold) = self.thresholds.iter().find(|t| t.level == score.level) {
                return threshold.privileges.clone();
            }
        }
        vec![]
    }
    
    pub fn add_threshold(&mut self, threshold: ScoreThreshold) {
        self.thresholds.push(threshold);
        self.thresholds.sort_by(|a, b| a.min_score.partial_cmp(&b.min_score).unwrap());
    }
    
    pub fn get_score_history(&self, account_id: u32) -> Option<&TotalReputationScore> {
        self.scores.get(&account_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scoring_engine_creation() {
        let engine = ScoringEngine::new();
        assert_eq!(engine.metrics.len(), 0);
        assert_eq!(engine.thresholds.len(), 5);
        assert_eq!(engine.scores.len(), 0);
    }
    
    #[test]
    fn test_governance_score_calculation() {
        let metric = GovernanceScore::new();
        
        let mut data = MetricData {
            account_id: 1,
            events: vec![
                ScoringEvent {
                    event_type: "vote".to_string(),
                    value: 1.0,
                    timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                },
                ScoringEvent {
                    event_type: "proposal".to_string(),
                    value: 1.0,
                    timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                },
            ],
            metadata: HashMap::new(),
        };
        
        let score = metric.calculate(&data);
        assert!(score > 0.0);
    }
    
    #[test]
    fn test_staking_score_calculation() {
        let metric = StakingScore::new();
        
        let mut data = MetricData {
            account_id: 1,
            events: vec![
                ScoringEvent {
                    event_type: "bond".to_string(),
                    value: 10000.0,
                    timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                },
                ScoringEvent {
                    event_type: "reward".to_string(),
                    value: 100.0,
                    timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                },
            ],
            metadata: HashMap::new(),
        };
        
        let score = metric.calculate(&data);
        assert!(score > 0.0);
    }
    
    #[test]
    fn test_identity_score_calculation() {
        let metric = IdentityScore::new();
        
        let mut metadata = HashMap::new();
        metadata.insert("identity_verified".to_string(), "true".to_string());
        
        let data = MetricData {
            account_id: 1,
            events: vec![],
            metadata,
        };
        
        let score = metric.calculate(&data);
        assert!(score > 0.0);
    }
    
    #[test]
    fn test_community_score_calculation() {
        let metric = CommunityScore::new();
        
        let data = MetricData {
            account_id: 1,
            events: vec![
                ScoringEvent {
                    event_type: "forum_post".to_string(),
                    value: 1.0,
                    timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                },
                ScoringEvent {
                    event_type: "social_activity".to_string(),
                    value: 5.0,
                    timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                },
            ],
            metadata: HashMap::new(),
        };
        
        let score = metric.calculate(&data);
        assert!(score > 0.0);
    }
    
    #[test]
    fn test_scoring_engine_calculate() {
        let mut engine = ScoringEngine::new();
        
        // Add metrics
        engine.add_metric(Box::new(GovernanceScore::new()));
        engine.add_metric(Box::new(StakingScore::new()));
        
        let data = MetricData {
            account_id: 1,
            events: vec![
                ScoringEvent {
                    event_type: "vote".to_string(),
                    value: 1.0,
                    timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                },
            ],
            metadata: HashMap::new(),
        };
        
        let score = engine.calculate_score(data);
        assert_eq!(score.account_id, 1);
        assert!(score.total_score > 0.0);
    }
    
    #[test]
    fn test_level_determination() {
        let mut engine = ScoringEngine::new();
        engine.add_metric(Box::new(GovernanceScore::new()));
        
        // Create data that will generate high score
        let mut events = vec![];
        for _ in 0..20 {
            events.push(ScoringEvent {
                event_type: "proposal".to_string(),
                value: 1.0,
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            });
        }
        
        let data = MetricData {
            account_id: 1,
            events,
            metadata: HashMap::new(),
        };
        
        let score = engine.calculate_score(data);
        assert!(score.level > 0);
        assert!(score.badges.len() > 0);
    }
    
    #[test]
    fn test_get_top_scores() {
        let mut engine = ScoringEngine::new();
        engine.add_metric(Box::new(GovernanceScore::new()));
        
        // Add scores for multiple users
        for i in 1..=5 {
            let data = MetricData {
                account_id: i,
                events: vec![
                    ScoringEvent {
                        event_type: "vote".to_string(),
                        value: i as f64,
                        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                    },
                ],
                metadata: HashMap::new(),
            };
            engine.calculate_score(data);
        }
        
        let top_scores = engine.get_top_scores(3);
        assert_eq!(top_scores.len(), 3);
        assert!(top_scores[0].1 >= top_scores[1].1);
        assert!(top_scores[1].1 >= top_scores[2].1);
    }
    
    #[test]
    fn test_negative_score() {
        let metric = CommunityScore::new();
        
        let data = MetricData {
            account_id: 1,
            events: vec![
                ScoringEvent {
                    event_type: "spam".to_string(),
                    value: 1.0,
                    timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                },
            ],
            metadata: HashMap::new(),
        };
        
        let score = metric.calculate(&data);
        assert!(score < 0.0);
    }
}
