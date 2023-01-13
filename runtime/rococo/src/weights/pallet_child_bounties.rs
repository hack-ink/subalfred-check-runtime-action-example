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
//! Autogenerated weights for `pallet_child_bounties`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --pallet=pallet_child_bounties
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

/// Weight functions for `pallet_child_bounties`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_child_bounties::WeightInfo for WeightInfo<T> {
	/// The range of component `d` is `[0, 16384]`.
	fn add_child_bounty(_d: u32, ) -> Weight {
		// Minimum execution time: 0 nanoseconds.
		Weight::from_ref_time(0)
	}
	fn propose_curator() -> Weight {
		// Minimum execution time: 0 nanoseconds.
		Weight::from_ref_time(0)
	}
	fn accept_curator() -> Weight {
		// Minimum execution time: 0 nanoseconds.
		Weight::from_ref_time(0)
	}
	fn unassign_curator() -> Weight {
		// Minimum execution time: 0 nanoseconds.
		Weight::from_ref_time(0)
	}
	fn award_child_bounty() -> Weight {
		// Minimum execution time: 0 nanoseconds.
		Weight::from_ref_time(0)
	}
	fn claim_child_bounty() -> Weight {
		// Minimum execution time: 0 nanoseconds.
		Weight::from_ref_time(0)
	}
	fn close_child_bounty_added() -> Weight {
		// Minimum execution time: 0 nanoseconds.
		Weight::from_ref_time(0)
	}
	fn close_child_bounty_active() -> Weight {
		// Minimum execution time: 0 nanoseconds.
		Weight::from_ref_time(0)
	}
}
