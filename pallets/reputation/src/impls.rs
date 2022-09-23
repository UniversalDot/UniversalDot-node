use crate::traits::*;
use crate::{CredibilityUnit, ReputationUnit};
use frame_support::pallet_prelude::*;


#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Reputable<T: frame_system::Config> {
    pub reputation: ReputationUnit,
    pub credibility: CredibilityUnit,
    pub aggregate_rating: u64,
    pub num_of_ratings: u64,
    pub account: T::AccountId,
}

pub struct ReputationHandler;


impl HasCredibility for Reputable 
 {
    fn get_credibility(&self) -> CredibilityUnit {
        self.credibility
    }
    
}

impl HasReputation for Reputable
{
    fn get_reputation(&self) -> ReputationUnit {
        self.reputation
    }
}

impl HasAccountId<T: frame_system::Config> for Reputable
{
    fn get_account_id(&self) -> T::AccountId {
        self.account
    } 
}