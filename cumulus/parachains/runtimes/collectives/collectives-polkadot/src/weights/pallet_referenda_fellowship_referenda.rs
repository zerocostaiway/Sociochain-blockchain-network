// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_referenda`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-11, STEPS: `2`, REPEAT: `2`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `cob`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("collectives-polkadot-dev")`, DB CACHE: 1024

// Executed Command:
// target/release/polkadot-parachain
// benchmark
// pallet
// --chain=collectives-polkadot-dev
// --wasm-execution=compiled
// --pallet=pallet_referenda
// --extrinsic=*
// --steps=2
// --repeat=2
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/collectives/collectives-polkadot/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_referenda`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_referenda::WeightInfo for WeightInfo<T> {
	/// Storage: `FellowshipCollective::Members` (r:1 w:0)
	/// Proof: `FellowshipCollective::Members` (`max_values`: None, `max_size`: Some(42), added: 2517, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipReferenda::ReferendumCount` (r:1 w:1)
	/// Proof: `FellowshipReferenda::ReferendumCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:0 w:1)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	fn submit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `389`
		//  Estimated: `159279`
		// Minimum execution time: 34_000_000 picoseconds.
		Weight::from_parts(36_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn place_decision_deposit_preparing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `400`
		//  Estimated: `317568`
		// Minimum execution time: 64_000_000 picoseconds.
		Weight::from_parts(67_000_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipReferenda::DecidingCount` (r:1 w:0)
	/// Proof: `FellowshipReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `FellowshipReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(812), added: 3287, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn place_decision_deposit_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2038`
		//  Estimated: `159279`
		// Minimum execution time: 99_000_000 picoseconds.
		Weight::from_parts(109_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipReferenda::DecidingCount` (r:1 w:0)
	/// Proof: `FellowshipReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `FellowshipReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(812), added: 3287, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn place_decision_deposit_not_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2079`
		//  Estimated: `159279`
		// Minimum execution time: 101_000_000 picoseconds.
		Weight::from_parts(111_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipReferenda::DecidingCount` (r:1 w:1)
	/// Proof: `FellowshipReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::MemberCount` (r:1 w:0)
	/// Proof: `FellowshipCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn place_decision_deposit_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `836`
		//  Estimated: `317568`
		// Minimum execution time: 135_000_000 picoseconds.
		Weight::from_parts(153_000_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipReferenda::DecidingCount` (r:1 w:1)
	/// Proof: `FellowshipReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::MemberCount` (r:1 w:0)
	/// Proof: `FellowshipCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn place_decision_deposit_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `735`
		//  Estimated: `317568`
		// Minimum execution time: 78_000_000 picoseconds.
		Weight::from_parts(82_000_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	fn refund_decision_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `351`
		//  Estimated: `4365`
		// Minimum execution time: 38_000_000 picoseconds.
		Weight::from_parts(39_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	fn refund_submission_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `201`
		//  Estimated: `4365`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(19_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn cancel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `345`
		//  Estimated: `317568`
		// Minimum execution time: 45_000_000 picoseconds.
		Weight::from_parts(46_000_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	/// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `FellowshipReferenda::MetadataOf` (r:1 w:0)
	/// Proof: `FellowshipReferenda::MetadataOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn kill() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `587`
		//  Estimated: `317568`
		// Minimum execution time: 185_000_000 picoseconds.
		Weight::from_parts(196_000_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `FellowshipReferenda::TrackQueue` (r:1 w:0)
	/// Proof: `FellowshipReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(812), added: 3287, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipReferenda::DecidingCount` (r:1 w:1)
	/// Proof: `FellowshipReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	fn one_fewer_deciding_queue_empty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `174`
		//  Estimated: `4277`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(15_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4277))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `FellowshipReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `FellowshipReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(812), added: 3287, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::MemberCount` (r:1 w:0)
	/// Proof: `FellowshipCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn one_fewer_deciding_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2452`
		//  Estimated: `159279`
		// Minimum execution time: 82_000_000 picoseconds.
		Weight::from_parts(90_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `FellowshipReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `FellowshipReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(812), added: 3287, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::MemberCount` (r:1 w:0)
	/// Proof: `FellowshipCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn one_fewer_deciding_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2452`
		//  Estimated: `159279`
		// Minimum execution time: 91_000_000 picoseconds.
		Weight::from_parts(99_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `FellowshipReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(812), added: 3287, mode: `MaxEncodedLen`)
	fn nudge_referendum_requeued_insertion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1841`
		//  Estimated: `4365`
		// Minimum execution time: 41_000_000 picoseconds.
		Weight::from_parts(44_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `FellowshipReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(812), added: 3287, mode: `MaxEncodedLen`)
	fn nudge_referendum_requeued_slide() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1808`
		//  Estimated: `4365`
		// Minimum execution time: 46_000_000 picoseconds.
		Weight::from_parts(55_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipReferenda::DecidingCount` (r:1 w:0)
	/// Proof: `FellowshipReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `FellowshipReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(812), added: 3287, mode: `MaxEncodedLen`)
	fn nudge_referendum_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1824`
		//  Estimated: `4365`
		// Minimum execution time: 49_000_000 picoseconds.
		Weight::from_parts(53_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipReferenda::DecidingCount` (r:1 w:0)
	/// Proof: `FellowshipReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `FellowshipReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(812), added: 3287, mode: `MaxEncodedLen`)
	fn nudge_referendum_not_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1865`
		//  Estimated: `4365`
		// Minimum execution time: 51_000_000 picoseconds.
		Weight::from_parts(54_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_no_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `297`
		//  Estimated: `159279`
		// Minimum execution time: 28_000_000 picoseconds.
		Weight::from_parts(30_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_preparing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `345`
		//  Estimated: `159279`
		// Minimum execution time: 28_000_000 picoseconds.
		Weight::from_parts(29_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	fn nudge_referendum_timed_out() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `242`
		//  Estimated: `4365`
		// Minimum execution time: 20_000_000 picoseconds.
		Weight::from_parts(21_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipReferenda::DecidingCount` (r:1 w:1)
	/// Proof: `FellowshipReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::MemberCount` (r:1 w:0)
	/// Proof: `FellowshipCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_begin_deciding_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `680`
		//  Estimated: `159279`
		// Minimum execution time: 42_000_000 picoseconds.
		Weight::from_parts(47_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipReferenda::DecidingCount` (r:1 w:1)
	/// Proof: `FellowshipReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::MemberCount` (r:1 w:0)
	/// Proof: `FellowshipCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_begin_deciding_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `781`
		//  Estimated: `159279`
		// Minimum execution time: 90_000_000 picoseconds.
		Weight::from_parts(95_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::MemberCount` (r:1 w:0)
	/// Proof: `FellowshipCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_begin_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `834`
		//  Estimated: `159279`
		// Minimum execution time: 84_000_000 picoseconds.
		Weight::from_parts(93_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::MemberCount` (r:1 w:0)
	/// Proof: `FellowshipCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_end_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `817`
		//  Estimated: `159279`
		// Minimum execution time: 88_000_000 picoseconds.
		Weight::from_parts(98_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::MemberCount` (r:1 w:0)
	/// Proof: `FellowshipCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_continue_not_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `834`
		//  Estimated: `159279`
		// Minimum execution time: 81_000_000 picoseconds.
		Weight::from_parts(93_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::MemberCount` (r:1 w:0)
	/// Proof: `FellowshipCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_continue_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `838`
		//  Estimated: `159279`
		// Minimum execution time: 74_000_000 picoseconds.
		Weight::from_parts(77_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::MemberCount` (r:1 w:0)
	/// Proof: `FellowshipCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Lookup` (r:1 w:1)
	/// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	fn nudge_referendum_approved() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `838`
		//  Estimated: `317568`
		// Minimum execution time: 105_000_000 picoseconds.
		Weight::from_parts(123_000_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipCollective::MemberCount` (r:1 w:0)
	/// Proof: `FellowshipCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_rejected() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `834`
		//  Estimated: `159279`
		// Minimum execution time: 90_000_000 picoseconds.
		Weight::from_parts(100_000_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipReferenda::MetadataOf` (r:0 w:1)
	/// Proof: `FellowshipReferenda::MetadataOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn set_some_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `453`
		//  Estimated: `4365`
		// Minimum execution time: 24_000_000 picoseconds.
		Weight::from_parts(24_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `FellowshipReferenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `FellowshipReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `FellowshipReferenda::MetadataOf` (r:1 w:1)
	/// Proof: `FellowshipReferenda::MetadataOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `319`
		//  Estimated: `4365`
		// Minimum execution time: 21_000_000 picoseconds.
		Weight::from_parts(23_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
