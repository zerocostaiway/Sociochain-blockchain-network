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

//! `NetworkService` implementation for `litep2p`.

use crate::{
	config::MultiaddrWithPeerId,
	peer_store::{PeerStoreHandle, PeerStoreProvider},
	service::traits::NetworkService,
	Event, IfDisconnected, NetworkDHTProvider, NetworkEventStream, NetworkPeers, NetworkRequest,
	NetworkSigner, NetworkStateInfo, NetworkStatus, NetworkStatusProvider, ProtocolName,
	RequestFailure, Signature,
};

use codec::DecodeAll;
use futures::{channel::oneshot, stream::BoxStream};
use libp2p::{identity::SigningError, kad::record::Key as KademliaKey, Multiaddr};
use litep2p::crypto::ed25519::Keypair;

use sc_network_common::{
	role::{ObservedRole, Roles},
	types::ReputationChange,
};
use sc_network_types::PeerId;
use sc_utils::mpsc::TracingUnboundedSender;

use std::collections::HashSet;

/// Logging target for the file.
const LOG_TARGET: &str = "sub-libp2p";

/// Commands sent by [`Litep2pNetworkService`] to
/// [`Litep2pNetworkBackend`](super::Litep2pNetworkBackend).
#[derive(Debug)]
pub enum NetworkServiceCommand {
	/// Get value from DHT.
	GetValue(KademliaKey),

	/// Put value to DHT.
	PutValue(KademliaKey, Vec<u8>),

	/// Query network status.
	Status(oneshot::Sender<NetworkStatus>),
}

/// `NetworkService` implementation for `litep2p`.
#[derive(Debug, Clone)]
pub struct Litep2pNetworkService {
	/// Local peer ID.
	local_peer_id: litep2p::PeerId,

	/// The `KeyPair` that defines the `PeerId` of the local node.
	keypair: Keypair,

	/// TX channel for sending commands to [`Litep2pNetworkBackend`](super::Litep2pNetworkBackend).
	cmd_tx: TracingUnboundedSender<NetworkServiceCommand>,

	/// Handle to `PeerStore`.
	peer_store_handle: PeerStoreHandle,
}

impl Litep2pNetworkService {
	/// Create new [`Litep2pNetworkService`].
	pub fn new(
		local_peer_id: litep2p::PeerId,
		keypair: Keypair,
		cmd_tx: TracingUnboundedSender<NetworkServiceCommand>,
		peer_store_handle: PeerStoreHandle,
	) -> Self {
		Self { local_peer_id, keypair, cmd_tx, peer_store_handle }
	}
}

impl NetworkService for Litep2pNetworkService {}

impl NetworkSigner for Litep2pNetworkService {
	fn sign_with_local_identity(&self, _msg: impl AsRef<[u8]>) -> Result<Signature, SigningError> {
		let _public_key = self.keypair.public();
		todo!();
		// let bytes = self.keypair.sign(message.as_ref())?;
		// Ok(Signature { public_key, bytes })
	}
}

impl NetworkDHTProvider for Litep2pNetworkService {
	fn get_value(&self, key: &KademliaKey) {
		let _ = self.cmd_tx.unbounded_send(NetworkServiceCommand::GetValue(key.clone()));
	}

	fn put_value(&self, key: KademliaKey, value: Vec<u8>) {
		let _ = self.cmd_tx.unbounded_send(NetworkServiceCommand::PutValue(key, value));
	}
}

// Manual implementation to avoid extra boxing here
#[async_trait::async_trait]
impl NetworkStatusProvider for Litep2pNetworkService {
	async fn status(&self) -> Result<NetworkStatus, ()> {
		let (tx, rx) = oneshot::channel();
		self.cmd_tx.unbounded_send(NetworkServiceCommand::Status(tx)).map_err(|_| ())?;

		rx.await.map_err(|_| ())
	}
}

// Manual implementation to avoid extra boxing here
impl NetworkPeers for Litep2pNetworkService {
	fn set_authorized_peers(&self, _peers: HashSet<PeerId>) {
		todo!();
	}

	fn set_authorized_only(&self, _reserved_only: bool) {
		todo!();
	}

	fn add_known_address(&self, _peer: PeerId, _address: Multiaddr) {
		todo!();
	}

	fn report_peer(&self, _peer: PeerId, _cost_benefit: ReputationChange) {
		todo!();
	}

	fn disconnect_peer(&self, _peer: PeerId, _protocol: ProtocolName) {
		todo!();
	}

	fn accept_unreserved_peers(&self) {
		todo!();
	}

	fn deny_unreserved_peers(&self) {
		todo!();
	}

	fn add_reserved_peer(&self, _peer: MultiaddrWithPeerId) -> Result<(), String> {
		todo!();
	}

	fn remove_reserved_peer(&self, _peer: PeerId) {
		todo!();
	}

	fn set_reserved_peers(
		&self,
		_protocol: ProtocolName,
		_peers: HashSet<Multiaddr>,
	) -> Result<(), String> {
		todo!();
	}

	fn add_peers_to_reserved_set(
		&self,
		_protocol: ProtocolName,
		_peers: HashSet<Multiaddr>,
	) -> Result<(), String> {
		todo!();
	}

	fn remove_peers_from_reserved_set(
		&self,
		_protocol: ProtocolName,
		_peers: Vec<PeerId>,
	) -> Result<(), String> {
		todo!();
	}

	fn sync_num_connected(&self) -> usize {
		todo!();
	}

	fn peer_role(&self, peer: PeerId, handshake: Vec<u8>) -> Option<ObservedRole> {
		match Roles::decode_all(&mut &handshake[..]) {
			Ok(role) => Some(role.into()),
			Err(_) => {
				log::debug!(target: LOG_TARGET, "handshake doesn't contain peer role: {handshake:?}");
				self.peer_store_handle.peer_role(&(peer.into()))
			},
		}
	}
}

impl NetworkEventStream for Litep2pNetworkService {
	fn event_stream(&self, _stream_name: &'static str) -> BoxStream<'static, Event> {
		todo!();
	}
}

impl NetworkStateInfo for Litep2pNetworkService {
	fn external_addresses(&self) -> Vec<Multiaddr> {
		todo!();
	}

	fn listen_addresses(&self) -> Vec<Multiaddr> {
		todo!();
	}

	fn local_peer_id(&self) -> PeerId {
		self.local_peer_id.into()
	}
}

// Manual implementation to avoid extra boxing here
#[async_trait::async_trait]
impl NetworkRequest for Litep2pNetworkService {
	async fn request(
		&self,
		_peer: PeerId,
		_protocol: ProtocolName,
		_request: Vec<u8>,
		_connect: IfDisconnected,
	) -> Result<Vec<u8>, RequestFailure> {
		todo!();
	}

	fn start_request(
		&self,
		_peer: PeerId,
		_protocol: ProtocolName,
		_request: Vec<u8>,
		_tx: oneshot::Sender<Result<Vec<u8>, RequestFailure>>,
		_connect: IfDisconnected,
	) {
		todo!();
	}
}
