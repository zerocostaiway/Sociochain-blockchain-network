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

#![allow(unused)]

use crate::{
	config::{
		FullNetworkConfiguration, IncomingRequest, NodeKeyConfig, NotificationHandshake, Params,
		SetConfig, TransportConfig,
	},
	error::Error,
	litep2p::{
		service::{Litep2pNetworkService, NetworkServiceCommand},
		shim::{
			notification::config::NotificationProtocolConfig,
			request_response::RequestResponseConfig,
		},
	},
	protocol,
	service::{ensure_addresses_consistent_with_transport, traits::NetworkBackend},
	NotificationService, ProtocolName,
};

use futures::StreamExt;
use libp2p::Multiaddr;
use litep2p::{
	config::Litep2pConfigBuilder,
	crypto::ed25519::{Keypair, SecretKey},
};
use parking_lot::Mutex;

use sc_network_common::ExHashT;
use sc_network_types::PeerId;
use sc_utils::mpsc::{tracing_unbounded, TracingUnboundedReceiver};
use sp_runtime::traits::Block as BlockT;

use std::{
	cmp,
	collections::{HashMap, HashSet},
	fs, io, iter,
	sync::{atomic::AtomicUsize, Arc},
	time::Duration,
};

mod service;
mod shim;

// TODO: metrics
// TODO: bandwidth sink
// TODO: add support for specifying external addresses

/// Logging target for the file.
const LOG_TARGET: &str = "sub-libp2p";

/// Networking backend for `litep2p`.
pub struct Litep2pNetworkBackend {
	/// `NetworkService` implementation for `Litep2pNetworkBackend`.
	network_service: Arc<Litep2pNetworkService>,

	/// RX channel for receiving commands from `Litep2pNetworkService`.
	cmd_rx: TracingUnboundedReceiver<NetworkServiceCommand>,

	/// Listen addresses. Do **NOT** include a trailing `/p2p/` with our `PeerId`.
	listen_addresses: Arc<Mutex<HashSet<Multiaddr>>>,
}

impl Litep2pNetworkBackend {
	/// Get `litep2p` keypair from `NodeKeyConfig`.
	fn get_keypair(node_key: &NodeKeyConfig) -> Result<(Keypair, litep2p::PeerId), Error> {
		let secret = libp2p::identity::Keypair::try_into_ed25519(node_key.clone().into_keypair()?)
			.map_err(|error| {
				log::error!(target: LOG_TARGET, "failed to convert to ed25519: {error:?}");
				Error::Io(io::ErrorKind::InvalidInput.into())
			})?
			.secret();

		// TODO: zzz
		let mut secret = secret.as_ref().iter().cloned().collect::<Vec<_>>();
		let secret = SecretKey::from_bytes(&mut secret)
			.map_err(|_| Error::Io(io::ErrorKind::InvalidInput.into()))?;
		let local_identity = Keypair::from(secret);
		let local_public = local_identity.public();
		let local_peer_id = local_public.to_peer_id();

		Ok((local_identity, local_peer_id))
	}

	// /// Configure transport protocols for `Litep2pNetworkBackend`.
	// fn configure_transport<B: BlockT + 'static, H: ExHashT>(
	// 	network_config: &FullNetworkConfiguration<B, H, Self>,
	// 	builder: &mut Litep2pConfigBuilder,
	// ) {
	// 	let FullNetworkConfiguration {
	// 		notification_protocols,
	// 		request_response_protocols,
	// 		mut network_config,
	// 	} = network_config;

	// 	let config_mem = match network_config.transport {
	// 		TransportConfig::MemoryOnly => panic!("memory transport not supported"),
	// 		TransportConfig::Normal { .. } => false,
	// 	};

	// 	// The yamux buffer size limit is configured to be equal to the maximum frame size
	// 	// of all protocols. 10 bytes are added to each limit for the length prefix that
	// 	// is not included in the upper layer protocols limit but is still present in the
	// 	// yamux buffer. These 10 bytes correspond to the maximum size required to encode
	// 	// a variable-length-encoding 64bits number. In other words, we make the
	// 	// assumption that no notification larger than 2^64 will ever be sent.
	// 	// TODO: make this a function of `NetworkConfiguration`?
	// 	let yamux_maximum_buffer_size = {
	// 		let requests_max = request_response_protocols
	// 			.iter()
	// 			.map(|cfg| usize::try_from(cfg.max_request_size).unwrap_or(usize::MAX));
	// 		let responses_max = request_response_protocols
	// 			.iter()
	// 			.map(|cfg| usize::try_from(cfg.max_response_size).unwrap_or(usize::MAX));
	// 		let notifs_max = notification_protocols
	// 			.iter()
	// 			.map(|cfg| usize::try_from(cfg.max_notification_size()).unwrap_or(usize::MAX));

