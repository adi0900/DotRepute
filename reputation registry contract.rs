#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod reputation_registry {
    use ink::storage::Mapping;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Role {
        Owner,
        VerifiedUser,
        GovernanceParticipant,
        StakeHolder,
        Unverified,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct UserReputation {
        pub total_score: u64,
        pub governance_score: u32,
        pub staking_score: u32,
        pub identity_score: u32,
        pub community_score: u32,
        pub last_updated: u64,
        pub role: Role,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct StakeRecord {
        pub amount: Balance,
        pub duration: u64,
        pub start_timestamp: u64,
        pub is_active: bool,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct GovernanceRecord {
        pub votes_count: u32,
        pub proposals_count: u32,
        pub last_participation: u64,
    }

    #[ink(storage)]
    pub struct ReputationRegistry {
        owner: AccountId,
        reputations: Mapping<AccountId, UserReputation>,
        stake_records: Mapping<AccountId, StakeRecord>,
        governance_records: Mapping<AccountId, GovernanceRecord>,
        verified_users: Mapping<AccountId, bool>,
        total_users: u32,
        minimum_score_threshold: u64,
    }

    #[ink(event)]
    pub struct ScoreUpdated {
        #[ink(topic)]
        account: AccountId,
        old_score: u64,
        new_score: u64,
        timestamp: u64,
    }

    #[ink(event)]
    pub struct UserVerified {
        #[ink(topic)]
        account: AccountId,
        timestamp: u64,
    }

    #[ink(event)]
    pub struct RoleGranted {
        #[ink(topic)]
        account: AccountId,
        role: Role,
    }

    #[ink(event)]
    pub struct StakeRecorded {
        #[ink(topic)]
        account: AccountId,
        amount: Balance,
        duration: u64,
    }

    #[ink(event)]
    pub struct GovernanceParticipation {
        #[ink(topic)]
        account: AccountId,
        action_type: u8,
        timestamp: u64,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        Unauthorized,
        UserNotFound,
        UserNotVerified,
        InvalidScore,
        InvalidStakeAmount,
        InsufficientReputation,
        AlreadyExists,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl ReputationRegistry {
        #[ink(constructor)]
        pub fn new(minimum_score_threshold: u64) -> Self {
            Self {
                owner: Self::env().caller(),
                reputations: Mapping::default(),
                stake_records: Mapping::default(),
                governance_records: Mapping::default(),
                verified_users: Mapping::default(),
                total_users: 0,
                minimum_score_threshold,
            }
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
            self.only_owner()?;

            let old_score = self.reputations
                .get(&account)
                .map(|r| r.total_score)
                .unwrap_or(0);

            let reputation = UserReputation {
                total_score,
                governance_score,
                staking_score,
                identity_score,
                community_score,
                last_updated: self.env().block_timestamp(),
                role: self.determine_role(total_score),
            };

            self.reputations.insert(account, &reputation);

            if old_score == 0 {
                self.total_users += 1;
            }

            self.env().emit_event(ScoreUpdated {
                account,
                old_score,
                new_score: total_score,
                timestamp: self.env().block_timestamp(),
            });

            Ok(())
        }

        #[ink(message)]
        pub fn get_score(&self, account: AccountId) -> Option<UserReputation> {
            self.reputations.get(&account)
        }

        #[ink(message)]
        pub fn verify_user(&mut self, account: AccountId) -> Result<()> {
            self.only_owner()?;

            self.verified_users.insert(account, &true);

            self.env().emit_event(UserVerified {
                account,
                timestamp: self.env().block_timestamp(),
            });

            Ok(())
        }

        #[ink(message)]
        pub fn is_verified(&self, account: AccountId) -> bool {
            self.verified_users.get(&account).unwrap_or(false)
        }

        #[ink(message)]
        pub fn grant_role(&mut self, account: AccountId, role: Role) -> Result<()> {
            self.only_owner()?;

            let mut reputation = self.reputations
                .get(&account)
                .ok_or(Error::UserNotFound)?;

            reputation.role = role.clone();
            self.reputations.insert(account, &reputation);

            self.env().emit_event(RoleGranted {
                account,
                role,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn get_role(&self, account: AccountId) -> Result<Role> {
            self.reputations
                .get(&account)
                .map(|r| r.role)
                .ok_or(Error::UserNotFound)
        }

        #[ink(message)]
        pub fn record_stake(
            &mut self,
            amount: Balance,
            duration: u64,
        ) -> Result<()> {
            let caller = self.env().caller();
            self.only_verified_user(caller)?;

            if amount == 0 {
                return Err(Error::InvalidStakeAmount);
            }

            let stake = StakeRecord {
                amount,
                duration,
                start_timestamp: self.env().block_timestamp(),
                is_active: true,
            };

            self.stake_records.insert(caller, &stake);

            self.env().emit_event(StakeRecorded {
                account: caller,
                amount,
                duration,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn get_stake_record(&self, account: AccountId) -> Option<StakeRecord> {
            self.stake_records.get(&account)
        }

        #[ink(message)]
        pub fn unstake(&mut self) -> Result<()> {
            let caller = self.env().caller();
            
            let mut stake = self.stake_records
                .get(&caller)
                .ok_or(Error::UserNotFound)?;

            stake.is_active = false;
            self.stake_records.insert(caller, &stake);

            Ok(())
        }

        #[ink(message)]
        pub fn record_governance_vote(&mut self) -> Result<()> {
            let caller = self.env().caller();
            self.only_verified_user(caller)?;

            let mut record = self.governance_records
                .get(&caller)
                .unwrap_or(GovernanceRecord {
                    votes_count: 0,
                    proposals_count: 0,
                    last_participation: 0,
                });

            record.votes_count += 1;
            record.last_participation = self.env().block_timestamp();
            self.governance_records.insert(caller, &record);

            self.env().emit_event(GovernanceParticipation {
                account: caller,
                action_type: 1,
                timestamp: self.env().block_timestamp(),
            });

            Ok(())
        }

        #[ink(message)]
        pub fn record_governance_proposal(&mut self) -> Result<()> {
            let caller = self.env().caller();
            self.only_verified_user(caller)?;

            let mut record = self.governance_records
                .get(&caller)
                .unwrap_or(GovernanceRecord {
                    votes_count: 0,
                    proposals_count: 0,
                    last_participation: 0,
                });

            record.proposals_count += 1;
            record.last_participation = self.env().block_timestamp();
            self.governance_records.insert(caller, &record);

            self.env().emit_event(GovernanceParticipation {
                account: caller,
                action_type: 2,
                timestamp: self.env().block_timestamp(),
            });

            Ok(())
        }

        #[ink(message)]
        pub fn get_governance_record(&self, account: AccountId) -> Option<GovernanceRecord> {
            self.governance_records.get(&account)
        }

        #[ink(message)]
        pub fn check_access(&self, account: AccountId, required_score: u64) -> bool {
            if let Some(reputation) = self.reputations.get(&account) {
                reputation.total_score >= required_score
            } else {
                false
            }
        }

        #[ink(message)]
        pub fn is_governance_participant(&self, account: AccountId) -> bool {
            if let Some(record) = self.governance_records.get(&account) {
                record.votes_count > 0 || record.proposals_count > 0
            } else {
                false
            }
        }

        #[ink(message)]
        pub fn get_total_users(&self) -> u32 {
            self.total_users
        }

        #[ink(message)]
        pub fn update_minimum_threshold(&mut self, new_threshold: u64) -> Result<()> {
            self.only_owner()?;
            self.minimum_score_threshold = new_threshold;
            Ok(())
        }

        #[ink(message)]
        pub fn get_minimum_threshold(&self) -> u64 {
            self.minimum_score_threshold
        }

        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<()> {
            self.only_owner()?;
            self.owner = new_owner;
            Ok(())
        }

        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        fn only_owner(&self) -> Result<()> {
            if self.env().caller() != self.owner {
                return Err(Error::Unauthorized);
            }
            Ok(())
        }

        fn only_verified_user(&self, account: AccountId) -> Result<()> {
            if !self.is_verified(account) {
                return Err(Error::UserNotVerified);
            }
            Ok(())
        }

        fn only_governance_participant(&self) -> Result<()> {
            let caller = self.env().caller();
            if !self.is_governance_participant(caller) {
                return Err(Error::Unauthorized);
            }
            Ok(())
        }

        fn determine_role(&self, score: u64) -> Role {
            if score >= 80 {
                Role::GovernanceParticipant
            } else if score >= 50 {
                Role::StakeHolder
            } else if score >= 30 {
                Role::VerifiedUser
            } else {
                Role::Unverified
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn constructor_works() {
            let contract = ReputationRegistry::new(50);
            assert_eq!(contract.get_minimum_threshold(), 50);
            assert_eq!(contract.get_total_users(), 0);
        }

        #[ink::test]
        fn default_constructor_works() {
            let contract = ReputationRegistry::default();
            assert_eq!(contract.get_minimum_threshold(), 50);
        }

        #[ink::test]
        fn set_and_get_score_works() {
            let mut contract = ReputationRegistry::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let result = contract.set_score(accounts.alice, 85, 30, 25, 20, 10);
            assert!(result.is_ok());

            let reputation = contract.get_score(accounts.alice).unwrap();
            assert_eq!(reputation.total_score, 85);
            assert_eq!(reputation.governance_score, 30);
            assert_eq!(reputation.staking_score, 25);
            assert_eq!(reputation.identity_score, 20);
            assert_eq!(reputation.community_score, 10);
        }

        #[ink::test]
        fn verify_user_works() {
            let mut contract = ReputationRegistry::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert!(!contract.is_verified(accounts.alice));

            let result = contract.verify_user(accounts.alice);
            assert!(result.is_ok());
            assert!(contract.is_verified(accounts.alice));
        }

        #[ink::test]
        fn only_owner_can_set_score() {
            let mut contract = ReputationRegistry::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);

            let result = contract.set_score(accounts.alice, 85, 30, 25, 20, 10);
            assert_eq!(result, Err(Error::Unauthorized));
        }

        #[ink::test]
        fn role_assignment_works() {
            let mut contract = ReputationRegistry::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let _ = contract.set_score(accounts.alice, 85, 30, 25, 20, 10);

            let role = contract.get_role(accounts.alice).unwrap();
            assert_eq!(role, Role::GovernanceParticipant);
        }

        #[ink::test]
        fn stake_recording_works() {
            let mut contract = ReputationRegistry::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let _ = contract.verify_user(accounts.alice);

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            let result = contract.record_stake(1000, 86400);
            assert!(result.is_ok());

            let stake = contract.get_stake_record(accounts.alice).unwrap();
            assert_eq!(stake.amount, 1000);
            assert_eq!(stake.duration, 86400);
            assert!(stake.is_active);
        }

        #[ink::test]
        fn governance_vote_recording_works() {
            let mut contract = ReputationRegistry::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let _ = contract.verify_user(accounts.alice);

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            let result = contract.record_governance_vote();
            assert!(result.is_ok());

            let record = contract.get_governance_record(accounts.alice).unwrap();
            assert_eq!(record.votes_count, 1);
        }

        #[ink::test]
        fn governance_proposal_recording_works() {
            let mut contract = ReputationRegistry::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let _ = contract.verify_user(accounts.alice);

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            let result = contract.record_governance_proposal();
            assert!(result.is_ok());

            let record = contract.get_governance_record(accounts.alice).unwrap();
            assert_eq!(record.proposals_count, 1);
        }

        #[ink::test]
        fn access_check_works() {
            let mut contract = ReputationRegistry::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let _ = contract.set_score(accounts.alice, 85, 30, 25, 20, 10);

            assert!(contract.check_access(accounts.alice, 50));
            assert!(contract.check_access(accounts.alice, 85));
            assert!(!contract.check_access(accounts.alice, 90));
        }

        #[ink::test]
        fn governance_participant_check_works() {
            let mut contract = ReputationRegistry::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert!(!contract.is_governance_participant(accounts.alice));

            let _ = contract.verify_user(accounts.alice);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            let _ = contract.record_governance_vote();

            assert!(contract.is_governance_participant(accounts.alice));
        }

        #[ink::test]
        fn unstake_works() {
            let mut contract = ReputationRegistry::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let _ = contract.verify_user(accounts.alice);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            let _ = contract.record_stake(1000, 86400);
            let _ = contract.unstake();

            let stake = contract.get_stake_record(accounts.alice).unwrap();
            assert!(!stake.is_active);
        }

        #[ink::test]
        fn ownership_transfer_works() {
            let mut contract = ReputationRegistry::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(contract.get_owner(), accounts.alice);

            let result = contract.transfer_ownership(accounts.bob);
            assert!(result.is_ok());
            assert_eq!(contract.get_owner(), accounts.bob);
        }

        #[ink::test]
        fn threshold_update_works() {
            let mut contract = ReputationRegistry::new(50);

            let result = contract.update_minimum_threshold(75);
            assert!(result.is_ok());
            assert_eq!(contract.get_minimum_threshold(), 75);
        }

        #[ink::test]
        fn unverified_user_cannot_stake() {
            let mut contract = ReputationRegistry::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            let result = contract.record_stake(1000, 86400);
            assert_eq!(result, Err(Error::UserNotVerified));
        }

        #[ink::test]
        fn invalid_stake_amount_rejected() {
            let mut contract = ReputationRegistry::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let _ = contract.verify_user(accounts.alice);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            let result = contract.record_stake(0, 86400);
            assert_eq!(result, Err(Error::InvalidStakeAmount));
        }

        #[ink::test]
        fn total_users_increments() {
            let mut contract = ReputationRegistry::new(50);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(contract.get_total_users(), 0);

            let _ = contract.set_score(accounts.alice, 85, 30, 25, 20, 10);
            assert_eq!(contract.get_total_users(), 1);

            let _ = contract.set_score(accounts.bob, 75, 25, 20, 15, 15);
            assert_eq!(contract.get_total_users(), 2);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn e2e_set_and_get_score(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ReputationRegistryRef::new(50);
            let contract_account_id = client
                .instantiate("reputation_registry", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let alice_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);

            let set_score = build_message::<ReputationRegistryRef>(contract_account_id.clone())
                .call(|contract| contract.set_score(alice_account, 85, 30, 25, 20, 10));

            client
                .call(&ink_e2e::alice(), set_score, 0, None)
                .await
                .expect("set_score failed");

            let get_score = build_message::<ReputationRegistryRef>(contract_account_id.clone())
                .call(|contract| contract.get_score(alice_account));

            let result = client
                .call_dry_run(&ink_e2e::alice(), &get_score, 0, None)
                .await;

            assert!(result.return_value().is_some());

            Ok(())
        }

        #[ink_e2e::test]
        async fn e2e_verify_user(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ReputationRegistryRef::new(50);
            let contract_account_id = client
                .instantiate("reputation_registry", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let alice_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);

            let verify = build_message::<ReputationRegistryRef>(contract_account_id.clone())
                .call(|contract| contract.verify_user(alice_account));

            client
                .call(&ink_e2e::alice(), verify, 0, None)
                .await
                .expect("verify_user failed");

            Ok(())
        }
    }
}
