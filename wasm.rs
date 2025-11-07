
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

use ink_lang as ink;

#[ink::contract]
pub mod activity_tracker {
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::SpreadAllocate,
    };
    
    // Activity types that can be tracked
    #[derive(
        Debug, 
        Clone, 
        PartialEq, 
        Eq,
        scale::Encode, 
        scale::Decode
    )]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum ActivityType {
        Voting,
        Proposal,
        Staking,
        Governance,
        Community,
    }
    
    // Activity record stored on-chain
    #[derive(
        Debug, 
        Clone, 
        PartialEq, 
        Eq,
        scale::Encode, 
        scale::Decode
    )]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct ActivityRecord {
        pub id: u32,
        pub account_id: AccountId,
        pub activity_type: ActivityType,
        pub timestamp: u64,
        pub success: bool,
    }
    
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(Default, SpreadAllocate)]
    pub struct ActivityTracker {
        /// Stores a single `bool` value on the storage.
        activities: StorageHashMap<u32, ActivityRecord>,
        account_activities: StorageHashMap<AccountId, Vec<u32>>,
        activity_counter: u32,
    }
    
    impl ActivityTracker {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            // This call is required in order to correctly initialize the
            // `Mapping`s of our contract.
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.activity_counter = 0;
            })
        }
        
        /// Add a new activity record
        #[ink(message)]
        pub fn add_activity(
            &mut self,
            activity_type: ActivityType,
            success: bool,
        ) -> u32 {
            let caller = self.env().caller();
            let timestamp = self.env().block_timestamp();
            self.activity_counter += 1;
            let id = self.activity_counter;
            
            let activity = ActivityRecord {
                id,
                account_id: caller,
                activity_type,
                timestamp,
                success,
            };
            
            // Store the activity
            self.activities.insert(id, activity);
            
            // Index by account
            let mut account_activities = self.account_activities.get(&caller).cloned().unwrap_or_default();
            account_activities.push(id);
            self.account_activities.insert(caller, account_activities);
            
            id
        }
        
        /// Get activities for the caller
        #[ink(message)]
        pub fn get_my_activities(&self) -> Vec<ActivityRecord> {
            let caller = self.env().caller();
            if let Some(activity_ids) = self.account_activities.get(&caller) {
                activity_ids
                    .iter()
                    .filter_map(|&id| self.activities.get(&id))
                    .cloned()
                    .collect()
            } else {
                Vec::new()
            }
        }
        
        /// Get activity by ID
        #[ink(message)]
        pub fn get_activity(&self, id: u32) -> Option<ActivityRecord> {
            self.activities.get(&id).cloned()
        }
        
        /// Get total activity count
        #[ink(message)]
        pub fn get_activity_count(&self) -> u32 {
            self.activity_counter
        }
        
        /// Get successful activities count for caller
        #[ink(message)]
        pub fn get_my_successful_activities_count(&self) -> u32 {
            let caller = self.env().caller();
            if let Some(activity_ids) = self.account_activities.get(&caller) {
                activity_ids
                    .iter()
                    .filter_map(|&id| self.activities.get(&id))
                    .filter(|activity| activity.success)
                    .count() as u32
            } else {
                0
            }
        }
    }
    
    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let activity_tracker = ActivityTracker::new();
            assert_eq!(activity_tracker.get_activity_count(), 0);
        }
        
        #[ink::test]
        fn add_activity_works() {
            let mut activity_tracker = ActivityTracker::new();
            let activity_id = activity_tracker.add_activity(ActivityType::Voting, true);
            
            assert_eq!(activity_id, 1);
            assert_eq!(activity_tracker.get_activity_count(), 1);
            
            let activity = activity_tracker.get_activity(1).unwrap();
            assert_eq!(activity.id, 1);
            assert_eq!(activity.activity_type, ActivityType::Voting);
            assert_eq!(activity.success, true);
        }
        
        #[ink::test]
        fn get_my_activities_works() {
            let mut activity_tracker = ActivityTracker::new();
            
            // Add activities for the default caller
            activity_tracker.add_activity(ActivityType::Voting, true);
            activity_tracker.add_activity(ActivityType::Proposal, false);
            
            let activities = activity_tracker.get_my_activities();
            assert_eq!(activities.len(), 2);
            assert_eq!(activities[0].activity_type, ActivityType::Voting);
            assert_eq!(activities[1].activity_type, ActivityType::Proposal);
        }
    }
}
