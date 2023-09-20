// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
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

//! # Storage Migrations Example Pallet
//!
//! An example pallet explaining why storage migrations are necessary and demonstrating
//! best-practices for writing them.
//!
//! It is intended to be a resource for guiding a developer from knowing nothing about storage
//! migrations to being able to write safe, well tested, versioned migrations for their own pallets.
//!
//! ## Prerequisites
//!
//! Before writing pallet storage migrations, you should be familiar with:
//! - [`Runtime upgrades`](https://docs.substrate.io/maintain/runtime-upgrades/)
//! - The [`GetStorageVersion`](frame_support::traits::GetStorageVersion) trait, and the difference
//!   between current and on-chain [`StorageVersion`]s

//! ## How to read these docs
//! - Run `cargo doc --features try-runtime --package pallet-example-storage-migrations --open`
//! to view the documentation in your browser.
//! - Read the relevant source code as your read the docs.
//!
//! ## Pallet Overview
//!
//! This example pallet contains a single storage item [`Value`](pallet::Value), which may be set by
//! any signed origin by calling the [`set_value`](crate::Call::set_value) extrinsic.
//!
//! For the purposes of this exercise, we imagine that in [`StorageVersion`] V0 of this pallet
//! [`Value`](pallet::Value) is a `u32`, and this what is currently stored on-chain.
#![doc = docify::embed!("src/lib.rs", Value__V0)]
//!
//!
//! In [`StorageVersion`] V1 of the pallet a new struct [`CurrentAndPreviousValue`] is introduced:
#![doc = docify::embed!("src/lib.rs", CurrentAndPreviousValue)]
//! and [`Value`](pallet::Value) is updated to store this new struct instead of a `u32`:
#![doc = docify::embed!("src/lib.rs", Value)]
//!
//! In StorageVersion V1 of the pallet when [`set_value`](crate::Call::set_value) is called, the
//! new value is stored in the `current` field of [`CurrentAndPreviousValue`], and the previous
//! value (if it exists) is stored in the `previous` field.
#![doc = docify::embed!("src/lib.rs", pallet_calls)]
//!
//! ## Why a migration is necessary
//!
//! Without a migration, there will be a discrepancy between the on-chain storage for [`Value`] (in
//! V0 it is a `u32`) and the current storage for [`Value`] (in V1 it was changed to a
//! [`CurrentAndPreviousValue`] struct).
//!
//! The on-chain storage for [`Value`] would be a `u32` but the runtime would try to read it as a
//! [`CurrentAndPreviousValue`]. This would result in unacceptable undefined behavior.
//!
//! ## Adding a migration module
//!
//! Containing a pallets migrations in a seperate module is strongly recommended.
//!
//! Here's how the migration module is defined for this pallet:
//!
//! ```text
//! substrate/frame/examples/storage-migrations/src/
//! ├── lib.rs       <-- pallet definition
//! ├── Cargo.toml   <-- pallet manifest
//! └── migrations/
//!    ├── mod.rs    <-- migrations module definition
//!    └── v1.rs     <-- migration logic for the V0 to V1 transition
//! ```
//!
//! This structure allows keeping migration logic separate from the pallet logic and
//! easily adding new migrations in the future.
//!
//! ## Writing the Migration
//!
//! All code related to the migration can be found under
//! [`v1.rs`](migrations::v1).
//!
//! See the migration source code for detailed comments.
//!
//! To keep the migration logic organised, it is split across additional modules:
//!
//! ### `mod old`
//!
//! Here we define a [`storage_alias`](frame_support::storage_alias) for the old [`Value`]
//! format.
//!
//! This allows reading the old value from storage during the migration.
//!
//! ### `mod version_unchecked`
//!
//! Here we define our raw migration logic,
//! [`MigrateV0ToV1`](crate::migrations::v1::version_unchecked::MigrateV0ToV1) which implements the
//! [`OnRuntimeUpgrade`](frame_support::traits::OnRuntimeUpgrade) trait.
//!
//! Importantly, it is kept in a private module so that it cannot be accidentally used in a runtime.
//!
//! #### Standalone Struct or Pallet Hook?
//!
//! Note that the storage migration logic is attached to a standalone struct implementing
//! [`OnRuntimeUpgrade`](frame_support::traits::OnRuntimeUpgrade), rather than implementing the
//! [`Hooks::on_runtime_upgrade`](frame_support::traits::Hooks::on_runtime_upgrade) hook directly on
//! the pallet. The pallet hook is better suited for executing other types of logic that needs to
//! execute on runtime upgrade, but not so much storage migrations.
//!
//! ### `pub mod versioned`
//!
//! Here,
//! [`version_unchecked::MigrateV0ToV1`](crate::migrations::v1::version_unchecked::MigrateV0ToV1) is
//! wrapped in a [`VersionedMigration`](frame_support::migrations::VersionedMigration) to define
//! [`versioned::MigrateV0ToV1`](crate::migrations::v1::versioned::MigrateV0ToV1), which may be used
//! in runtimes.
//!
//! Using
//! [`VersionedMigration`](frame_support::migrations::VersionedMigration) ensures that
//! - The migration only runs once when the on-chain storage version is `0`
//! - The on-chain storage version is updated to `1` after the migration executes
//! - Reads and writes from checking and setting the on-chain storage version are accounted for in
//!   the final [`Weight`](frame_support::weights::Weight)
//!
//! This is the only public module exported from `v1`.
//!
//! ### `mod test`
//!
//! Here basic unit tests are defined for the migration.
//!
//! When writing migration tests, don't forget to check:
//! - `on_runtime_upgrade` returns the expected weight
//! - `post_upgrade` succeeds when given the bytes returned by `pre_upgrade`
//! - Pallet storage is in the expected state after the migration
//!
//! ## Scheduling the Migration to Run Next Runtime Upgrade
//!
//! Almost done! The last step is to schedule the migration to run next runtime upgrade passing it
//! as a generic parameter to your [`Executive`](frame_executive) pallet:
//!
//! ```ignore
//! // Tuple of migrations (structs that implement `OnRuntimeUpgrade`)
//! type Migrations = (
//! 	pallet_example_storage_migration::migrations::v1::versioned::MigrateV0ToV1
//! 	// ...more migrations here
//! );
//! pub type Executive = frame_executive::Executive<
//! 	Runtime,
//! 	Block,
//! 	frame_system::ChainContext<Runtime>,
//! 	Runtime,
//! 	AllPalletsWithSystem,
//! 	Migrations, // <-- pass your migrations to Executive here
//! >;
//! ```
//!
//! ## Ensuring Migraiton Safety
//!
//! "My migration unit tests pass, so it should be safe to deploy right?"
//!
//! No! Unit tests execute the migration in a very simple test environment, and cannot account
//! for the complexities of a real runtime or real on-chain state.
//!
//! Prior to deploying migrations, it is CRITICAL to perform additional checks to ensure that when
//! run in our real runtime they will not brick the chain due to:
//! - Panicing
//! - Touching too many storage keys and resulting in an excessively large PoV
//! - Taking too long to execute
//!
//! The [`try-runtime-cli`](https://github.com/paritytech/try-runtime-cli) tool has a sub-command
//! [`on-runtime-upgrade`](https://paritytech.github.io/try-runtime-cli/try_runtime_core/commands/enum.Action.html#variant.OnRuntimeUpgrade)
//! which is designed to help with exactly this.
//!
//! Developers MUST run this command before deploying migrations to ensure they will not
//! inadvertently result in a bricked chain.
//!
//! ### Note on the Manipulability of PoV Size and Execution Time
//!
//! While [`try-runtime-cli`](https://github.com/paritytech/try-runtime-cli) can help ensure with
//! very high certianty that a migration will succeed given **existing** on-chain state, it cannot
//! prevent a malicious actor from manipulating state in a way that will cause the migration to take
//! longer or produce a PoV much larger than previously measured.
//!
//! Therefore, it is important to write migrations in such a way that the execution time or PoV size
//! it adds to the block cannot be easily manipulated. e.g., in your migration, do not iterate over
//! storage that can quickly or cheaply be bloated.
//!
//! ### Note on Multi-Block Migrations
//!
//! For large migrations that cannot be safely executed in a single block, a feature for writing
//! simple and safe [multi-block migrations](https://github.com/paritytech/polkadot-sdk/issues/198)
//! feature is [under active development](https://github.com/paritytech/substrate/pull/14275) and
//! planned for release before the end of 2023.
//!
//! ### Note on Other Tools
//!
//! [`Chopsticks`](https://github.com/AcalaNetwork/chopsticks) is another tool in the Substrate
//! ecosystem which developers may find useful to use in addition to `try-runtime-cli` when testing
//! their migrations.

// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
// allow non-camel-case names for storage version V0 value
#![allow(non_camel_case_types)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// Export migrations so they may be used in the runtime.
pub mod migrations;
#[doc(hidden)]
mod mock;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::traits::StorageVersion;
use sp_runtime::RuntimeDebug;

/// Example struct holding the most recently set [`u32`] and the
/// second most recently set [`u32`] (if one existed).
#[docify::export]
#[derive(
	Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, scale_info::TypeInfo, MaxEncodedLen,
)]
pub struct CurrentAndPreviousValue {
	/// The most recently set value.
	pub current: u32,
	/// The previous value, if one existed.
	pub previous: Option<u32>,
}

// Pallet for demonstrating storage migrations.
#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Define the current [`StorageVersion`] of the pallet.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	/// [`StorageVersion`] V0 of [`Value`].
	///
	/// Not used anywhere anymore. Here for demonstration purposes only.
	#[docify::export]
	#[pallet::storage]
	pub type Value__V0<T: Config> = StorageValue<_, u32>;

	/// [`StorageVersion`] V1 of [`Value`].
	///
	/// Currently used.
	#[docify::export]
	#[pallet::storage]
	pub type Value<T: Config> = StorageValue<_, CurrentAndPreviousValue>;

	#[docify::export(pallet_calls)]
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		pub fn set_value(origin: OriginFor<T>, value: u32) -> DispatchResult {
			ensure_signed(origin)?;

			let previous = Value::<T>::get().map(|v| v.current);
			let new_struct = CurrentAndPreviousValue { current: value, previous };
			<Value<T>>::put(new_struct);

			Ok(())
		}
	}
}