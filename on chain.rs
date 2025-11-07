use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Participation types for tracking different activities
#[derive(Debug, Clone, PartialEq)]
pub enum ParticipationType {
    ReferendumVoting,          // Referendum voting participation
    TreasuryProposalSeconding, // Treasury proposal seconding/endorsement
    TreasuryTipsBounties,      // Treasury tips and bounties contributions
    ValidatorNominatorSupport, // Validator/nominator support history
    RewardPayoutStake,        // Reward payout and stake history
    SlashingPenalty,          // Slashing or penalty history
    ProposalExtrinsic,        // Proposal extrinsic history
    OpenGovTrack,             // OpenGov/track based participation
    Delegation,               // Delegation giving/receiving history
}

// Referendum voting participation record
#[derive(Debug, Clone)]
pub struct ReferendumParticipation {
    pub referendum_id: u32,        // Referendum ID
    pub voted: bool,               // Whether voted
    pub vote_type: Option<String>, // Vote type (Aye/Nay/Abstain)
    pub balance: u128,             // Voting balance
    pub conviction: u8,            // Conviction multiplier
    pub timestamp: u64,            // Vote timestamp
}

// Treasury proposal seconding/endorsement record
#[derive(Debug, Clone)]
pub struct TreasurySeconding {
    pub proposal_id: u32,          // Proposal ID
    pub seconder: u32,             // Seconding account
    pub deposit: u128,             // Deposit amount
    pub timestamp: u64,            // Seconding timestamp
}

// Treasury tips and bounties contribution record
#[derive(Debug, Clone)]
pub struct TreasuryContribution {
    pub contribution_id: u32,      // Contribution ID
    pub contributor: u32,          // Contributor account
    pub amount: u128,              // Contribution amount
    pub contribution_type: String, // Tip or Bounty
    pub timestamp: u64,            // Contribution timestamp
}

// Validator/Nominator support history
#[derive(Debug, Clone)]
pub struct ValidatorNominatorHistory {
    pub validator_id: u32,         // Validator ID
    pub nominator_id: u32,         // Nominator ID
    pub amount: u128,              // Staked amount
    pub start_time: u64,           // Start timestamp
    pub end_time: Option<u64>,     // End timestamp (if ended)
    pub is_active: bool,           // Active status
}

// Reward payout and stake history
#[derive(Debug, Clone)]
pub struct RewardStakeHistory {
    pub account_id: u32,           // Account ID
    pub reward_amount: u128,       // Reward amount
    pub stake_amount: u128,        // Stake amount
    pub session_index: u32,        // Session index
    pub timestamp: u64,            // History timestamp
}

// Slashing or penalty history
#[derive(Debug, Clone)]
pub struct SlashingHistory {
    pub account_id: u32,           // Account ID
    pub slash_amount: u128,        // Slashed amount
    pub slash_type: String,        // Slash type
    pub reason: String,            // Slash reason
    pub session_index: u32,        // Session index
    pub timestamp: u64,            // Slash timestamp
}

// Proposal extrinsic history
#[derive(Debug, Clone)]
pub struct ProposalExtrinsicHistory {
    pub extrinsic_id: u32,         // Extrinsic ID
    pub account_id: u32,           // Account ID
    pub extrinsic_type: String,    // Extrinsic type
    pub data: String,              // Extrinsic data
    pub timestamp: u64,            // Extrinsic timestamp
}

// OpenGov/Track based participation metrics
#[derive(Debug, Clone)]
pub struct OpenGovParticipation {
    pub track_id: u32,             // Track ID
    pub account_id: u32,           // Account ID
    pub participation_type: String, // Participation type
    pub count: u32,                // Participation count
    pub last_participation: u64,   // Last participation timestamp
}

// Delegation history (giving/receiving)
#[derive(Debug, Clone)]
pub struct DelegationHistory {
    pub delegator: u32,            // Delegator account
    pub delegatee: u32,            // Delegatee account
    pub amount: u128,              // Delegated amount
    pub start_time: u64,           // Start timestamp
    pub end_time: Option<u64>,     // End timestamp (if ended)
    pub is_active: bool,           // Active status
    pub delegation_type: String,   // Giving or Receiving
}

// Social trust score metrics
#[derive(Debug, Clone)]
pub struct SocialTrustMetrics {
    pub account_id: u32,                           // Account ID
    pub total_participations: u32,                 // Total participations
    pub referendum_votes: Vec<ReferendumParticipation>, // Referendum voting history
    pub treasury_secondings: Vec<TreasurySeconding>,    // Treasury seconding history
    pub treasury_contributions: Vec<TreasuryContribution>, // Treasury contributions
    pub validator_nominator_history: Vec<ValidatorNominatorHistory>, // Validator/nominator history
    pub reward_stake_history: Vec<RewardStakeHistory>,   // Reward/stake history
    pub slashing_history: Vec<SlashingHistory>,          // Slashing history
    pub proposal_extrinsic_history: Vec<ProposalExtrinsicHistory>, // Proposal extrinsic history
    pub opengov_participation: Vec<OpenGovParticipation>, // OpenGov participation
    pub delegation_history: Vec<DelegationHistory>,      // Delegation history
    pub trust_score: f64,                            // Overall trust score
    pub last_updated: u64,                           // Last update timestamp
}

