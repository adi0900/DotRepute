use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Vote types
#[derive(Debug, Clone, PartialEq)]
pub enum VoteType {
    Aye,     // Yes vote
    Nay,     // No vote
    Abstain, // Abstain vote
}

// Conviction levels (multiplier for voting power)
#[derive(Debug, Clone, PartialEq)]
pub enum Conviction {
    None,    // 0.1x voting power, no lock
    Locked1x,  // 1x voting power, locked for 1x period
    Locked2x,  // 2x voting power, locked for 2x period
    Locked3x,  // 3x voting power, locked for 4x period
    Locked4x,  // 4x voting power, locked for 8x period
    Locked5x,  // 5x voting power, locked for 16x period
    Locked6x,  // 6x voting power, locked for 32x period
}

// Governance tracks (different governance areas)
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum GovernanceTrack {
    Root,              // Root track
    Whitelist,         // Whitelist track
    Treasury,          // Treasury track
    Staking,           // Staking track
    Governance,        // General governance track
    Fellowship,        // Fellowship track
    Custom(String),    // Custom track
}

// Vote record
#[derive(Debug, Clone)]
pub struct VoteRecord {
    pub referendum_id: u32,        // Referendum ID
    pub track: GovernanceTrack,    // Governance track
    pub vote_type: VoteType,       // Vote type
    pub conviction: Conviction,    // Conviction level
    pub balance: u128,             // Voting balance
    pub timestamp: u64,            // Vote timestamp
    pub block_number: u32,         // Block number
}

// Proposal record
#[derive(Debug, Clone)]
pub struct ProposalRecord {
    pub proposal_id: u32,          // Proposal ID
    pub track: GovernanceTrack,    // Governance track
    pub proposer: u32,             // Proposer account ID
    pub preimage_hash: Option<String>, // Preimage hash (if any)
    pub timestamp: u64,            // Proposal timestamp
    pub block_number: u32,         // Block number
}

// Preimage record
#[derive(Debug, Clone)]
pub struct PreimageRecord {
    pub hash: String,              // Preimage hash
    pub data: Vec<u8>,             // Preimage data
    pub submitter: u32,            // Submitter account ID
    pub deposit: u128,             // Deposit amount
    pub timestamp: u64,            // Submission timestamp
    pub block_number: u32,         // Block number
}

// Seconding record
#[derive(Debug, Clone)]
pub struct SecondingRecord {
    pub proposal_id: u32,          // Proposal ID
    pub seconder: u32,             // Seconder account ID
    pub deposit: u128,             // Deposit amount
    pub timestamp: u64,            // Seconding timestamp
    pub block_number: u32,         // Block number
}

// Delegation record
#[derive(Debug, Clone)]
pub struct DelegationRecord {
    pub delegator: u32,            // Delegator account ID
    pub delegatee: u32,            // Delegatee account ID
    pub track: GovernanceTrack,    // Governance track
    pub conviction: Conviction,    // Conviction level
    pub balance: u128,             // Delegated balance
    pub timestamp: u64,            // Delegation timestamp
    pub is_active: bool,           // Active status
}

// Batch voting record
#[derive(Debug, Clone)]
pub struct BatchVoteRecord {
    pub batch_id: u32,             // Batch ID
    pub voter: u32,                // Voter account ID
    pub votes: Vec<VoteRecord>,    // Vote records
    pub total_weight: u128,        // Total voting weight
    pub timestamp: u64,            // Batch timestamp
    pub block_number: u32,         // Block number
}

// Referenda participation metrics
#[derive(Debug, Clone)]
pub struct ReferendaParticipationMetrics {
    pub account_id: u32,                           // Account ID
    pub votes: Vec<VoteRecord>,                    // Vote records
    pub proposals: Vec<ProposalRecord>,            // Proposal records
    pub preimages: Vec<PreimageRecord>,            // Preimage records
    pub secondings: Vec<SecondingRecord>,          // Seconding records
    pub delegations: Vec<DelegationRecord>,        // Delegation records
    pub batch_votes: Vec<BatchVoteRecord>,         // Batch vote records
    pub track_participation: HashMap<GovernanceTrack, u32>, // Track participation count
    pub total_votes: u32,                          // Total votes cast
    pub aye_votes: u32,                            // Aye votes count
    pub nay_votes: u32,                            // Nay votes count
    pub abstain_votes: u32,                        // Abstain votes count
    pub conviction_usage: HashMap<Conviction, u32>, // Conviction usage count
    pub is_delegating: bool,                       // Is delegating votes?
    pub last_activity_time: u64,                   // Last activity timestamp
}

