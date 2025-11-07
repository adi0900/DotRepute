use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Identity field types
#[derive(Debug, Clone, PartialEq)]
pub enum IdentityField {
    Display,
    Legal,
    Web,
    Riot,
    Email,
    PgpFingerprint,
    Image,
    Twitter,
    GitHub,
    Discord,
    Telegram,
}

// Judgement types for identity verification
#[derive(Debug, Clone, PartialEq)]
pub enum Judgement {
    Unknown,
    FeePaid,
    Reasonable,
    KnownGood,
    OutOfDate,
    LowQuality,
    Erroneous,
}

// Registrar information
#[derive(Debug, Clone)]
pub struct Registrar {
    pub id: u32,
    pub account_id: u32,
    pub fee: u128,
    pub fields: u64, // Bitmask of required fields
}

// Identity field data
#[derive(Debug, Clone)]
pub struct IdentityFieldData {
    pub field_type: IdentityField,
    pub value: String,
    pub is_set: bool,
    pub last_updated: u64,
    pub judgements: Vec<(u32, Judgement)>, // Registrar ID -> Judgement
    pub verified: bool,
}

// Identity activity record
#[derive(Debug, Clone)]
pub struct IdentityActivity {
    pub operation: String, // setIdentity, clearIdentity, killIdentity, updateField
    pub timestamp: u64,
    pub block_number: u32,
}

// Identity metrics
#[derive(Debug, Clone)]
pub struct IdentityMetrics {
    pub account_id: u32,
    pub is_identity_set: bool,              // Is identity defined?
    pub is_identity_cleared: bool,          // Is identity removed?
    pub identity_creation_time: Option<u64>, // When identity was first set
    pub identity_fields: HashMap<IdentityField, IdentityFieldData>,
    pub total_fields_filled: u32,           // How many fields are filled
    pub field_updates_count: u32,           // How many times fields updated
    pub verified_fields_count: u32,         // How many fields verified
    pub registrar_judgements: HashMap<u32, Vec<(IdentityField, Judgement)>>, // Registrar ID -> Field judgements
    pub last_field_change_time: Option<u64>, // When fields last changed
    pub is_identity_killed: bool,           // Is identity killed?
    pub social_media_links: u32,            // Social media connections count
    pub identity_activities: Vec<IdentityActivity>, // Identity activity history
    pub last_activity_time: u64,            // Last activity timestamp
}

