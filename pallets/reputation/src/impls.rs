
use crate::{
    pallet::{CredibilityUnit, ReputationUnit, Reputable, Rating},
    traits::{HasReputation, HasCredibility, HasAccountId}

};
use frame_support::pallet_prelude::*;



pub struct ReputationHandler;

impl<T: frame_system::Config> crate::traits::ReputationHandler<T> for ReputationHandler {
    
    fn calculate_credibility<N>(entity: &N, ratings: &Vec<Rating>) -> CredibilityUnit 
    where N: HasCredibility
    {
        CredibilityUnit::default()
    }

    fn calculate_reputation<N>(entity: &N, ratings: &Vec<Rating>) -> ReputationUnit
    where N: HasCredibility + HasReputation + HasAccountId<T>
    {
        ReputationUnit::default()
    }
}


impl<T> HasCredibility for Reputable<T> 
where T: frame_system::Config
{
    fn get_credibility(&self) -> CredibilityUnit {
        self.credibility
    }
    
}

impl<T> HasReputation for Reputable<T>
where T: frame_system::Config
{
    fn get_reputation(&self) -> ReputationUnit {
        self.reputation
    }
}

impl<T> HasAccountId<T> for Reputable<T>
where T: frame_system::Config
{
    fn get_account_id(&self) -> &T::AccountId {
        &self.account
    } 
}