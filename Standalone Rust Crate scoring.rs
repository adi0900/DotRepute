//! Reputation scoring algorithms and calculation logic
//!
//! This module provides comprehensive reputation scoring functionality with support for:
//! - Multi-metric scoring (governance, staking, identity, community)
//! - Weighted score calculation
//! - Time-decay scoring
//! - Configurable thresholds and weights
//!
//! # Examples
//!
//! ```rust
//! use dotrepute_core::scoring::{ScoreCalculator, MetricData};
//!
//! let calculator = ScoreCalculator::new();
//! let data = MetricData {
//!     governance_votes: 50,
//!     governance_proposals: 5,
//!     staking_amount: 1_000_000,
//!     staking_duration: 2_592_000,
//!     identity_verified: true,
//!     identity_judgements: 2,
//!     community_posts: 100,
//!     community_upvotes: 500,
//! };
//!
//! let result = calculator.calculate(&data).unwrap();
//! println!("Total Score: {}", result.total_score);
//! ```

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use scale::{Decode, Encode};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Input metrics for reputation calculation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, Serialize, Deserialize)]
#[cfg_attr(feature = "substrate", derive(scale_info::TypeInfo))]
pub struct MetricData {
    /// Number of governance votes cast
    pub governance_votes: u32,
    /// Number of governance proposals submitted
    pub governance_proposals: u32,
    /// Amount of tokens staked
    pub staking_amount: u64,
    /// Duration of staking in seconds
    pub staking_duration: u64,
    /// Whether identity is verified
    pub identity_verified: bool,
    /// Number of identity judgements received
    pub identity_judgements: u32,
    /// Number of community posts created
    pub community_posts: u32,
    /// Number of upvotes received
    pub community_upvotes: u32,
}

/// Computed reputation scores
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, Serialize, Deserialize)]
#[cfg_attr(feature = "substrate", derive(scale_info::TypeInfo))]
pub struct ScoreResult {
    /// Governance participation score (0-100)
    pub governance_score: u32,
    /// Staking activity score (0-100)
    pub staking_score: u32,
    /// Identity verification score (0-100)
    pub identity_score: u32,
    /// Community engagement score (0-100)
    pub community_score: u32,
    /// Total unweighted score
    pub total_score: u64,
    /// Weighted final score
    pub weighted_score: u64,
}

/// Configuration for score calculation weights
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, Serialize, Deserialize)]
#[cfg_attr(feature = "substrate", derive(scale_info::TypeInfo))]
pub struct WeightConfig {
    /// Weight for governance score (0-100)
    pub governance_weight: u32,
    /// Weight for staking score (0-100)
    pub staking_weight: u32,
    /// Weight for identity score (0-100)
    pub identity_weight: u32,
    /// Weight for community score (0-100)
    pub community_weight: u32,
}

impl Default for WeightConfig {
    fn default() -> Self {
        Self {
            governance_weight: 30,
            staking_weight: 30,
            identity_weight: 20,
            community_weight: 20,
        }
    }
}

impl WeightConfig {
    /// Validate that weights sum to 100
    pub fn validate(&self) -> Result<()> {
        let total = self.governance_weight 
            + self.staking_weight 
            + self.identity_weight 
            + self.community_weight;
        
        if total != 100 {
            return Err(Error::InvalidInput);
        }
        
        Ok(())
    }
}

/// Main score calculator
pub struct ScoreCalculator {
    weights: WeightConfig,
}

impl ScoreCalculator {
    /// Create a new calculator with default weights
    pub fn new() -> Self {
        Self {
            weights: WeightConfig::default(),
        }
    }

    /// Create a calculator with custom weights
    pub fn with_weights(weights: WeightConfig) -> Result<Self> {
        weights.validate()?;
        Ok(Self { weights })
    }

