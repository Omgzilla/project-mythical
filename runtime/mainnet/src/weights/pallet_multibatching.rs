
//! Autogenerated weights for `pallet_multibatching`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2024-12-18, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `blockdeep-ref-hw`, CPU: `AMD EPYC 7232P 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("mainnet-local-v")`, DB CACHE: 1024

// Executed Command:
// ./target/release/mythos-node
// benchmark
// pallet
// --chain
// mainnet-local-v
// --pallet
// pallet_multibatching
// --extrinsic
// *
// --wasm-execution
// compiled
// --steps
// 50
// --repeat
// 20
// --output
// ./runtime/mainnet/src/weights/pallet_multibatching.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_multibatching`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multibatching::WeightInfo for WeightInfo<T> {
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Multibatching::Applied` (r:1 w:1)
	/// Proof: `Multibatching::Applied` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 128]`.
	/// The range of component `s` is `[1, 128]`.
	fn batch(c: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `84`
		//  Estimated: `3497`
		// Minimum execution time: 477_370_000 picoseconds.
		Weight::from_parts(482_930_000, 0)
			.saturating_add(Weight::from_parts(0, 3497))
			// Standard Error: 201_698
			.saturating_add(Weight::from_parts(6_096_073, 0).saturating_mul(c.into()))
			// Standard Error: 201_698
			.saturating_add(Weight::from_parts(57_158_490, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Multibatching::Applied` (r:1 w:1)
	/// Proof: `Multibatching::Applied` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 128]`.
	/// The range of component `s` is `[1, 128]`.
	fn batch_v2(c: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `84`
		//  Estimated: `3497`
		// Minimum execution time: 479_130_000 picoseconds.
		Weight::from_parts(481_200_000, 0)
			.saturating_add(Weight::from_parts(0, 3497))
			// Standard Error: 201_474
			.saturating_add(Weight::from_parts(6_095_922, 0).saturating_mul(c.into()))
			// Standard Error: 201_474
			.saturating_add(Weight::from_parts(57_276_559, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
