
//! Autogenerated weights for `orml_tokens`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-16, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/egg-collator
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --log=warn
// --pallet=orml-tokens
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --output=./runtime/src/weights/orml_tokens.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `orml_tokens`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> orml_tokens::WeightInfo for WeightInfo<T> {
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn transfer() -> Weight {
		(41_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn transfer_all() -> Weight {
		(44_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn transfer_keep_alive() -> Weight {
		(37_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Storage: System Account (r:2 w:1)
	fn force_transfer() -> Weight {
		(41_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn set_balance() -> Weight {
		(29_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}
