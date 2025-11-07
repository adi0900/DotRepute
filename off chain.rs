use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Off-chain participation types
#[derive(Debug, Clone, PartialEq)]
pub enum OffChainParticipationType {
    PolkassemblyDiscussion,    // Polkassembly discussions and comments
    GitHubContributions,       // GitHub contributions
    SocialMediaContent,        // Social media and educational content
    CommunityRoles,            // Community roles and event participation
    ThirdPartyReferences,      // References and third-party approvals
    CommunityVoting,           // Community polls and off-chain signatures
    LocalCommunityInteraction, // Local language community interaction
}

// Polkassembly discussion and comment record
#[derive(Debug, Clone)]
pub struct PolkassemblyActivity {
    pub post_id: u32,              // Post ID
    pub account_id: u32,           // Account ID
    pub activity_type: String,     // Post, Comment, Reply
    pub content_length: u32,       // Content length
    pub upvotes: u32,              // Upvotes received
    pub replies: u32,              // Replies received
    pub timestamp: u64,            // Activity timestamp
}

// GitHub contribution record
#[derive(Debug, Clone)]
pub struct GitHubContribution {
    pub repo_name: String,         // Repository name
    pub account_id: u32,           // Account ID
    pub contribution_type: String, // PR, Issue, Review
    pub pr_count: u32,             // Pull request count
    pub issue_count: u32,          // Issue participation count
    pub review_count: u32,         // Code review count
    pub repo_diversity: u32,       // Number of different repositories
    pub timestamp: u64,            // Contribution timestamp
}

// Social media and educational content record
#[derive(Debug, Clone)]
pub struct SocialMediaContent {
    pub content_id: u32,           // Content ID
    pub account_id: u32,           // Account ID
    pub platform: String,          // Twitter, Blog, YouTube, etc.
    pub content_type: String,      // Tweet, Thread, Blog, Video
    pub engagement_score: f64,     // Engagement score
    pub reach: u32,                // Estimated reach
    pub shares: u32,               // Share count
    pub timestamp: u64,            // Content timestamp
}

// Community roles and event participation record
#[derive(Debug, Clone)]
pub struct CommunityRole {
    pub role_id: u32,              // Role ID
    pub account_id: u32,           // Account ID
    pub role_type: String,         // Moderator, DAO member, Speaker
    pub event_name: String,        // Event name
    pub participation_type: String, // Organizer, Speaker, Participant
    pub duration: u32,             // Participation duration (hours)
    pub timestamp: u64,            // Role timestamp
}

// Third-party references and approvals record
#[derive(Debug, Clone)]
pub struct ThirdPartyReference {
    pub reference_id: u32,         // Reference ID
    pub account_id: u32,           // Account ID
    pub reference_type: String,    // Grant, Partnership, Project
    pub source: String,            // Reference source
    pub approval_status: String,   // Approved, Pending, Rejected
    pub credibility_score: f64,    // Credibility score (0-1)
    pub timestamp: u64,            // Reference timestamp
}

// Community polls and off-chain signatures record
#[derive(Debug, Clone)]
pub struct CommunityVoting {
    pub poll_id: u32,              // Poll ID
    pub account_id: u32,           // Account ID
    pub poll_type: String,         // Poll, Signature, Survey
    pub participation: bool,       // Whether participated
    pub vote_choice: Option<String>, // Vote choice (if applicable)
    pub signature_count: u32,      // Signature count (for petitions)
    pub timestamp: u64,            // Voting timestamp
}

// Local language community interaction record
#[derive(Debug, Clone)]
pub struct LocalCommunityInteraction {
    pub interaction_id: u32,       // Interaction ID
    pub account_id: u32,           // Account ID
    pub language: String,          // Language code
    pub interaction_type: String,  // Translation, Local support, Meetup
    pub community_size: u32,       // Estimated community size
    pub impact_score: f64,         // Impact score (0-1)
    pub timestamp: u64,            // Interaction timestamp
}

// Off-chain social trust metrics
#[derive(Debug, Clone)]
pub struct OffChainSocialTrustMetrics {
    pub account_id: u32,                           // Account ID
    pub total_offchain_activities: u32,            // Total off-chain activities
    pub polkassembly_activities: Vec<PolkassemblyActivity>, // Polkassembly activities
    pub github_contributions: Vec<GitHubContribution>,      // GitHub contributions
    pub social_media_content: Vec<SocialMediaContent>,      // Social media content
    pub community_roles: Vec<CommunityRole>,                // Community roles
    pub third_party_references: Vec<ThirdPartyReference>,   // Third-party references
    pub community_voting: Vec<CommunityVoting>,             // Community voting
    pub local_interactions: Vec<LocalCommunityInteraction>, // Local interactions
    pub offchain_trust_score: f64,                          // Overall off-chain trust score
    pub last_updated: u64,                                  // Last update timestamp
}

