//! Rust Scoring Engine - High-Performance Reputation Scoring System
//! 
//! Professional implementation with WASM compatibility, modular architecture,
//! and comprehensive data processing capabilities for blockchain reputation systems.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{vec::Vec, string::String, collections::BTreeMap as HashMap};

#[cfg(feature = "std")]
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainData {
    pub account_id: String,
    pub governance_votes: u32,
    pub governance_proposals: u32,
    pub staking_amount: u64,
    pub staking_duration: u64,
    pub identity_verified: bool,
    pub identity_judgements: u32,
    pub community_posts: u32,
    pub community_upvotes: u32,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreResult {
    pub account_id: String,
    pub total_score: f64,
    pub governance_score: f64,
    pub staking_score: f64,
    pub identity_score: f64,
    pub community_score: f64,
    pub timestamp: u64,
    pub breakdown: ScoreBreakdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreBreakdown {
    pub weighted_governance: f64,
    pub weighted_staking: f64,
    pub weighted_identity: f64,
    pub weighted_community: f64,
    pub time_decay_factor: f64,
    pub negative_adjustments: f64,
}

#[derive(Debug, Clone)]
pub struct ScoringConfig {
    pub governance_weight: f64,
    pub staking_weight: f64,
    pub identity_weight: f64,
    pub community_weight: f64,
    pub time_decay_enabled: bool,
    pub time_decay_rate: f64,
    pub negative_scoring_enabled: bool,
    pub min_score: f64,
    pub max_score: f64,
}

impl Default for ScoringConfig {
    fn default() -> Self {
        Self {
            governance_weight: 0.3,
            staking_weight: 0.3,
            identity_weight: 0.2,
            community_weight: 0.2,
            time_decay_enabled: true,
            time_decay_rate: 0.95,
            negative_scoring_enabled: true,
            min_score: 0.0,
            max_score: 100.0,
        }
    }
}

pub trait ScoreMetric {
    fn calculate(&self, data: &ChainData, config: &ScoringConfig) -> f64;
    fn get_weight(&self, config: &ScoringConfig) -> f64;
    fn get_name(&self) -> &'static str;
    fn validate_data(&self, data: &ChainData) -> Result<(), &'static str>;
}

pub struct GovernanceScoreMetric;

impl ScoreMetric for GovernanceScoreMetric {
    fn calculate(&self, data: &ChainData, _config: &ScoringConfig) -> f64 {
        let vote_score = (data.governance_votes as f64 * 2.0).min(50.0);
        let proposal_score = (data.governance_proposals as f64 * 5.0).min(50.0);
        vote_score + proposal_score
    }

    fn get_weight(&self, config: &ScoringConfig) -> f64 {
        config.governance_weight
    }

    fn get_name(&self) -> &'static str {
        "governance"
    }

    fn validate_data(&self, data: &ChainData) -> Result<(), &'static str> {
        if data.governance_votes > 10000 {
            return Err("Unrealistic governance votes count");
        }
        if data.governance_proposals > 1000 {
            return Err("Unrealistic proposals count");
        }
        Ok(())
    }
}

pub struct StakingScoreMetric;

impl ScoreMetric for StakingScoreMetric {
    fn calculate(&self, data: &ChainData, _config: &ScoringConfig) -> f64 {
        let amount_score = ((data.staking_amount as f64).ln() * 10.0).min(60.0);
        let duration_score = ((data.staking_duration as f64 / 86400.0).sqrt() * 5.0).min(40.0);
        amount_score + duration_score
    }

    fn get_weight(&self, config: &ScoringConfig) -> f64 {
        config.staking_weight
    }

    fn get_name(&self) -> &'static str {
        "staking"
    }

    fn validate_data(&self, data: &ChainData) -> Result<(), &'static str> {
        if data.staking_amount == 0 && data.staking_duration > 0 {
            return Err("Invalid staking data: duration without amount");
        }
        Ok(())
    }
}

