//! HTTP JSON RPC client implemented with curl.

use crate::{
    jsonrpc::{
        self,
        batch::{self, Batch},
    },
    method::Method,
    types::Empty,
};
pub use curl;
use curl::easy::{Easy, List};
use serde::Serialize;
use std::{
    cell::RefCell,
    env,
    io::{Read, Write},
    string::FromUtf8Error,
};
use thiserror::Error;

/// An Ethereum RPC HTTP client.
pub struct Client {
    handle: RefCell<Easy>,
}

impl Client {
    /// Creates a new JSON RPC HTTP client for the specified URL with the
    /// default HTTP client.
    pub fn new(url: impl AsRef<str>) -> Result<Self, Error> {
        let mut handle = Easy::new();
        handle.url(url.as_ref())?;
        handle.http_headers({
            let mut list = List::new();
            list.append("Content-Type: application/json")?;
            list
        })?;
        Ok(Self::with_handle(handle))
    }

    /// Creates a new JSON RPC HTTP client for the specified curl [`curl::easy::Easy`]
    /// handle instance.
    ///
    /// This method assumes that the `url` has been set.
    pub fn with_handle(handle: Easy) -> Self {
        Self {
            handle: RefCell::new(handle),
        }
    }

    /// Creates a new JSON RPC HTTP client from the environment. This method
    /// uses the `ETHRPC` environment variable. This is useful for testing.
    ///
    /// # Panics
    ///
    /// This method panics if the environment variable is not pressent, or if it
    /// is not a valid HTTP url.
    pub fn from_env() -> Self {
        Self::new(env::var("ETHRPC").expect("missing ETHRPC environment variable")).unwrap()
    }

    fn roundtrip(&self, request: String) -> Result<String, Error> {
        let mut handle = self
            .handle
            .try_borrow_mut()
            .expect("unexpected sharing of curl handle");

        let mut request = request.as_bytes();
        let mut response = Vec::new();
        {
            let mut transfer = handle.transfer();
            transfer.read_function(|chunk| Ok(request.read(chunk).unwrap()))?;
            transfer.write_function(|chunk| Ok(response.write(chunk).unwrap()))?;
            transfer.perform()?;
        }

        let status = handle.response_code()?;
        let response = String::from_utf8(response)?;
        if !(200..300).contains(&status) {
            return Err(Error::Status(status, response));
        }

        Ok(response)
    }

    /// Executes a JSON RPC call.
    pub fn call<M>(&self, method: M, params: M::Params) -> Result<M::Result, Error>
    where
        M: Method + Serialize,
    {
        jsonrpc::call(method, params, |request| self.roundtrip(request))
    }

    /// Executes a JSON RPC call with empty parameters.
    pub fn exec<M>(&self, method: M) -> Result<M::Result, Error>
    where
        M: Method<Params = Empty> + Serialize,
    {
        jsonrpc::call(method, Empty, |request| self.roundtrip(request))
    }

    /// Executes a JSON RPC batch request.
    pub fn batch<B>(&self, batch: B) -> Result<B::Values, Error>
    where
        B: Batch,
    {
        batch::call(batch, |requests| self.roundtrip(requests))
    }

    /// Executes a JSON RPC batch request, returning individual JSON RPC results
    /// for each batched requests. This allows fine-grained error handling
    /// for individual methods.
    pub fn try_batch<B>(&self, batch: B) -> Result<B::Results, Error>
    where
        B: Batch,
    {
        batch::try_call(batch, |requests| self.roundtrip(requests))
    }
}

/// An error code.
#[derive(Debug, Error)]
pub enum Error {
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("HTTP error: {0}")]
    Http(#[from] curl::Error),
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] FromUtf8Error),
    #[error("HTTP {0} error: {1}")]
    Status(u32, String),
    #[error(transparent)]
    Rpc(#[from] jsonrpc::Error),
    #[error(transparent)]
    Batch(#[from] batch::Error),
}
