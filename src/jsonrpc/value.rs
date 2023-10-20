//! Arbitrary JSON value.
//!
//! This is a wrapper around [`serde_json::Value`]. This is done in order to
//! allow using the `ethrpc` crate without additional dependencies, and without
//! requiring to re-export the [`serde_json`] crate.

use crate::method::Method;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};
use thiserror::Error;

/// An arbitrary JSON value.
#[derive(Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Value(pub serde_json::Value);

impl Value {
    /// Creates a new value with the specified serialization implementation.
    pub fn new<T, S>(data: T, with: S) -> Result<Self, JsonError>
    where
        S: FnOnce(
            &T,
            serde_json::value::Serializer,
        ) -> Result<serde_json::Value, serde_json::Error>,
    {
        let value = with(&data, serde_json::value::Serializer)?;
        Ok(Self(value))
    }

    /// Creates a new value with the default serialization implementation.
    pub fn serialize<T>(data: T) -> Result<Self, JsonError>
    where
        T: Serialize,
    {
        Self::new(data, T::serialize)
    }

    /// Creates a value for method parameters.
    pub fn for_params<M>(params: M::Params) -> Result<Self, JsonError>
    where
        M: Method,
    {
        Self::new(params, M::serialize_params)
    }

    /// Creates a value for method parameters.
    pub fn for_result<M>(result: M::Result) -> Result<Self, JsonError>
    where
        M: Method,
    {
        Self::new(result, M::serialize_result)
    }

    /// Returns the value as typed data.
    pub fn data<T, D>(self, with: D) -> Result<T, JsonError>
    where
        D: FnOnce(serde_json::Value) -> Result<T, serde_json::Error>,
    {
        with(self.0).map_err(JsonError::from)
    }

    /// Returns the value as typed data.
    pub fn deserialize<T>(self) -> Result<T, JsonError>
    where
        T: DeserializeOwned,
    {
        self.data(T::deserialize)
    }

    /// Parses the value as method parameters.
    pub fn params<M>(self) -> Result<M::Params, JsonError>
    where
        M: Method,
    {
        self.data(M::deserialize_params)
    }

    /// Parses the value as a method result.
    pub fn result<M>(self) -> Result<M::Result, JsonError>
    where
        M: Method,
    {
        self.data(M::deserialize_result)
    }
}

impl Default for Value {
    fn default() -> Self {
        Self(serde_json::Value::Null)
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let to_string = if f.alternate() {
            serde_json::to_string_pretty
        } else {
            serde_json::to_string
        };
        let json = to_string(&self.0).map_err(|_| fmt::Error)?;
        f.debug_tuple("Value")
            .field(&format_args!("{json}"))
            .finish()
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Value {
    type Err = JsonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

/// A JSON error.
#[derive(Debug, Error)]
#[error(transparent)]
pub struct JsonError(#[from] pub serde_json::Error);

impl JsonError {
    /// Creates a JSON error with a custom message.
    pub fn custom(msg: impl AsRef<str>) -> Self {
        Self(serde::ser::Error::custom(msg.as_ref()))
    }
}
