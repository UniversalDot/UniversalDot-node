// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_profile
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-05, STEPS: `100`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:

// ./target/release/node-template

// benchmark

// --chain

// dev

// --execution

// wasm

// --wasm-execution

// compiled

// --pallet

// pallet_profile

// --extrinsic

// *

// --steps

// 100

// --repeat

// 50

// --output

// ./pallets/profile/src/weights.rs

// --template

// .maintain/frame-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_profile.
pub trait WeightInfo {
	
	fn create_profile(x: u32, s: u32, ) -> Weight;
	
	fn update_profile(s: u32, ) -> Weight;
	
	fn remove_profile(s: u32, ) -> Weight;
	
}

/// Weights for pallet_profile using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {

	
	
	// Storage: Profile Profiles (r:1 w:1)
	
	// Storage: Profile ProfileCount (r:1 w:1)
	
	// Storage: Profile CompletedTasks (r:0 w:1)
	
	fn create_profile(x: u32, s: u32, ) -> Weight {
		(29_189_000 as Weight)
			
			// Standard Error: 0
			.saturating_add((7_000 as Weight).saturating_mul(x as Weight))
			
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(s as Weight))
			
			
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			
			
			
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			
			
	}
	
	
	// Storage: Profile Profiles (r:1 w:1)
	
	fn update_profile(_s: u32, ) -> Weight {
		(27_697_000 as Weight)
			
			
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			
			
			
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			
			
	}
	
	
	// Storage: Profile Profiles (r:1 w:1)
	
	// Storage: Profile ProfileCount (r:1 w:1)
	
	fn remove_profile(_s: u32, ) -> Weight {
		(20_793_000 as Weight)
			
			
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			
			
			
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			
			
	}
	
}

// For backwards compatibility and tests
impl WeightInfo for () {
	
	
	// Storage: Profile Profiles (r:1 w:1)
	
	// Storage: Profile ProfileCount (r:1 w:1)
	
	// Storage: Profile CompletedTasks (r:0 w:1)
	
	fn create_profile(x: u32, s: u32, ) -> Weight {
		(29_189_000 as Weight)
			
			// Standard Error: 0
			.saturating_add((7_000 as Weight).saturating_mul(x as Weight))
			
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(s as Weight))
			
			
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			
			
			
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			
			
	}
	
	
	// Storage: Profile Profiles (r:1 w:1)
	
	fn update_profile(_s: u32, ) -> Weight {
		(27_697_000 as Weight)
			
			
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			
			
			
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
			
			
	}
	
	
	// Storage: Profile Profiles (r:1 w:1)
	
	// Storage: Profile ProfileCount (r:1 w:1)
	
	fn remove_profile(_s: u32, ) -> Weight {
		(20_793_000 as Weight)
			
			
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			
			
			
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
			
			
	}
	
}