impl SocialTrustMetrics {
    pub fn new(account_id: u32) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        SocialTrustMetrics {
            account_id,
            total_participations: 0,
            referendum_votes: Vec::new(),
            treasury_secondings: Vec::new(),
            treasury_contributions: Vec::new(),
            validator_nominator_history: Vec::new(),
            reward_stake_history: Vec::new(),
            slashing_history: Vec::new(),
            proposal_extrinsic_history: Vec::new(),
            opengov_participation: Vec::new(),
            delegation_history: Vec::new(),
            trust_score: 0.0,
            last_updated: now,
        }
    }

    // Referendum Voting Participation
    pub fn add_referendum_vote(&mut self, referendum_id: u32, voted: bool, vote_type: Option<String>, 
                              balance: u128, conviction: u8) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let vote = ReferendumParticipation {
            referendum_id,
            voted,
            vote_type,
            balance,
            conviction,
            timestamp,
        };
        
        self.referendum_votes.push(vote);
        self.total_participations += 1;
        self.update_trust_score();
    }

    pub fn get_referendum_voting_history(&self) -> &Vec<ReferendumParticipation> {
        &self.referendum_votes
    }

    // Treasury Proposal Seconding / Endorsement
    pub fn add_treasury_seconding(&mut self, proposal_id: u32, seconder: u32, deposit: u128) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let seconding = TreasurySeconding {
            proposal_id,
            seconder,
            deposit,
            timestamp,
        };
        
        self.treasury_secondings.push(seconding);
        self.total_participations += 1;
        self.update_trust_score();
    }

    pub fn get_treasury_seconding_history(&self) -> &Vec<TreasurySeconding> {
        &self.treasury_secondings
    }

    // Treasury Tips and Bounties Contributions
    pub fn add_treasury_contribution(&mut self, contribution_id: u32, contributor: u32, 
                                   amount: u128, contribution_type: String) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let contribution = TreasuryContribution {
            contribution_id,
            contributor,
            amount,
            contribution_type,
            timestamp,
        };
        
        self.treasury_contributions.push(contribution);
        self.total_participations += 1;
        self.update_trust_score();
    }

    pub fn get_treasury_contributions_history(&self) -> &Vec<TreasuryContribution> {
        &self.treasury_contributions
    }

    // Validator / Nominator History
    pub fn add_validator_nominator_support(&mut self, validator_id: u32, nominator_id: u32, 
                                         amount: u128, is_active: bool) {
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let support = ValidatorNominatorHistory {
            validator_id,
            nominator_id,
            amount,
            start_time,
            end_time: None,
            is_active,
        };
        
        self.validator_nominator_history.push(support);
        self.total_participations += 1;
        self.update_trust_score();
    }

    pub fn get_validator_nominator_history(&self) -> &Vec<ValidatorNominatorHistory> {
        &self.validator_nominator_history
    }

    // Reward (payout) and Stake History
    pub fn add_reward_stake_history(&mut self, account_id: u32, reward_amount: u128, 
                                  stake_amount: u128, session_index: u32) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let history = RewardStakeHistory {
            account_id,
            reward_amount,
            stake_amount,
            session_index,
            timestamp,
        };
        
        self.reward_stake_history.push(history);
        self.total_participations += 1;
        self.update_trust_score();
    }

    pub fn get_reward_stake_history(&self) -> &Vec<RewardStakeHistory> {
        &self.reward_stake_history
    }

    // Slashing or Penalty History
    pub fn add_slashing_history(&mut self, account_id: u32, slash_amount: u128, 
                              slash_type: String, reason: String, session_index: u32) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let slashing = SlashingHistory {
            account_id,
            slash_amount,
            slash_type,
            reason,
            session_index,
            timestamp,
        };
        
        self.slashing_history.push(slashing);
        self.total_participations += 1;
        self.update_trust_score();
    }

    pub fn get_slashing_history(&self) -> &Vec<SlashingHistory> {
        &self.slashing_history
    }

    // Proposal Extrinsic History
    pub fn add_proposal_extrinsic_history(&mut self, extrinsic_id: u32, account_id: u32, 
                                        extrinsic_type: String, data: String) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let extrinsic = ProposalExtrinsicHistory {
            extrinsic_id,
            account_id,
            extrinsic_type,
            data,
            timestamp,
        };
        
        self.proposal_extrinsic_history.push(extrinsic);
        self.total_participations += 1;
        self.update_trust_score();
    }

    pub fn get_proposal_extrinsic_history(&self) -> &Vec<ProposalExtrinsicHistory> {
        &self.proposal_extrinsic_history
    }

    // OpenGov / Track based participation metrics
    pub fn add_opengov_participation(&mut self, track_id: u32, account_id: u32, 
                                   participation_type: String) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        // Check if participation already exists for this track and type
        let mut found = false;
        for participation in &mut self.opengov_participation {
            if participation.track_id == track_id && participation.participation_type == participation_type {
                participation.count += 1;
                participation.last_participation = timestamp;
                found = true;
                break;
            }
        }
        
        if !found {
            let participation = OpenGovParticipation {
                track_id,
                account_id,
                participation_type,
                count: 1,
                last_participation: timestamp,
            };
            self.opengov_participation.push(participation);
        }
        
        self.total_participations += 1;
        self.update_trust_score();
    }

    pub fn get_opengov_participation(&self) -> &Vec<OpenGovParticipation> {
        &self.opengov_participation
    }

    // Delegation giving/receiving history
    pub fn add_delegation_history(&mut self, delegator: u32, delegatee: u32, amount: u128, 
                                delegation_type: String) {
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let delegation = DelegationHistory {
            delegator,
            delegatee,
            amount,
            start_time,
            end_time: None,
            is_active: true,
            delegation_type,
        };
        
        self.delegation_history.push(delegation);
        self.total_participations += 1;
        self.update_trust_score();
    }

    pub fn get_delegation_history(&self) -> &Vec<DelegationHistory> {
        &self.delegation_history
    }

    // Update trust score based on participation metrics
    fn update_trust_score(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        // Base score calculation based on various factors
        let mut score = 0.0;
        
        // Positive contributions
        score += self.referendum_votes.len() as f64 * 0.5;
        score += self.treasury_secondings.len() as f64 * 1.0;
        score += self.treasury_contributions.len() as f64 * 1.5;
        score += self.validator_nominator_history.len() as f64 * 1.0;
        score += self.reward_stake_history.len() as f64 * 0.5;
        score += self.proposal_extrinsic_history.len() as f64 * 1.0;
        score += self.opengov_participation.iter().map(|p| p.count as f64).sum::<f64>() * 0.3;
        score += self.delegation_history.len() as f64 * 0.5;
        
        // Negative factors (slashing history reduces score)
        score -= self.slashing_history.len() as f64 * 2.0;
        
        // Recency factor - more recent activities get higher weight
        let recent_activities = self.get_recent_activities_count(now);
        score += recent_activities as f64 * 0.1;
        
        // Normalize score to 0-100 range
        self.trust_score = score.max(0.0).min(100.0);
        self.last_updated = now;
    }

    // Get count of recent activities (within last 30 days)
    fn get_recent_activities_count(&self, current_time: u64) -> u32 {
        let thirty_days_ago = current_time - (30 * 24 * 60 * 60); // 30 days in seconds
        let mut count = 0;
        
        count += self.referendum_votes.iter()
            .filter(|v| v.timestamp > thirty_days_ago).count();
        count += self.treasury_secondings.iter()
            .filter(|s| s.timestamp > thirty_days_ago).count();
        count += self.treasury_contributions.iter()
            .filter(|c| c.timestamp > thirty_days_ago).count();
        count += self.proposal_extrinsic_history.iter()
            .filter(|e| e.timestamp > thirty_days_ago).count();
        count += self.opengov_participation.iter()
            .filter(|p| p.last_participation > thirty_days_ago).map(|p| p.count).sum::<u32>() as usize;
            
        count as u32
    }

    // Get overall trust score
    pub fn get_trust_score(&self) -> f64 {
        self.trust_score
    }

    // Get total participations
    pub fn get_total_participations(&self) -> u32 {
        self.total_participations
    }
}