pub struct IdentityScoreMetric;

impl ScoreMetric for IdentityScoreMetric {
    fn calculate(&self, data: &ChainData, _config: &ScoringConfig) -> f64 {
        let verified_score = if data.identity_verified { 50.0 } else { 0.0 };
        let judgement_score = (data.identity_judgements as f64 * 10.0).min(50.0);
        verified_score + judgement_score
    }

    fn get_weight(&self, config: &ScoringConfig) -> f64 {
        config.identity_weight
    }

    fn get_name(&self) -> &'static str {
        "identity"
    }

    fn validate_data(&self, data: &ChainData) -> Result<(), &'static str> {
        if data.identity_judgements > 10 {
            return Err("Unrealistic judgements count");
        }
        Ok(())
    }
}

pub struct CommunityScoreMetric;

impl ScoreMetric for CommunityScoreMetric {
    fn calculate(&self, data: &ChainData, _config: &ScoringConfig) -> f64 {
        let post_score = (data.community_posts as f64 * 1.0).min(40.0);
        let upvote_score = (data.community_upvotes as f64 * 0.5).min(60.0);
        post_score + upvote_score
    }

    fn get_weight(&self, config: &ScoringConfig) -> f64 {
        config.community_weight
    }

    fn get_name(&self) -> &'static str {
        "community"
    }

    fn validate_data(&self, data: &ChainData) -> Result<(), &'static str> {
        if data.community_upvotes > data.community_posts * 100 {
            return Err("Suspicious upvote ratio");
        }
        Ok(())
    }
}

pub struct ScoringEngine {
    config: ScoringConfig,
    metrics: Vec<Box<dyn ScoreMetric>>,
    score_history: HashMap<String, Vec<ScoreResult>>,
}

impl ScoringEngine {
    pub fn new(config: ScoringConfig) -> Self {
        let metrics: Vec<Box<dyn ScoreMetric>> = vec![
            Box::new(GovernanceScoreMetric),
            Box::new(StakingScoreMetric),
            Box::new(IdentityScoreMetric),
            Box::new(CommunityScoreMetric),
        ];

        Self {
            config,
            metrics,
            score_history: HashMap::new(),
        }
    }

    pub fn calculate_score(&mut self, data: ChainData) -> Result<ScoreResult, &'static str> {
        for metric in &self.metrics {
            metric.validate_data(&data)?;
        }

        let governance_score = GovernanceScoreMetric.calculate(&data, &self.config);
        let staking_score = StakingScoreMetric.calculate(&data, &self.config);
        let identity_score = IdentityScoreMetric.calculate(&data, &self.config);
        let community_score = CommunityScoreMetric.calculate(&data, &self.config);

        let weighted_governance = governance_score * self.config.governance_weight;
        let weighted_staking = staking_score * self.config.staking_weight;
        let weighted_identity = identity_score * self.config.identity_weight;
        let weighted_community = community_score * self.config.community_weight;

        let mut total_score = weighted_governance 
            + weighted_staking 
            + weighted_identity 
            + weighted_community;

        let time_decay_factor = if self.config.time_decay_enabled {
            self.apply_time_decay(&data.account_id, data.timestamp)
        } else {
            1.0
        };

        total_score *= time_decay_factor;

        let negative_adjustments = if self.config.negative_scoring_enabled {
            self.calculate_negative_adjustments(&data)
        } else {
            0.0
        };

        total_score -= negative_adjustments;
        total_score = total_score.max(self.config.min_score).min(self.config.max_score);

        let result = ScoreResult {
            account_id: data.account_id.clone(),
            total_score,
            governance_score,
            staking_score,
            identity_score,
            community_score,
            timestamp: data.timestamp,
            breakdown: ScoreBreakdown {
                weighted_governance,
                weighted_staking,
                weighted_identity,
                weighted_community,
                time_decay_factor,
                negative_adjustments,
            },
        };

        self.log_score_calculation(&result);
        self.store_score_history(result.clone());

