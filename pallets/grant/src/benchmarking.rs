// This file is part of Substrate.

// Copyright UNIVERSALDOT FOUNDATION
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

//! Benchmarking setup for pallet-grant

use super::*;

#[allow(unused)]
use crate::Pallet as PalletGrant;
use crate::Config as PalletConfig;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use frame_support::traits::{Currency, Get};

const SEED: u32 = 10; 


// Helper function to assert event thrown during verification
fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn setup_account<T: Config>(is_empty: bool) -> (T::AccountId, BalanceOf<T>)
where
T: PalletConfig
{
	let account = account("account", SEED, SEED);
	let mut value: BalanceOf<T> = T::ExistentialDeposit::get().into();
	if !is_empty {
		value = 10_000_000u32.into();
	}
	let _ = T::Currency::make_free_balance_be(&account, value);
	(account, value)	
}

benchmarks! {
	request_grant {
		/* setup initial state */

		let (caller, _)= setup_account::<T>(true);
	}: request_grant(RawOrigin::Signed(caller.clone()))

	verify {
		/* verifying final state */
		assert_last_event::<T>(Event::<T>::GrantRequested { who: caller }.into());
	}

	transfer_to_treasury {
		let (account_id, value) = setup_account::<T>(false);
		
	}: transfer_to_treasury(RawOrigin::Signed(account_id.clone()), 1000u32.into())
	verify {
		assert_last_event::<T>(Event::<T>::TreasuryDonation{ who: account_id }.into());
	}

	 winner_is {
		 let (account_id, _) = setup_account::<T>(false);
		 let _ = PalletGrant::<T>::transfer_to_treasury(RawOrigin::Signed(account_id.clone()).into(), 1000u32.into())?;

		 let (grant_receiver, _) = setup_account::<T>(true);
		 <Winner<T>>::put(&grant_receiver);

	 }: winner_is(RawOrigin::Signed(grant_receiver.clone()))
	 verify {
	 	assert_last_event::<T>(Event::<T>::WinnerSelected { who: grant_receiver }.into());
	 }
}

impl_benchmark_test_suite!(PalletGrant, crate::mock::new_test_ext(), crate::mock::Test);