// Social trust metrics manager
pub struct SocialTrustManager {
    pub metrics: HashMap<u32, SocialTrustMetrics>, // Account ID -> Metrics
}

impl SocialTrustManager {
    pub fn new() -> Self {
        SocialTrustManager {
            metrics: HashMap::new(),
        }
    }

    pub fn create_metrics(&mut self, account_id: u32) -> &SocialTrustMetrics {
        let metrics = SocialTrustMetrics::new(account_id);
        self.metrics.insert(account_id, metrics);
        self.metrics.get(&account_id).unwrap()
    }

    pub fn get_metrics(&self, account_id: u32) -> Option<&SocialTrustMetrics> {
        self.metrics.get(&account_id)
    }

    pub fn get_all_metrics(&self) -> &HashMap<u32, SocialTrustMetrics> {
        &self.metrics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_social_trust_metrics_creation() {
        let mut manager = SocialTrustManager::new();
        let metrics = manager.create_metrics(1);
        
        assert_eq!(metrics.account_id, 1);
        assert_eq!(metrics.total_participations, 0);
        assert_eq!(metrics.trust_score, 0.0);
    }

    #[test]
    fn test_referendum_voting_participation() {
        let mut manager = SocialTrustManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.add_referendum_vote(1, true, Some("Aye".to_string()), 1000, 1);
        metrics.add_referendum_vote(2, true, Some("Nay".to_string()), 500, 2);
        
        assert_eq!(metrics.get_referendum_voting_history().len(), 2);
        assert_eq!(metrics.get_total_participations(), 2);
    }

    #[test]
    fn test_treasury_seconding() {
        let mut manager = SocialTrustManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.add_treasury_seconding(1, 100, 1000);
        metrics.add_treasury_seconding(2, 100, 2000);
        
        assert_eq!(metrics.get_treasury_seconding_history().len(), 2);
        assert_eq!(metrics.get_total_participations(), 2);
    }

    #[test]
    fn test_treasury_contributions() {
        let mut manager = SocialTrustManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.add_treasury_contribution(1, 100, 500, "Tip".to_string());
        metrics.add_treasury_contribution(2, 100, 1000, "Bounty".to_string());
        
        assert_eq!(metrics.get_treasury_contributions_history().len(), 2);
        assert_eq!(metrics.get_total_participations(), 2);
    }

    #[test]
    fn test_validator_nominator_history() {
        let mut manager = SocialTrustManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.add_validator_nominator_support(100, 1, 1000, true);
        metrics.add_validator_nominator_support(200, 1, 2000, true);
        
        assert_eq!(metrics.get_validator_nominator_history().len(), 2);
        assert_eq!(metrics.get_total_participations(), 2);
    }

    #[test]
    fn test_reward_stake_history() {
        let mut manager = SocialTrustManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.add_reward_stake_history(1, 100, 1000, 1);
        metrics.add_reward_stake_history(1, 200, 2000, 2);
        
        assert_eq!(metrics.get_reward_stake_history().len(), 2);
        assert_eq!(metrics.get_total_participations(), 2);
    }

    #[test]
    fn test_slashing_history() {
        let mut manager = SocialTrustManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.add_slashing_history(1, 100, "Offence".to_string(), "Misbehavior".to_string(), 1);
        metrics.add_slashing_history(1, 200, "Liveness".to_string(), "Offline".to_string(), 2);
        
        assert_eq!(metrics.get_slashing_history().len(), 2);
        assert_eq!(metrics.get_total_participations(), 2);
    }

    #[test]
    fn test_proposal_extrinsic_history() {
        let mut manager = SocialTrustManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.add_proposal_extrinsic_history(1, 1, "Preimage".to_string(), "data1".to_string());
        metrics.add_proposal_extrinsic_history(2, 1, "Nominate".to_string(), "data2".to_string());
        
        assert_eq!(metrics.get_proposal_extrinsic_history().len(), 2);
        assert_eq!(metrics.get_total_participations(), 2);
    }

    #[test]
    fn test_opengov_participation() {
        let mut manager = SocialTrustManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.add_opengov_participation(1, 1, "Voting".to_string());
        metrics.add_opengov_participation(1, 1, "Voting".to_string()); // Should increment count
        metrics.add_opengov_participation(2, 1, "Proposal".to_string());
        
        assert_eq!(metrics.get_opengov_participation().len(), 2);
        assert_eq!(metrics.get_opengov_participation()[0].count, 2);
        assert_eq!(metrics.get_total_participations(), 3);
    }

    #[test]
    fn test_delegation_history() {
        let mut manager = SocialTrustManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.add_delegation_history(1, 100, 1000, "Giving".to_string());
        metrics.add_delegation_history(200, 1, 2000, "Receiving".to_string());
        
        assert_eq!(metrics.get_delegation_history().len(), 2);
        assert_eq!(metrics.get_total_participations(), 2);
    }

    #[test]
    fn test_trust_score_calculation() {
        let mut manager = SocialTrustManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        // Add various participations
        metrics.add_referendum_vote(1, true, Some("Aye".to_string()), 1000, 1);
        metrics.add_treasury_seconding(1, 100, 1000);
        metrics.add_treasury_contribution(1, 100, 500, "Tip".to_string());
        metrics.add_validator_nominator_support(100, 1, 1000, true);
        
        // Trust score should be positive
        assert!(metrics.get_trust_score() > 0.0);
    }
}