        Ok(result)
    }

    fn apply_time_decay(&self, account_id: &str, current_timestamp: u64) -> f64 {
        if let Some(history) = self.score_history.get(account_id) {
            if let Some(last_score) = history.last() {
                let time_diff = current_timestamp.saturating_sub(last_score.timestamp);
                let days_elapsed = time_diff as f64 / 86400.0;
                return self.config.time_decay_rate.powf(days_elapsed);
            }
        }
        1.0
    }

    fn calculate_negative_adjustments(&self, data: &ChainData) -> f64 {
        let mut penalty = 0.0;

        if !data.identity_verified {
            penalty += 5.0;
        }

        if data.governance_votes == 0 && data.governance_proposals == 0 {
            penalty += 3.0;
        }

        if data.staking_amount == 0 {
            penalty += 2.0;
        }

        penalty
    }

    fn log_score_calculation(&self, result: &ScoreResult) {
        #[cfg(feature = "std")]
        {
            println!("📊 Score Calculation Log");
            println!("Account: {}", result.account_id);
            println!("Total Score: {:.2}", result.total_score);
            println!("Governance: {:.2} (weighted: {:.2})", 
                result.governance_score, result.breakdown.weighted_governance);
            println!("Staking: {:.2} (weighted: {:.2})", 
                result.staking_score, result.breakdown.weighted_staking);
            println!("Identity: {:.2} (weighted: {:.2})", 
                result.identity_score, result.breakdown.weighted_identity);
            println!("Community: {:.2} (weighted: {:.2})", 
                result.community_score, result.breakdown.weighted_community);
            println!("Time Decay: {:.4}", result.breakdown.time_decay_factor);
            println!("Penalties: {:.2}", result.breakdown.negative_adjustments);
            println!("Timestamp: {}", result.timestamp);
            println!("---");
        }
    }

    fn store_score_history(&mut self, result: ScoreResult) {
        let account_id = result.account_id.clone();
        self.score_history
            .entry(account_id)
            .or_insert_with(Vec::new)
            .push(result);
    }

    pub fn get_score_history(&self, account_id: &str) -> Option<&Vec<ScoreResult>> {
        self.score_history.get(account_id)
    }

    pub fn calculate_batch_scores(&mut self, data_batch: Vec<ChainData>) -> Vec<Result<ScoreResult, &'static str>> {
        data_batch.into_iter()
            .map(|data| self.calculate_score(data))
            .collect()
    }

    pub fn update_config(&mut self, new_config: ScoringConfig) {
        self.config = new_config;
    }

    pub fn export_history_json(&self, account_id: &str) -> Result<String, &'static str> {
        #[cfg(feature = "std")]
        {
            if let Some(history) = self.score_history.get(account_id) {
                serde_json::to_string_pretty(history)
                    .map_err(|_| "JSON serialization failed")
            } else {
                Err("No history found for account")
            }
        }
        
        #[cfg(not(feature = "std"))]
        Err("JSON export not available in no_std environment")
    }

    pub fn clear_old_history(&mut self, max_age_seconds: u64, current_timestamp: u64) {
        for history in self.score_history.values_mut() {
            history.retain(|score| {
                current_timestamp.saturating_sub(score.timestamp) <= max_age_seconds
            });
        }
    }
}

pub struct DataCleaner;

impl DataCleaner {
    pub fn normalize_chain_data(data: &mut ChainData) {
        if data.governance_votes > 10000 {
            data.governance_votes = 10000;
        }
        
        if data.governance_proposals > 1000 {
            data.governance_proposals = 1000;
        }
        
        if data.community_upvotes > data.community_posts * 100 {
            data.community_upvotes = data.community_posts * 100;
        }
        
        if data.staking_amount == 0 {
            data.staking_duration = 0;
        }
    }

    pub fn detect_anomalies(data: &ChainData) -> Vec<&'static str> {
        let mut anomalies = Vec::new();