impl OffChainSocialTrustMetrics {
    pub fn new(account_id: u32) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        OffChainSocialTrustMetrics {
            account_id,
            total_offchain_activities: 0,
            polkassembly_activities: Vec::new(),
            github_contributions: Vec::new(),
            social_media_content: Vec::new(),
            community_roles: Vec::new(),
            third_party_references: Vec::new(),
            community_voting: Vec::new(),
            local_interactions: Vec::new(),
            offchain_trust_score: 0.0,
            last_updated: now,
        }
    }

    // Polkassembly discussion and comment interactions
    pub fn add_polkassembly_activity(&mut self, post_id: u32, account_id: u32, activity_type: String,
                                   content_length: u32, upvotes: u32, replies: u32) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let activity = PolkassemblyActivity {
            post_id,
            account_id,
            activity_type,
            content_length,
            upvotes,
            replies,
            timestamp,
        };
        
        self.polkassembly_activities.push(activity);
        self.total_offchain_activities += 1;
        self.update_offchain_trust_score();
    }

    pub fn get_polkassembly_activities(&self) -> &Vec<PolkassemblyActivity> {
        &self.polkassembly_activities
    }

    // GitHub contributions (PR count, issue participation, repo diversity)
    pub fn add_github_contribution(&mut self, repo_name: String, account_id: u32, contribution_type: String,
                                 pr_count: u32, issue_count: u32, review_count: u32, repo_diversity: u32) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let contribution = GitHubContribution {
            repo_name,
            account_id,
            contribution_type,
            pr_count,
            issue_count,
            review_count,
            repo_diversity,
            timestamp,
        };
        
        self.github_contributions.push(contribution);
        self.total_offchain_activities += 1;
        self.update_offchain_trust_score();
    }

    pub fn get_github_contributions(&self) -> &Vec<GitHubContribution> {
        &self.github_contributions
    }

    // Social media / educational content (tweet/thread count, blog posts, videos)
    pub fn add_social_media_content(&mut self, content_id: u32, account_id: u32, platform: String,
                                  content_type: String, engagement_score: f64, reach: u32, shares: u32) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let content = SocialMediaContent {
            content_id,
            account_id,
            platform,
            content_type,
            engagement_score,
            reach,
            shares,
            timestamp,
        };
        
        self.social_media_content.push(content);
        self.total_offchain_activities += 1;
        self.update_offchain_trust_score();
    }

    pub fn get_social_media_content(&self) -> &Vec<SocialMediaContent> {
        &self.social_media_content
    }

    // Community roles and event participation (moderator, DAO tasks, event speaking)
    pub fn add_community_role(&mut self, role_id: u32, account_id: u32, role_type: String,
                           event_name: String, participation_type: String, duration: u32) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let role = CommunityRole {
            role_id,
            account_id,
            role_type,
            event_name,
            participation_type,
            duration,
            timestamp,
        };
        
        self.community_roles.push(role);
        self.total_offchain_activities += 1;
        self.update_offchain_trust_score();
    }

    pub fn get_community_roles(&self) -> &Vec<CommunityRole> {
        &self.community_roles
    }

    // References and third-party approvals (grant references, joint projects)
    pub fn add_third_party_reference(&mut self, reference_id: u32, account_id: u32, reference_type: String,
                                   source: String, approval_status: String, credibility_score: f64) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let reference = ThirdPartyReference {
            reference_id,
            account_id,
            reference_type,
            source,
            approval_status,
            credibility_score,
            timestamp,
        };
        
        self.third_party_references.push(reference);
        self.total_offchain_activities += 1;
        self.update_offchain_trust_score();
    }

    pub fn get_third_party_references(&self) -> &Vec<ThirdPartyReference> {
        &self.third_party_references
    }

    // Community polls and off-chain signatures
    pub fn add_community_voting(&mut self, poll_id: u32, account_id: u32, poll_type: String,
                              participation: bool, vote_choice: Option<String>, signature_count: u32) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let voting = CommunityVoting {
            poll_id,
            account_id,
            poll_type,
            participation,
            vote_choice,
            signature_count,
            timestamp,
        };
        
        self.community_voting.push(voting);
        self.total_offchain_activities += 1;
        self.update_offchain_trust_score();
    }

    pub fn get_community_voting(&self) -> &Vec<CommunityVoting> {
        &self.community_voting
    }

    // Local language community interaction metrics
    pub fn add_local_community_interaction(&mut self, interaction_id: u32, account_id: u32, language: String,
                                         interaction_type: String, community_size: u32, impact_score: f64) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let interaction = LocalCommunityInteraction {
            interaction_id,
            account_id,
            language,
            interaction_type,
            community_size,
            impact_score,
            timestamp,
        };
        
        self.local_interactions.push(interaction);
        self.total_offchain_activities += 1;
        self.update_offchain_trust_score();
    }

    pub fn get_local_community_interactions(&self) -> &Vec<LocalCommunityInteraction> {
        &self.local_interactions
    }

    // Update off-chain trust score based on participation metrics
    fn update_offchain_trust_score(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        // Base score calculation based on various factors
        let mut score = 0.0;
        
        // Positive contributions
        score += self.polkassembly_activities.len() as f64 * 1.0;
        score += self.github_contributions.iter().map(|c| c.pr_count + c.issue_count + c.review_count).sum::<u32>() as f64 * 0.5;
        score += self.social_media_content.len() as f64 * 1.5;
        score += self.community_roles.len() as f64 * 2.0;
        score += self.third_party_references.iter().filter(|r| r.approval_status == "Approved").count() as f64 * 3.0;
        score += self.community_voting.iter().filter(|v| v.participation).count() as f64 * 0.5;
        score += self.local_interactions.len() as f64 * 1.0;
        
        // Quality factors
        let avg_engagement: f64 = if !self.social_media_content.is_empty() {
            self.social_media_content.iter().map(|c| c.engagement_score).sum::<f64>() / self.social_media_content.len() as f64
        } else {
            0.0
        };
        score += avg_engagement * 10.0;
        
        let avg_credibility: f64 = if !self.third_party_references.is_empty() {
            self.third_party_references.iter().map(|r| r.credibility_score).sum::<f64>() / self.third_party_references.len() as f64
        } else {
            0.0
        };
        score += avg_credibility * 15.0;
        
        let avg_impact: f64 = if !self.local_interactions.is_empty() {
            self.local_interactions.iter().map(|i| i.impact_score).sum::<f64>() / self.local_interactions.len() as f64
        } else {
            0.0
        };
        score += avg_impact * 10.0;
        
        // Recency factor - more recent activities get higher weight
        let recent_activities = self.get_recent_offchain_activities_count(now);
        score += recent_activities as f64 * 0.2;
        
        // Normalize score to 0-100 range
        self.offchain_trust_score = score.max(0.0).min(100.0);
        self.last_updated = now;
    }

    // Get count of recent off-chain activities (within last 90 days)
    fn get_recent_offchain_activities_count(&self, current_time: u64) -> u32 {
        let ninety_days_ago = current_time - (90 * 24 * 60 * 60); // 90 days in seconds
        let mut count = 0;
        
        count += self.polkassembly_activities.iter()
            .filter(|a| a.timestamp > ninety_days_ago).count();
        count += self.github_contributions.iter()
            .filter(|c| c.timestamp > ninety_days_ago).count();
        count += self.social_media_content.iter()
            .filter(|c| c.timestamp > ninety_days_ago).count();
        count += self.community_roles.iter()
            .filter(|r| r.timestamp > ninety_days_ago).count();
        count += self.third_party_references.iter()
            .filter(|r| r.timestamp > ninety_days_ago).count();
        count += self.community_voting.iter()
            .filter(|v| v.timestamp > ninety_days_ago).count();
        count += self.local_interactions.iter()
            .filter(|i| i.timestamp > ninety_days_ago).count();
            
        count as u32
    }

    // Get overall off-chain trust score
    pub fn get_offchain_trust_score(&self) -> f64 {
        self.offchain_trust_score
    }

    // Get total off-chain activities
    pub fn get_total_offchain_activities(&self) -> u32 {
        self.total_offchain_activities
    }
}