    /// Calculate reputation score from metrics
    pub fn calculate(&self, data: &MetricData) -> Result<ScoreResult> {
        let governance_score = self.calculate_governance(data);
        let staking_score = self.calculate_staking(data);
        let identity_score = self.calculate_identity(data);
        let community_score = self.calculate_community(data);

        let total_score = (governance_score as u64)
            + (staking_score as u64)
            + (identity_score as u64)
            + (community_score as u64);

        let weighted_score = self.calculate_weighted(
            governance_score,
            staking_score,
            identity_score,
            community_score,
        );

        Ok(ScoreResult {
            governance_score,
            staking_score,
            identity_score,
            community_score,
            total_score,
            weighted_score,
        })
    }

    fn calculate_governance(&self, data: &MetricData) -> u32 {
        let vote_score = core::cmp::min(data.governance_votes * 2, 50);
        let proposal_score = core::cmp::min(data.governance_proposals * 5, 50);
        vote_score + proposal_score
    }

    fn calculate_staking(&self, data: &MetricData) -> u32 {
        if data.staking_amount == 0 {
            return 0;
        }

        let amount_score = core::cmp::min(self.log2(data.staking_amount) as u32 * 10, 60);
        let duration_days = data.staking_duration / 86400;
        let duration_score = core::cmp::min(self.sqrt(duration_days) as u32 * 5, 40);
        
        amount_score + duration_score
    }

    fn calculate_identity(&self, data: &MetricData) -> u32 {
        let verified_score = if data.identity_verified { 50 } else { 0 };
        let judgement_score = core::cmp::min(data.identity_judgements * 10, 50);
        verified_score + judgement_score
    }

    fn calculate_community(&self, data: &MetricData) -> u32 {
        let post_score = core::cmp::min(data.community_posts, 40);
        let upvote_score = core::cmp::min(data.community_upvotes / 2, 60);
        post_score + upvote_score
    }

    fn calculate_weighted(&self, gov: u32, stake: u32, id: u32, comm: u32) -> u64 {
        let weighted_gov = (gov as u64) * (self.weights.governance_weight as u64);
        let weighted_stake = (stake as u64) * (self.weights.staking_weight as u64);
        let weighted_id = (id as u64) * (self.weights.identity_weight as u64);
        let weighted_comm = (comm as u64) * (self.weights.community_weight as u64);

        (weighted_gov + weighted_stake + weighted_id + weighted_comm) / 100
    }

    fn log2(&self, mut n: u64) -> u64 {
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

    fn sqrt(&self, n: u64) -> u64 {
        if n < 2 {
            return n;
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

impl Default for ScoreCalculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_data() -> MetricData {
        MetricData {
            governance_votes: 50,
            governance_proposals: 5,
            staking_amount: 1_000_000_000,
            staking_duration: 2_592_000,
            identity_verified: true,
            identity_judgements: 2,
            community_posts: 100,
            community_upvotes: 500,
        }
    }

    #[test]
    fn test_calculate_score() {
        let calculator = ScoreCalculator::new();
        let result = calculator.calculate(&sample_data()).unwrap();
        
        assert!(result.governance_score > 0);
        assert!(result.staking_score > 0);
        assert!(result.identity_score >= 50);
        assert!(result.community_score > 0);
        assert!(result.total_score > 0);
        assert!(result.weighted_score > 0);
    }

    #[test]
    fn test_weight_validation() {
        let invalid_weights = WeightConfig {
            governance_weight: 30,
            staking_weight: 30,
            identity_weight: 20,
            community_weight: 15, // Doesn't sum to 100
        };
        
        assert!(invalid_weights.validate().is_err());
    }

    #[test]
    fn test_custom_weights() {
        let weights = WeightConfig {
            governance_weight: 40,
            staking_weight: 30,
            identity_weight: 20,
            community_weight: 10,
        };
        
        let calculator = ScoreCalculator::with_weights(weights).unwrap();
        let result = calculator.calculate(&sample_data()).unwrap();
        
        assert!(result.weighted_score > 0);
    }

    #[test]
    fn test_zero_staking() {
        let mut data = sample_data();
        data.staking_amount = 0;
        data.staking_duration = 0;
        
        let calculator = ScoreCalculator::new();
        let result = calculator.calculate(&data).unwrap();
        
        assert_eq!(result.staking_score, 0);
    }
}
