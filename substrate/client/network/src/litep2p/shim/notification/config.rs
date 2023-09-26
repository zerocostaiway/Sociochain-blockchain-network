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

//! `litep2p` notification protocol configuration.

use crate::{
	config::{MultiaddrWithPeerId, NonReservedPeerMode, NotificationHandshake, SetConfig},
	service::traits::NotificationConfig,
	NotificationService, ProtocolName,
};

#[derive(Debug)]
/// Configuration for the notification protocol.
pub struct NotificationProtocolConfig {
	/// Name of the notifications protocols of this set. A substream on this set will be
	/// considered established once this protocol is open.
	protocol_name: ProtocolName,

	/// If the remote reports that it doesn't support the protocol indicated in the
	/// `notifications_protocol` field, then each of these fallback names will be tried one by
	/// one.
	fallback_names: Vec<ProtocolName>,

	/// Handshake of the protocol
	handshake: Option<NotificationHandshake>,

	/// Maximum allowed size of single notifications.
	max_notification_size: u64,

	/// Base configuration.
	set_config: SetConfig,
	// TODO: add litep2p stuff
}

impl NotificationProtocolConfig {
	/// Creates a new [`NonDefaultSetConfig`]. Zero slots and accepts only reserved nodes.
	/// Also returns an object which allows the protocol to communicate with `Notifications`.
	pub fn new(
		protocol_name: ProtocolName,
		fallback_names: Vec<ProtocolName>,
		max_notification_size: u64,
		handshake: Option<NotificationHandshake>,
		set_config: SetConfig,
	) -> (Self, Box<dyn NotificationService>) {
		// TODO: add litep2p stuff

		(
			Self { protocol_name, max_notification_size, fallback_names, handshake, set_config },
			todo!(),
		)
	}

	/// Get reference to protocol name.
	pub fn protocol_name(&self) -> &ProtocolName {
		&self.protocol_name
	}

	/// Get reference to fallback protocol names.
	pub fn fallback_names(&self) -> impl Iterator<Item = &ProtocolName> {
		self.fallback_names.iter()
	}

	/// Get reference to handshake.
	pub fn handshake(&self) -> &Option<NotificationHandshake> {
		&self.handshake
	}

	/// Get maximum notification size.
	pub fn max_notification_size(&self) -> u64 {
		self.max_notification_size
	}

	/// Get reference to `SetConfig`.
	pub fn set_config(&self) -> &SetConfig {
		&self.set_config
	}

	/// Modifies the configuration to allow non-reserved nodes.
	pub fn allow_non_reserved(&mut self, in_peers: u32, out_peers: u32) {
		self.set_config.in_peers = in_peers;
		self.set_config.out_peers = out_peers;
		self.set_config.non_reserved_mode = NonReservedPeerMode::Accept;
	}

	/// Add a node to the list of reserved nodes.
	pub fn add_reserved(&mut self, peer: MultiaddrWithPeerId) {
		self.set_config.reserved_nodes.push(peer);
	}

	/// Add a list of protocol names used for backward compatibility.
	///
	/// See the explanations in [`NonDefaultSetConfig::fallback_names`].
	pub fn add_fallback_names(&mut self, fallback_names: Vec<ProtocolName>) {
		self.fallback_names.extend(fallback_names);
	}
}

impl NotificationConfig for NotificationProtocolConfig {
	fn set_config(&self) -> &SetConfig {
		&self.set_config
	}

	fn allow_non_reserved(&mut self, in_peers: u32, out_peers: u32) {
		self.set_config.in_peers = in_peers;
		self.set_config.out_peers = out_peers;
		self.set_config.non_reserved_mode = NonReservedPeerMode::Accept;
	}

	/// Get reference to protocol name.
	fn protocol_name(&self) -> &ProtocolName {
		&self.protocol_name
	}
}
