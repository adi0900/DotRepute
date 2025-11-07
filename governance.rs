use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Referandum statuses
#[derive(Debug, Clone, PartialEq)]
pub enum ReferendumStatus {
    Proposed,
    Voting,
    Passed,
    Rejected,
    Executed,
    Cancelled,
}

// Vote types
#[derive(Debug, Clone, PartialEq)]
pub enum VoteType {
    Aye,     // Yes vote
    Nay,     // No vote
    Abstain, // Abstain vote
}

// Vote data structure
#[derive(Debug, Clone)]
pub struct Vote {
    pub voter: u32,        // Voter account ID
    pub vote_type: VoteType,
    pub balance: u128,     // Voting power
    pub conviction: u8,     // Conviction multiplier
}

// Referendum Voting
#[derive(Debug, Clone)]
pub struct Referendum {
    pub id: u32,
    pub proposer: u32,     // Proposal submitter
    pub status: ReferendumStatus,
    pub votes: Vec<Vote>,
    pub ayes: u128,        // Total yes votes
    pub nays: u128,        // Total no votes
    pub abstains: u128,    // Total abstain votes
    pub start_time: u64,
    pub end_time: u64,
    pub threshold: f64,    // Passing threshold
}

impl Referendum {
    pub fn new(id: u32, proposer: u32, duration: u64, threshold: f64) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        Referendum {
            id,
            proposer,
            status: ReferendumStatus::Proposed,
            votes: Vec::new(),
            ayes: 0,
            nays: 0,
            abstains: 0,
            start_time: now,
            end_time: now + duration,
            threshold,
        }
    }

    pub fn start_voting(&mut self) {
        self.status = ReferendumStatus::Voting;
    }

    pub fn cast_vote(&mut self, vote: Vote) {
        if self.status != ReferendumStatus::Voting {
            return;
        }

        let weighted_balance = vote.balance * vote.conviction as u128;
        
        match vote.vote_type {
            VoteType::Aye => self.ayes += weighted_balance,
            VoteType::Nay => self.nays += weighted_balance,
            VoteType::Abstain => self.abstains += weighted_balance,
        }
        
        self.votes.push(vote);
    }

    pub fn finalize(&mut self) -> bool {
        if self.status != ReferendumStatus::Voting {
            return false;
        }

        let total_votes = self.ayes + self.nays + self.abstains;
        if total_votes == 0 {
            self.status = ReferendumStatus::Rejected;
            return false;
        }

        let aye_ratio = self.ayes as f64 / total_votes as f64;
        
        if aye_ratio >= self.threshold {
            self.status = ReferendumStatus::Passed;
            true
        } else {
            self.status = ReferendumStatus::Rejected;
            false
        }
    }
}

// Preimage Submission
#[derive(Debug, Clone)]
pub struct Preimage {
    pub hash: String,           // Preimage hash
    pub data: Vec<u8>,          // Actual data
    pub submitter: u32,         // Submitter
    pub deposit: u128,          // Deposit
    pub timestamp: u64,         // Timestamp
}

impl Preimage {
    pub fn new(hash: String, data: Vec<u8>, submitter: u32, deposit: u128) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        Preimage {
            hash,
            data,
            submitter,
            deposit,
            timestamp,
        }
    }
}

// Treasury Proposal Submission
#[derive(Debug, Clone)]
pub struct TreasuryProposal {
    pub id: u32,
    pub proposer: u32,          // Proposal submitter
    pub beneficiary: u32,        // Beneficiary
    pub amount: u128,           // Amount
    pub description: String,    // Description
    pub preimage_hash: Option<String>, // Preimage hash
    pub status: String,         // Status
}

impl TreasuryProposal {
    pub fn new(id: u32, proposer: u32, beneficiary: u32, amount: u128, description: String) -> Self {
        TreasuryProposal {
            id,
            proposer,
            beneficiary,
            amount,
            description,
            preimage_hash: None,
            status: "Proposed".to_string(),
        }
    }