impl IdentityMetrics {
    pub fn new(account_id: u32) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        IdentityMetrics {
            account_id,
            is_identity_set: false,
            is_identity_cleared: false,
            identity_creation_time: None,
            identity_fields: HashMap::new(),
            total_fields_filled: 0,
            field_updates_count: 0,
            verified_fields_count: 0,
            registrar_judgements: HashMap::new(),
            last_field_change_time: None,
            is_identity_killed: false,
            social_media_links: 0,
            identity_activities: Vec::new(),
            last_activity_time: now,
        }
    }

    // 1. Is identity defined? (setIdentity)
    pub fn is_identity_defined(&self) -> bool {
        self.is_identity_set
    }

    // Set identity with initial fields
    pub fn set_identity(&mut self, fields: Vec<(IdentityField, String)>, block_number: u32) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        // Set identity creation time
        if self.identity_creation_time.is_none() {
            self.identity_creation_time = Some(timestamp);
        }
        
        // Add fields
        for (field_type, value) in fields {
            self.add_or_update_field(field_type, value, timestamp);
        }
        
        self.is_identity_set = true;
        self.is_identity_cleared = false;
        
        // Record activity
        let activity = IdentityActivity {
            operation: "setIdentity".to_string(),
            timestamp,
            block_number,
        };
        self.identity_activities.push(activity);
        self.last_activity_time = timestamp;
    }

    // 2. Is identity removed? (clearIdentity)
    pub fn is_identity_removed(&self) -> bool {
        self.is_identity_cleared
    }

    // Clear identity
    pub fn clear_identity(&mut self, block_number: u32) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        self.is_identity_cleared = true;
        self.is_identity_set = false;
        self.identity_fields.clear();
        self.total_fields_filled = 0;
        self.verified_fields_count = 0;
        self.registrar_judgements.clear();
        self.social_media_links = 0;
        
        // Record activity
        let activity = IdentityActivity {
            operation: "clearIdentity".to_string(),
            timestamp,
            block_number,
        };
        self.identity_activities.push(activity);
        self.last_activity_time = timestamp;
    }

    // 3. How long has identity been active?
    pub fn get_identity_active_duration(&self) -> Option<u64> {
        if let Some(creation_time) = self.identity_creation_time {
            if !self.is_identity_cleared && !self.is_identity_killed {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs();
                Some(now - creation_time)
            } else {
                None // Identity is not active
            }
        } else {
            None // Identity never set
        }
    }

    // 4. How many fields filled?
    pub fn get_filled_fields_count(&self) -> u32 {
        self.total_fields_filled
    }

    // Get all filled fields
    pub fn get_filled_fields(&self) -> Vec<&IdentityFieldData> {
        self.identity_fields.values()
            .filter(|field| field.is_set)
            .collect()
    }

    // 5. How many times fields updated?
    pub fn get_field_updates_count(&self) -> u32 {
        self.field_updates_count
    }

    // 6. Are fields verified? (judgement received?)
    pub fn get_verified_fields_count(&self) -> u32 {
        self.verified_fields_count
    }

    // Check if identity has any verified fields
    pub fn has_verified_fields(&self) -> bool {
        self.verified_fields_count > 0
    }

    // 7. Which registrar verified fields?
    pub fn get_registrar_verifications(&self) -> &HashMap<u32, Vec<(IdentityField, Judgement)>> {
        &self.registrar_judgements
    }

    // Get good judgements from registrars
    pub fn get_good_registrar_judgements(&self) -> Vec<u32> {
        self.registrar_judgements.iter()
            .filter(|(_, judgements)| {
                judgements.iter().any(|(_, judgement)| {
                    matches!(judgement, Judgement::Reasonable | Judgement::KnownGood)
                })
            })
            .map(|(registrar_id, _)| *registrar_id)
            .collect()
    }

    // 8. How long have fields remained unchanged on-chain?
    pub fn get_fields_unchanged_duration(&self) -> Option<u64> {
        if let Some(last_change) = self.last_field_change_time {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();
            Some(now - last_change)
        } else {
            self.identity_creation_time.map(|creation_time| {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs();
                now - creation_time
            })
        }
    }

    // 9. Is identity killed? (killIdentity)
    pub fn is_identity_killed(&self) -> bool {
        self.is_identity_killed
    }

    // Kill identity
    pub fn kill_identity(&mut self, block_number: u32) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        self.is_identity_killed = true;
        self.is_identity_set = false;
        self.is_identity_cleared = true;
        self.identity_fields.clear();
        self.total_fields_filled = 0;
        self.verified_fields_count = 0;
        self.registrar_judgements.clear();
        self.social_media_links = 0;
        
        // Record activity
        let activity = IdentityActivity {
            operation: "killIdentity".to_string(),
            timestamp,
            block_number,
        };
        self.identity_activities.push(activity);
        self.last_activity_time = timestamp;
    }

    // 10. Are there social media links in identity fields?
    pub fn get_social_media_links_count(&self) -> u32 {
        self.social_media_links
    }

    // Check if identity has social media connections
    pub fn has_social_media_links(&self) -> bool {
        self.social_media_links > 0
    }

    // Add or update identity field
    fn add_or_update_field(&mut self, field_type: IdentityField, value: String, timestamp: u64) {
        let is_social_media = matches!(field_type, 
            IdentityField::Twitter | 
            IdentityField::GitHub | 
            IdentityField::Discord | 
            IdentityField::Telegram |
            IdentityField::Riot
        );
        
        let is_new_field = !self.identity_fields.contains_key(&field_type);
        let was_filled = if let Some(existing_field) = self.identity_fields.get(&field_type) {
            existing_field.is_set
        } else {
            false
        };
        
        let field_data = IdentityFieldData {
            field_type: field_type.clone(),
            value: value.clone(),
            is_set: !value.is_empty(),
            last_updated: timestamp,
            judgements: Vec::new(),
            verified: false,
        };
        
        // Update counters
        if field_data.is_set && !was_filled {
            self.total_fields_filled += 1;
            if is_social_media {
                self.social_media_links += 1;
            }
        } else if !field_data.is_set && was_filled {
            self.total_fields_filled = self.total_fields_filled.saturating_sub(1);
            if is_social_media {
                self.social_media_links = self.social_media_links.saturating_sub(1);
            }
        }
        
        if !is_new_field {
            self.field_updates_count += 1;
        }
        
        self.identity_fields.insert(field_type, field_data);
        self.last_field_change_time = Some(timestamp);
    }

    // Update identity field
    pub fn update_field(&mut self, field_type: IdentityField, value: String, block_number: u32) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        self.add_or_update_field(field_type, value, timestamp);
        
        // Record activity
        let activity = IdentityActivity {
            operation: "updateField".to_string(),
            timestamp,
            block_number,
        };
        self.identity_activities.push(activity);
        self.last_activity_time = timestamp;
    }

    // Add judgement to field
    pub fn add_judgement(&mut self, field_type: IdentityField, registrar_id: u32, judgement: Judgement) {
        // Update field with judgement
        if let Some(field_data) = self.identity_fields.get_mut(&field_type) {
            field_data.judgements.push((registrar_id, judgement.clone()));
            
            // Update verification status
            let is_verified = matches!(judgement, Judgement::Reasonable | Judgement::KnownGood);
            if is_verified && !field_data.verified {
                field_data.verified = true;
                self.verified_fields_count += 1;
            } else if !is_verified && field_data.verified {
                field_data.verified = false;
                self.verified_fields_count = self.verified_fields_count.saturating_sub(1);
            }
        }
        
        // Update registrar judgements
        let registrar_entry = self.registrar_judgements
            .entry(registrar_id)
            .or_insert_with(Vec::new);
        
        // Remove existing judgement for this field if any
        registrar_entry.retain(|(f, _)| *f != field_type);
        registrar_entry.push((field_type, judgement));
    }

    // Get identity activities
    pub fn get_identity_activities(&self) -> &Vec<IdentityActivity> {
        &self.identity_activities
    }

    // Get total identity activities count
    pub fn get_activities_count(&self) -> usize {
        self.identity_activities.len()
    }

    // Get identity score based on various factors
    pub fn get_identity_score(&self) -> f64 {
        let mut score = 0.0;
        
        // Base score for having identity set
        if self.is_identity_set {
            score += 10.0;
        }
        
        // Score for filled fields
        score += self.total_fields_filled as f64 * 2.0;
        
        // Score for verified fields
        score += self.verified_fields_count as f64 * 5.0;
        
        // Score for social media connections
        score += self.social_media_links as f64 * 3.0;
        
        // Score for good registrar judgements
        let good_registrars = self.get_good_registrar_judgements().len();
        score += good_registrars as f64 * 4.0;
        
        // Score for long-term identity (days)
        if let Some(duration) = self.get_identity_active_duration() {
            let days = duration / (24 * 60 * 60);
            score += (days as f64).min(365.0) / 36.5; // Max 10 points for 1 year
        }
        
        // Penalty for too many updates (suggests instability)
        if self.field_updates_count > 20 {
            score -= (self.field_updates_count - 20) as f64 * 0.5;
        }
        
        score.max(0.0) // Ensure non-negative score
    }

    // Get identity field data
    pub fn get_field_data(&self, field_type: &IdentityField) -> Option<&IdentityFieldData> {
        self.identity_fields.get(field_type)
    }

    // Get all identity fields
    pub fn get_all_fields(&self) -> &HashMap<IdentityField, IdentityFieldData> {
        &self.identity_fields
    }
}

