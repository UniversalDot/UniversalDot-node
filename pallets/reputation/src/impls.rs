
use crate::{
    pallet::{CredibilityUnit, ReputationUnit, Reputable, Rating},
    traits::{HasReputation, HasCredibility, HasAccountId}

};
use frame_support::{
    pallet_prelude::*,
    inherent::Vec,
    BoundedVec
};

pub struct ReputationHandler;

impl<T> crate::traits::ReputationHandler<T> for ReputationHandler
where T: frame_system::Config + crate::Config
{
    
    fn calculate_credibility<N>(entity: &N, ratings: &BoundedVec<Rating, T::MaximumRatingsPer>) -> CredibilityUnit 
    where N: HasCredibility
    {
        CredibilityUnit::default()
    }

    fn calculate_reputation<N>(entity: &N, ratings: &BoundedVec<Rating, T::MaximumRatingsPer>) -> ReputationUnit
    where N: HasCredibility + HasReputation + HasAccountId<T>
    {
        let mut rep = entity.get_reputation();

        let _: Vec<_> = ratings.iter().map(|r|{
            let diff: i32 = *r as i32 - 3i32;
            rep += diff;    
        }).collect::<_>();
    
        rep.try_into().expect("input vec is bounded, output is same length; qed")
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