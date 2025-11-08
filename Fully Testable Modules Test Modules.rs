/! Fully Testable Modules - Comprehensive Testing Infrastructure
//!
//! Provides complete test coverage with mock data support, edge case handling,
//! and CI/CD integration for blockchain reputation systems.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{vec::Vec, string::String};

#[cfg(feature = "std")]
use std::{vec::Vec, string::String};

use serde::{Deserialize, Serialize};
use scale::{Decode, Encode};

/// Mock account structure for testing
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, Serialize, Deserialize)]
pub struct MockAccount {
    pub id: u32,
    pub name: String,
    pub reputation_score: u64,
}

impl MockAccount {
    /// Create a new mock account
    ///
    /// # Examples
    ///
    /// ```
    /// use testable_modules::MockAccount;
    /// let account = MockAccount::new(1, "Alice");
    /// assert_eq!(account.id, 1);
    /// ```
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            reputation_score: 0,
        }
    }

    /// Create a high reputation mock account
    pub fn high_reputation(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            reputation_score: 95,
        }
    }

    /// Create a low reputation mock account
    pub fn low_reputation(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            reputation_score: 15,
        }
    }
}

/// Fake stake data for testing staking logic
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub struct FakeStakeData {
    pub amount: u64,
    pub duration: u64,
    pub is_active: bool,
}

impl FakeStakeData {
    /// Create valid stake data
    pub fn valid() -> Self {
        Self {
            amount: 1_000_000_000,
            duration: 2_592_000,
            is_active: true,
        }
    }

    /// Create invalid stake data (zero amount)
    pub fn invalid_zero_amount() -> Self {
        Self {
            amount: 0,
            duration: 2_592_000,
            is_active: true,
        }
    }

    /// Create inactive stake data
    pub fn inactive() -> Self {
        Self {
            amount: 1_000_000_000,
            duration: 2_592_000,
            is_active: false,
        }
    }

    /// Create edge case: very large amount
    pub fn edge_large_amount() -> Self {
        Self {
            amount: u64::MAX,
            duration: 2_592_000,
            is_active: true,
        }
    }
}

/// Dummy governance activity for testing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub struct DummyGovernanceActivity {
    pub votes_count: u32,
    pub proposals_count: u32,
    pub last_activity_timestamp: u64,
}

impl DummyGovernanceActivity {
    /// Create active governance participant
    pub fn active_participant() -> Self {
        Self {
            votes_count: 50,
            proposals_count: 5,
            last_activity_timestamp: 1699430400,
        }
    }

    /// Create inactive participant
    pub fn inactive_participant() -> Self {
        Self {
            votes_count: 0,
            proposals_count: 0,
            last_activity_timestamp: 0,
        }
    }

    /// Create edge case: excessive votes
    pub fn excessive_votes() -> Self {
        Self {
            votes_count: 10000,
            proposals_count: 1000,
            last_activity_timestamp: 1699430400,
        }
    }
}

/// Error types for testable modules
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub enum ScoreError {
    InvalidStakeAmount,
    InvalidDuration,
    InvalidVoteCount,
    InvalidProposalCount,
    CalculationOverflow,
    DivisionByZero,
}

/// Result type for score calculations
pub type Result<T> = core::result::Result<T, ScoreError>;

/// Staking score calculator with full test coverage
pub struct StakingScoreCalculator;

impl StakingScoreCalculator {
    /// Calculate staking score from stake data
    ///
    /// # Examples
    ///
    /// ```
    /// use testable_modules::{StakingScoreCalculator, FakeStakeData};
    /// let data = FakeStakeData::valid();
    /// let score = StakingScoreCalculator::calculate(&data).unwrap();
    /// assert!(score > 0);
    /// ```
    pub fn calculate(stake: &FakeStakeData) -> Result<u64> {
        if stake.amount == 0 {
            return Err(ScoreError::InvalidStakeAmount);
        }

        if !stake.is_active {
            return Ok(0);
        }

        let amount_score = Self::calculate_amount_score(stake.amount)?;
        let duration_score = Self::calculate_duration_score(stake.duration);

        Ok(amount_score.saturating_add(duration_score))
    }

    fn calculate_amount_score(amount: u64) -> Result<u64> {
        if amount == 0 {
            return Err(ScoreError::InvalidStakeAmount);
        }

        let log_value = Self::integer_log2(amount);
        Ok(log_value.saturating_mul(10).min(60))
    }