	// 		// A "default" max is added to cover all the other protocols: ping, identify,
	// 		// kademlia, block announces, and transactions.
	// 		let default_max = cmp::max(
	// 			1024 * 1024,
	// 			usize::try_from(protocol::BLOCK_ANNOUNCES_TRANSACTIONS_SUBSTREAM_SIZE)
	// 				.unwrap_or(usize::MAX),
	// 		);

	// 		iter::once(default_max)
	// 			.chain(requests_max)
	// 			.chain(responses_max)
	// 			.chain(notifs_max)
	// 			.max()
	// 			.expect("iterator known to always yield at least one element; qed")
	// 			.saturating_add(10)
	// 	};

	// 	let multiplexing_config = {
	// 		let mut yamux_config = litep2p::yamux::Config::default();
	// 		// Enable proper flow-control: window updates are only sent when
	// 		// buffered data has been consumed.
	// 		yamux_config.set_window_update_mode(litep2p::yamux::WindowUpdateMode::OnRead);
	// 		yamux_config.set_max_buffer_size(yamux_maximum_buffer_size);

	// 		if let Some(yamux_window_size) = network_config.yamux_window_size {
	// 			yamux_config.set_receive_window(yamux_window_size);
	// 		}

	// 		yamux_config
	// 	};

	// 	// // Listen on multiaddresses.
	// 	// for addr in &network_config.listen_addresses {
	// 	// 	if let Err(err) = Swarm::<Behaviour<B>>::listen_on(&mut swarm, addr.clone()) {
	// 	// 		warn!(target: "sub-libp2p", "Can't listen on {} because: {:?}", addr, err)
	// 	// 	}
	// 	// }
	// 	// TODO: parse listen addresses and enable transports

	// 	// transport::build_transport(
	// 	// 	keypair.clone(),
	// 	// 	config_mem,
	// 	// 	network_config.yamux_window_size,
	// 	// 	yamux_maximum_buffer_size,
	// 	// )

	// 	todo!();
	// }
}

