#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod integrated_scoring {
    use ink::storage::Mapping;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct MetricWeights {
        pub governance_weight: u32,
        pub staking_weight: u32,
        pub identity_weight: u32,
        pub community_weight: u32,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct RawMetrics {
        pub governance_votes: u32,
        pub governance_proposals: u32,
        pub staking_amount: u64,
        pub staking_duration: u64,
        pub identity_verified: bool,
        pub identity_judgements: u32,
        pub community_posts: u32,
        pub community_upvotes: u32,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct ComputedScores {
        pub governance_score: u32,
        pub staking_score: u32,
        pub identity_score: u32,
        pub community_score: u32,
        pub total_score: u64,
        pub weighted_total: u64,
    }

    #[ink(storage)]
    pub struct IntegratedScoring {
        owner: AccountId,
        user_scores: Mapping<AccountId, ComputedScores>,
        user_metrics: Mapping<AccountId, RawMetrics>,
        last_update: Mapping<AccountId, u64>,
        default_weights: MetricWeights,
        total_users: u32,
    }

    #[ink(event)]
    pub struct ScoreComputed {
        #[ink(topic)]
        user: AccountId,
        total_score: u64,
        weighted_score: u64,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        Unauthorized,
        UserNotFound,
        InvalidMetrics,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl IntegratedScoring {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                user_scores: Mapping::default(),
                user_metrics: Mapping::default(),
                last_update: Mapping::default(),
                default_weights: MetricWeights {
                    governance_weight: 30,
                    staking_weight: 30,
                    identity_weight: 20,
                    community_weight: 20,
                },
                total_users: 0,
            }
        }

        #[ink(message)]
        pub fn submit_metrics(&mut self, metrics: RawMetrics) -> Result<()> {
            let caller = self.env().caller();
            self.user_metrics.insert(caller, &metrics);
            self.last_update.insert(caller, &self.env().block_timestamp());
            
            if self.user_scores.get(caller).is_none() {
                self.total_users += 1;
            }
            
            self.compute_and_store(caller)
        }

        #[ink(message)]
        pub fn get_score(&self, user: AccountId) -> Option<ComputedScores> {
            self.user_scores.get(user)
        }

        #[ink(message)]
        pub fn get_total_users(&self) -> u32 {
            self.total_users
        }

        fn compute_and_store(&mut self, user: AccountId) -> Result<()> {
            let metrics = self.user_metrics.get(user).ok_or(Error::UserNotFound)?;
            
            let gov_score = Self::calc_gov(&metrics);
            let stake_score = Self::calc_stake(&metrics);
            let id_score = Self::calc_identity(&metrics);
            let comm_score = Self::calc_community(&metrics);
            
            let total = (gov_score + stake_score + id_score + comm_score) as u64;
            let weighted = ((gov_score * 30 + stake_score * 30 + id_score * 20 + comm_score * 20) / 100) as u64;
            
            let scores = ComputedScores {
                governance_score: gov_score,
                staking_score: stake_score,
                identity_score: id_score,
                community_score: comm_score,
                total_score: total,
                weighted_total: weighted,
            };
            
            self.user_scores.insert(user, &scores);
            self.env().emit_event(ScoreComputed {
                user,
                total_score: total,
                weighted_score: weighted,
            });
            
            Ok(())
        }

        fn calc_gov(m: &RawMetrics) -> u32 {
            let v = if m.governance_votes * 2 > 50 { 50 } else { m.governance_votes * 2 };
            let p = if m.governance_proposals * 5 > 50 { 50 } else { m.governance_proposals * 5 };
            v + p
        }

        fn calc_stake(m: &RawMetrics) -> u32 {
            if m.staking_amount == 0 { return 0; }
            let a = Self::log2(m.staking_amount) as u32 * 10;
            let d = Self::sqrt(m.staking_duration / 86400) as u32 * 5;
            if a + d > 100 { 100 } else { a + d }
        }

        fn calc_identity(m: &RawMetrics) -> u32 {
            let v = if m.identity_verified { 50 } else { 0 };
            let j = if m.identity_judgements * 10 > 50 { 50 } else { m.identity_judgements * 10 };
            v + j
        }

        fn calc_community(m: &RawMetrics) -> u32 {
            let p = if m.community_posts > 40 { 40 } else { m.community_posts };
            let u = if m.community_upvotes / 2 > 60 { 60 } else { m.community_upvotes / 2 };
            p + u
        }

        fn log2(mut n: u64) -> u64 {
            let mut log = 0;
            while n > 1 { n >>= 1; log += 1; }
            log
        }

        fn sqrt(n: u64) -> u64 {
            if n < 2 { return n; }
            let mut x = n;
            let mut y = (x + 1) / 2;
            while y < x { x = y; y = (x + n / x) / 2; }
            x
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn works() {
            let contract = IntegratedScoring::new();
            assert_eq!(contract.get_total_users(), 0);
        }
    }
}
