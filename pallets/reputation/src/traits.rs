 // Copyright (C) 2022 UNIVERSALDOT FOUNDATION.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
   ReputationUnit,
   CredibilityUnit,
   Rating,
};
use frame_support::BoundedVec;

/// Trait used to handle the reputation of a system.
/// Opinionated so that the user must submit a credibility rating.
/// This should be used to weigh the votes of a consumer's reputation against their credibility.
pub trait ReputationHandler<T> 
where T: frame_system::Config + crate::Config
{
   /// Calculate the new reputation of a voter based of a new score given.
   fn calculate_reputation<N>(item: &N, score: &BoundedVec<Rating, T::MaximumRatingsPer>) -> ReputationUnit
   where N: HasCredibility + HasReputation + HasAccountId<T>;  

   /// Calculate the new credibility of the voter, it is used to determine how to weigh the votes.
   /// Must return a value between 0 and 1000 higher is better
   fn calculate_credibility<N: HasCredibility>(item: &N, score: &BoundedVec<Rating, T::MaximumRatingsPer>) -> CredibilityUnit;

 }

pub trait HasReputation {

   /// Return the reputation for a given struct.
   fn get_reputation(&self) -> ReputationUnit;
}

pub trait HasCredibility {

   /// Return the credibility for a given struct.
   fn get_credibility(&self) -> CredibilityUnit;
}

pub trait HasAccountId<T: frame_system::Config> {
   fn get_account_id(&self) -> &T::AccountId;
}