        if data.governance_votes > 5000 {
            anomalies.push("Unusually high governance votes");
        }

        if data.community_upvotes > data.community_posts * 50 {
            anomalies.push("Suspicious upvote ratio");
        }

        if data.identity_judgements > 5 && !data.identity_verified {
            anomalies.push("Judgements without verification");
        }

        anomalies
    }

    pub fn fill_missing_data(data: &mut ChainData) {
        if data.timestamp == 0 {
            #[cfg(feature = "std")]
            {
                use std::time::{SystemTime, UNIX_EPOCH};
                data.timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
            }
        }

        if data.account_id.is_empty() {
            data.account_id = String::from("unknown");
        }
    }
}

#[cfg(feature = "std")]
pub mod parsers {
    use super::*;
    use serde_json;

    pub fn parse_json_data(json_str: &str) -> Result<ChainData, &'static str> {
        serde_json::from_str(json_str)
            .map_err(|_| "JSON parsing failed")
    }

    pub fn parse_csv_line(csv_line: &str) -> Result<ChainData, &'static str> {
        let fields: Vec<&str> = csv_line.split(',').collect();
        
        if fields.len() < 10 {
            return Err("Invalid CSV format");
        }

        Ok(ChainData {
            account_id: fields[0].to_string(),
            governance_votes: fields[1].parse().map_err(|_| "Invalid votes")?,
            governance_proposals: fields[2].parse().map_err(|_| "Invalid proposals")?,
            staking_amount: fields[3].parse().map_err(|_| "Invalid staking amount")?,
            staking_duration: fields[4].parse().map_err(|_| "Invalid staking duration")?,
            identity_verified: fields[5].parse().map_err(|_| "Invalid identity flag")?,
            identity_judgements: fields[6].parse().map_err(|_| "Invalid judgements")?,
            community_posts: fields[7].parse().map_err(|_| "Invalid posts")?,
            community_upvotes: fields[8].parse().map_err(|_| "Invalid upvotes")?,
            timestamp: fields[9].parse().map_err(|_| "Invalid timestamp")?,
        })
    }
}

#[cfg(feature = "std")]
use rayon::prelude::*;

