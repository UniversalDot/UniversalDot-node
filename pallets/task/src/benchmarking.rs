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

//! Benchmarking setup for pallet-task

use super::*;

#[allow(unused)]
use crate::Pallet as PalletTask;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller, vec, Vec};
use frame_system::RawOrigin;
use frame_support::{
	traits::Currency,
	sp_runtime::traits::Hash
};
use pallet_profile::Pallet as PalletProfile;
use pallet_dao::Pallet as PalletDao;
use sp_std::convert::TryInto;

const SEED: u32 = 0;

// Helper function to assert event thrown during verification
fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}


// Helper function to create a profile
fn create_profile<T: Config>(){

	let username = Vec::new();
	let interests = Vec::new();
	let information = Vec::new();
	let available_hours_per_week = 40_u8;

	let caller: T::AccountId = whitelisted_caller();
	let _profile = PalletProfile::<T>::create_profile(RawOrigin::Signed(caller).into(), username.try_into().unwrap(), interests.try_into().unwrap(), available_hours_per_week, Some(information.try_into().unwrap()));

}


benchmarks! {
	create_task {
		/* setup initial state */
		let caller: T::AccountId = whitelisted_caller();

		// Populate data fields
		let s in 1 .. u8::MAX.into(); // max bytes for specification
		let x in 1 .. 2000;
		let title = vec![0u8, s as u8];
		let specification = vec![0u8, s as u8];
		let attachments = vec![0u8, s as u8];
		let keywords = vec![0u8, s as u8];

		let budget = <T as pallet::Config>::Currency::total_balance(&caller);

		// Create profile before creating a task
		create_profile::<T>();

	}:
	/* the code to be benchmarked */
	create_task(RawOrigin::Signed(caller.clone()), title.try_into().unwrap(), specification.try_into().unwrap(), budget, x.into(), attachments.try_into().unwrap(), keywords.try_into().unwrap(), None)

	verify {
		/* verifying final state */
		let caller: T::AccountId = whitelisted_caller();
		let task_id = PalletTask::<T>::tasks_owned(&caller)[0];

		assert_last_event::<T>(Event::<T>::TaskCreated(caller, task_id).into());
	}

	update_task {
		/* setup initial state */
		let caller: T::AccountId = whitelisted_caller();

		// Populate data fields
		let s in 1 .. u8::MAX.into(); // max bytes for specification
		let x in 1 .. 2000;
		let title = vec![0u8, s as u8];
		
		let specification = vec![0u8, s as u8];
		let budget = <T as pallet::Config>::Currency::total_balance(&caller);
		let attachments = vec![0u8, s as u8];
		let keywords = vec![0u8, s as u8];
		
		
		// Create profile before creating a task
		create_profile::<T>();
		let _ = PalletTask::<T>::create_task(RawOrigin::Signed(caller.clone()).into(), title.clone().try_into().unwrap(), specification.clone().try_into().unwrap(), budget, x.into(), attachments.clone().try_into().unwrap(), keywords.clone().try_into().unwrap(), None);
		let hash_task = PalletTask::<T>::tasks_owned(&caller)[0];

	}:
	/* the code to be benchmarked */
	update_task(RawOrigin::Signed(caller.clone()), hash_task, title.try_into().unwrap(), specification.try_into().unwrap(), budget, x.into(), attachments.try_into().unwrap(), keywords.try_into().unwrap(), None)

	verify {
		/* verifying final state */
		let caller: T::AccountId = whitelisted_caller();
		let hash = PalletTask::<T>::tasks_owned(&caller)[0];

		assert_last_event::<T>(Event::<T>::TaskUpdated(caller, hash).into());
	}

	start_task {
		/* setup initial state */
		let task_creator: T::AccountId = whitelisted_caller();
		let volunteer: T::AccountId = account("volunteer", 0, SEED);

		// Populate data fields
		let s in 1 .. u8::MAX.into(); // max bytes for specification
		let x in 1 .. 2000;
		let title = vec![0u8, s as u8];
		let specification = vec![0u8, s as u8];
		let budget = <T as pallet::Config>::Currency::total_balance(&task_creator);
		let attachments = vec![0u8, s as u8];
		let keywords = vec![0u8, s as u8];

		// Create profile before creating a task
		create_profile::<T>();
		let _ = PalletTask::<T>::create_task(RawOrigin::Signed(task_creator.clone()).into(), title.try_into().unwrap(), specification.try_into().unwrap(), budget, x.into(), attachments.try_into().unwrap(), keywords.try_into().unwrap(), None);
		let hash_task = PalletTask::<T>::tasks_owned(&task_creator)[0];

	}: start_task(RawOrigin::Signed(volunteer.clone()), hash_task)
		/* the code to be benchmarked */

	verify {
		/* verifying final state */
		assert_last_event::<T>(Event::<T>::TaskAssigned(volunteer, hash_task).into());
	}

	remove_task {
		/* setup initial state */
		let task_creator: T::AccountId = whitelisted_caller();
		let volunteer: T::AccountId = account("volunteer", 0, SEED);

		// Populate data fields
		let s in 1 .. u8::MAX.into(); // max bytes for specification
		let x in 1 .. 2000;
		let title = vec![0u8, s as u8];
		let specification = vec![0u8, s as u8];
		let budget = <T as pallet::Config>::Currency::total_balance(&task_creator);
		let attachments = vec![0u8, s as u8];
		let keywords = vec![0u8, s as u8];

		// Create profile before creating a task
		create_profile::<T>();
		let _ = PalletTask::<T>::create_task(RawOrigin::Signed(task_creator.clone()).into(), title.try_into().unwrap(), specification.try_into().unwrap(), budget, x.into(), attachments.try_into().unwrap(), keywords.try_into().unwrap(), None);
		let hash_task = PalletTask::<T>::tasks_owned(&task_creator)[0];

	}: remove_task(RawOrigin::Signed(task_creator.clone()), hash_task)
		/* the code to be benchmarked */

	verify {
		/* verifying final state */
		assert_last_event::<T>(Event::<T>::TaskRemoved(task_creator, hash_task).into());
	}

	complete_task {
		/* setup initial state */
		let task_creator: T::AccountId = whitelisted_caller();
		let volunteer: T::AccountId = account("volunteer", 0, SEED);

		// Populate data fields
		let s in 1 .. u8::MAX.into(); // max bytes for specification
		let x in 1 .. 2000;
		let title = vec![0u8, s as u8];
		let specification = vec![0u8, s as u8];
		let budget = <T as pallet::Config>::Currency::total_balance(&task_creator);
		let attachments = vec![0u8, s as u8];
		let keywords = vec![0u8, s as u8];

		// Create profile before creating a task
		create_profile::<T>();
		let _ = PalletTask::<T>::create_task(RawOrigin::Signed(task_creator.clone()).into(), title.try_into().unwrap(), specification.try_into().unwrap(), budget, x.into(), attachments.try_into().unwrap(), keywords.try_into().unwrap(), None);
		let hash_task = PalletTask::<T>::tasks_owned(&task_creator)[0];
		let _ = PalletTask::<T>::start_task(RawOrigin::Signed(volunteer.clone()).into(), hash_task.clone());

	}: complete_task(RawOrigin::Signed(volunteer.clone()), hash_task)
		/* the code to be benchmarked */

	verify {
		/* verifying final state */
		assert_last_event::<T>(Event::<T>::TaskCompleted(volunteer, hash_task).into());
	}

	accept_task {
		/* setup initial state */
		let task_creator: T::AccountId = whitelisted_caller();
		let volunteer: T::AccountId = whitelisted_caller();

		// Populate data fields
		let s in 1 .. u8::MAX.into(); // max bytes for specification
		let x in 1 .. 4000;
		let title = vec![0u8, s as u8];
		let specification = vec![0u8, s as u8];
		let budget = <T as pallet::Config>::Currency::total_balance(&task_creator);
		let attachments = vec![0u8, s as u8];
		let keywords = vec![0u8, s as u8];

		// Create profile before creating a task
		create_profile::<T>();
		let _ = PalletTask::<T>::create_task(RawOrigin::Signed(task_creator.clone()).into(), title.try_into().unwrap(), specification.try_into().unwrap(), budget, x.into(), attachments.try_into().unwrap(), keywords.try_into().unwrap(), None);
		let hash_task = PalletTask::<T>::tasks_owned(&task_creator)[0];
		let _ = PalletTask::<T>::start_task(RawOrigin::Signed(volunteer.clone()).into(), hash_task.clone());
		let _ = PalletTask::<T>::complete_task(RawOrigin::Signed(volunteer.clone()).into(), hash_task.clone());

	}: accept_task(RawOrigin::Signed(task_creator.clone()), hash_task)
		/* the code to be benchmarked */

	verify {
		/* verifying final state */
		assert_last_event::<T>(Event::<T>::TaskAccepted(task_creator, hash_task).into());
	}

	reject_task {
		/* setup initial state */
		let task_creator: T::AccountId = whitelisted_caller();
		let volunteer: T::AccountId = account("volunteer", 0, SEED);

		// Populate data fields
		let s in 1 .. u8::MAX.into(); // max bytes for specification
		let x in 1 .. 4000;
		let title = vec![0u8, s as u8];
		let specification = vec![0u8, s as u8];
		let feedback = vec![0u8, s as u8];
		let budget = <T as pallet::Config>::Currency::total_balance(&task_creator);
		let attachments = vec![0u8, s as u8];
		let keywords = vec![0u8, s as u8];
		let feedback = vec![0u8, s as u8];

		// Create profile before creating a task
		create_profile::<T>();
		let _ = PalletTask::<T>::create_task(RawOrigin::Signed(task_creator.clone()).into(), title.try_into().unwrap(), specification.try_into().unwrap(), budget, x.into(), attachments.try_into().unwrap(), keywords.try_into().unwrap(), None);
		let hash_task = PalletTask::<T>::tasks_owned(&task_creator)[0];
		let _ = PalletTask::<T>::start_task(RawOrigin::Signed(volunteer.clone()).into(), hash_task.clone());
		let _ = PalletTask::<T>::complete_task(RawOrigin::Signed(volunteer.clone()).into(), hash_task.clone());

	}: reject_task(RawOrigin::Signed(task_creator.clone()), hash_task, feedback.try_into().unwrap())
		/* the code to be benchmarked */

	verify {
		/* verifying final state */
		assert_last_event::<T>(Event::<T>::TaskRejected(task_creator, hash_task).into());
	}

	revive_task {
		/* setup initial state */
		let task_creator: T::AccountId = whitelisted_caller();
		let volunteer: T::AccountId = account("volunteer", 0, SEED);

		// Populate data fields
		let s in 1 .. u8::MAX.into(); // max bytes for specification
		let x in 1 .. 4000;
		let title = vec![0u8, s as u8];
		let specification = vec![0u8, s as u8];
		let feedback = vec![0u8, s as u8];
		let budget = <T as pallet::Config>::Currency::total_balance(&task_creator);
		let attachments = vec![0u8, s as u8];
		let keywords = vec![0u8, s as u8];
		let feedback = vec![0u8, s as u8];
		let organization = T::Hashing::hash_of(&"some dao");
		let name = vec![0u8, s as u8];
		let description = vec![0u8, s as u8];
		let vision = vec![0u8, s as u8];

	// Create profile before creating a task
	create_profile::<T>();
	let _ = PalletDao::<T>::create_organization(RawOrigin::Signed(whitelisted_caller()), name, description.into(), vision.into());
	
	let org_id = PalletDao::<T>::member_of(whitelisted_caller())[0];

	let _ = PalletTask::<T>::create_task(RawOrigin::Signed(task_creator.clone()).into(), title.try_into().unwrap(), specification.try_into().unwrap(),
		budget, x.into(), attachments.try_into().unwrap(), keywords.try_into().unwrap(), Some(org_id));

	let hash_task = PalletTask::<T>::tasks_owned(&task_creator)[0];
	let _ = PalletTask::<T>::start_task(RawOrigin::Signed(volunteer.clone()).into(), hash_task.clone());
	let _ = PalletTask::<T>::complete_task(RawOrigin::Signed(volunteer.clone()).into(), hash_task.clone());

	
	}: reject_task(RawOrigin::Signed(task_creator.clone()), hash_task, feedback.try_into().unwrap())
	//revive_expired_task(RawOrigin::Signed(task_creator.clone()), hash_task, feedback.try_into().unwrap())
		/* the code to be benchmarked */

	verify {
		/* verifying final state */
		assert_last_event::<T>(Event::<T>::TaskRejected(task_creator, hash_task).into());
	}
}

impl_benchmark_test_suite!(PalletTask, crate::mock::new_test_ext(), crate::mock::Test,);
