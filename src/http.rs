//! HTTP JSON RPC client.

use crate::{
    jsonrpc::{
        self,
        batch::{self, Batch},
        Id, Request, Response, Version,
    },
    method::Method,
    types::Empty,
};
use reqwest::{StatusCode, Url};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    env,
    sync::atomic::{AtomicU32, Ordering},
};
use thiserror::Error;

/// An Ethereum RPC HTTP client.
pub struct Client {
    client: reqwest::Client,
    url: Url,
    id: AtomicU32,
}

impl Client {
    /// Creates a new JSON RPC HTTP client for the specified URL with the
    /// default HTTP client.
    pub fn new(url: Url) -> Self {
        Self::with_client(reqwest::Client::new(), url)
    }

    /// Creates a new JSON RPC HTTP client for the specified client instance and
    /// URL.
    pub fn with_client(client: reqwest::Client, url: Url) -> Self {
        Self {
            client,
            url,
            id: Default::default(),
        }
    }

    /// Creates a new JSON RPC HTTP client from the environment. This method
    /// uses the `NODE_URL` environment variable. This is useful for testing.
    ///
    /// # Panics
    ///
    /// This method panics if the environment variable is not pressent, or if it
    /// is not a valid HTTP url.
    pub fn from_env() -> Self {
        Self::new(
            env::var("NODE_URL")
                .expect("missing NODE_URL environment variable")
                .parse()
                .unwrap(),
        )
    }

    fn next_id(&self) -> Id {
        Id(self.id.fetch_add(1, Ordering::Relaxed))
    }

    async fn roundtrip<P, R>(&self, request: P) -> Result<R, ClientError>
    where
        P: Serialize,
        R: DeserializeOwned,
    {
        let request = serde_json::to_string(&request)?;
        let response = self
            .client
            .post(self.url.clone())
            .header("content-type", "application/json")
            .body(request)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            return Err(ClientError::Status(status, body));
        }

        let result = serde_json::from_str(&body)?;
        Ok(result)
    }

    /// Executes a JSON RPC method.
    pub async fn execute<M>(&self, method: M, params: M::Params) -> Result<M::Result, ClientError>
    where
        M: Method + Serialize,
    {
        Ok(self
            .roundtrip::<_, Response<M>>(Request {
                jsonrpc: Version::V2,
                method,
                params,
                id: self.next_id(),
            })
            .await?
            .result?)
    }

    /// Executes a JSON RPC method with empty parameters.
    pub async fn call<M>(&self, method: M) -> Result<M::Result, ClientError>
    where
        M: Method<Params = Empty> + Serialize,
    {
        self.execute::<M>(method, Empty).await
    }

    /// Executes a JSON RPC batch request.
    pub async fn batch<B>(&self, batch: B) -> Result<B::Values, ClientError>
    where
        B: Batch,
    {
        let results = self.try_batch(batch).await?;
        let values = B::values(results)?;
        Ok(values)
    }

    /// Executes a JSON RPC batch request, returning individual JSON RPC results
    /// for each batched requests. This allows fine-grained error handling
    /// for individual methods.
    pub async fn try_batch<B>(&self, batch: B) -> Result<B::Results, ClientError>
    where
        B: Batch,
    {
        let mut requests = batch.serialize_requests()?;
        for request in &mut requests {
            request.id = self.next_id();
        }

        let mut responses = self.roundtrip::<_, Vec<batch::Response>>(&requests).await?;
        if responses.len() != requests.len()
            || responses.iter().any(|response| response.id.is_none())
        {
            return Err(ClientError::Batch);
        }

        responses.sort_unstable_by_key(|response| response.id.unwrap().0);
        if responses
            .iter()
            .zip(requests.iter())
            .any(|(response, request)| response.id.unwrap() != request.id)
        {
            return Err(ClientError::Batch);
        }

        let results = B::deserialize_responses(responses)?;
        Ok(results)
    }
}

/// An error code.
#[derive(Debug, Error)]
pub enum ClientError {
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("{0}: {1}")]
    Status(StatusCode, String),
    #[error("RPC error: {0}")]
    Rpc(#[from] jsonrpc::Error),
    #[error("batch responses do not match requests")]
    Batch,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        eth,
        types::{BlockId, BlockTag, Empty, Hydrated, TransactionCall},
        web3,
    };
    use ethprim::{address, Digest};
    use hex_literal::hex;

    #[tokio::test]
    #[ignore]
    async fn connect_to_node() {
        let client = Client::from_env();
        let version = client.call(web3::ClientVersion).await.unwrap();
        println!("client version: {version}");
    }

    #[tokio::test]
    #[ignore]
    async fn uses_conversion_types_for_serialization() {
        let client = Client::from_env();
        let domain = Digest::from_slice(
            &client
                .execute(
                    eth::Call,
                    (
                        TransactionCall {
                            to: Some(address!("0x9008D19f58AAbD9eD0D60971565AA8510560ab41")),
                            input: Some(hex!("f698da25").to_vec()),
                            ..Default::default()
                        },
                        BlockId::default(),
                    ),
                )
                .await
                .unwrap(),
        );
        println!("CoW Protocol domain separator: {domain}");
    }

    #[tokio::test]
    #[ignore]
    async fn batch_request() {
        let client = Client::from_env();
        let (latest, safe) = client
            .batch((
                (eth::BlockNumber, Empty),
                (eth::GetBlockByNumber, (BlockTag::Safe.into(), Hydrated::No)),
            ))
            .await
            .unwrap();
        println!("Latest block: {latest}");
        println!("Safe block: {}", safe.unwrap().number);
    }
}
