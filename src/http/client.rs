//! Ethereum JSON RPC HTTP client.

use crate::{
    jsonrpc::{
        self, JsonError,
        batch::{self, Batch},
    },
    method::Method,
    types::Empty,
};
use reqwest::{StatusCode, Url};
use serde::{Serialize, de::DeserializeOwned};
use std::{env, sync::Arc};
use thiserror::Error;

/// An Ethereum JSON RPC HTTP client.
pub struct Client {
    client: reqwest::Client,
    url: Url,
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
        Self { client, url }
    }

    /// Creates a new JSON RPC HTTP client from the environment. This method
    /// uses the `ETHRPC` environment variable. This is useful for testing.
    ///
    /// # Panics
    ///
    /// This method panics if the environment variable is not pressent, or if it
    /// is not a valid HTTP url.
    pub fn from_env() -> Self {
        Self::new(
            env::var("ETHRPC")
                .expect("missing ETHRPC environment variable")
                .parse()
                .unwrap(),
        )
    }

    pub(super) async fn roundtrip<T, R>(&self, request: T) -> Result<R, Error>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let response = self
            .client
            .post(self.url.clone())
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            return Err(Error::Status(status, response.text().await?));
        }

        let body = response.json().await?;
        Ok(body)
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

    /// Executes a JSON RPC batch request.
    pub async fn batch<B>(&self, batch: B) -> Result<B::Values, Error>
    where
        B: Batch,
    {
        batch::call_async(batch, |requests| self.roundtrip(requests)).await
    }

    /// Executes a JSON RPC batch request, returning individual JSON RPC results
    /// for each batched requests. This allows fine-grained error handling
    /// for individual methods.
    pub async fn try_batch<B>(&self, batch: B) -> Result<B::Results, Error>
    where
        B: Batch,
    {
        batch::try_call_async(batch, |requests| self.roundtrip(requests)).await
    }
}

/// An error code.
#[derive(Debug, Error)]
pub enum Error {
    #[error("JSON error: {0}")]
    Json(#[from] Arc<JsonError>),
    #[error("HTTP error: {0}")]
    Http(#[from] Arc<reqwest::Error>),
    #[error("HTTP {0} error: {1}")]
    Status(StatusCode, String),
    #[error(transparent)]
    Rpc(#[from] jsonrpc::Error),
    #[error(transparent)]
    Batch(#[from] batch::Error),
}

impl Error {
    /// Duplicate an error.
    ///
    /// This has the exact same semantics as `Clone`, but we don't want to
    /// expose that since the implementation is messy to say the least.
    pub(super) fn duplicate(&self) -> Self {
        match self {
            Self::Json(err) => Self::Json(err.clone()),
            Self::Http(err) => Self::Http(err.clone()),
            Self::Status(code, body) => Self::Status(*code, body.clone()),
            Self::Rpc(err) => Self::Rpc(err.clone()),
            Self::Batch(batch::Error) => Self::Batch(batch::Error),
        }
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Self::from(Arc::new(err))
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::from(Arc::new(err))
    }
}
