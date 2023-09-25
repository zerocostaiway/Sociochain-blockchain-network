// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use multihash::{Code, Error, Multihash};
use rand::Rng;

use std::{fmt, hash::Hash, str::FromStr};

/// Public keys with byte-lengths smaller than `MAX_INLINE_KEY_LENGTH` will be
/// automatically used as the peer id using an identity multihash.
const MAX_INLINE_KEY_LENGTH: usize = 42;

/// Identifier of a peer of the network.
///
/// The data is a CIDv0 compatible multihash of the protobuf encoded public key of the peer
/// as specified in [specs/peer-ids](https://github.com/libp2p/specs/blob/master/peer-ids/peer-ids.md).
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PeerId {
	multihash: Multihash,
}

impl fmt::Debug for PeerId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple("PeerId").field(&self.to_base58()).finish()
	}
}

impl fmt::Display for PeerId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.to_base58().fmt(f)
	}
}

impl PeerId {
	/// Generate random peer ID.
	pub fn random() -> PeerId {
		let peer = rand::thread_rng().gen::<[u8; 32]>();
		PeerId {
			multihash: Multihash::wrap(0x0, &peer).expect("The digest size is never too large"),
		}
	}

	/// Tries to turn a `Multihash` into a `PeerId`.
	///
	/// If the multihash does not use a valid hashing algorithm for peer IDs,
	/// or the hash value does not satisfy the constraints for a hashed
	/// peer ID, it is returned as an `Err`.
	pub fn from_multihash(multihash: Multihash) -> Result<PeerId, Multihash> {
		match Code::try_from(multihash.code()) {
			Ok(Code::Sha2_256) => Ok(PeerId { multihash }),
			Ok(Code::Identity) if multihash.digest().len() <= MAX_INLINE_KEY_LENGTH =>
				Ok(PeerId { multihash }),
			_ => Err(multihash),
		}
	}

	/// Parses a `PeerId` from bytes.
	pub fn from_bytes(data: &[u8]) -> Result<PeerId, Error> {
		PeerId::from_multihash(Multihash::from_bytes(data)?)
			.map_err(|mh| Error::UnsupportedCode(mh.code()))
	}

	/// Returns a raw bytes representation of this `PeerId`.
	pub fn to_bytes(&self) -> Vec<u8> {
		self.multihash.to_bytes()
	}

	/// Returns a base-58 encoded string of this `PeerId`.
	pub fn to_base58(&self) -> String {
		bs58::encode(self.to_bytes()).into_string()
	}
}

impl From<PeerId> for Multihash {
	fn from(peer: PeerId) -> Self {
		peer.multihash
	}
}

impl From<libp2p_identity::PeerId> for PeerId {
	fn from(peer: libp2p_identity::PeerId) -> Self {
		PeerId { multihash: Multihash::from_bytes(&peer.to_bytes()).expect("to succeed") }
	}
}

impl From<PeerId> for libp2p_identity::PeerId {
	fn from(peer: PeerId) -> Self {
		libp2p_identity::PeerId::from_bytes(&peer.to_bytes()).expect("to succeed")
	}
}

impl From<&libp2p_identity::PeerId> for PeerId {
	fn from(peer: &libp2p_identity::PeerId) -> Self {
		PeerId { multihash: Multihash::from_bytes(&peer.to_bytes()).expect("to succeed") }
	}
}

impl From<&PeerId> for libp2p_identity::PeerId {
	fn from(peer: &PeerId) -> Self {
		libp2p_identity::PeerId::from_bytes(&peer.to_bytes()).expect("to succeed")
	}
}

impl From<litep2p::PeerId> for PeerId {
	fn from(peer: litep2p::PeerId) -> Self {
		PeerId { multihash: Multihash::from_bytes(&peer.to_bytes()).expect("to succeed") }
	}
}

impl From<PeerId> for litep2p::PeerId {
	fn from(peer: PeerId) -> Self {
		litep2p::PeerId::from_bytes(&peer.to_bytes()).expect("to succeed")
	}
}

impl From<&litep2p::PeerId> for PeerId {
	fn from(peer: &litep2p::PeerId) -> Self {
		PeerId { multihash: Multihash::from_bytes(&peer.to_bytes()).expect("to succeed") }
	}
}

impl From<&PeerId> for litep2p::PeerId {
	fn from(peer: &PeerId) -> Self {
		litep2p::PeerId::from_bytes(&peer.to_bytes()).expect("to succeed")
	}
}

/// Error when parsing a [`PeerId`] from string or bytes.
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
	#[error("base-58 decode error: {0}")]
	B58(#[from] bs58::decode::Error),
	#[error("unsupported multihash code '{0}'")]
	UnsupportedCode(u64),
	#[error("invalid multihash")]
	InvalidMultihash(#[from] multihash::Error),
}

impl FromStr for PeerId {
	type Err = ParseError;

	#[inline]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let bytes = bs58::decode(s).into_vec()?;
		let peer_id = PeerId::from_bytes(&bytes)?;

		Ok(peer_id)
	}
}
