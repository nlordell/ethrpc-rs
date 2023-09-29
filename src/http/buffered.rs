//! Buffered Ethereum JSON RPC HTTP client.
//!
//! Concurrent JSON RPC calls will be automatically buffered and executed as a
//! single JSON RPC batch call.

use super::client::{Client, Error};
use crate::{
    jsonrpc::{self, batch},
    method::Method,
    types::Empty,
};
use futures::{
    channel::{mpsc, oneshot},
    stream::StreamExt as _,
};
use serde::Serialize;
use serde_json::Value;
use std::{fmt::Write, num::NonZeroUsize, sync::Arc, time::Duration};
use tokio_stream::StreamExt as _;

/// A buffered JSON RPC HTTP client.
pub struct Buffered {
    client: Arc<Client>,
    calls: mpsc::UnboundedSender<Call>,
}

struct Call {
    request: String,
    response: oneshot::Sender<Result<String, Error>>,
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
                                let requests = join_requests(&chunk);
                                let responses = client.roundtrip(requests).await;
                                for (call, response) in
                                    chunk.into_iter().zip(split_responses(n, responses))
                                {
                                    let _ = call.response.send(response);
                                }
                            }
                        }
                    }
                },
            )
            .await;
    }

    async fn roundtrip(&self, request: String) -> Result<String, Error> {
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

    /// Executes a JSON RPC call with empty parameters.
    pub async fn exec<M>(&self, method: M) -> Result<M::Result, Error>
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

// TODO(nlordell): We can optimize this a bit:
// 1. First of all, we can use the `reqwest::Body::wrap_stream` to send chunks
//    potentially as requests come in, and without allocating a string buffer
//    to hold all of the requests.
// 2. We can split the responses without parsing them into JSON values and then
//    re-serializing each array item into a string to save on allocations.

fn join_requests(calls: &[Call]) -> String {
    let total = calls.iter().map(|call| call.request.len() + 2).sum();
    let mut buffer = String::with_capacity(total);
    let mut calls = calls.iter();

    let _ = write!(buffer, "[");
    if let Some(Call { request, .. }) = calls.next() {
        let _ = write!(buffer, "{request}");
    }
    for Call { request, .. } in calls {
        let _ = write!(buffer, ",{request}");
    }
    let _ = write!(buffer, "]");

    buffer
}

fn split_responses(n: usize, responses: Result<String, Error>) -> Vec<Result<String, Error>> {
    match responses.and_then(|r| Ok(serde_json::from_str::<Vec<Value>>(&r)?)) {
        Ok(responses) => responses
            .into_iter()
            .map(|response| Ok(response.to_string()))
            .collect(),
        Err(_) => (0..n).map(|_| Err(Error::Batch(batch::Error))).collect(),
    }
}
