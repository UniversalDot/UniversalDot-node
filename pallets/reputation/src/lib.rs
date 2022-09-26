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
		pub aggregate_rating: u64,
		pub num_of_ratings: u64,
		pub account: T::AccountId,
	}
	
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type ReputationHandler: ReputationHandler<Self>;
		type DefaultReputation: Get<i32>;
	}

	#[pallet::storage]
	#[pallet::getter(fn reputation_of)]
	pub type RepInfoOf<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Reputable<T>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ReputationRecordCreated{who: T::AccountId},
		ReputationRecordRemoved{who: T::AccountId},
		AccountRated{who: T::AccountId},
	}

	#[pallet::error]
	pub enum Error<T> {
		ReputationAlreadyExists,
		CannotRemoveNothing,
		RecordNotFound
	}

	impl<T: Config> Pallet<T> {

		/// Creates a reputation record for a given account id.
		pub fn create_reputation_record(account: &T::AccountId) -> DispatchResult {
			let rep_record = Self::reputation_of(&account); 
			ensure!(rep_record.is_none(), Error::<T>::ReputationAlreadyExists);

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
			let rep_record = Self::reputation_of(&account); 
			ensure!(rep_record.is_some(), Error::<T>::CannotRemoveNothing);

			RepInfoOf::<T>::remove(&account);
			Self::deposit_event(Event::ReputationRecordRemoved{who: account.clone()});

			Ok(())
		}

		/// Rate the account and adjust the reputation and credibility as defined by the ReputationHandler.
		pub fn rate_account(account: &T::AccountId, ratings: &Vec<u8>) -> DispatchResult {
			
			let mut record: Reputable<T> = RepInfoOf::<T>::get(account).ok_or(Error::<T>::RecordNotFound).unwrap();

			let new_credibility = T::ReputationHandler::calculate_credibility(&record, ratings);
			let new_reputation = T::ReputationHandler::calculate_reputation(&record, ratings);
			let ratings_sum = ratings.iter().map(|i| *i as u64).sum();

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
