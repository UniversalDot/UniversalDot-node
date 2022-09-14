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

//! # Grant Pallet
//! 
//! ## Version: 0.0.1
//!
//! - [`Config`]
//! - [`Pallet`]
//!
//! ## Overview
//!
//! The Grant Pallet is used to Grant tokens to new AccountIDs.
//! In order to create Profile, Tasks, Organizations users need initial tokens. 
//! 
//! The Grant pallet is used to issue tokens to new users that intent to use the dApp.
//! 
//! The grants are issued in random fashion, such that requesters are awarded tokens in a random manner.
//! The intention is that initially, when there are only few users of the platform, every grant_request is
//! automatically approved. However, later on when the application reaches more use, grants are offered randomly
//! to requesting accounts. 
//!
//! The Process is envisioned as follows:
//! 1. Anyone can send Funds into a Treasury Account. The Treasury account is used to distribute grant rewards.
//! 2. Anyone can request a grant each block.
//! 3. Each block a grant is offered randomly to selected grant requester.
//!
//! ## Interface
//!
//! ### Public Functions
//!  -  request_grant()
//!     Function used to request grants.
//!
//!  -  transfer_funds()
//!     Function used to transfer funds into a Treasury Account. Anyone can transfer into Treasury.
//!
//!  -  winner_is()
//!     Function that announces the winner of the block.
//!
//! ## Related Modules
//!

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_support::inherent::Vec;
	use frame_system::pallet_prelude::*;
	use frame_support::{ 
		sp_runtime::traits::{Hash, Zero, AccountIdConversion,  Saturating},
		traits::{
			Currency, 
			Randomness,
			tokens::ExistenceRequirement,
		}};
	use scale_info::TypeInfo;
	use crate::weights::WeightInfo;
	use frame_support::PalletId;
	use core::convert::TryInto;

	// Account, Balance
	type AccountOf<T> = <T as frame_system::Config>::AccountId;
	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	// Struct for holding Request information.
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Requesters<T: Config> {
		pub owner: AccountOf<T>,
		pub block_number: <T as frame_system::Config>::BlockNumber,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_randomness_collective_flip::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The Currency handler for the Profile pallet.
		type Currency: Currency<Self::AccountId>;

		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// WeightInfo provider.
		type WeightInfo: WeightInfo;

		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;

		/// The configured account of the treasury.
		type TreasuryAccount: Get<Self::AccountId>;

		/// The total amount of tokens per grant.
		type GrantAmount: Get<BalanceOf<Self>>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn winner)]
	/// Stores the current winner for the block
	pub(super) type Winner<T: Config> = StorageValue<_, T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn storage_requesters)]
	/// Stores a Requesters unique properties in a StorageMap.
	pub(super) type StorageRequesters<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Requesters<T>>;

	#[pallet::storage]
	#[pallet::getter(fn requesters_count)]
	/// Store requester count
	pub(super) type RequestersCount<T: Config> = StorageValue<_, u32, ValueQuery>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Profile was successfully created.
		GrantIssued { who: T::AccountId },

		/// Profile was successfully deleted.
		GrantRequested { who: T::AccountId },

		/// Profile was successfully updated.
		WinnerSelected { who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Cant grant to receiving account
		CantGrantToSelf,
		// User has already made requests
		RequestAlreadyMade,
		// You must have empty balance to receive tokens.
		NonEmptyBalance,
		// Too many requesters in current block
		TooManyRequesters,
		// No winner exists
		NoWinner
		// Treasury is out of fund!
		TreasuryEmpty
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// Dispatchable call that ensures grants can be requested
		#[pallet::weight(<T as Config>::WeightInfo::request_grant())]
		pub fn request_grant(origin: OriginFor<T>, grant_requester: T::AccountId) -> DispatchResult {

			// Check that the extrinsic was signed and get the signer.
			let _account = ensure_signed(origin)?;

			// Generate requests and store them 
			let _requests = Self::generate_requests(&grant_requester)?;

			// Deposit event for grant requested			
			Self::deposit_event(Event::GrantRequested{ who:grant_requester });

			// pays no fees
			Ok(())
		}

		/// Dispatchable call that enables transfer of funds
		#[pallet::weight(<T as Config>::WeightInfo::request_grant())]
		pub fn transfer_funds(origin: OriginFor<T>, treasury: T::AccountId, amount: BalanceOf<T>) -> DispatchResult {

			// Check that the extrinsic was signed and get the signer.
			let account = ensure_signed(origin)?;

			// Ensure no conflicts of interest
			ensure!(account != treasury, Error::<T>::CantGrantToSelf);

			// Transfer ammount from one account to treasury
            <T as self::Config>::Currency::transfer(&account, &treasury, amount, ExistenceRequirement::KeepAlive)?;

			// Emit an event.
			Self::deposit_event(Event::GrantIssued{ who:account });

			Ok(())
		}

		// Dispatchable calls that allows to query the winner
		#[pallet::weight(<T as Config>::WeightInfo::request_grant())]
		pub fn winner_is(origin: OriginFor<T>) -> DispatchResult {

			// Check that the extrinsic was signed and get the signer.
			let _account = ensure_signed(origin)?;

			// Get the winner
			let winner = <Winner<T>>::get().ok_or(<Error<T>>::NoWinner)?; // AccountId should not use default: https://substrate.stackexchange.com/a/1814
			
			// Deposit event
			Self::deposit_event(Event::WinnerSelected{ who:winner });

			Ok(())
		}
	}

	#[pallet::hooks]
	impl<T:Config> Hooks<T::BlockNumber> for Pallet<T> {

		// Each block, check if there are requests for grants and award a grant to random account
		fn on_initialize(_n: T::BlockNumber) -> frame_support::weights::Weight {
			
			let weight = 10000;
			let requests = Self::requesters_count();

			// Only select winners when we have requests
			if requests > 0u32 {
				let _winner = Self::select_winner();
				
				// Flush Requests each block
				<RequestersCount<T>>::kill();
				<StorageRequesters<T>>::drain();
			}
			
			// return weight
			weight
		}
	}

	// ** Helper internal functions ** //
	impl<T:Config> Pallet<T> {

		fn treasury_account() -> (T::AccountId, BalanceOf<T>) {
			let account_id = T::TreasuryAccount::get();
			let balance =
				T::Currency::free_balance(&account_id).saturating_sub(T::Currency::minimum_balance());
	
			(account_id, balance)
		}
		
		// Generates requests in storage
		fn generate_requests(grant_receiver: &T::AccountId) -> Result<T::Hash, DispatchError> {

			// Get current balance of owner
			let balance = T::Currency::free_balance(grant_receiver);
			
			// Ensure only accounts with empty balance can make grant requests
			ensure!(balance.is_zero() , Error::<T>::NonEmptyBalance);
			
			// Populate Requesters struct
			let requesters = Requesters::<T> {
				owner: grant_receiver.clone(),
				block_number: <frame_system::Pallet<T>>::block_number(),
			};
			
			// Get hash of profile
			let requesters_id = T::Hashing::hash_of(&requesters);

			// Insert profile into HashMap
			<StorageRequesters<T>>::insert(grant_receiver, requesters);

			// Increase count for requesters
			let new_count = Self::requesters_count().checked_add(1).ok_or(<Error<T>>::TooManyRequesters)?;
			<RequestersCount<T>>::put(new_count);

			Ok(requesters_id)
		}

		fn select_winner() -> Result<(), DispatchError> {

			let requestor: Vec<T::AccountId> = <StorageRequesters<T>>::iter_keys().collect();

			//TODO: come up with a proper seed. Unix time in ns?
			let get_random_number = Self::generate_random_number(0);
			let total_requestors: u32 = requestor.len().try_into().unwrap();
			let winner_index: usize = (get_random_number % total_requestors).try_into().unwrap();
			let winner = &requestor[winner_index];

			<Winner<T>>::put(winner);

			let _ = Self::transfer_funds_to_winner()?;

			Ok(())
		}


		// Generating randomness
		fn generate_random_number(seed: u32) -> u32 {
			let (random_seed, _) = T::Randomness::random(&(T::PalletId::get(), seed).encode());
			let random_number = <u32>::decode(&mut random_seed.as_ref()).expect("secure hashes should always be bigger than u32; qed");
			random_number
		}

		// Function that allows funds to be sent to winner
		fn transfer_funds_to_winner() -> Result<(), DispatchError> {

			let (treasury_account, treasury_balance) = Self::treasury_account();
			let grant_total = T::GrantAmount::get();

			ensure!(treasury_balance > grant_total, Error::<T>::TreasuryEmpty);

			let winner = &Self::winner().ok_or(<Error<T>>::NoWinner)?; // AccountId should not use default: https://substrate.stackexchange.com/a/1814
			
			let transfer = T::Currency::transfer(treasury_account, winner, grant_total, ExistenceRequirement::KeepAlive);
			debug_assert!(transfer.is_ok());

			Ok(())
		}
	}
}
