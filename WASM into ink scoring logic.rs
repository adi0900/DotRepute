/! WASM-Compilable Scoring Logic Module
//! 
//! Professional no_std compatible module for integration into ink! smart contracts.
//! Provides core scoring algorithms and data validation for on-chain reputation systems.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use scale::{Decode, Encode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct MetricWeights {
    pub governance_weight: u32,
    pub staking_weight: u32,
    pub identity_weight: u32,
    pub community_weight: u32,
    pub total_weight: u32,
}

impl MetricWeights {
    pub fn new(governance: u32, staking: u32, identity: u32, community: u32) -> Self {
        let total = governance + staking + identity + community;
        Self {
            governance_weight: governance,
            staking_weight: staking,
            identity_weight: identity,
            community_weight: community,
            total_weight: total,
        }
    }

    pub fn default() -> Self {
        Self::new(30, 30, 20, 20)
    }

    pub fn validate(&self) -> bool {
        self.total_weight > 0 && self.total_weight == 100
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct RawMetrics {
    pub governance_votes: u32,
    pub governance_proposals: u32,
    pub staking_amount: u64,
    pub staking_duration: u64,
    pub identity_verified: bool,
    pub identity_judgements: u32,
    pub community_posts: u32,
    pub community_upvotes: u32,
}

impl RawMetrics {
    pub fn validate(&self) -> Result<(), ScoringError> {
        if self.governance_votes > 10000 {
            return Err(ScoringError::InvalidGovernanceVotes);
        }
        if self.governance_proposals > 1000 {
            return Err(ScoringError::InvalidGovernanceProposals);
        }
        if self.identity_judgements > 10 {
            return Err(ScoringError::InvalidIdentityJudgements);
        }
        if self.community_upvotes > self.community_posts.saturating_mul(100) {
            return Err(ScoringError::SuspiciousUpvoteRatio);
        }
        if self.staking_amount == 0 && self.staking_duration > 0 {
            return Err(ScoringError::InvalidStakingData);
        }
        Ok(())
    }

    pub fn normalize(&mut self) {
        if self.governance_votes > 10000 {
            self.governance_votes = 10000;
        }
        if self.governance_proposals > 1000 {
            self.governance_proposals = 1000;
        }
        if self.identity_judgements > 10 {
            self.identity_judgements = 10;
        }
        if self.community_upvotes > self.community_posts.saturating_mul(100) {
            self.community_upvotes = self.community_posts.saturating_mul(100);
        }
        if self.staking_amount == 0 {
            self.staking_duration = 0;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct ComputedScores {
    pub governance_score: u32,
    pub staking_score: u32,
    pub identity_score: u32,
    pub community_score: u32,
    pub total_score: u64,
    pub weighted_total: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ScoringError {
    InvalidGovernanceVotes,
    InvalidGovernanceProposals,
    InvalidIdentityJudgements,
    SuspiciousUpvoteRatio,
    InvalidStakingData,
    InvalidWeights,
    ScoreOverflow,
    DivisionByZero,
}

pub struct ScoringEngine;

impl ScoringEngine {
    pub fn calculate_governance_score(metrics: &RawMetrics) -> Result<u32, ScoringError> {
        let vote_score = Self::safe_min(metrics.governance_votes.saturating_mul(2), 50);
        let proposal_score = Self::safe_min(metrics.governance_proposals.saturating_mul(5), 50);
        Ok(vote_score.saturating_add(proposal_score))
    }

    pub fn calculate_staking_score(metrics: &RawMetrics) -> Result<u32, ScoringError> {
        if metrics.staking_amount == 0 {
            return Ok(0);
        }

        let amount_score = Self::log_scale(metrics.staking_amount, 10).min(60);
        
        let duration_days = metrics.staking_duration / 86400;
        let duration_score = Self::sqrt_scale(duration_days, 5).min(40);
        
        Ok(amount_score.saturating_add(duration_score))
    }

    pub fn calculate_identity_score(metrics: &RawMetrics) -> Result<u32, ScoringError> {
        let verified_score = if metrics.identity_verified { 50 } else { 0 };
        let judgement_score = Self::safe_min(metrics.identity_judgements.saturating_mul(10), 50);
        Ok(verified_score.saturating_add(judgement_score))
    }

    pub fn calculate_community_score(metrics: &RawMetrics) -> Result<u32, ScoringError> {
        let post_score = Self::safe_min(metrics.community_posts, 40);
        let upvote_score = Self::safe_min(metrics.community_upvotes / 2, 60);
        Ok(post_score.saturating_add(upvote_score))
    }

    pub fn calculate_weighted_score(
        scores: &ComputedScores,
        weights: &MetricWeights,
    ) -> Result<u64, ScoringError> {
        if !weights.validate() {
            return Err(ScoringError::InvalidWeights);
        }

        if weights.total_weight == 0 {
            return Err(ScoringError::DivisionByZero);
        }

        let weighted_governance = (scores.governance_score as u64)
            .saturating_mul(weights.governance_weight as u64);
        
        let weighted_staking = (scores.staking_score as u64)
            .saturating_mul(weights.staking_weight as u64);
        
        let weighted_identity = (scores.identity_score as u64)
            .saturating_mul(weights.identity_weight as u64);
        
        let weighted_community = (scores.community_score as u64)
            .saturating_mul(weights.community_weight as u64);

        let total = weighted_governance
            .saturating_add(weighted_staking)
            .saturating_add(weighted_identity)
            .saturating_add(weighted_community);

        Ok(total / (weights.total_weight as u64))
    }

    pub fn compute_all_scores(
        metrics: &RawMetrics,
        weights: &MetricWeights,
    ) -> Result<ComputedScores, ScoringError> {
        metrics.validate()?;

        let governance_score = Self::calculate_governance_score(metrics)?;
        let staking_score = Self::calculate_staking_score(metrics)?;
        let identity_score = Self::calculate_identity_score(metrics)?;
        let community_score = Self::calculate_community_score(metrics)?;

        let total_score = (governance_score as u64)
            .saturating_add(staking_score as u64)
            .saturating_add(identity_score as u64)
            .saturating_add(community_score as u64);

        let scores = ComputedScores {
            governance_score,
            staking_score,
            identity_score,
            community_score,
            total_score,
            weighted_total: 0,
        };

        let weighted_total = Self::calculate_weighted_score(&scores, weights)?;

        Ok(ComputedScores {
            weighted_total,
            ..scores
        })
    }

    fn safe_min(value: u32, max: u32) -> u32 {
        if value > max {
            max
        } else {
            value
        }
    }

    fn log_scale(value: u64, multiplier: u32) -> u32 {
        if value == 0 {
            return 0;
        }
        
        let log_value = Self::integer_log2(value);
        (log_value as u32).saturating_mul(multiplier)
    }

    fn sqrt_scale(value: u64, multiplier: u32) -> u32 {
        let sqrt_value = Self::integer_sqrt(value);
        (sqrt_value as u32).saturating_mul(multiplier)
    }

    fn integer_log2(mut n: u64) -> u64 {
        if n == 0 {
            return 0;
        }
        let mut log = 0;
        while n > 1 {
            n >>= 1;
            log += 1;
        }
        log
    }

    fn integer_sqrt(n: u64) -> u64 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }

        let mut x = n;
        let mut y = (x + 1) / 2;

        while y < x {
            x = y;
            y = (x + n / x) / 2;
        }
        x
    }
}

pub struct DataValidator;

impl DataValidator {
    pub fn validate_metrics(metrics: &RawMetrics) -> Result<(), ScoringError> {
        metrics.validate()
    }

    pub fn validate_weights(weights: &MetricWeights) -> bool {
        weights.validate()
    }

    pub fn detect_anomalies(metrics: &RawMetrics) -> Vec<AnomalyType> {
        let mut anomalies = Vec::new();

        if metrics.governance_votes > 5000 {
            anomalies.push(AnomalyType::HighGovernanceActivity);
        }

        if metrics.community_upvotes > metrics.community_posts.saturating_mul(50) {
            anomalies.push(AnomalyType::SuspiciousEngagement);
        }

        if metrics.identity_judgements > 5 && !metrics.identity_verified {
            anomalies.push(AnomalyType::InconsistentIdentity);
        }

        if metrics.staking_amount > 1_000_000_000_000_000 {
            anomalies.push(AnomalyType::ExtremeStaking);
        }

        anomalies
    }

    pub fn is_valid_score_range(score: u64, min: u64, max: u64) -> bool {
        score >= min && score <= max
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum AnomalyType {
    HighGovernanceActivity,
    SuspiciousEngagement,
    InconsistentIdentity,
    ExtremeStaking,
}

pub struct TimeDecayCalculator;

impl TimeDecayCalculator {
    pub fn calculate_decay(
        elapsed_seconds: u64,
        decay_rate_percent: u32,
    ) -> u32 {
        if decay_rate_percent >= 100 {
            return 0;
        }

        let days_elapsed = elapsed_seconds / 86400;
        
        if days_elapsed == 0 {
            return 100;
        }

        let retention_rate = 100u32.saturating_sub(decay_rate_percent);
        
        let mut factor = 100u32;
        for _ in 0..days_elapsed {
            factor = factor.saturating_mul(retention_rate) / 100;
            if factor == 0 {
                break;
            }
        }
        
        factor
    }

    pub fn apply_decay(score: u64, decay_factor: u32) -> u64 {
        score.saturating_mul(decay_factor as u64) / 100
    }
}

pub struct PenaltyCalculator;

impl PenaltyCalculator {
    pub fn calculate_penalties(metrics: &RawMetrics) -> u32 {
        let mut penalty = 0u32;

        if !metrics.identity_verified {
            penalty = penalty.saturating_add(5);
        }

        if metrics.governance_votes == 0 && metrics.governance_proposals == 0 {
            penalty = penalty.saturating_add(3);
        }

        if metrics.staking_amount == 0 {
            penalty = penalty.saturating_add(2);
        }

        if metrics.community_posts == 0 {
            penalty = penalty.saturating_add(1);
        }

        penalty
    }

    pub fn apply_penalties(score: u64, penalty_points: u32) -> u64 {
        score.saturating_sub(penalty_points as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_metrics() -> RawMetrics {
        RawMetrics {
            governance_votes: 50,
            governance_proposals: 5,
            staking_amount: 1_000_000_000_000,
            staking_duration: 2_592_000,
            identity_verified: true,
            identity_judgements: 2,
            community_posts: 100,
            community_upvotes: 500,
        }
    }

    #[test]
    fn test_metric_weights_creation() {
        let weights = MetricWeights::new(30, 30, 20, 20);
        assert_eq!(weights.total_weight, 100);
        assert!(weights.validate());
    }

    #[test]
    fn test_metric_weights_default() {
        let weights = MetricWeights::default();
        assert_eq!(weights.governance_weight, 30);
        assert_eq!(weights.staking_weight, 30);
        assert_eq!(weights.identity_weight, 20);
        assert_eq!(weights.community_weight, 20);
        assert!(weights.validate());
    }

    #[test]
    fn test_raw_metrics_validation() {
        let metrics = create_test_metrics();
        assert!(metrics.validate().is_ok());
    }

    #[test]
    fn test_invalid_governance_votes() {
        let mut metrics = create_test_metrics();
        metrics.governance_votes = 20000;
        assert_eq!(metrics.validate(), Err(ScoringError::InvalidGovernanceVotes));
    }

    #[test]
    fn test_metrics_normalization() {
        let mut metrics = create_test_metrics();
        metrics.governance_votes = 15000;
        metrics.normalize();
        assert_eq!(metrics.governance_votes, 10000);
    }

    #[test]
    fn test_governance_score_calculation() {
        let metrics = create_test_metrics();
        let score = ScoringEngine::calculate_governance_score(&metrics).unwrap();
        assert!(score > 0);
        assert!(score <= 100);
    }

    #[test]
    fn test_staking_score_calculation() {
        let metrics = create_test_metrics();
        let score = ScoringEngine::calculate_staking_score(&metrics).unwrap();
        assert!(score > 0);
        assert!(score <= 100);
    }

    #[test]
    fn test_identity_score_calculation() {
        let metrics = create_test_metrics();
        let score = ScoringEngine::calculate_identity_score(&metrics).unwrap();
        assert!(score >= 50);
        assert!(score <= 100);
    }

    #[test]
    fn test_community_score_calculation() {
        let metrics = create_test_metrics();
        let score = ScoringEngine::calculate_community_score(&metrics).unwrap();
        assert!(score > 0);
        assert!(score <= 100);
    }

    #[test]
    fn test_compute_all_scores() {
        let metrics = create_test_metrics();
        let weights = MetricWeights::default();
        let scores = ScoringEngine::compute_all_scores(&metrics, &weights).unwrap();
        
        assert!(scores.governance_score > 0);
        assert!(scores.staking_score > 0);
        assert!(scores.identity_score > 0);
        assert!(scores.community_score > 0);
        assert!(scores.total_score > 0);
        assert!(scores.weighted_total > 0);
    }

    #[test]
    fn test_weighted_score_calculation() {
        let scores = ComputedScores {
            governance_score: 75,
            staking_score: 60,
            identity_score: 80,
            community_score: 50,
            total_score: 265,
            weighted_total: 0,
        };
        let weights = MetricWeights::default();
        
        let weighted = ScoringEngine::calculate_weighted_score(&scores, &weights).unwrap();
        assert!(weighted > 0);
        assert!(weighted <= 100);
    }

    #[test]
    fn test_anomaly_detection() {
        let mut metrics = create_test_metrics();
        metrics.governance_votes = 6000;
        
        let anomalies = DataValidator::detect_anomalies(&metrics);
        assert!(!anomalies.is_empty());
        assert!(anomalies.contains(&AnomalyType::HighGovernanceActivity));
    }

    #[test]
    fn test_time_decay_calculation() {
        let decay_factor = TimeDecayCalculator::calculate_decay(86400, 5);
        assert!(decay_factor <= 100);
        assert!(decay_factor >= 90);
    }

    #[test]
    fn test_apply_decay() {
        let score = 100u64;
        let decayed = TimeDecayCalculator::apply_decay(score, 95);
        assert_eq!(decayed, 95);
    }

    #[test]
    fn test_penalty_calculation() {
        let mut metrics = create_test_metrics();
        metrics.identity_verified = false;
        metrics.governance_votes = 0;
        metrics.governance_proposals = 0;
        
        let penalty = PenaltyCalculator::calculate_penalties(&metrics);
        assert!(penalty >= 8);
    }

    #[test]
    fn test_apply_penalties() {
        let score = 100u64;
        let penalized = PenaltyCalculator::apply_penalties(score, 10);
        assert_eq!(penalized, 90);
    }

    #[test]
    fn test_integer_sqrt() {
        assert_eq!(ScoringEngine::integer_sqrt(0), 0);
        assert_eq!(ScoringEngine::integer_sqrt(1), 1);
        assert_eq!(ScoringEngine::integer_sqrt(4), 2);
        assert_eq!(ScoringEngine::integer_sqrt(9), 3);
        assert_eq!(ScoringEngine::integer_sqrt(16), 4);
        assert_eq!(ScoringEngine::integer_sqrt(100), 10);
    }

    #[test]
    fn test_integer_log2() {
        assert_eq!(ScoringEngine::integer_log2(0), 0);
        assert_eq!(ScoringEngine::integer_log2(1), 0);
        assert_eq!(ScoringEngine::integer_log2(2), 1);
        assert_eq!(ScoringEngine::integer_log2(4), 2);
        assert_eq!(ScoringEngine::integer_log2(8), 3);
        assert_eq!(ScoringEngine::integer_log2(16), 4);
    }

    #[test]
    fn test_safe_min() {
        assert_eq!(ScoringEngine::safe_min(10, 20), 10);
        assert_eq!(ScoringEngine::safe_min(30, 20), 20);
        assert_eq!(ScoringEngine::safe_min(20, 20), 20);
    }

    #[test]
    fn test_zero_staking_score() {
        let mut metrics = create_test_metrics();
        metrics.staking_amount = 0;
        metrics.staking_duration = 0;
        
        let score = ScoringEngine::calculate_staking_score(&metrics).unwrap();
        assert_eq!(score, 0);
    }

    #[test]
    fn test_invalid_weights() {
        let weights = MetricWeights::new(30, 30, 20, 15);
        assert!(!weights.validate());
    }

    #[test]
    fn test_score_overflow_protection() {
        let metrics = RawMetrics {
            governance_votes: 10000,
            governance_proposals: 1000,
            staking_amount: u64::MAX,
            staking_duration: u64::MAX,
            identity_verified: true,
            identity_judgements: 10,
            community_posts: u32::MAX,
            community_upvotes: u32::MAX / 2,
        };
        
        let weights = MetricWeights::default();
        let result = ScoringEngine::compute_all_scores(&metrics, &weights);
        assert!(result.is_ok());
    }
}
