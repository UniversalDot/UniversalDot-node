
use crate::traits::*;
use crate::pallet::{CredibilityUnit, ReputationUnit, Reputable};
use frame_support::pallet_prelude::*;

pub struct ReputationHandler;

impl ReputationHandler<T> for ReputationHandler
where T: frame_system::Config
{
    
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