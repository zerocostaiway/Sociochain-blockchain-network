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

//! Substrate RPC implementation.
//!
//! A core implementation of Substrate RPC interfaces.

#![warn(missing_docs)]

pub use jsonrpsee::core::{
	id_providers::{
		RandomIntegerIdProvider as RandomIntegerSubscriptionId,
		RandomStringIdProvider as RandomStringSubscriptionId,
	},
	traits::IdProvider as RpcSubscriptionIdProvider,
};
pub use sc_rpc_api::DenyUnsafe;

pub mod author;
pub mod chain;
pub mod dev;
pub mod offchain;
pub mod state;
pub mod statement;
pub mod system;

#[cfg(any(test, feature = "test-helpers"))]
pub mod testing;

/// Task executor that is being used by RPC subscriptions.
pub type SubscriptionTaskExecutor = std::sync::Arc<dyn sp_core::traits::SpawnNamed>;

/// JSON-RPC helpers.
pub mod utils {
	use crate::SubscriptionTaskExecutor;
	use futures::{
		future::{self, Either, Fuse, FusedFuture},
		Future, FutureExt, Stream, StreamExt,
	};
	use jsonrpsee::{PendingSubscriptionSink, SubscriptionMessage, SubscriptionSink};
	use sp_runtime::Serialize;
	use std::collections::VecDeque;

	/// Similar to [`pipe_from_stream`] but also attempts to accept the subscription.
	pub async fn accept_and_pipe_from_stream<S, T>(
		pending: PendingSubscriptionSink,
		stream: S,
		cap: usize,
	) where
		S: Stream<Item = T> + Unpin + Send + 'static,
		T: Serialize + Send + 'static,
	{
		let sink = match tokio::time::timeout(std::time::Duration::from_secs(60), pending.accept())
			.await
		{
			Ok(Ok(sink)) => sink,
			Ok(Err(_)) => return,
			Err(_) => {
				log::error!(target: "rpc", "Subscription::accept timeout expired (1 min)");
				return
			},
		};
		pipe_from_stream(sink, stream, cap).await
	}

	/// Feed items to the subscription from the underlying stream.
	/// If the subscription can't keep up with the underlying stream
	/// then it's dropped.
	///
	/// This is simply a way to keep previous behaviour with unbounded streams
	/// and should be replaced by specific RPC endpoint behaviour.
	pub async fn pipe_from_stream<S, T>(sink: SubscriptionSink, mut stream: S, max_cap: usize)
	where
		S: Stream<Item = T> + Unpin + Send + 'static,
		T: Serialize + Send + 'static,
	{
		let mut next_fut = Box::pin(Fuse::terminated());
		let mut buf = VecDeque::with_capacity(max_cap);

		let mut next_item = stream.next();
		let closed = sink.closed();

		futures::pin_mut!(closed);

		loop {
			if next_fut.is_terminated() {
				if let Some(v) = buf.pop_front() {
					let val = to_sub_message_v2(&sink, &v);
					next_fut.set(async { sink.send(val).await }.fuse());
				}
			}

			match future::select(future::select(next_fut, next_item), closed).await {
				// Pending returned.
				Either::Left((Either::Left((_, n)), c)) => {
					next_item = n;
					closed = c;
					next_fut = Box::pin(Fuse::terminated());
				},

				Either::Left((Either::Right((Some(v), n)), c)) => {
					if buf.len() > max_cap + 1 {
						log::error!(target: "rpc", "Subscription buffer exceed; dropping subscription");
						break
					};

					buf.push_back(v);

					next_fut = n;
					closed = c;
					next_item = stream.next();
				},

				_ => break,
			}
		}
	}

	/// Build a subscription message.
	///
	/// # Panics
	///
	/// This function panics if the `Serialize` fails and is treated a bug.
	pub fn to_sub_message(result: &impl Serialize) -> SubscriptionMessage {
		SubscriptionMessage::from_json(result).expect("JSON serialization infallible; qed")
	}

	/// Build a subscription message.
	///
	/// # Panics
	///
	/// This function panics if the `Serialize` fails and is treated a bug.
	pub fn to_sub_message_v2(
		sink: &SubscriptionSink,
		result: &impl Serialize,
	) -> SubscriptionMessage {
		SubscriptionMessage::new(sink.method_name(), sink.subscription_id(), result)
			.expect("Serialize infallible; qed")
	}

	/// Spawn a subscription task and wait until it completes.
	pub fn spawn_subscription_task(
		label: &'static str,
		executor: &SubscriptionTaskExecutor,
		fut: impl Future<Output = ()> + Send + 'static,
	) {
		executor.spawn(label, Some("rpc"), fut.boxed());
	}
}
