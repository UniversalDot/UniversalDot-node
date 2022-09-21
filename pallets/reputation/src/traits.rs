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
   Score,
};

 pub trait ReputationHandler {

   /// Calculate the reputation of a voter.
   fn calculate_reputation<N, P>(item: N, scores: P) -> ReputationUnit
   where N: HasCredibility + HasReputation,
         P: Scored;

   /// Calculate the credibility of the voter, it is used to determine how to weigh the votes.
   /// Must return a value between 0 and 1000 higher is better
   fn calculate_credibility<T: HasCredibility>(item: T) -> u16;

 }

pub trait Scored {
   fn collect_scores() -> Vec<Score>;
}


pub trait HasReputation {

   /// Return the reputation for a given struct.
   fn get_reputation(&self) -> ReputationUnit;
}

pub trait HasCredibility {

   /// Return the credibility for a given struct.
   fn get_credibility(&self) -> CredibilityUnit;
}

