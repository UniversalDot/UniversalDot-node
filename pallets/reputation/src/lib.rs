#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod traits;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::{
		pallet_prelude::*,
		WeightInfo
	};

	pub type ReputationUnit = i32;
	pub type CredibilityUnit = u32;
	pub type Score = u16;
	use crate::traits::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type WeightInfo: WeightInfo;
		type ReputationHandler: WeightInfo;
	}

	#[pallet::storage]
	#[pallet::getter(fn reputation_of)]
	pub type ReputationOf<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, ReputationUnit, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn credibility_of)]
	pub type CredibilityOf<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, CredibilityUnit, OptionQuery>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
	}

	#[pallet::error]
	pub enum Error<T > {
		ReputationAlreadyExists,
		CannotRemoveNothing

	}

	impl<T: Config> Pallet<T> {
		pub fn create_reputation_record(account: T::AccountId, default_reputation: ReputationUnit) -> DispatchResult {
			let rep_record = Self::reputation_of(&account); 
			ensure!(rep_record.is_none(), Error::<T>::ReputationAlreadyExists);

			ReputationOf::<T>::insert(account, default_reputation);
			Ok(())
		}

		pub fn remove_reputation_record(account: T::AccountId) -> DispatchResult {
			let rep_record = Self::reputation_of(&account); 
			ensure!(rep_record.is_some(), Error::<T>::CannotRemoveNothing);

			ReputationOf::<T>::remove(account);
			Ok(())
		}

		pub fn handle_reputation_change(account: T::AccountId) -> DispatchResult {
			
			Ok(())
		}
	}
}