    pub fn set_preimage(&mut self, hash: String) {
        self.preimage_hash = Some(hash);
    }

    pub fn approve(&mut self) {
        self.status = "Approved".to_string();
    }

    pub fn reject(&mut self) {
        self.status = "Rejected".to_string();
    }
}

// Vote Delegation
#[derive(Debug, Clone)]
pub struct Delegation {
    pub delegator: u32,         // Delegator
    pub delegatee: u32,         // Delegatee
    pub balance: u128,          // Delegated balance
    pub conviction: u8,         // Conviction level
    pub active: bool,           // Active status
}

impl Delegation {
    pub fn new(delegator: u32, delegatee: u32, balance: u128, conviction: u8) -> Self {
        Delegation {
            delegator,
            delegatee,
            balance,
            conviction,
            active: true,
        }
    }

    pub fn revoke(&mut self) {
        self.active = false;
    }
}

// Reward and Tip Contributions
#[derive(Debug, Clone)]
pub struct RewardTip {
    pub id: u32,
    pub finder: u32,            // Finder
    pub beneficiary: u32,       // Beneficiary
    pub reason: String,         // Reason
    pub tips: HashMap<u32, u128>, // Tippers and amounts
    pub total_amount: u128,     // Total amount
    pub closed: bool,           // Closed status
}

impl RewardTip {
    pub fn new(id: u32, finder: u32, beneficiary: u32, reason: String) -> Self {
        RewardTip {
            id,
            finder,
            beneficiary,
            reason,
            tips: HashMap::new(),
            total_amount: 0,
            closed: false,
        }
    }

    pub fn add_tip(&mut self, tipper: u32, amount: u128) {
        if self.closed {
            return;
        }
        
        self.tips.insert(tipper, amount);
        self.total_amount += amount;
    }

    pub fn close(&mut self) {
        self.closed = true;
    }
}

// Batch Voting
#[derive(Debug, Clone)]
pub struct BatchVote {
    pub voter: u32,
    pub votes: Vec<Vote>,
    pub total_weight: u128,
}

impl BatchVote {
    pub fn new(voter: u32) -> Self {
        BatchVote {
            voter,
            votes: Vec::new(),
            total_weight: 0,
        }
    }

    pub fn add_vote(&mut self, mut vote: Vote) {
        vote.voter = self.voter;
        let weight = vote.balance * vote.conviction as u128;
        self.total_weight += weight;
        self.votes.push(vote);
    }
}

// OpenGov Watcher Participation
#[derive(Debug, Clone)]
pub struct GovWatcher {
    pub watcher_id: u32,
    pub account: u32,
    pub tracked_referendums: Vec<u32>,  // Tracked referendums
    pub notifications: Vec<String>,     // Notifications
    pub active: bool,
}

impl GovWatcher {
    pub fn new(watcher_id: u32, account: u32) -> Self {
        GovWatcher {
            watcher_id,
            account,
            tracked_referendums: Vec::new(),
            notifications: Vec::new(),
            active: true,
        }
    }

    pub fn track_referendum(&mut self, referendum_id: u32) {
        if !self.tracked_referendums.contains(&referendum_id) {
            self.tracked_referendums.push(referendum_id);
        }
    }

