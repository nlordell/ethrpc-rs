//! Buffered Ethereum JSON RPC HTTP client.
//!
//! Concurrent JSON RPC calls will be automatically buffered and executed as a
//! single JSON RPC batch call.

use super::client::{Client, Error};
use crate::{jsonrpc, method::Method, types::Empty};
use futures::{
    channel::{mpsc, oneshot},
    stream::StreamExt as _,
};
use serde::Serialize;
use std::{num::NonZeroUsize, sync::Arc, time::Duration};
use tokio_stream::StreamExt as _;

/// A buffered JSON RPC HTTP client.
pub struct Buffered {
    client: Arc<Client>,
    calls: mpsc::UnboundedSender<Call>,
}

struct Call {
    request: jsonrpc::Request,
    response: oneshot::Sender<Result<jsonrpc::Response, Error>>,
}

impl Client {
    /// Returns a [`Buffered`] HTTP JSON RPC client.
    pub fn buffered(self, config: Configuration) -> Buffered {
        let client = Arc::new(self);
        let (calls, receiver) = mpsc::unbounded();

        tokio::task::spawn(Buffered::background_worker(
            client.clone(),
            receiver,
            config,
        ));

        Buffered { client, calls }
    }
}

impl Buffered {
    /// Creates a new JSON RPC HTTP client for the specified client instance and
    /// URL.
    pub fn client(&self) -> &Client {
        &self.client
    }

    async fn background_worker(
        client: Arc<Client>,
        calls: mpsc::UnboundedReceiver<Call>,
        config: Configuration,
    ) {
        calls
            .chunks_timeout(config.max_size, config.delay)
            .for_each_concurrent(
                config.max_concurrent_requests.map(NonZeroUsize::get),
                |mut chunk| {
                    let client = client.clone();
                    async move {
                        match chunk.len() {
                            0 => (),
                            1 => {
                                let call = chunk.remove(0);
                                let response = client.roundtrip(call.request).await;
                                let _ = call.response.send(response);
                            }
                            n => {
                                let (requests, channels): (Vec<_>, Vec<_>) = chunk
                                    .into_iter()
                                    .map(|call| (call.request, call.response))
                                    .unzip();
                                let responses = client
                                    .roundtrip::<_, Vec<jsonrpc::Response>>(requests)
                                    .await
                                    .map(|responses| {
                                        responses.into_iter().map(Ok).collect::<Vec<_>>()
                                    })
                                    .unwrap_or_else(|err| {
                                        (0..n).map(|_| Err(err.duplicate())).collect()
                                    });
                                for (channel, response) in channels.into_iter().zip(responses) {
                                    let _ = channel.send(response);
                                }
                            }
                        }
                    }
                },
            )
            .await;
    }

    async fn roundtrip(&self, request: jsonrpc::Request) -> Result<jsonrpc::Response, Error> {
        async {
            let (sender, receiver) = oneshot::channel();
            self.calls
                .unbounded_send(Call {
                    request,
                    response: sender,
                })
                .ok()?;
            receiver.await.ok()
        }
        .await
        .expect("background worker unexpectedly stopped")
    }

    /// Executes a JSON RPC call.
    pub async fn call<M>(&self, method: M, params: M::Params) -> Result<M::Result, Error>
    where
        M: Method + Serialize,
    {
        jsonrpc::call_async(method, params, |request| self.roundtrip(request)).await
    }

    /// Executes a JSON RPC call with no parameters.
    pub async fn call_np<M>(&self, method: M) -> Result<M::Result, Error>
    where
        M: Method<Params = Empty> + Serialize,
    {
        jsonrpc::call_async(method, Empty, |request| self.roundtrip(request)).await
    }
}

/// Buffered JSON RPC configuration.
pub struct Configuration {
    /// The maximum amount of concurrent batches to send to the node.
    ///
    /// Specifying `None` means no limit on concurrency.
    pub max_concurrent_requests: Option<NonZeroUsize>,
    /// The maximum batch size.
    pub max_size: usize,
    /// An additional minimum delay to wait for collecting requests.
    ///
    /// The delay starts counting after receiving the first request.
    pub delay: Duration,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            max_concurrent_requests: NonZeroUsize::new(1),
            max_size: 20,
            delay: Duration::default(),
        }
    }
}