// Identity metrics manager
pub struct IdentityMetricsManager {
    pub metrics: HashMap<u32, IdentityMetrics>, // Account ID -> Metrics
}

impl IdentityMetricsManager {
    pub fn new() -> Self {
        IdentityMetricsManager {
            metrics: HashMap::new(),
        }
    }

    pub fn create_metrics(&mut self, account_id: u32) -> &IdentityMetrics {
        let metrics = IdentityMetrics::new(account_id);
        self.metrics.insert(account_id, metrics);
        self.metrics.get(&account_id).unwrap()
    }

    pub fn get_metrics(&self, account_id: u32) -> Option<&IdentityMetrics> {
        self.metrics.get(&account_id)
    }

    pub fn get_all_metrics(&self) -> &HashMap<u32, IdentityMetrics> {
        &self.metrics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_metrics_creation() {
        let mut manager = IdentityMetricsManager::new();
        let metrics = manager.create_metrics(1);
        
        assert_eq!(metrics.account_id, 1);
        assert_eq!(metrics.is_identity_defined(), false);
        assert_eq!(metrics.is_identity_removed(), false);
        assert_eq!(metrics.get_filled_fields_count(), 0);
    }

    #[test]
    fn test_set_identity() {
        let mut manager = IdentityMetricsManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        let fields = vec![
            (IdentityField::Display, "Alice".to_string()),
            (IdentityField::Email, "alice@example.com".to_string()),
            (IdentityField::Twitter, "@alice".to_string()),
        ];
        
        metrics.set_identity(fields, 1000);
        
        assert_eq!(metrics.is_identity_defined(), true);
        assert_eq!(metrics.get_filled_fields_count(), 3);
        assert_eq!(metrics.get_social_media_links_count(), 1);
        assert!(metrics.get_identity_active_duration().is_some());
    }

    #[test]
    fn test_clear_identity() {
        let mut manager = IdentityMetricsManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        let fields = vec![
            (IdentityField::Display, "Alice".to_string()),
            (IdentityField::Email, "alice@example.com".to_string()),
        ];
        
        metrics.set_identity(fields, 1000);
        metrics.clear_identity(1001);
        
        assert_eq!(metrics.is_identity_removed(), true);
        assert_eq!(metrics.get_filled_fields_count(), 0);
        assert_eq!(metrics.get_social_media_links_count(), 0);
    }

    #[test]
    fn test_identity_active_duration() {
        let mut manager = IdentityMetricsManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        let fields = vec![(IdentityField::Display, "Alice".to_string())];
        metrics.set_identity(fields, 1000);
        
        // Active duration should be some positive value
        assert!(metrics.get_identity_active_duration().is_some());
    }

    #[test]
    fn test_field_updates() {
        let mut manager = IdentityMetricsManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.set_identity(vec![(IdentityField::Display, "Alice".to_string())], 1000);
        metrics.update_field(IdentityField::Display, "Alice Smith".to_string(), 1001);
        metrics.update_field(IdentityField::Email, "alice@example.com".to_string(), 1002);
        
        assert_eq!(metrics.get_field_updates_count(), 2);
        assert_eq!(metrics.get_filled_fields_count(), 2);
    }

    #[test]
    fn test_judgements() {
        let mut manager = IdentityMetricsManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.set_identity(vec![(IdentityField::Display, "Alice".to_string())], 1000);
        metrics.add_judgement(IdentityField::Display, 1, Judgement::Reasonable);
        metrics.add_judgement(IdentityField::Display, 2, Judgement::KnownGood);
        
        assert_eq!(metrics.get_verified_fields_count(), 1);
        assert_eq!(metrics.has_verified_fields(), true);
        assert_eq!(metrics.get_good_registrar_judgements().len(), 2);
    }

    #[test]
    fn test_kill_identity() {
        let mut manager = IdentityMetricsManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        let fields = vec![(IdentityField::Display, "Alice".to_string())];
        metrics.set_identity(fields, 1000);
        metrics.kill_identity(1001);
        
        assert_eq!(metrics.is_identity_killed(), true);
        assert_eq!(metrics.is_identity_removed(), true);
        assert_eq!(metrics.get_filled_fields_count(), 0);
    }

    #[test]
    fn test_social_media_links() {
        let mut manager = IdentityMetricsManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        let fields = vec![
            (IdentityField::Twitter, "@alice".to_string()),
            (IdentityField::GitHub, "alice".to_string()),
            (IdentityField::Discord, "alice#1234".to_string()),
        ];
        
        metrics.set_identity(fields, 1000);
        
        assert_eq!(metrics.get_social_media_links_count(), 3);
        assert_eq!(metrics.has_social_media_links(), true);
    }

    #[test]
    fn test_identity_score() {
        let mut manager = IdentityMetricsManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        let fields = vec![
            (IdentityField::Display, "Alice".to_string()),
            (IdentityField::Email, "alice@example.com".to_string()),
            (IdentityField::Twitter, "@alice".to_string()),
        ];
        
        metrics.set_identity(fields, 1000);
        metrics.add_judgement(IdentityField::Display, 1, Judgement::Reasonable);
        metrics.add_judgement(IdentityField::Email, 2, Judgement::KnownGood);
        
        let score = metrics.get_identity_score();
        assert!(score > 0.0);
    }

    #[test]
    fn test_identity_activities() {
        let mut manager = IdentityMetricsManager::new();
        let metrics = manager.create_metrics(1);
        let metrics = manager.metrics.get_mut(&1).unwrap();
        
        metrics.set_identity(vec![(IdentityField::Display, "Alice".to_string())], 1000);
        metrics.update_field(IdentityField::Display, "Alice Smith".to_string(), 1001);
        metrics.clear_identity(1002);
        
        assert_eq!(metrics.get_activities_count(), 3);
        let activities = metrics.get_identity_activities();
        assert_eq!(activities[0].operation, "setIdentity");
        assert_eq!(activities[1].operation, "updateField");
        assert_eq!(activities[2].operation, "clearIdentity");
    }
}
