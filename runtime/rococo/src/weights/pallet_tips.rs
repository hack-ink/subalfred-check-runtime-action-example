// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_tips`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-16, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --pallet=pallet_tips
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/rococo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_tips`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_tips::WeightInfo for WeightInfo<T> {
	// Storage: Tips Reasons (r:1 w:1)
	// Storage: Tips Tips (r:1 w:1)
	/// The range of component `r` is `[0, 16384]`.
	fn report_awesome(r: u32, ) -> Weight {
		// Minimum execution time: 31_819 nanoseconds.
		Weight::from_ref_time(37_509_130)
			// Standard Error: 32
			.saturating_add(Weight::from_ref_time(1_498).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Storage: Tips Reasons (r:0 w:1)
	fn retract_tip() -> Weight {
		// Minimum execution time: 30_639 nanoseconds.
		Weight::from_ref_time(31_311_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: Tips Reasons (r:1 w:1)
	// Storage: Tips Tips (r:0 w:1)
	/// The range of component `r` is `[0, 16384]`.
	/// The range of component `t` is `[1, 19]`.
	fn tip_new(r: u32, t: u32, ) -> Weight {
		// Minimum execution time: 24_210 nanoseconds.
		Weight::from_ref_time(22_697_785)
			// Standard Error: 6
			.saturating_add(Weight::from_ref_time(1_735).saturating_mul(r.into()))
			// Standard Error: 5_408
			.saturating_add(Weight::from_ref_time(160_994).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: Tips Tips (r:1 w:1)
	/// The range of component `t` is `[1, 19]`.
	fn tip(t: u32, ) -> Weight {
		// Minimum execution time: 16_280 nanoseconds.
		Weight::from_ref_time(16_568_900)
			// Standard Error: 1_763
			.saturating_add(Weight::from_ref_time(147_225).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Tips Reasons (r:0 w:1)
	/// The range of component `t` is `[1, 19]`.
	fn close_tip(t: u32, ) -> Weight {
		// Minimum execution time: 47_536 nanoseconds.
		Weight::from_ref_time(49_081_421)
			// Standard Error: 5_332
			.saturating_add(Weight::from_ref_time(157_457).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Storage: Tips Reasons (r:0 w:1)
	/// The range of component `t` is `[1, 19]`.
	fn slash_tip(t: u32, ) -> Weight {
		// Minimum execution time: 20_372 nanoseconds.
		Weight::from_ref_time(21_245_012)
			// Standard Error: 1_586
			.saturating_add(Weight::from_ref_time(28_121).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
