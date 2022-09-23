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
pub mod impls;


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
	use crate::impls::Rep;

	pub const MAX_CREDIBILITY: CredibilityUnit = 1000;

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
	pub type RepInfoOf<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Rep, OptionQuery>;

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

			let rep = Rep {
				reputation: default_reputation,
				credibility: MAX_CREDIBILITY / 2,
				aggregate_rating: Default::default(),
				num_of_ratings: Default::default(),
			};

			RepInfoOf::<T>::insert(account, rep);
			Ok(())
		}

		pub fn remove_reputation_record(account: T::AccountId) -> DispatchResult {
			let rep_record = Self::reputation_of(&account); 
			ensure!(rep_record.is_some(), Error::<T>::CannotRemoveNothing);

			RepInfoOf::<T>::remove(account);
			Ok(())
		}

		pub fn handle_reputation_change(account: T::AccountId) -> DispatchResult {
			
			Ok(())
		}
	}
}
