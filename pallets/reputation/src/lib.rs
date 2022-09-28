// This file is part of Substrate.

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

//! # Reputation Pallet
//! 
//! ## Version: 0.0.1
//!
//! ## Overview
//!
//! Given an account id, create some way of recording the reputation of that entity.
//! Implementation assumes that ratings will be given by another credible accountid.
//!
//! implementation incomplete. 

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


pub mod traits;
pub mod impls;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
		BoundedVec
	};
	use frame_system::{
		pallet_prelude::*,
		WeightInfo
	};

	pub type ReputationUnit = i32;
	pub type CredibilityUnit = u32;
	pub type Rating = u8;
	use crate::traits::ReputationHandler;

	pub const MAX_CREDIBILITY: CredibilityUnit = 1000;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);
	
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Reputable<T: frame_system::Config> {
		pub reputation: ReputationUnit,
		pub credibility: CredibilityUnit,
		// The aggregate of all ratings
		pub aggregate_rating: u64,
		pub num_of_ratings: u64,
		pub account: T::AccountId,
	}
	
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The trait that defines how the pallet deals with reputation and credibility.
		type ReputationHandler: ReputationHandler<Self>;

		/// The default reputation of an account.
		type DefaultReputation: Get<i32>;

		/// Maximum number of ratings per action
		type MaximumRatingsPer: Get<u32	> + MaxEncodedLen + TypeInfo;
	}

	#[pallet::storage]
	#[pallet::getter(fn reputation_of)]
	pub type RepInfoOf<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Reputable<T>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Reputation record created.
		ReputationRecordCreated{who: T::AccountId},
		/// Reputation record removed.
		ReputationRecordRemoved{who: T::AccountId},
		/// Account rated.
		AccountRated{who: T::AccountId},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// You cannot create duplicate reputation records.
		ReputationAlreadyExists,
		/// Reputation record does not exist.
		CannotRemoveNothing,
		/// Reputation Record not found.
		RecordNotFound
	}

	impl<T: Config> Pallet<T> {

		/// Creates a reputation record for a given account id.
		pub fn create_reputation_record(account: &T::AccountId) -> DispatchResult {

			// Ensure that a record does not exist.
			let rep_record = Self::reputation_of(&account); 
			ensure!(rep_record.is_none(), Error::<T>::ReputationAlreadyExists);

			// Instantiate and insert into storage.
			let rep = Reputable {
				account: account.clone(),
				reputation: T::DefaultReputation::get(),
				credibility: MAX_CREDIBILITY / 2,
				aggregate_rating: Default::default(),
				num_of_ratings: Default::default(),
			};

			RepInfoOf::<T>::insert(account, rep);
			Self::deposit_event(Event::ReputationRecordCreated{who: account.clone()});
			Ok(())
		}

		/// Remove a reputation record from storage.
		pub fn remove_reputation_record(account: T::AccountId) -> DispatchResult {

			// Ensure record exists.
			let rep_record = Self::reputation_of(&account); 
			ensure!(rep_record.is_some(), Error::<T>::CannotRemoveNothing);

			// Remove from storage.
			RepInfoOf::<T>::remove(&account);
			Self::deposit_event(Event::ReputationRecordRemoved{who: account.clone()});

			Ok(())
		}

		/// Rate the account and adjust the reputation and credibility as defined by the ReputationHandler.
		pub fn rate_account(account: &T::AccountId, ratings: &BoundedVec<Rating, T::MaximumRatingsPer>) -> DispatchResult {
			
			// Get the old record.
			let mut record: Reputable<T> = RepInfoOf::<T>::get(account).ok_or(Error::<T>::RecordNotFound).unwrap();

			// Calculate the new totals as defined in the ReputationHandle.
			let new_credibility = T::ReputationHandler::calculate_credibility(&record, ratings);
			let new_reputation = T::ReputationHandler::calculate_reputation(&record, ratings);
			let ratings_sum = ratings.iter().map(|i| *i as u64).sum();

			// Update the record and insert into storage.
			record.reputation = new_reputation;
			record.num_of_ratings = record.num_of_ratings.saturating_add(ratings.len() as u64);
			record.aggregate_rating = record.aggregate_rating.saturating_add(ratings_sum);
			record.credibility = new_credibility;
			
			let _  = RepInfoOf::<T>::insert(&account, record);

			Self::deposit_event(Event::AccountRated{who: account.clone()});
			Ok(())
		}
	}
}
