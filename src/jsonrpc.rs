//! Module containing serializable JSON RPC data types.

pub mod batch;

use crate::method::Method;
use serde::{
    de::{self, Deserializer},
    Deserialize, Serialize, Serializer,
};
use serde_json::Value;
use std::{
    fmt::{self, Formatter},
    marker::PhantomData,
};
use thiserror::Error;

/// JSON RPC supported version.
#[derive(Debug, Deserialize, Serialize)]
pub enum Version {
    /// Version 2.0 of the JSON RPC specification.
    #[serde(rename = "2.0")]
    V2,
}

/// Request and response ID.
///
/// Note that `u32` is used. This is so it always fits in a `f64` and obeys the
/// "SHOULD NOT have fractional parts" rule from the specification.  Since the
/// ID is set by the client, we shouldn't run into issues where a numerical ID
/// does not fit into this value or a string ID is used.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, Hash, PartialEq)]
#[serde(transparent)]
pub struct Id(pub u32);

/// A request object.
#[derive(Debug, Deserialize, Serialize)]
pub struct Request<M>
where
    M: Method,
{
    pub jsonrpc: Version,
    pub method: M,
    #[serde(
        deserialize_with = "M::deserialize_params",
        serialize_with = "M::serialize_params"
    )]
    pub params: M::Params,
    pub id: Id,
}

/// Notification object.
#[derive(Debug, Deserialize, Serialize)]
pub struct Notification<M>
where
    M: Method,
{
    pub jsonrpc: Version,
    pub method: M,
    #[serde(
        deserialize_with = "M::deserialize_params",
        serialize_with = "M::serialize_params"
    )]
    pub params: M::Params,
}

/// Response object.
#[derive(Debug)]
pub struct Response<M>
where
    M: Method,
{
    pub jsonrpc: Version,
    pub result: Result<M::Result, Error>,
    pub id: Option<Id>,
}

impl<'de, M> Deserialize<'de> for Response<M>
where
    M: Method,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "lowercase")]
        enum Key {
            JsonRpc,
            Result,
            Error,
            Id,
        }

        struct Visitor<M>(PhantomData<M>);

        impl<'de, M> de::Visitor<'de> for Visitor<M>
        where
            M: Method,
        {
            type Value = Response<M>;

            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("JSON RPC response")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let mut jsonrpc = None;
                let mut result = None;
                let mut error = None;
                let mut id = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Key::JsonRpc => {
                            if jsonrpc.is_some() {
                                return Err(de::Error::duplicate_field("jsonrpc"));
                            }
                            jsonrpc = Some(map.next_value()?);
                        }
                        Key::Result => {
                            if result.is_some() {
                                return Err(de::Error::duplicate_field("result"));
                            }
                            result = Some(map.next_value::<MethodResult<M>>()?);
                        }
                        Key::Error => {
                            if error.is_some() {
                                return Err(de::Error::duplicate_field("error"));
                            }
                            error = Some(map.next_value()?);
                        }
                        Key::Id => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                    }
                }

                Ok(Response {
                    jsonrpc: jsonrpc.ok_or_else(|| de::Error::missing_field("jsonrpc"))?,
                    result: match (result, error) {
                        (Some(result), _) => Ok(result.0),
                        (None, Some(error)) => Err(error),
                        (None, None) => {
                            return Err(de::Error::custom("missing 'result' or 'error' field"))
                        }
                    },
                    id,
                })
            }
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct MethodResult<M>(#[serde(deserialize_with = "M::deserialize_result")] M::Result)
        where
            M: Method;

        deserializer.deserialize_struct(
            "Response",
            &["jsonrpc", "result", "error", "id"],
            Visitor::<M>(PhantomData),
        )
    }
}

impl<M> Serialize for Response<M>
where
    M: Method,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        #[serde(transparent)]
        struct MethodResult<'a, M>(#[serde(serialize_with = "M::serialize_result")] &'a M::Result)
        where
            M: Method;

        #[derive(Serialize)]
        #[serde(bound(serialize = "M: Method"))]
        struct Response<'a, M>
        where
            M: Method,
        {
            jsonrpc: Version,
            result: Option<MethodResult<'a, M>>,
            error: Option<&'a Error>,
            id: Option<Id>,
        }

        let (result, error) = match &self.result {
            Ok(result) => (Some(MethodResult::<M>(result)), None),
            Err(error) => (None, Some(error)),
        };
        Response {
            jsonrpc: Version::V2,
            result,
            error,
            id: self.id,
        }
        .serialize(serializer)
    }
}

mod response {
    use super::{Error, Id, Version};
    use crate::method::Method;
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    #[serde(transparent)]
    pub struct Res<M>(
        #[serde(
            deserialize_with = "M::deserialize_result",
            serialize_with = "M::serialize_result"
        )]
        pub M::Result,
    )
    where
        M: Method;

    #[derive(Deserialize, Serialize)]
    #[serde(
        bound(deserialize = "M: Method", serialize = "M: Method"),
        deny_unknown_fields
    )]
    pub struct Raw<M>
    where
        M: Method,
    {
        pub jsonrpc: Version,
        pub result: Option<Res<M>>,
        pub error: Option<Error>,
        pub id: Option<Id>,
    }
}

/// An RPC error that may be produced on a response.
#[derive(Clone, Debug, Deserialize, Error, Serialize)]
#[error("{code}: {message}")]
#[serde(deny_unknown_fields)]
pub struct Error {
    pub code: ErrorCode,
    pub message: String,
    pub data: Value,
}

/// An error code.
#[derive(Clone, Copy, Debug, Deserialize, Error, Serialize)]
#[serde(from = "i32", into = "i32")]
pub enum ErrorCode {
    #[error("parse error")]
    ParseError,
    #[error("invalid request")]
    InvalidRequest,
    #[error("method not found")]
    MethodNotFound,
    #[error("invalid params")]
    InvalidParams,
    #[error("internal error")]
    InternalError,
    #[error("server error ({0})")]
    ServerError(i32),
    #[error("reserved ({0})")]
    Reserved(i32),
    #[error("{0}")]
    Other(i32),
}

impl From<i32> for ErrorCode {
    fn from(code: i32) -> Self {
        #[allow(clippy::match_overlapping_arm)]
        match code {
            -32700 => Self::ParseError,
            -32600 => Self::InvalidRequest,
            -32601 => Self::MethodNotFound,
            -32602 => Self::InvalidParams,
            -32603 => Self::InternalError,
            -32099..=-32000 => Self::ServerError(code),
            -32768..=-32000 => Self::Reserved(code),
            _ => Self::Other(code),
        }
    }
}

impl From<ErrorCode> for i32 {
    fn from(code: ErrorCode) -> Self {
        match code {
            ErrorCode::ParseError => -32700,
            ErrorCode::InvalidRequest => -32600,
            ErrorCode::MethodNotFound => -32601,
            ErrorCode::InvalidParams => -32602,
            ErrorCode::InternalError => -32603,
            ErrorCode::ServerError(code) => code,
            ErrorCode::Reserved(code) => code,
            ErrorCode::Other(code) => code,
        }
    }
}
