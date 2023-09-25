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

//! `NetworkBackend` implementation for `litep2p`.

use crate::{
	config::{NotificationHandshake, Params, SetConfig},
	litep2p::service::Litep2pNetworkService,
	service::traits::NetworkBackend,
	ProtocolName,
};

use sc_network_common::ExHashT;
use sp_runtime::traits::Block as BlockT;

mod peerset;
mod service;

pub struct Litep2pNetworkBackend {}

impl Litep2pNetworkBackend {}

#[async_trait::async_trait]
impl<B: BlockT + 'static, H: ExHashT> NetworkBackend<B, H> for Litep2pNetworkBackend {
	type NotificationProtocolConfig = ();
	type NetworkService<Block, Hash> = Litep2pNetworkService;

	/// Create new `NetworkBackend`.
	async fn new(_params: Params<B>) -> Self
	where
		Self: Sized,
	{
		todo!();
	}

	/// Get handle to `NetworkService` of the `NetworkBackend`.
	fn network_service(&self) -> Self::NetworkService<B, H> {
		todo!();
	}

	/// Create notification protocol configuration for `protocol`.
	fn notification_config(
		_protocol_name: ProtocolName,
		_fallback_names: Vec<ProtocolName>,
		_max_notification_size: u64,
		_handshake: Option<NotificationHandshake>,
		_set_config: SetConfig,
	) -> Self::NotificationProtocolConfig {
		todo!();
	}

	/// Start [`NetworkBackend`] event loop.
	async fn run(mut self) {
		todo!();
	}
}