    fn calculate_duration_score(duration: u64) -> u64 {
        let days = duration / 86400;
        let sqrt_days = Self::integer_sqrt(days);
        sqrt_days.saturating_mul(5).min(40)
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

/// Governance score calculator with full test coverage
pub struct GovernanceScoreCalculator;

impl GovernanceScoreCalculator {
    /// Calculate governance score from activity data
    ///
    /// # Examples
    ///
    /// ```
    /// use testable_modules::{GovernanceScoreCalculator, DummyGovernanceActivity};
    /// let activity = DummyGovernanceActivity::active_participant();
    /// let score = GovernanceScoreCalculator::calculate(&activity).unwrap();
    /// assert!(score > 0);
    /// ```
    pub fn calculate(activity: &DummyGovernanceActivity) -> Result<u64> {
        if activity.votes_count > 10000 {
            return Err(ScoreError::InvalidVoteCount);
        }

        if activity.proposals_count > 1000 {
            return Err(ScoreError::InvalidProposalCount);
        }

        let vote_score = (activity.votes_count as u64).saturating_mul(2).min(50);
        let proposal_score = (activity.proposals_count as u64).saturating_mul(5).min(50);

        Ok(vote_score.saturating_add(proposal_score))
    }

    /// Check if user is active participant
    pub fn is_active(activity: &DummyGovernanceActivity) -> bool {
        activity.votes_count > 0 || activity.proposals_count > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Mock Account Tests ==========

    #[test]
    fn test_mock_account_creation() {
        let account = MockAccount::new(1, "Alice");
        assert_eq!(account.id, 1);
        assert_eq!(account.name, "Alice");
        assert_eq!(account.reputation_score, 0);
    }

    #[test]
    fn test_high_reputation_account() {
        let account = MockAccount::high_reputation(2, "Bob");
        assert_eq!(account.reputation_score, 95);
    }

    #[test]
    fn test_low_reputation_account() {
        let account = MockAccount::low_reputation(3, "Charlie");
        assert_eq!(account.reputation_score, 15);
    }

    // ========== Fake Stake Data Tests ==========

    #[test]
    fn test_valid_stake_data() {
        let stake = FakeStakeData::valid();
        assert!(stake.amount > 0);
        assert!(stake.is_active);
    }

    #[test]
    fn test_invalid_zero_amount_stake() {
        let stake = FakeStakeData::invalid_zero_amount();
        assert_eq!(stake.amount, 0);
    }

    #[test]
    fn test_inactive_stake() {
        let stake = FakeStakeData::inactive();
        assert!(!stake.is_active);
    }

    #[test]
    fn test_edge_large_amount_stake() {
        let stake = FakeStakeData::edge_large_amount();
        assert_eq!(stake.amount, u64::MAX);
    }

    // ========== Dummy Governance Tests ==========

    #[test]
    fn test_active_governance_participant() {
        let activity = DummyGovernanceActivity::active_participant();
        assert!(activity.votes_count > 0);
        assert!(activity.proposals_count > 0);
    }

    #[test]
    fn test_inactive_governance_participant() {
        let activity = DummyGovernanceActivity::inactive_participant();
        assert_eq!(activity.votes_count, 0);
        assert_eq!(activity.proposals_count, 0);
    }

    #[test]
    fn test_excessive_votes_governance() {
        let activity = DummyGovernanceActivity::excessive_votes();
        assert_eq!(activity.votes_count, 10000);
    }

    // ========== Staking Score Calculator Tests ==========

    #[test]
    fn test_calculate_valid_stake_score() {
        let stake = FakeStakeData::valid();
        let result = StakingScoreCalculator::calculate(&stake);
        assert!(result.is_ok());
        assert!(result.unwrap() > 0);
    }

    #[test]
    fn test_calculate_zero_amount_returns_error() {
        let stake = FakeStakeData::invalid_zero_amount();
        let result = StakingScoreCalculator::calculate(&stake);
        assert_eq!(result, Err(ScoreError::InvalidStakeAmount));
    }

    #[test]
    fn test_calculate_inactive_stake_returns_zero() {
        let stake = FakeStakeData::inactive();
        let result = StakingScoreCalculator::calculate(&stake);
        assert_eq!(result, Ok(0));
    }

    #[test]
    fn test_calculate_large_amount_no_overflow() {
        let stake = FakeStakeData::edge_large_amount();
        let result = StakingScoreCalculator::calculate(&stake);
        assert!(result.is_ok());
    }

    #[test]
    fn test_amount_score_calculation() {
        let result = StakingScoreCalculator::calculate_amount_score(1_000_000_000);
        assert!(result.is_ok());
        assert!(result.unwrap() <= 60);
    }

    #[test]
    fn test_duration_score_calculation() {
        let score = StakingScoreCalculator::calculate_duration_score(2_592_000);
        assert!(score <= 40);
    }

    #[test]
    fn test_integer_log2() {
        assert_eq!(StakingScoreCalculator::integer_log2(0), 0);
        assert_eq!(StakingScoreCalculator::integer_log2(1), 0);
        assert_eq!(StakingScoreCalculator::integer_log2(2), 1);
        assert_eq!(StakingScoreCalculator::integer_log2(8), 3);
        assert_eq!(StakingScoreCalculator::integer_log2(16), 4);
    }

    #[test]
    fn test_integer_sqrt() {
        assert_eq!(StakingScoreCalculator::integer_sqrt(0), 0);
        assert_eq!(StakingScoreCalculator::integer_sqrt(1), 1);
        assert_eq!(StakingScoreCalculator::integer_sqrt(4), 2);
        assert_eq!(StakingScoreCalculator::integer_sqrt(9), 3);
        assert_eq!(StakingScoreCalculator::integer_sqrt(16), 4);
        assert_eq!(StakingScoreCalculator::integer_sqrt(100), 10);
    }

    // ========== Governance Score Calculator Tests ==========

    #[test]
    fn test_calculate_active_governance_score() {
        let activity = DummyGovernanceActivity::active_participant();
        let result = GovernanceScoreCalculator::calculate(&activity);
        assert!(result.is_ok());
        assert!(result.unwrap() > 0);
    }

    #[test]
    fn test_calculate_inactive_governance_score() {
        let activity = DummyGovernanceActivity::inactive_participant();
        let result = GovernanceScoreCalculator::calculate(&activity);
        assert_eq!(result, Ok(0));
    }

    #[test]
    fn test_excessive_votes_returns_error() {
        let mut activity = DummyGovernanceActivity::excessive_votes();
        activity.votes_count = 10001;
        let result = GovernanceScoreCalculator::calculate(&activity);
        assert_eq!(result, Err(ScoreError::InvalidVoteCount));
    }

    #[test]
    fn test_excessive_proposals_returns_error() {
        let mut activity = DummyGovernanceActivity::active_participant();
        activity.proposals_count = 1001;
        let result = GovernanceScoreCalculator::calculate(&activity);
        assert_eq!(result, Err(ScoreError::InvalidProposalCount));
    }

    #[test]
    fn test_is_active_participant() {
        let active = DummyGovernanceActivity::active_participant();
        assert!(GovernanceScoreCalculator::is_active(&active));

        let inactive = DummyGovernanceActivity::inactive_participant();
        assert!(!GovernanceScoreCalculator::is_active(&inactive));
    }

    #[test]
    fn test_vote_score_boundary() {
        let activity = DummyGovernanceActivity {
            votes_count: 25,
            proposals_count: 0,
            last_activity_timestamp: 0,
        };
        let result = GovernanceScoreCalculator::calculate(&activity);
        assert_eq!(result, Ok(50));
    }

    #[test]
    fn test_proposal_score_boundary() {
        let activity = DummyGovernanceActivity {
            votes_count: 0,
            proposals_count: 10,
            last_activity_timestamp: 0,
        };
        let result = GovernanceScoreCalculator::calculate(&activity);
        assert_eq!(result, Ok(50));
    }

    #[test]
    fn test_combined_score_max() {
        let activity = DummyGovernanceActivity {
            votes_count: 1000,
            proposals_count: 100,
            last_activity_timestamp: 0,
        };
        let result = GovernanceScoreCalculator::calculate(&activity);
        assert_eq!(result, Ok(100));
    }

    // ========== Edge Case Tests ==========

    #[test]
    fn test_saturating_addition_no_overflow() {
        let stake = FakeStakeData {
            amount: u64::MAX,
            duration: u64::MAX,
            is_active: true,
        };
        let result = StakingScoreCalculator::calculate(&stake);
        assert!(result.is_ok());
    }

    #[test]
    fn test_error_types_equality() {
        assert_eq!(ScoreError::InvalidStakeAmount, ScoreError::InvalidStakeAmount);
        assert_ne!(ScoreError::InvalidStakeAmount, ScoreError::InvalidDuration);
    }

    // ========== Integration Tests ==========

    #[test]
    fn test_full_workflow_valid_data() {
        let account = MockAccount::new(1, "Alice");
        let stake = FakeStakeData::valid();
        let governance = DummyGovernanceActivity::active_participant();

        let stake_score = StakingScoreCalculator::calculate(&stake).unwrap();
        let gov_score = GovernanceScoreCalculator::calculate(&governance).unwrap();

        let total_score = stake_score.saturating_add(gov_score);
        assert!(total_score > 0);
        assert_eq!(account.id, 1);
    }

    #[test]
    fn test_full_workflow_invalid_data() {
        let stake = FakeStakeData::invalid_zero_amount();
        let result = StakingScoreCalculator::calculate(&stake);
        assert!(result.is_err());
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod integration_tests {
    use super::*;

    #[test]
    fn integration_test_complete_scoring() {
        let accounts = vec![
            MockAccount::high_reputation(1, "Alice"),
            MockAccount::low_reputation(2, "Bob"),
            MockAccount::new(3, "Charlie"),
        ];

        for account in &accounts {
            assert!(account.name.len() > 0);
        }

        let stakes = vec![
            FakeStakeData::valid(),
            FakeStakeData::inactive(),
        ];

        for stake in &stakes {
            let _ = StakingScoreCalculator::calculate(stake);
        }
    }
}