// Off-chain social trust metrics manager
pub struct OffChainTrustManager {
    pub metrics: HashMap<u32, OffChainSocialTrustMetrics>, // Account ID -> Metrics
}

impl OffChainTrustManager {
    pub fn new() -> Self {
        OffChainTrustManager {
            metrics: HashMap::new(),
        }
    }

    pub fn create_metrics(&mut self, account_id: u32) -> &OffChainSocialTrustMetrics {
        let metrics = OffChainSocialTrustMetrics::new(account_id);
        self.metrics.insert(account_id, metrics);
        self.metrics.get(&account_id).unwrap()
    }

    pub fn get_metrics(&self, account_id: u32) -> Option<&OffChainSocialTrustMetrics> {
        self.metrics.get(&account_id)
    }

    pub fn get_all_metrics(&self) -> &HashMap<u32, OffChainSocialTrustMetrics> {
        &self.metrics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offchain_trust_metrics_creation() {
        let mut manager = OffChainTrustManager::new();
        let metrics = manager.create_metrics(1);
        
        assert_eq!(metrics.account_id, 1);
        assert_eq!(metrics.total_offchain_activities, 0);
        assert_eq!(metrics.offchain_trust_score, 0.0);
    }

    #[test]
    fn test_polkassembly_activities() {
        let mut manager = OffChainTrustManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.add_polkassembly_activity(1, 1, "Post".to_string(), 500, 10, 5);
        metrics.add_polkassembly_activity(2, 1, "Comment".to_string(), 200, 3, 1);
        
        assert_eq!(metrics.get_polkassembly_activities().len(), 2);
        assert_eq!(metrics.get_total_offchain_activities(), 2);
    }

    #[test]
    fn test_github_contributions() {
        let mut manager = OffChainTrustManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.add_github_contribution(
            "polkadot-sdk".to_string(), 
            1, 
            "PR".to_string(), 
            5, 
            3, 
            2, 
            3
        );
        
        assert_eq!(metrics.get_github_contributions().len(), 1);
        assert_eq!(metrics.get_total_offchain_activities(), 1);
    }

    #[test]
    fn test_social_media_content() {
        let mut manager = OffChainTrustManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.add_social_media_content(
            1, 
            1, 
            "Twitter".to_string(), 
            "Tweet".to_string(), 
            0.8, 
            1000, 
            50
        );
        
        assert_eq!(metrics.get_social_media_content().len(), 1);
        assert_eq!(metrics.get_total_offchain_activities(), 1);
    }

    #[test]
    fn test_community_roles() {
        let mut manager = OffChainTrustManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.add_community_role(
            1, 
            1, 
            "Moderator".to_string(), 
            "Polkadot Hackathon".to_string(), 
            "Organizer".to_string(), 
            24
        );
        
        assert_eq!(metrics.get_community_roles().len(), 1);
        assert_eq!(metrics.get_total_offchain_activities(), 1);
    }

    #[test]
    fn test_third_party_references() {
        let mut manager = OffChainTrustManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.add_third_party_reference(
            1, 
            1, 
            "Grant".to_string(), 
            "Web3 Foundation".to_string(), 
            "Approved".to_string(), 
            0.9
        );
        
        assert_eq!(metrics.get_third_party_references().len(), 1);
        assert_eq!(metrics.get_total_offchain_activities(), 1);
    }

    #[test]
    fn test_community_voting() {
        let mut manager = OffChainTrustManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.add_community_voting(
            1, 
            1, 
            "Poll".to_string(), 
            true, 
            Some("Yes".to_string()), 
            0
        );
        
        assert_eq!(metrics.get_community_voting().len(), 1);
        assert_eq!(metrics.get_total_offchain_activities(), 1);
    }

    #[test]
    fn test_local_community_interaction() {
        let mut manager = OffChainTrustManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.add_local_community_interaction(
            1, 
            1, 
            "tr".to_string(), 
            "Translation".to_string(), 
            500, 
            0.8
        );
        
        assert_eq!(metrics.get_local_community_interactions().len(), 1);
        assert_eq!(metrics.get_total_offchain_activities(), 1);
    }

    #[test]
    fn test_offchain_trust_score_calculation() {
        let mut manager = OffChainTrustManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        // Add various off-chain activities
        metrics.add_polkassembly_activity(1, 1, "Post".to_string(), 500, 10, 5);
        metrics.add_social_media_content(1, 1, "Twitter".to_string(), "Tweet".to_string(), 0.8, 1000, 50);
        metrics.add_community_role(1, 1, "Moderator".to_string(), "Event".to_string(), "Organizer".to_string(), 24);
        
        // Off-chain trust score should be positive
        assert!(metrics.get_offchain_trust_score() > 0.0);
    }
}