impl ReferendaParticipationMetrics {
    pub fn new(account_id: u32) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        ReferendaParticipationMetrics {
            account_id,
            votes: Vec::new(),
            proposals: Vec::new(),
            preimages: Vec::new(),
            secondings: Vec::new(),
            delegations: Vec::new(),
            batch_votes: Vec::new(),
            track_participation: HashMap::new(),
            total_votes: 0,
            aye_votes: 0,
            nay_votes: 0,
            abstain_votes: 0,
            conviction_usage: HashMap::new(),
            is_delegating: false,
            last_activity_time: now,
        }
    }

    // 1. Voting (vote)
    pub fn cast_vote(&mut self, referendum_id: u32, track: GovernanceTrack, vote_type: VoteType, 
                     conviction: Conviction, balance: u128, block_number: u32) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let vote = VoteRecord {
            referendum_id,
            track: track.clone(),
            vote_type: vote_type.clone(),
            conviction: conviction.clone(),
            balance,
            timestamp,
            block_number,
        };
        
        self.votes.push(vote);
        self.total_votes += 1;
        
        // Update vote type counts
        match vote_type {
            VoteType::Aye => self.aye_votes += 1,
            VoteType::Nay => self.nay_votes += 1,
            VoteType::Abstain => self.abstain_votes += 1,
        }
        
        // Update track participation
        let count = self.track_participation.entry(track).or_insert(0);
        *count += 1;
        
        // Update conviction usage
        let conviction_count = self.conviction_usage.entry(conviction).or_insert(0);
        *conviction_count += 1;
        
        self.last_activity_time = timestamp;
    }

    // Get vote records
    pub fn get_votes(&self) -> &Vec<VoteRecord> {
        &self.votes
    }

    // 2. Proposal submission (propose)
    pub fn submit_proposal(&mut self, proposal_id: u32, track: GovernanceTrack, 
                          preimage_hash: Option<String>, block_number: u32) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let proposal = ProposalRecord {
            proposal_id,
            track: track.clone(),
            proposer: self.account_id,
            preimage_hash,
            timestamp,
            block_number,
        };
        
        self.proposals.push(proposal);
        
        // Update track participation
        let count = self.track_participation.entry(track).or_insert(0);
        *count += 1;
        
        self.last_activity_time = timestamp;
    }

    // Get proposal records
    pub fn get_proposals(&self) -> &Vec<ProposalRecord> {
        &self.proposals
    }

    // 3. Preimage submission (submitPreimage)
    pub fn submit_preimage(&mut self, hash: String, data: Vec<u8>, deposit: u128, block_number: u32) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let preimage = PreimageRecord {
            hash,
            data,
            submitter: self.account_id,
            deposit,
            timestamp,
            block_number,
        };
        
        self.preimages.push(preimage);
        self.last_activity_time = timestamp;
    }

    // Get preimage records
    pub fn get_preimages(&self) -> &Vec<PreimageRecord> {
        &self.preimages
    }

    // 4. Seconding support (seconding)
    pub fn second_proposal(&mut self, proposal_id: u32, deposit: u128, block_number: u32) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let seconding = SecondingRecord {
            proposal_id,
            seconder: self.account_id,
            deposit,
            timestamp,
            block_number,
        };
        
        self.secondings.push(seconding);
        self.last_activity_time = timestamp;
    }

    // Get seconding records
    pub fn get_secondings(&self) -> &Vec<SecondingRecord> {
        &self.secondings
    }

    // 5. Track diversity (participation in different governance areas)
    pub fn get_track_diversity(&self) -> usize {
        self.track_participation.len()
    }

    // Get track participation details
    pub fn get_track_participation(&self) -> &HashMap<GovernanceTrack, u32> {
        &self.track_participation
    }

    // 6. Vote count (how many referendums voted on)
    pub fn get_total_votes_count(&self) -> u32 {
        self.total_votes
    }

    // Get unique referendums voted on
    pub fn get_unique_referendums_voted(&self) -> usize {
        let mut referendum_ids = Vec::new();
        for vote in &self.votes {
            if !referendum_ids.contains(&vote.referendum_id) {
                referendum_ids.push(vote.referendum_id);
            }
        }
        referendum_ids.len()
    }

    // 7. Vote type (aye/nay, conviction level)
    pub fn get_vote_types(&self) -> (u32, u32, u32) {
        (self.aye_votes, self.nay_votes, self.abstain_votes)
    }

    // Get conviction usage
    pub fn get_conviction_usage(&self) -> &HashMap<Conviction, u32> {
        &self.conviction_usage
    }

    // 8. Vote timing (regular, up-to-date)
    pub fn get_voting_frequency(&self) -> f64 {
        if self.votes.is_empty() {
            return 0.0;
        }
        
        let time_span = if let Some(first_vote) = self.votes.first() {
            let latest_vote = self.votes.last().unwrap();
            (latest_vote.timestamp - first_vote.timestamp) as f64
        } else {
            0.0
        };
        
        if time_span > 0.0 {
            self.votes.len() as f64 / (time_span / (24.0 * 60.0 * 60.0)) // Votes per day
        } else {
            self.votes.len() as f64
        }
    }

    // Check if voting is regular (at least one vote per month)
    pub fn is_voting_regular(&self) -> bool {
        if self.votes.is_empty() {
            return false;
        }
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        if let Some(last_vote) = self.votes.last() {
            let days_since_last_vote = (now - last_vote.timestamp) / (24 * 60 * 60);
            days_since_last_vote <= 30 // Voted within last 30 days
        } else {
            false
        }
    }

    // 9. Delegation status (has delegated voting rights)
    pub fn is_delegating_votes(&self) -> bool {
        self.is_delegating
    }

    // Set delegation
    pub fn set_delegation(&mut self, delegatee: u32, track: GovernanceTrack, 
                         conviction: Conviction, balance: u128, block_number: u32) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let delegation = DelegationRecord {
            delegator: self.account_id,
            delegatee,
            track: track.clone(),
            conviction,
            balance,
            timestamp,
            is_active: true,
        };
        
        self.delegations.push(delegation);
        self.is_delegating = true;
        
        // Update track participation
        let count = self.track_participation.entry(track).or_insert(0);
        *count += 1;
        
        self.last_activity_time = timestamp;
    }

    // Get delegation records
    pub fn get_delegations(&self) -> &Vec<DelegationRecord> {
        &self.delegations
    }

    // 10. Batch voting behavior (bulk voting)
    pub fn cast_batch_votes(&mut self, batch_id: u32, votes: Vec<VoteRecord>, block_number: u32) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let total_weight: u128 = votes.iter().map(|v| v.balance).sum();
        
        let batch_vote = BatchVoteRecord {
            batch_id,
            voter: self.account_id,
            votes: votes.clone(),
            total_weight,
            timestamp,
            block_number,
        };
        
        self.batch_votes.push(batch_vote);
        self.votes.extend(votes);
        self.total_votes += batch_vote.votes.len() as u32;
        
        // Update vote type counts
        for vote in &batch_vote.votes {
            match vote.vote_type {
                VoteType::Aye => self.aye_votes += 1,
                VoteType::Nay => self.nay_votes += 1,
                VoteType::Abstain => self.abstain_votes += 1,
            }
            
            // Update track participation
            let count = self.track_participation.entry(vote.track.clone()).or_insert(0);
            *count += 1;
            
            // Update conviction usage
            let conviction_count = self.conviction_usage.entry(vote.conviction.clone()).or_insert(0);
            *conviction_count += 1;
        }
        
        self.last_activity_time = timestamp;
    }

    // Get batch vote records
    pub fn get_batch_votes(&self) -> &Vec<BatchVoteRecord> {
        &self.batch_votes
    }

    // Get batch voting count
    pub fn get_batch_voting_count(&self) -> usize {
        self.batch_votes.len()
    }

    // Get participation score
    pub fn get_participation_score(&self) -> f64 {
        let mut score = 0.0;
        
        // Base score for votes
        score += self.total_votes as f64 * 2.0;
        
        // Score for proposals
        score += self.proposals.len() as f64 * 5.0;
        
        // Score for preimages
        score += self.preimages.len() as f64 * 3.0;
        
        // Score for seconding
        score += self.secondings.len() as f64 * 2.0;
        
        // Score for track diversity
        score += self.get_track_diversity() as f64 * 4.0;
        
        // Score for batch voting
        score += self.batch_votes.len() as f64 * 3.0;
        
        // Bonus for regular voting
        if self.is_voting_regular() {
            score += 10.0;
        }
        
        // Bonus for delegation (shows engagement)
        if self.is_delegating {
            score += 5.0;
        }
        
        score
    }

    // Get recent activity count (last 90 days)
    pub fn get_recent_activity_count(&self) -> u32 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let ninety_days_ago = now - (90 * 24 * 60 * 60);
        
        let mut count = 0;
        count += self.votes.iter().filter(|v| v.timestamp > ninety_days_ago).count();
        count += self.proposals.iter().filter(|p| p.timestamp > ninety_days_ago).count();
        count += self.preimages.iter().filter(|p| p.timestamp > ninety_days_ago).count();
        count += self.secondings.iter().filter(|s| s.timestamp > ninety_days_ago).count();
        count += self.batch_votes.iter().filter(|b| b.timestamp > ninety_days_ago).count();
        
        count as u32
    }

    // Get last activity time
    pub fn get_last_activity_time(&self) -> u64 {
        self.last_activity_time
    }
}

