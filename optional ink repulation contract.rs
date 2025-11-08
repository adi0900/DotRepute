#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod optional_reputation {
    use ink::storage::Mapping;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum UserRole {
        Owner,
        VerifiedUser,
        GovernanceParticipant,
        Regular,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct ReputationScore {
        pub total_score: u64,
        pub governance_score: u32,
        pub staking_score: u32,
        pub identity_score: u32,
        pub community_score: u32,
        pub last_updated: u64,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct VerificationStatus {
        pub is_verified: bool,
        pub verification_timestamp: u64,
        pub verifier: Option<AccountId>,
    }

    #[ink(storage)]
    pub struct OptionalReputation {
        owner: AccountId,
        scores: Mapping<AccountId, ReputationScore>,
        roles: Mapping<AccountId, UserRole>,
        verifications: Mapping<AccountId, VerificationStatus>,
        score_writers: Mapping<AccountId, bool>,
        total_registered: u32,
        minimum_score_threshold: u64,
    }

    #[ink(event)]
    pub struct ScoreSet {
        #[ink(topic)]
        account: AccountId,
        total_score: u64,
        timestamp: u64,
    }

    #[ink(event)]
    pub struct UserVerified {
        #[ink(topic)]
        account: AccountId,
        verifier: AccountId,
        timestamp: u64,
    }

    #[ink(event)]
    pub struct RoleGranted {
        #[ink(topic)]
        account: AccountId,
        role: UserRole,
    }

    #[ink(event)]
    pub struct ScoreWriterAdded {
        #[ink(topic)]
        writer: AccountId,
    }

    #[ink(event)]
    pub struct AccessGranted {
        #[ink(topic)]
        account: AccountId,
        required_score: u64,
        user_score: u64,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        Unauthorized,
        UserNotFound,
        UserNotVerified,
        InsufficientScore,
        InvalidScore,
        AlreadyVerified,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl OptionalReputation {
        #[ink(constructor)]
        pub fn new(minimum_score_threshold: u64) -> Self {
            let caller = Self::env().caller();
            let mut instance = Self {
                owner: caller,
                scores: Mapping::default(),
                roles: Mapping::default(),
                verifications: Mapping::default(),
                score_writers: Mapping::default(),
                total_registered: 0,
                minimum_score_threshold,
            };
            
            instance.roles.insert(caller, &UserRole::Owner);
            instance.score_writers.insert(caller, &true);
            
            instance
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(50)
        }

        #[ink(message)]
        pub fn set_score(
            &mut self,
            account: AccountId,
            total_score: u64,
            governance_score: u32,
            staking_score: u32,
            identity_score: u32,
            community_score: u32,
        ) -> Result<()> {
            self.only_score_writer()?;

            let is_new = self.scores.get(&account).is_none();

            let score = ReputationScore {
                total_score,
                governance_score,
                staking_score,
                identity_score,
                community_score,
                last_updated: self.env().block_timestamp(),
            };

            self.scores.insert(account, &score);

            if is_new {
                self.total_registered += 1;
                self.roles.insert(account, &UserRole::Regular);
            }

            self.auto_assign_role(account, total_score);

            self.env().emit_event(ScoreSet {
                account,
                total_score,
                timestamp: self.env().block_timestamp(),
            });

            Ok(())
        }

        #[ink(message)]
        pub fn get_score(&self, account: AccountId) -> Option<ReputationScore> {
            self.scores.get(&account)
        }

        #[ink(message)]
        pub fn verify_user(&mut self, account: AccountId) -> Result<()> {
            self.only_owner()?;

            if let Some(status) = self.verifications.get(&account) {
                if status.is_verified {
                    return Err(Error::AlreadyVerified);
                }
            }

            let verification = VerificationStatus {
                is_verified: true,
                verification_timestamp: self.env().block_timestamp(),
                verifier: Some(self.env().caller()),
            };

            self.verifications.insert(account, &verification);

            self.env().emit_event(UserVerified {
                account,
                verifier: self.env().caller(),
                timestamp: self.env().block_timestamp(),
            });

            Ok(())
        }

        #[ink(message)]
        pub fn is_verified(&self, account: AccountId) -> bool {
            self.verifications
                .get(&account)
                .map(|v| v.is_verified)
                .unwrap_or(false)
        }

        #[ink(message)]
        pub fn grant_role(&mut self, account: AccountId, role: UserRole) -> Result<()> {
            self.only_owner()?;

            self.roles.insert(account, &role);

            self.env().emit_event(RoleGranted {
                account,
                role,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn get_role(&self, account: AccountId) -> UserRole {
            self.roles.get(&account).unwrap_or(UserRole::Regular)
        }

        #[ink(message)]
        pub fn add_score_writer(&mut self, writer: AccountId) -> Result<()> {
            self.only_owner()?;

            self.score_writers.insert(writer, &true);

            self.env().emit_event(ScoreWriterAdded {
                writer,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn remove_score_writer(&mut self, writer: AccountId) -> Result<()> {
            self.only_owner()?;

            self.score_writers.remove(writer);

            Ok(())
        }

        #[ink(message)]
        pub fn is_score_writer(&self, account: AccountId) -> bool {
            self.score_writers.get(&account).unwrap_or(false)
        }

        #[ink(message)]
        pub fn check_access(&self, account: AccountId, required_score: u64) -> Result<bool> {
            let score = self.scores.get(&account).ok_or(Error::UserNotFound)?;

            if score.total_score >= required_score {
                Ok(true)
            } else {
                Err(Error::InsufficientScore)
            }
        }

        #[ink(message)]
        pub fn grant_access(&mut self, account: AccountId, required_score: u64) -> Result<()> {
            let score = self.scores.get(&account).ok_or(Error::UserNotFound)?;

            if score.total_score < required_score {
                return Err(Error::InsufficientScore);
            }

            self.env().emit_event(AccessGranted {
                account,
                required_score,
                user_score: score.total_score,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn require_verified(&self, account: AccountId) -> Result<()> {
            if !self.is_verified(account) {
                return Err(Error::UserNotVerified);
            }
            Ok(())
        }

        #[ink(message)]
        pub fn require_governance_participant(&self, account: AccountId) -> Result<()> {
            let role = self.get_role(account);
            
            if role != UserRole::GovernanceParticipant && role != UserRole::Owner {
                return Err(Error::Unauthorized);
            }
            
            Ok(())
        }

        #[ink(message)]
        pub fn get_total_registered(&self) -> u32 {
            self.total_registered
        }

        #[ink(message)]
        pub fn get_minimum_threshold(&self) -> u64 {
            self.minimum_score_threshold
        }

        #[ink(message)]
        pub fn update_minimum_threshold(&mut self, new_threshold: u64) -> Result<()> {
            self.only_owner()?;
            self.minimum_score_threshold = new_threshold;
            Ok(())
        }

        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<()> {
            self.only_owner()?;
            
            self.roles.insert(new_owner, &UserRole::Owner);
            self.score_writers.insert(new_owner, &true);
            self.roles.insert(self.owner, &UserRole::Regular);
            
            self.owner = new_owner;
            
            Ok(())
        }

        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        #[ink(message)]
        pub fn batch_set_scores(&mut self, accounts: Vec<AccountId>, scores: Vec<u64>) -> Result<u32> {
            self.only_score_writer()?;

            if accounts.len() != scores.len() {
                return Err(Error::InvalidScore);
            }

            let mut success_count = 0u32;

            for (account, total_score) in accounts.iter().zip(scores.iter()) {
                let score = ReputationScore {
                    total_score: *total_score,
                    governance_score: 0,
                    staking_score: 0,
                    identity_score: 0,
                    community_score: 0,
                    last_updated: self.env().block_timestamp(),
                };

                self.scores.insert(*account, &score);
                success_count += 1;
            }

            Ok(success_count)
        }

        fn only_owner(&self) -> Result<()> {
            if self.env().caller() != self.owner {
                return Err(Error::Unauthorized);
            }
            Ok(())
        }

        fn only_score_writer(&self) -> Result<()> {
            let caller = self.env().caller();
            
            if caller == self.owner {
                return Ok(());
            }

            if !self.score_writers.get(&caller).unwrap_or(false) {
                return Err(Error::Unauthorized);
            }

            Ok(())
        }

        fn auto_assign_role(&mut self, account: AccountId, total_score: u64) {
            let role = if total_score >= 80 {
                UserRole::GovernanceParticipant
            } else if total_score >= 50 {
                UserRole::VerifiedUser
            } else {
                UserRole::Regular
            };

            self.roles.insert(account, &role);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn constructor_works() {
            let contract = OptionalReputation::new(50);
            assert_eq!(contract.get_minimum_threshold(), 50);
            assert_eq!(contract.get_total_registered(), 0);
        }

        #[ink::test]
        fn default_constructor_works() {
            let contract = OptionalReputation::default();
            assert_eq!(contract.get_minimum_threshold(), 50);
        }

        #[ink::test]
        fn set_score_works() {
            let mut contract = OptionalReputation::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let result = contract.set_score(accounts.bob, 75, 25, 25, 15, 10);
            assert!(result.is_ok());

            let score = contract.get_score(accounts.bob).unwrap();
            assert_eq!(score.total_score, 75);
            assert_eq!(score.governance_score, 25);
        }

        #[ink::test]
        fn get_score_works() {
            let mut contract = OptionalReputation::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let _ = contract.set_score(accounts.bob, 85, 30, 30, 15, 10);
            let score = contract.get_score(accounts.bob);
            
            assert!(score.is_some());
            assert_eq!(score.unwrap().total_score, 85);
        }

        #[ink::test]
        fn verify_user_works() {
            let mut contract = OptionalReputation::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert!(!contract.is_verified(accounts.bob));

            let result = contract.verify_user(accounts.bob);
            assert!(result.is_ok());
            assert!(contract.is_verified(accounts.bob));
        }

        #[ink::test]
        fn only_owner_can_verify() {
            let mut contract = OptionalReputation::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);

            let result = contract.verify_user(accounts.charlie);
            assert_eq!(result, Err(Error::Unauthorized));
        }

        #[ink::test]
        fn grant_role_works() {
            let mut contract = OptionalReputation::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let result = contract.grant_role(accounts.bob, UserRole::GovernanceParticipant);
            assert!(result.is_ok());

            let role = contract.get_role(accounts.bob);
            assert_eq!(role, UserRole::GovernanceParticipant);
        }

        #[ink::test]
        fn auto_role_assignment_works() {
            let mut contract = OptionalReputation::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let _ = contract.set_score(accounts.bob, 85, 30, 30, 15, 10);
            let role = contract.get_role(accounts.bob);
            assert_eq!(role, UserRole::GovernanceParticipant);

            let _ = contract.set_score(accounts.charlie, 60, 20, 20, 10, 10);
            let role2 = contract.get_role(accounts.charlie);
            assert_eq!(role2, UserRole::VerifiedUser);
        }

        #[ink::test]
        fn add_score_writer_works() {
            let mut contract = OptionalReputation::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert!(!contract.is_score_writer(accounts.bob));

            let result = contract.add_score_writer(accounts.bob);
            assert!(result.is_ok());
            assert!(contract.is_score_writer(accounts.bob));
        }

        #[ink::test]
        fn score_writer_can_set_scores() {
            let mut contract = OptionalReputation::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let _ = contract.add_score_writer(accounts.bob);

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);

            let result = contract.set_score(accounts.charlie, 70, 25, 25, 10, 10);
            assert!(result.is_ok());
        }

        #[ink::test]
        fn remove_score_writer_works() {
            let mut contract = OptionalReputation::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let _ = contract.add_score_writer(accounts.bob);
            assert!(contract.is_score_writer(accounts.bob));

            let _ = contract.remove_score_writer(accounts.bob);
            assert!(!contract.is_score_writer(accounts.bob));
        }

        #[ink::test]
        fn check_access_works() {
            let mut contract = OptionalReputation::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let _ = contract.set_score(accounts.bob, 75, 25, 25, 15, 10);

            assert!(contract.check_access(accounts.bob, 50).is_ok());
            assert!(contract.check_access(accounts.bob, 75).is_ok());
            assert_eq!(contract.check_access(accounts.bob, 80), Err(Error::InsufficientScore));
        }

        #[ink::test]
        fn grant_access_works() {
            let mut contract = OptionalReputation::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let _ = contract.set_score(accounts.bob, 75, 25, 25, 15, 10);

            let result = contract.grant_access(accounts.bob, 70);
            assert!(result.is_ok());
        }

        #[ink::test]
        fn require_verified_works() {
            let mut contract = OptionalReputation::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(contract.require_verified(accounts.bob), Err(Error::UserNotVerified));

            let _ = contract.verify_user(accounts.bob);
            assert!(contract.require_verified(accounts.bob).is_ok());
        }

        #[ink::test]
        fn require_governance_participant_works() {
            let mut contract = OptionalReputation::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let _ = contract.set_score(accounts.bob, 85, 30, 30, 15, 10);

            assert!(contract.require_governance_participant(accounts.bob).is_ok());
        }

        #[ink::test]
        fn update_threshold_works() {
            let mut contract = OptionalReputation::new(50);

            let result = contract.update_minimum_threshold(75);
            assert!(result.is_ok());
            assert_eq!(contract.get_minimum_threshold(), 75);
        }

        #[ink::test]
        fn transfer_ownership_works() {
            let mut contract = OptionalReputation::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let old_owner = contract.get_owner();
            assert_eq!(old_owner, accounts.alice);

            let result = contract.transfer_ownership(accounts.bob);
            assert!(result.is_ok());
            assert_eq!(contract.get_owner(), accounts.bob);
        }

        #[ink::test]
        fn batch_set_scores_works() {
            let mut contract = OptionalReputation::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let account_list = vec![accounts.bob, accounts.charlie];
            let score_list = vec![70, 80];

            let result = contract.batch_set_scores(account_list, score_list);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 2);
        }

        #[ink::test]
        fn total_registered_increments() {
            let mut contract = OptionalReputation::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(contract.get_total_registered(), 0);

            let _ = contract.set_score(accounts.bob, 75, 25, 25, 15, 10);
            assert_eq!(contract.get_total_registered(), 1);

            let _ = contract.set_score(accounts.charlie, 65, 20, 20, 15, 10);
            assert_eq!(contract.get_total_registered(), 2);
        }
    }
}
