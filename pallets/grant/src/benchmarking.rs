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
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

// Helper function to assert event thrown during verification
fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}


benchmarks! {
	request_grant {
		/* setup initial state */

		let caller: T::AccountId = whitelisted_caller();
		let grant_receiver:  T::AccountId = whitelisted_caller();
	}: request_grant(RawOrigin::Signed(caller), grant_receiver)

	verify {
		/* verifying final state */
		let caller: T::AccountId = whitelisted_caller();
		assert_last_event::<T>(Event::<T>::GrantRequested { who: caller }.into());
	}

	transfer_funds {
		/* setup initial state */
		let caller: T::AccountId = whitelisted_caller();
		let grant_receiver:  T::AccountId = whitelisted_caller();
	}: transfer_funds(RawOrigin::Signed(caller), grant_receiver)
	verify {
			/* verifying final state */
		let caller: T::AccountId = whitelisted_caller();
		assert_last_event::<T>(Event::<T>::GrantIssued{ who: caller }.into());
	}

	 winner_is {
	 	/* setup initial state */
	 	let treasury_account: T::AccountId = whitelisted_caller();
	 	let grant_receiver: T::AccountId = whitelisted_caller()
	 }: winner_is(RawOrigin::Signed(treasury_account)
	 verify {
	 	/* verifying final state */
	 	let caller: T::AccountId = whitelisted_caller();
	 	assert_last_event::<T>(Event::<T>::WinnerSelected { who: caller }.into());
	 }

}

impl_benchmark_test_suite!(PalletGrant, crate::mock::new_test_ext(), crate::mock::Test,);
