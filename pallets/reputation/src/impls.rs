use crate::traits::*;
use crate::{CredibilityUnit, ReputationUnit};
use frame_support::pallet_prelude::*;


#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Rep {
    pub reputation: ReputationUnit,
    pub credibility: CredibilityUnit,
    pub aggregate_rating: u64,
    pub num_of_ratings: u64,
}


impl HasCredibility for Rep 
 {
    fn get_credibility(&self) -> CredibilityUnit {
        self.credibility
    }
    
}

impl HasReputation for Rep
{
    fn get_reputation(&self) -> ReputationUnit {
        self.reputation
    }
}