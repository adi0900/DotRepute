use std::collections::HashMap;

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

#[derive(Debug, Clone)]
pub struct OnChainIdentity {
    pub account_id: u32,
    pub is_identity_set: bool,
    pub is_identity_cleared: bool,
    pub judgements: Vec<Judgement>,
}

impl OnChainIdentity {
    pub fn new(account_id: u32) -> Self {
        OnChainIdentity {
            account_id,
            is_identity_set: false,
            is_identity_cleared: false,
            judgements: Vec::new(),
        }
    }

    // Is Identity Set - Kimlik tanımlı mı kontrolü
    pub fn is_identity_set(&self) -> bool {
        self.is_identity_set
    }

    // Is Identity Cleared - Kimlik silinmiş mi kontrolü
    pub fn is_identity_cleared(&self) -> bool {
        self.is_identity_cleared
    }

    // Judgement - Yargı durumu yönetimi
    pub fn get_judgements(&self) -> &Vec<Judgement> {
        &self.judgements
    }

    pub fn add_judgement(&mut self, judgement: Judgement) {
        self.judgements.push(judgement);
    }

    pub fn set_identity(&mut self) {
        self.is_identity_set = true;
        self.is_identity_cleared = false;
    }

    pub fn clear_identity(&mut self) {
        self.is_identity_cleared = true;
        self.is_identity_set = false;
        self.judgements.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_creation() {
        let identity = OnChainIdentity::new(1);
        assert_eq!(identity.account_id, 1);
        assert!(!identity.is_identity_set());
        assert!(!identity.is_identity_cleared());
    }

    #[test]
    fn test_set_and_clear_identity() {
        let mut identity = OnChainIdentity::new(1);
        
        // Set identity
        identity.set_identity();
        assert!(identity.is_identity_set());
        assert!(!identity.is_identity_cleared());
        
        // Clear identity
        identity.clear_identity();
        assert!(!identity.is_identity_set());
        assert!(identity.is_identity_cleared());
    }

    #[test]
    fn test_judgements() {
        let mut identity = OnChainIdentity::new(1);
        
        // Add judgements
        identity.add_judgement(Judgement::Reasonable);
        identity.add_judgement(Judgement::KnownGood);
        
        let judgements = identity.get_judgements();
        assert_eq!(judgements.len(), 2);
        assert_eq!(judgements[0], Judgement::Reasonable);
        assert_eq!(judgements[1], Judgement::KnownGood);
    }
}
