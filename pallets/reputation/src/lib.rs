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
		pub aggregate_rating: u64,
		pub num_of_ratings: u64,
		pub account: T::AccountId,
	}
	
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type WeightInfo: WeightInfo;
		type ReputationHandler: ReputationHandler<Self>;
	}

	#[pallet::storage]
	#[pallet::getter(fn reputation_of)]
	pub type RepInfoOf<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Reputable<T>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ReputationRecordCreated{who: &AccountId}
		ReputationRecordRemoved{who: &AccountId}
		EntityRated{who: &AccountId}
	}

	#[pallet::error]
	pub enum Error<T > {
		ReputationAlreadyExists,
		CannotRemoveNothing
	}

	impl<T: Config> Pallet<T> {

		/// Creates a reputation record for a given account id.
		pub fn create_reputation_record(account: T::AccountId, default_reputation: ReputationUnit) -> DispatchResult {
			let rep_record = Self::reputation_of(&account); 
			ensure!(rep_record.is_none(), Error::<T>::ReputationAlreadyExists);

			let rep = Reputable {
				account: account.clone(),
				reputation: default_reputation,
				credibility: MAX_CREDIBILITY / 2,
				aggregate_rating: Default::default(),
				num_of_ratings: Default::default(),
			};

			RepInfoOf::<T>::insert(account, rep);
			Self::deposit_event(Event::ReputationRecordCreated{who: account});
			Ok(())
		}

		/// Remove a reputation record from storage.
		pub fn remove_reputation_record(account: T::AccountId) -> DispatchResult {
			let rep_record = Self::reputation_of(&account); 
			ensure!(rep_record.is_some(), Error::<T>::CannotRemoveNothing);

			RepInfoOf::<T>::remove(account);
			Self::deposit_event(Event::ReputationRecordRemoved{who: account});

			Ok(())
		}

		/// Rate the account and adjust the reputation and credibility as defined by the ReputationHandler.
		pub fn rate_account(account: &T::AccountId, ratings: &Vec<u8>) -> DispatchResult {
			
			let mut record: Reputable<T> = RepInfoOf::<T>::get(account);
			let new_credibility = T::ReputationHander::calculate_credibility(record, ratings);
			let new_reputation = T::ReputationHandler::	(record, ratings);

			record.reputation = new_reputation;
			record.num_of_ratings += ratings.len();
			record.aggregate_rating += ratings.iter().sum();
			record.credibility = new_credibility;
			
			let _  = RepInfoOf::<T>::insert(account, record);
			Ok(())
		}
	}
}