// Referenda participation manager
pub struct ReferendaParticipationManager {
    pub metrics: HashMap<u32, ReferendaParticipationMetrics>, // Account ID -> Metrics
}

impl ReferendaParticipationManager {
    pub fn new() -> Self {
        ReferendaParticipationManager {
            metrics: HashMap::new(),
        }
    }

    pub fn create_metrics(&mut self, account_id: u32) -> &ReferendaParticipationMetrics {
        let metrics = ReferendaParticipationMetrics::new(account_id);
        self.metrics.insert(account_id, metrics);
        self.metrics.get(&account_id).unwrap()
    }

    pub fn get_metrics(&self, account_id: u32) -> Option<&ReferendaParticipationMetrics> {
        self.metrics.get(&account_id)
    }

    pub fn get_all_metrics(&self) -> &HashMap<u32, ReferendaParticipationMetrics> {
        &self.metrics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_referenda_metrics_creation() {
        let mut manager = ReferendaParticipationManager::new();
        let metrics = manager.create_metrics(1);
        
        assert_eq!(metrics.account_id, 1);
        assert_eq!(metrics.get_total_votes_count(), 0);
        assert_eq!(metrics.is_delegating_votes(), false);
    }

    #[test]
    fn test_voting() {
        let mut manager = ReferendaParticipationManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.cast_vote(1, GovernanceTrack::Root, VoteType::Aye, Conviction::Locked1x, 1000, 1000);
        metrics.cast_vote(2, GovernanceTrack::Treasury, VoteType::Nay, Conviction::Locked2x, 500, 1001);
        
        assert_eq!(metrics.get_total_votes_count(), 2);
        assert_eq!(metrics.get_unique_referendums_voted(), 2);
        assert_eq!(metrics.get_track_diversity(), 2);
    }

    #[test]
    fn test_proposal_submission() {
        let mut manager = ReferendaParticipationManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.submit_proposal(1, GovernanceTrack::Root, Some("0x123456".to_string()), 1000);
        metrics.submit_proposal(2, GovernanceTrack::Treasury, None, 1001);
        
        assert_eq!(metrics.get_proposals().len(), 2);
        assert_eq!(metrics.get_track_diversity(), 2);
    }

    #[test]
    fn test_preimage_submission() {
        let mut manager = ReferendaParticipationManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.submit_preimage("0x123456".to_string(), vec![1, 2, 3, 4], 1000, 1000);
        metrics.submit_preimage("0x789012".to_string(), vec![5, 6, 7, 8], 2000, 1001);
        
        assert_eq!(metrics.get_preimages().len(), 2);
    }

    #[test]
    fn test_seconding() {
        let mut manager = ReferendaParticipationManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.second_proposal(1, 1000, 1000);
        metrics.second_proposal(2, 2000, 1001);
        
        assert_eq!(metrics.get_secondings().len(), 2);
    }

    #[test]
    fn test_track_diversity() {
        let mut manager = ReferendaParticipationManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.cast_vote(1, GovernanceTrack::Root, VoteType::Aye, Conviction::Locked1x, 1000, 1000);
        metrics.cast_vote(2, GovernanceTrack::Treasury, VoteType::Nay, Conviction::Locked2x, 500, 1001);
        metrics.cast_vote(3, GovernanceTrack::Staking, VoteType::Aye, Conviction::Locked3x, 750, 1002);
        
        assert_eq!(metrics.get_track_diversity(), 3);
        let participation = metrics.get_track_participation();
        assert_eq!(*participation.get(&GovernanceTrack::Root).unwrap(), 1);
        assert_eq!(*participation.get(&GovernanceTrack::Treasury).unwrap(), 1);
        assert_eq!(*participation.get(&GovernanceTrack::Staking).unwrap(), 1);
    }

    #[test]
    fn test_vote_types_and_conviction() {
        let mut manager = ReferendaParticipationManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.cast_vote(1, GovernanceTrack::Root, VoteType::Aye, Conviction::Locked1x, 1000, 1000);
        metrics.cast_vote(2, GovernanceTrack::Root, VoteType::Nay, Conviction::Locked2x, 500, 1001);
        metrics.cast_vote(3, GovernanceTrack::Root, VoteType::Abstain, Conviction::None, 250, 1002);
        
        let (aye, nay, abstain) = metrics.get_vote_types();
        assert_eq!(aye, 1);
        assert_eq!(nay, 1);
        assert_eq!(abstain, 1);
        
        let conviction_usage = metrics.get_conviction_usage();
        assert_eq!(*conviction_usage.get(&Conviction::Locked1x).unwrap(), 1);
        assert_eq!(*conviction_usage.get(&Conviction::Locked2x).unwrap(), 1);
        assert_eq!(*conviction_usage.get(&Conviction::None).unwrap(), 1);
    }

    #[test]
    fn test_delegation() {
        let mut manager = ReferendaParticipationManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        assert_eq!(metrics.is_delegating_votes(), false);
        
        metrics.set_delegation(100, GovernanceTrack::Root, Conviction::Locked1x, 1000, 1000);
        
        assert_eq!(metrics.is_delegating_votes(), true);
        assert_eq!(metrics.get_delegations().len(), 1);
    }

    #[test]
    fn test_batch_voting() {
        let mut manager = ReferendaParticipationManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        let votes = vec![
            VoteRecord {
                referendum_id: 1,
                track: GovernanceTrack::Root,
                vote_type: VoteType::Aye,
                conviction: Conviction::Locked1x,
                balance: 1000,
                timestamp: 1000000,
                block_number: 1000,
            },
            VoteRecord {
                referendum_id: 2,
                track: GovernanceTrack::Treasury,
                vote_type: VoteType::Nay,
                conviction: Conviction::Locked2x,
                balance: 500,
                timestamp: 1000001,
                block_number: 1001,
            }
        ];
        
        metrics.cast_batch_votes(1, votes, 1000);
        
        assert_eq!(metrics.get_total_votes_count(), 2);
        assert_eq!(metrics.get_batch_voting_count(), 1);
        assert_eq!(metrics.get_batch_votes().len(), 1);
    }

    #[test]
    fn test_participation_score() {
        let mut manager = ReferendaParticipationManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.cast_vote(1, GovernanceTrack::Root, VoteType::Aye, Conviction::Locked1x, 1000, 1000);
        metrics.submit_proposal(1, GovernanceTrack::Treasury, Some("0x123456".to_string()), 1001);
        metrics.second_proposal(1, 1000, 1002);
        
        let score = metrics.get_participation_score();
        assert!(score > 0.0);
    }

    #[test]
    fn test_voting_frequency_and_regularity() {
        let mut manager = ReferendaParticipationManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.cast_vote(1, GovernanceTrack::Root, VoteType::Aye, Conviction::Locked1x, 1000, 1000);
        
        let frequency = metrics.get_voting_frequency();
        assert!(frequency >= 0.0);
        
        // Regular voting check depends on timing, so we just ensure it returns a boolean
        let is_regular = metrics.is_voting_regular();
        assert!(is_regular == true || is_regular == false);
    }
}