#[async_trait::async_trait]
impl<B: BlockT + 'static, H: ExHashT> NetworkBackend<B, H> for Litep2pNetworkBackend {
	type NotificationProtocolConfig = NotificationProtocolConfig;
	type RequestResponseProtocolConfig = RequestResponseConfig;
	type NetworkService<Block, Hash> = Arc<Litep2pNetworkService>;

	/// Create new `NetworkBackend`.
	fn new(params: Params<B, H, Self>) -> Result<Self, Error>
	where
		Self: Sized,
	{
		let FullNetworkConfiguration {
			notification_protocols,
			request_response_protocols,
			mut network_config,
		} = params.network_config;

		// get local keypair and local peer id
		let (keypair, local_peer_id) = Self::get_keypair(&network_config.node_key)?;
		let (cmd_tx, cmd_rx) = tracing_unbounded("mpsc_network_worker", 100_000);

		network_config.boot_nodes = network_config
			.boot_nodes
			.into_iter()
			.filter(|boot_node| boot_node.peer_id != local_peer_id.into())
			.collect();
		network_config.default_peers_set.reserved_nodes = network_config
			.default_peers_set
			.reserved_nodes
			.into_iter()
			.filter(|reserved_node| {
				if reserved_node.peer_id == local_peer_id.into() {
					log::warn!(
						target: LOG_TARGET,
						"Local peer ID used in reserved node, ignoring: {reserved_node}",
					);
					false
				} else {
					true
				}
			})
			.collect();

		// Ensure the listen addresses are consistent with the transport.
		ensure_addresses_consistent_with_transport(
			network_config.listen_addresses.iter(),
			&network_config.transport,
		)?;
		ensure_addresses_consistent_with_transport(
			network_config.boot_nodes.iter().map(|x| &x.multiaddr),
			&network_config.transport,
		)?;
		ensure_addresses_consistent_with_transport(
			network_config.default_peers_set.reserved_nodes.iter().map(|x| &x.multiaddr),
			&network_config.transport,
		)?;
		for notification_protocol in &notification_protocols {
			ensure_addresses_consistent_with_transport(
				notification_protocol.set_config().reserved_nodes.iter().map(|x| &x.multiaddr),
				&network_config.transport,
			)?;
		}
		ensure_addresses_consistent_with_transport(
			network_config.public_addresses.iter(),
			&network_config.transport,
		)?;

		if let Some(path) = &network_config.net_config_path {
			fs::create_dir_all(path)?;
		}

		log::info!(
			target: LOG_TARGET,
			"🏷  Local node identity is: {local_peer_id}",
		);

		let mut config_builder = Litep2pConfigBuilder::new();

		// Self::configure_transport(&params.network_config, &mut config_builder);

		let known_addresses = {
			// Collect all reserved nodes and bootnodes addresses.
			let mut addresses: Vec<_> = network_config
				.default_peers_set
				.reserved_nodes
				.iter()
				.map(|reserved| (reserved.peer_id, reserved.multiaddr.clone()))
				.chain(notification_protocols.iter().flat_map(|protocol| {
					protocol
						.set_config()
						.reserved_nodes
						.iter()
						.map(|reserved| (reserved.peer_id, reserved.multiaddr.clone()))
				}))
				.chain(
					network_config
						.boot_nodes
						.iter()
						.map(|bootnode| (bootnode.peer_id, bootnode.multiaddr.clone())),
				)
				.collect();

			// Remove possible duplicates.
			addresses.sort();
			addresses.dedup();

			addresses
		};

		// Check for duplicate bootnodes.
		network_config.boot_nodes.iter().try_for_each(|bootnode| {
			if let Some(other) = network_config
				.boot_nodes
				.iter()
				.filter(|o| o.multiaddr == bootnode.multiaddr)
				.find(|o| o.peer_id != bootnode.peer_id)
			{
				Err(Error::DuplicateBootnode {
					address: bootnode.multiaddr.clone(),
					first_id: bootnode.peer_id.into(),
					second_id: other.peer_id.into(),
				})
			} else {
				Ok(())
			}
		})?;

		// List of bootnode multiaddresses.
		let mut boot_node_ids = HashMap::<PeerId, Vec<Multiaddr>>::new();

		for bootnode in network_config.boot_nodes.iter() {
			boot_node_ids
				.entry(bootnode.peer_id.into())
				.or_default()
				.push(bootnode.multiaddr.clone());
		}

		let boot_node_ids = Arc::new(boot_node_ids);

		let num_connected = Arc::new(AtomicUsize::new(0));
		// let external_addresses = Arc::new(Mutex::new(HashSet::new()));

		// let protocol = Protocol::new(
		// 	From::from(&params.role),
		// 	&params.metrics_registry,
		// 	notification_protocols,
		// 	params.block_announce_config,
		// 	params.peer_store.clone(),
		// 	protocol_handles.clone(),
		// 	from_protocol_controllers,
		// )?;

		// // Build the swarm.
		// let (mut swarm, bandwidth): (Swarm<Behaviour<B>>, _) = {
		// 	let user_agent =
		// 		format!("{} ({})", network_config.client_version, network_config.node_name);

		// 	let discovery_config = {
		// 		let mut config = DiscoveryConfig::new(local_public.to_peer_id());
		// 		config.with_permanent_addresses(
		// 			known_addresses
		// 				.iter()
		// 				.map(|(peer, address)| (peer.into(), address.clone()))
		// 				.collect::<Vec<_>>(),
		// 		);
		// 		config.discovery_limit(u64::from(network_config.default_peers_set.out_peers) + 15);
		// 		config.with_kademlia(
		// 			params.genesis_hash,
		// 			params.fork_id.as_deref(),
		// 			&params.protocol_id,
		// 		);
		// 		config.with_dht_random_walk(network_config.enable_dht_random_walk);
		// 		config.allow_non_globals_in_dht(network_config.allow_non_globals_in_dht);
		// 		config.use_kademlia_disjoint_query_paths(
		// 			network_config.kademlia_disjoint_query_paths,
		// 		);
		// 		config.with_kademlia_replication_factor(network_config.kademlia_replication_factor);

		// 		match network_config.transport {
		// 			TransportConfig::MemoryOnly => panic!("memory transport not supported"),
		// 			TransportConfig::Normal {
		// 				enable_mdns,
		// 				allow_private_ip: allow_private_ipv4,
		// 				..
		// 			} => {
		// 				config.with_mdns(enable_mdns);
		// 				config.allow_private_ip(allow_private_ipv4);
		// 			},
		// 		}

		// 		config
		// 	};

		// 	// let behaviour = {
		// 	// 	let result = Behaviour::new(
		// 	// 		protocol,
		// 	// 		user_agent,
		// 	// 		local_public,
		// 	// 		discovery_config,
		// 	// 		request_response_protocols,
		// 	// 		params.peer_store.clone(),
		// 	// 		external_addresses.clone(),
		// 	// 	);

		// 	// 	match result {
		// 	// 		Ok(b) => b,
		// 	// 		Err(crate::request_responses::RegisterError::DuplicateProtocol(proto)) =>
		// 	// 			return Err(Error::DuplicateRequestResponseProtocol { protocol: proto }),
		// 	// 	}
		// 	// };

		// 	// let builder = {
		// 	// 	struct SpawnImpl<F>(F);
		// 	// 	impl<F: Fn(Pin<Box<dyn Future<Output = ()> + Send>>)> Executor for SpawnImpl<F> {
		// 	// 		fn exec(&self, f: Pin<Box<dyn Future<Output = ()> + Send>>) {
		// 	// 			(self.0)(f)
		// 	// 		}
		// 	// 	}
		// 	// 	SwarmBuilder::with_executor(
		// 	// 		transport,
		// 	// 		behaviour,
		// 	// 		local_peer_id,
		// 	// 		SpawnImpl(params.executor),
		// 	// 	)
		// 	// };

		// 	// (builder.build(), bandwidth)
		// 	todo!();
		// };

		// // Initialize the metrics.
		// let metrics = match &params.metrics_registry {
		// 	Some(registry) => Some(metrics::register(
		// 		registry,
		// 		MetricSources {
		// 			bandwidth: bandwidth.clone(),
		// 			connected_peers: num_connected.clone(),
		// 		},
		// 	)?),
		// 	None => None,
		// };

		// // Add external addresses.
		// for addr in &network_config.public_addresses {
		// 	Swarm::<Behaviour<B>>::add_external_address(
		// 		&mut swarm,
		// 		addr.clone(),
		// 		AddressScore::Infinite,
		// 	);
		// }

		let listen_addresses = Arc::new(Mutex::new(HashSet::new()));

		let network_service = Arc::new(Litep2pNetworkService::new(
			local_peer_id,
			keypair.clone(),
			cmd_tx,
			params.peer_store.clone(),
		));

		Ok(Self { network_service, cmd_rx, listen_addresses })
	}

	/// Get handle to `NetworkService` of the `NetworkBackend`.
	fn network_service(&self) -> Self::NetworkService<B, H> {
		self.network_service.clone()
	}

	/// Create notification protocol configuration for `protocol`.
	fn notification_config(
		_protocol_name: ProtocolName,
		_fallback_names: Vec<ProtocolName>,
		_max_notification_size: u64,
		_handshake: Option<NotificationHandshake>,
		_set_config: SetConfig,
	) -> (Self::NotificationProtocolConfig, Box<dyn NotificationService>) {
		todo!();
	}

	/// Create request-response protocol configuration.
	fn request_response_config(
		_protocol_name: ProtocolName,
		_fallback_names: Vec<ProtocolName>,
		_max_request_size: u64,
		_max_response_size: u64,
		_request_timeout: Duration,
		_inbound_queue: Option<async_channel::Sender<IncomingRequest>>,
	) -> Self::RequestResponseProtocolConfig {
		todo!();
	}

	/// Start [`NetworkBackend`] event loop.
	async fn run(mut self) {
		loop {
			tokio::select! {
				command = self.cmd_rx.next() => match command {
					None => return,
					Some(command) => match command {
						NetworkServiceCommand::GetValue(_key) => {
							todo!();
						}
						NetworkServiceCommand::PutValue(_key, _value) => {
							todo!();
						}
						NetworkServiceCommand::Status(_result_tx) => {
							todo!();
						}
					}
				}
			}
		}
	}
}