    pub fn add_notification(&mut self, message: String) {
        self.notifications.push(message);
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

// Council or Fellowship Membership
#[derive(Debug, Clone, PartialEq)]
pub enum MemberRole {
    Council,
    Fellowship,
}

#[derive(Debug, Clone)]
pub struct GovernanceMember {
    pub member_id: u32,
    pub account: u32,
    pub role: MemberRole,
    pub rank: u8,               // Rank (for Fellowship)
    pub join_date: u64,         // Join date
    pub active: bool,
}

impl GovernanceMember {
    pub fn new(member_id: u32, account: u32, role: MemberRole, rank: u8) -> Self {
        let join_date = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        GovernanceMember {
            member_id,
            account,
            role,
            rank,
            join_date,
            active: true,
        }
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

// Proposal Approval or Seconding
#[derive(Debug, Clone)]
pub struct ProposalSecond {
    pub proposal_id: u32,
    pub seconder: u32,          // Seconding party
    pub timestamp: u64,
    pub deposit: u128,          // Deposit
}

impl ProposalSecond {
    pub fn new(proposal_id: u32, seconder: u32, deposit: u128) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        ProposalSecond {
            proposal_id,
            seconder,
            timestamp,
            deposit,
        }
    }
}

// Main governance system
pub struct PolkadotGovernance {
    pub referendums: HashMap<u32, Referendum>,
    pub preimages: HashMap<String, Preimage>,
    pub treasury_proposals: HashMap<u32, TreasuryProposal>,
    pub delegations: Vec<Delegation>,
    pub reward_tips: HashMap<u32, RewardTip>,
    pub gov_watchers: HashMap<u32, GovWatcher>,
    pub members: HashMap<u32, GovernanceMember>,
    pub proposal_seconds: Vec<ProposalSecond>,
}

impl PolkadotGovernance {
    pub fn new() -> Self {
        PolkadotGovernance {
            referendums: HashMap::new(),
            preimages: HashMap::new(),
            treasury_proposals: HashMap::new(),
            delegations: Vec::new(),
            reward_tips: HashMap::new(),
            gov_watchers: HashMap::new(),
            members: HashMap::new(),
            proposal_seconds: Vec::new(),
        }
    }

    // Referendum management
    pub fn create_referendum(&mut self, id: u32, proposer: u32, duration: u64, threshold: f64) -> &Referendum {
        let referendum = Referendum::new(id, proposer, duration, threshold);
        self.referendums.insert(id, referendum);
        self.referendums.get(&id).unwrap()
    }

    // Preimage management
    pub fn submit_preimage(&mut self, hash: String, data: Vec<u8>, submitter: u32, deposit: u128) -> &Preimage {
        let preimage = Preimage::new(hash.clone(), data, submitter, deposit);
        self.preimages.insert(hash, preimage);
        self.preimages.get(&hash).unwrap()
    }

    // Treasury proposal management
    pub fn create_treasury_proposal(&mut self, id: u32, proposer: u32, beneficiary: u32, amount: u128, description: String) -> &TreasuryProposal {
        let proposal = TreasuryProposal::new(id, proposer, beneficiary, amount, description);
        self.treasury_proposals.insert(id, proposal);
        self.treasury_proposals.get(&id).unwrap()
    }

    // Delegation management
    pub fn create_delegation(&mut self, delegator: u32, delegatee: u32, balance: u128, conviction: u8) -> &Delegation {
        let delegation = Delegation::new(delegator, delegatee, balance, conviction);
        self.delegations.push(delegation);
        self.delegations.last().unwrap()
    }

    // Reward/tip management
    pub fn create_reward_tip(&mut self, id: u32, finder: u32, beneficiary: u32, reason: String) -> &RewardTip {
        let tip = RewardTip::new(id, finder, beneficiary, reason);
        self.reward_tips.insert(id, tip);
        self.reward_tips.get(&id).unwrap()
    }

    // Gov watcher management
    pub fn create_gov_watcher(&mut self, watcher_id: u32, account: u32) -> &GovWatcher {
        let watcher = GovWatcher::new(watcher_id, account);
        self.gov_watchers.insert(watcher_id, watcher);
        self.gov_watchers.get(&watcher_id).unwrap()
    }

    // Member management
    pub fn add_member(&mut self, member_id: u32, account: u32, role: MemberRole, rank: u8) -> &GovernanceMember {
        let member = GovernanceMember::new(member_id, account, role, rank);
        self.members.insert(member_id, member);
        self.members.get(&member_id).unwrap()
    }

    // Proposal seconding
    pub fn second_proposal(&mut self, proposal_id: u32, seconder: u32, deposit: u128) -> &ProposalSecond {
        let second = ProposalSecond::new(proposal_id, seconder, deposit);
        self.proposal_seconds.push(second);
        self.proposal_seconds.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_referendum_voting() {
        let mut governance = PolkadotGovernance::new();
        let mut referendum = Referendum::new(1, 100, 86400, 0.5); // 1 day, 50% threshold
        
        referendum.start_voting();
        
        // Create votes
        let vote1 = Vote {
            voter: 1,
            vote_type: VoteType::Aye,
            balance: 1000,
            conviction: 1,
        };
        
        let vote2 = Vote {
            voter: 2,
            vote_type: VoteType::Nay,
            balance: 500,
            conviction: 1,
        };
        
        referendum.cast_vote(vote1);
        referendum.cast_vote(vote2);
        
        assert_eq!(referendum.ayes, 1000);
        assert_eq!(referendum.nays, 500);
        
        let passed = referendum.finalize();
        assert!(passed);
        assert_eq!(referendum.status, ReferendumStatus::Passed);
    }

    #[test]
    fn test_preimage_submission() {
        let mut governance = PolkadotGovernance::new();
        let data = vec![1, 2, 3, 4, 5];
        let hash = "0x1234567890abcdef".to_string();
        
        let preimage = governance.submit_preimage(hash.clone(), data.clone(), 100, 1000);
        
        assert_eq!(preimage.hash, hash);
        assert_eq!(preimage.data, data);
        assert_eq!(preimage.submitter, 100);
        assert_eq!(preimage.deposit, 1000);
    }

    #[test]
    fn test_treasury_proposal() {
        let mut governance = PolkadotGovernance::new();
        let proposal = governance.create_treasury_proposal(
            1, 
            100, 
            200, 
            5000, 
            "Development funding".to_string()
        );
        
        assert_eq!(proposal.id, 1);
        assert_eq!(proposal.proposer, 100);
        assert_eq!(proposal.beneficiary, 200);
        assert_eq!(proposal.amount, 5000);
        assert_eq!(proposal.status, "Proposed");
    }

    #[test]
    fn test_delegation() {
        let mut governance = PolkadotGovernance::new();
        let delegation = governance.create_delegation(100, 200, 1000, 3);
        
        assert_eq!(delegation.delegator, 100);
        assert_eq!(delegation.delegatee, 200);
        assert_eq!(delegation.balance, 1000);
        assert_eq!(delegation.conviction, 3);
        assert!(delegation.active);
    }

    #[test]
    fn test_reward_tip() {
        let mut governance = PolkadotGovernance::new();
        let tip = governance.create_reward_tip(
            1, 
            100, 
            200, 
            "Great contribution".to_string()
        );
        
        tip.add_tip(300, 500);
        tip.add_tip(400, 300);
        
        assert_eq!(tip.total_amount, 800);
        assert_eq!(tip.tips.len(), 2);
    }

    #[test]
    fn test_gov_watcher() {
        let mut governance = PolkadotGovernance::new();
        let watcher = governance.create_gov_watcher(1, 100);
        
        watcher.track_referendum(5);
        watcher.track_referendum(10);
        watcher.add_notification("Referendum 5 updated".to_string());
        
        assert_eq!(watcher.tracked_referendums.len(), 2);
        assert_eq!(watcher.notifications.len(), 1);
        assert!(watcher.active);
    }

    #[test]
    fn test_governance_member() {
        let mut governance = PolkadotGovernance::new();
        let member = governance.add_member(1, 100, MemberRole::Council, 5);
        
        assert_eq!(member.member_id, 1);
        assert_eq!(member.account, 100);
        assert_eq!(member.role, MemberRole::Council);
        assert_eq!(member.rank, 5);
        assert!(member.active);
    }

    #[test]
    fn test_proposal_seconding() {
        let mut governance = PolkadotGovernance::new();
        let second = governance.second_proposal(1, 100, 1000);
        
        assert_eq!(second.proposal_id, 1);
        assert_eq!(second.seconder, 100);
        assert_eq!(second.deposit, 1000);
    }
}