#[cfg(feature = "std")]
impl ScoringEngine {
    pub fn calculate_parallel_scores(&self, data_batch: Vec<ChainData>) -> Vec<Result<ScoreResult, &'static str>> {
        data_batch.par_iter()
            .map(|data| {
                for metric in &self.metrics {
                    if let Err(e) = metric.validate_data(data) {
                        return Err(e);
                    }
                }

                let governance_score = GovernanceScoreMetric.calculate(data, &self.config);
                let staking_score = StakingScoreMetric.calculate(data, &self.config);
                let identity_score = IdentityScoreMetric.calculate(data, &self.config);
                let community_score = CommunityScoreMetric.calculate(data, &self.config);

                let weighted_governance = governance_score * self.config.governance_weight;
                let weighted_staking = staking_score * self.config.staking_weight;
                let weighted_identity = identity_score * self.config.identity_weight;
                let weighted_community = community_score * self.config.community_weight;

                let mut total_score = weighted_governance 
                    + weighted_staking 
                    + weighted_identity 
                    + weighted_community;

                total_score = total_score.max(self.config.min_score).min(self.config.max_score);

                Ok(ScoreResult {
                    account_id: data.account_id.clone(),
                    total_score,
                    governance_score,
                    staking_score,
                    identity_score,
                    community_score,
                    timestamp: data.timestamp,
                    breakdown: ScoreBreakdown {
                        weighted_governance,
                        weighted_staking,
                        weighted_identity,
                        weighted_community,
                        time_decay_factor: 1.0,
                        negative_adjustments: 0.0,
                    },
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_data() -> ChainData {
        ChainData {
            account_id: String::from("test_account"),
            governance_votes: 50,
            governance_proposals: 5,
            staking_amount: 1000000000000,
            staking_duration: 2592000,
            identity_verified: true,
            identity_judgements: 2,
            community_posts: 100,
            community_upvotes: 500,
            timestamp: 1699430400,
        }
    }

    #[test]
    fn test_governance_metric() {
        let metric = GovernanceScoreMetric;
        let data = create_test_data();
        let config = ScoringConfig::default();
        
        let score = metric.calculate(&data, &config);
        assert!(score > 0.0);
        assert!(score <= 100.0);
    }

    #[test]
    fn test_staking_metric() {
        let metric = StakingScoreMetric;
        let data = create_test_data();
        let config = ScoringConfig::default();
        
        let score = metric.calculate(&data, &config);
        assert!(score > 0.0);
        assert!(score <= 100.0);
    }

    #[test]
    fn test_identity_metric() {
        let metric = IdentityScoreMetric;
        let data = create_test_data();
        let config = ScoringConfig::default();
        
        let score = metric.calculate(&data, &config);
        assert!(score >= 50.0);
    }

    #[test]
    fn test_community_metric() {
        let metric = CommunityScoreMetric;
        let data = create_test_data();
        let config = ScoringConfig::default();
        
        let score = metric.calculate(&data, &config);
        assert!(score > 0.0);
        assert!(score <= 100.0);
    }

    #[test]
    fn test_scoring_engine() {
        let mut engine = ScoringEngine::new(ScoringConfig::default());
        let data = create_test_data();
        
        let result = engine.calculate_score(data);
        assert!(result.is_ok());
        
        let score = result.unwrap();
        assert!(score.total_score >= 0.0);
        assert!(score.total_score <= 100.0);
    }

    #[test]
    fn test_data_validation() {
        let metric = GovernanceScoreMetric;
        let mut data = create_test_data();
        data.governance_votes = 20000;
        
        let result = metric.validate_data(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_data_normalization() {
        let mut data = create_test_data();
        data.governance_votes = 15000;
        
        DataCleaner::normalize_chain_data(&mut data);
        assert_eq!(data.governance_votes, 10000);
    }

    #[test]
    fn test_anomaly_detection() {
        let mut data = create_test_data();
        data.governance_votes = 6000;
        
        let anomalies = DataCleaner::detect_anomalies(&data);
        assert!(!anomalies.is_empty());
    }

    #[test]
    fn test_score_history() {
        let mut engine = ScoringEngine::new(ScoringConfig::default());
        let data = create_test_data();
        
        let _ = engine.calculate_score(data);
        let history = engine.get_score_history("test_account");
        
        assert!(history.is_some());
        assert_eq!(history.unwrap().len(), 1);
    }

    #[test]
    fn test_batch_scoring() {
        let mut engine = ScoringEngine::new(ScoringConfig::default());
        let batch = vec![create_test_data(), create_test_data()];
        
        let results = engine.calculate_batch_scores(batch);
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.is_ok()));
    }

    #[test]
    fn test_time_decay() {
        let mut engine = ScoringEngine::new(ScoringConfig::default());
        let mut data1 = create_test_data();
        data1.timestamp = 1000000;
        
        let _ = engine.calculate_score(data1);
        
        let mut data2 = create_test_data();
        data2.timestamp = 1086400;
        
        let result = engine.calculate_score(data2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_negative_scoring() {
        let mut config = ScoringConfig::default();
        config.negative_scoring_enabled = true;
        
        let mut engine = ScoringEngine::new(config);
        let mut data = create_test_data();
        data.identity_verified = false;
        data.governance_votes = 0;
        data.governance_proposals = 0;
        
        let result = engine.calculate_score(data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_update() {
        let mut engine = ScoringEngine::new(ScoringConfig::default());
        
        let mut new_config = ScoringConfig::default();
        new_config.governance_weight = 0.5;
        
        engine.update_config(new_config);
        
        let data = create_test_data();
        let result = engine.calculate_score(data);
        assert!(result.is_ok());
    }
}
