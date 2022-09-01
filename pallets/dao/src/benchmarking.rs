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

//! Benchmarking setup for pallet-dao

use super::*;
#[allow(unused)]
use crate::Pallet as PalletDao;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller, vec};
use frame_system::RawOrigin;
use sp_core::crypto::UncheckedFrom;

const SEED: u32 = 1;

// Helper function to assert event thrown during verification
fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
	where_clause { where
		T::AccountId: UncheckedFrom<T::Hash>,
	}
	create_vision {
		/* setup initial state */
		let caller: T::AccountId = whitelisted_caller();

		let s in 1 .. u8::MAX.into();
		let vision = vec![0u8, s as u8];

	}: create_vision(RawOrigin::Signed(caller.clone()), vision.clone())
	verify {
		/* verifying final state */
		assert_last_event::<T>(Event::<T>::VisionCreated (caller, vision ).into());
	}

	remove_vision {
		/* setup initial state */
		let caller: T::AccountId = whitelisted_caller();

		let s in 1 .. u8::MAX.into();
		let vision = vec![0u8, s as u8];

		// Create vision before removing
		let _ = PalletDao::<T>::create_vision(RawOrigin::Signed(caller.clone()).into(), vision.clone());

	}: remove_vision(RawOrigin::Signed(caller.clone()), vision.clone())
	verify {
		/* verifying final state */
		assert_last_event::<T>(Event::<T>::VisionRemoved (caller, vision ).into());
	}

	sign_vision {
		/* setup initial state */
		let caller: T::AccountId = whitelisted_caller();

		let s in 1 .. u8::MAX.into();
		let vision = vec![0u8, s as u8];

		// Create vision before removing
		let _ = PalletDao::<T>::create_vision(RawOrigin::Signed(caller.clone()).into(), vision.clone());

	}: sign_vision(RawOrigin::Signed(caller.clone()), vision.clone())
	verify {
		/* verifying final state */
		assert_last_event::<T>(Event::<T>::VisionSigned (caller, vision ).into());
	}

	unsign_vision {
		/* setup initial state */
		let caller: T::AccountId = whitelisted_caller();

		let s in 1 .. u8::MAX.into();
		let vision = vec![0u8, s as u8];

		// Create vision before removing
		let _ = PalletDao::<T>::create_vision(RawOrigin::Signed(caller.clone()).into(), vision.clone());
		let _ = PalletDao::<T>::sign_vision(RawOrigin::Signed(caller.clone()).into(), vision.clone());


	}: unsign_vision(RawOrigin::Signed(caller.clone()), vision.clone())
	verify {
		/* verifying final state */
		assert_last_event::<T>(Event::<T>::VisionUnsigned (caller, vision ).into());
	}

	create_organization {
		/* setup initial state */
		let caller: T::AccountId = whitelisted_caller();

		let s in 1 .. u8::MAX.into();
		let name = vec![0u8, s as u8].try_into().unwrap();
		let description = vec![0u8, s as u8].try_into().unwrap();
		let vision = vec![0u8, s as u8].try_into().unwrap();


	}: create_organization(RawOrigin::Signed(caller.clone()), name, description, vision)
	verify {
		// let hash = PalletDao::<T>::get_hash_for_dao(&caller, &name, &description, &vision, 1_u32.into(), 1_u32.into());
		let hash = PalletDao::<T>::member_of(&caller)[0];
		assert_last_event::<T>(Event::<T>::OrganizationCreated(caller, hash.into()).into())
	}

	update_organization {
		/* setup initial state */
		let caller: T::AccountId = whitelisted_caller();

		let s in 1 .. u8::MAX.into();
		let name = vec![0u8, s as u8];
		let description = vec![0u8, s as u8];
		let vision = vec![0u8, s as u8];

		let _ = PalletDao::<T>::create_organization(RawOrigin::Signed(caller.clone()).into(), name.clone().try_into().unwrap(),
			description.clone().try_into().unwrap(), vision.clone().try_into().unwrap());
		let org_id = PalletDao::<T>::member_of(&caller)[0];

	}: update_organization(RawOrigin::Signed(caller.clone()), org_id, Some(name.try_into().unwrap()), Some(description.try_into().unwrap()), Some(vision.try_into().unwrap()))
	verify {
		let hash = PalletDao::<T>::member_of(&caller)[0];
		assert_last_event::<T>(Event::<T>::OrganizationUpdated(caller, hash.into()).into())
	}

	transfer_ownership {
		/* setup initial state */
		let caller: T::AccountId = whitelisted_caller();

		let s in 1 .. u8::MAX.into();
		let name = vec![0u8, s as u8].try_into().unwrap();
		let description = vec![0u8, s as u8].try_into().unwrap();
		let vision = vec![0u8, s as u8].try_into().unwrap();

		let _ = PalletDao::<T>::create_organization(RawOrigin::Signed(caller.clone()).into(), name, description, vision);
		let org_id = PalletDao::<T>::member_of(&caller)[0];

	}: transfer_ownership(RawOrigin::Signed(caller.clone()), org_id, caller.clone())
	verify {
		let hash = PalletDao::<T>::member_of(&caller)[0];
		assert_last_event::<T>(Event::<T>::OrganizationOwnerChanged(caller.clone(), hash.into(), caller).into())
	}

	dissolve_organization {
		/* setup initial state */
		let caller: T::AccountId = whitelisted_caller();

		let s in 1 .. u8::MAX.into();
		let name = vec![0u8, s as u8].try_into().unwrap();
		let description = vec![0u8, s as u8].try_into().unwrap();
		let vision = vec![0u8, s as u8].try_into().unwrap();

		// Create organization before dissolving it
		let _ = PalletDao::<T>::create_organization(RawOrigin::Signed(caller.clone()).into(), name, description, vision);
		// let org_id = PalletDao::<T>::get_hash_for_dao(&caller, &name, &description, &vision, 0_u32.into(), 0_u32.into());
		let org_id = PalletDao::<T>::member_of(&caller)[0];

	}: dissolve_organization(RawOrigin::Signed(caller.clone()), org_id.clone())
		/* the code to be benchmarked */

	verify {
		/* verifying final state */
		assert_last_event::<T>(Event::<T>::OrganizationDissolved(caller, org_id).into())
	}

	add_members {
		/* setup initial state */
		let caller: T::AccountId = whitelisted_caller();

		let s in 1 .. u8::MAX.into();
		let name = vec![0u8, s as u8].try_into().unwrap();
		let description = vec![0u8, s as u8].try_into().unwrap();
		let vision = vec![0u8, s as u8].try_into().unwrap();

		// Create account for member
		let account: T::AccountId = account("member", s, SEED);

		// Create organization before adding members to it
		let _ = PalletDao::<T>::create_organization(RawOrigin::Signed(caller.clone()).into(), name, description, vision);
		// let org_id = PalletDao::<T>::get_hash_for_dao(&caller, &name, &description, &vision, 0_u32.into(), 0_u32.into());
		let org_id = PalletDao::<T>::member_of(&caller)[0];

	}: add_members(RawOrigin::Signed(caller.clone()), org_id.clone(), account.clone())
		/* the code to be benchmarked */
	verify {
		/* verifying final state */
		assert_last_event::<T>(Event::<T>::MemberAdded (caller, account, org_id).into());
	}

	remove_members {
		/* setup initial state */
		let caller: T::AccountId = whitelisted_caller();

		let s in 1 .. u8::MAX.into();
		let name = vec![0u8, s as u8].try_into().unwrap();
		let description = vec![0u8, s as u8].try_into().unwrap();
		let vision = vec![0u8, s as u8].try_into().unwrap();

		// Create account for member
		let u:u32 = 7;
		let account: T::AccountId = account("member", u, SEED);

		// Create organization before adding members to it
		let _ = PalletDao::<T>::create_organization(RawOrigin::Signed(caller.clone()).into(), name, description, vision);
		// let org_id = PalletDao::<T>::get_hash_for_dao(&caller, &name, &description, &vision, 0_u32.into(), 0_u32.into());
		let org_id = PalletDao::<T>::member_of(&caller)[0];
		let _ = PalletDao::<T>::add_members(RawOrigin::Signed(caller.clone()).into(), org_id.clone(), account.clone());
		assert_eq!(PalletDao::<T>::members(org_id.clone()).len(), 2);

	}: remove_members(RawOrigin::Signed(caller.clone()), org_id.clone(), account.clone() )
		/* the code to be benchmarked */
	verify {
		/* verifying final state */
		assert_eq!(PalletDao::<T>::members(org_id.clone()).len(), 1);
		assert_last_event::<T>(Event::<T>::MemberRemoved (caller, account, org_id).into());
	}
}

impl_benchmark_test_suite!(PalletDao, crate::mock::new_test_ext(), crate::mock::Test,);