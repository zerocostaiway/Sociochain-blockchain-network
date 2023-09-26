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

//! `Peerset` implementation for `litep2p`.

use crate::{peer_store::PeerStoreHandle, ProtocolName};

use futures::{Stream, StreamExt};

use litep2p::protocol::notification::ValidationResult;
use sc_network_types::PeerId;
use sc_utils::mpsc::{tracing_unbounded, TracingUnboundedReceiver, TracingUnboundedSender};

use std::{
	collections::HashSet,
	pin::Pin,
	task::{Context, Poll},
};

/// Logging target for the file.
const LOG_TARGET: &str = "sub-libp2p";

/// Commands emitted by other subsystems of the blockchain to this [`Peerset`].
#[derive(Debug)]
pub enum PeersetCommand {
	/// Set current reserved peer set.
	///
	/// This command removes all reserved peers that are not in `peers`.
	SetReservedPeers {
		/// New seserved peer set.
		peers: HashSet<PeerId>,
	},

	/// Add one or more reserved peers.
	///
	/// This command doesn't remove any reserved peers but only add new peers.
	AddReservePeers {
		/// Reserved peers to add.
		peers: HashSet<PeerId>,
	},

	/// Remove reserved peers.
	RemoveReservedPeers {
		/// Reserved peers to remove.
		peers: HashSet<PeerId>,
	},

	/// Set reserved-only mode to true/false.
	SetReservedOnly {
		/// Should the protocol only accept/establish connections to reserved peers.
		reserved_only: bool,
	},
}

/// Commands emitted by [`Peerset`] to the notification protocol.
pub enum PeersetNotificationCommand {
	/// Open substream to peer.
	OpenSubstream {
		/// Peer Id.
		peer: PeerId,
	},

	/// Close substream to peer.
	CloseSubstream {
		/// Peer ID.
		peer: PeerId,
	},
}

/// `Peerset` implementation.
///
/// `Peerset` allows other subsystems of the blockchain to modify the connection state
/// of the notification protocol by adding and removing reserved peers.
///
/// `Peerset` is also responsible for maintaining the desired amount of peers the protocol is
/// connected to by establishing outbound connections and accepting/rejecting inbound connections.
#[derive(Debug)]
pub struct Peerset {
	/// Protocol name.
	protocol: ProtocolName,

	/// RX channel for receiving commands.
	cmd_rx: TracingUnboundedReceiver<PeersetCommand>,

	/// Maximum number of outbound peers.
	max_out: usize,

	/// Current number of outbound peers.
	num_out: usize,

	/// Maximum number of inbound peers.
	max_in: usize,

	/// Current number of inbound peers.
	num_in: usize,

	/// Current reserved peer set.
	reserved_peers: HashSet<PeerId>,

	/// Handle to `Peerstore`.
	peer_store_handle: PeerStoreHandle,
}

impl Peerset {
	/// Create new [`Peerset`].
	pub fn new(
		protocol: ProtocolName,
		max_out: usize,
		max_in: usize,
		reserved_peers: HashSet<PeerId>,
		peer_store_handle: PeerStoreHandle,
	) -> (Self, TracingUnboundedSender<PeersetCommand>) {
		let (cmd_tx, cmd_rx) = tracing_unbounded("mpsc-peerset-protocol", 100_000);

		(
			Self {
				protocol,
				max_out,
				num_out: 0usize,
				max_in,
				num_in: 0usize,
				reserved_peers,
				cmd_rx,
				peer_store_handle,
			},
			cmd_tx,
		)
	}

	/// Report to [`Peerset`] that a substream was opened.
	pub fn report_substream_opened(&mut self, peer: PeerId) {}

	/// Report to [`Peerset`] that a substream was closed.
	pub fn report_substream_closed(&mut self, peer: PeerId) {}

	/// Report to [`Peerset`] that an inbound substream was opened and that it should validate it.
	pub fn report_inbound_substream(&mut self, peer: PeerId) -> ValidationResult {
		ValidationResult::Reject
	}

	/// Report to [`Peerset`] that an inbound substream was opened and that it should validate it.
	pub fn report_substream_open_failure(&mut self, peer: PeerId) {}
}

impl Stream for Peerset {
	type Item = PeersetNotificationCommand;

	fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
		Poll::Ready(None)
	}
}
