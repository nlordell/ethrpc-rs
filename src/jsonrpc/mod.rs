//! Module containing serializable JSON RPC data types.

pub mod batch;
mod value;

pub use self::value::{JsonError, Value};
use serde::{
    Deserialize, Serialize, Serializer,
    de::{self, Deserializer},
};
use std::{
    borrow::Cow,
    fmt::{self, Debug, Formatter},
    future::Future,
    sync::atomic::{self, AtomicU32},
};
use thiserror::Error;

/// Executes a JSON RPC call with the provided roundtrip implementation.
pub fn call<M, F, E>(method: M, params: M::Params, roundtrip: F) -> Result<M::Result, E>
where
    M: crate::method::Method + Serialize,
    F: FnOnce(Request) -> Result<Response, E>,
    E: From<Error> + From<JsonError>,
{
    let request = Request::new(method, params)?;
    let response = roundtrip(request)?;
    Ok(response.result::<M>()??)
}

/// Executes a JSON RPC call with the provided `async` roundtrip implementation.
pub async fn call_async<M, F, Fut, E>(
    method: M,
    params: M::Params,
    roundtrip: F,
) -> Result<M::Result, E>
where
    M: crate::method::Method + Serialize,
    F: FnOnce(Request) -> Fut,
    Fut: Future<Output = Result<Response, E>>,
    E: From<Error> + From<JsonError>,
{
    let request = Request::new(method, params)?;
    let response = roundtrip(request).await?;
    Ok(response.result::<M>()??)
}

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

impl Id {
    fn next() -> Self {
        static ID: AtomicU32 = AtomicU32::new(0);
        Self(ID.fetch_add(1, atomic::Ordering::Relaxed))
    }
}

/// A JSON RPC method name.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Method(pub Cow<'static, str>);

impl Method {
    /// Creates a new method name from a string.
    pub fn new(name: impl Into<String>) -> Self {
        Self(Cow::Owned(name.into()))
    }

    /// Creates a new method name from a static string reference.
    pub fn from_static(name: &'static str) -> Self {
        Self(Cow::Borrowed(name))
    }

    /// Returns the method as a string reference.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

/// A request object.
#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub jsonrpc: Version,
    pub method: Method,
    pub params: Value,
    pub id: Id,
}

impl Request {
    /// Creates a new request for the specified method and paramters.
    pub fn new<M>(method: M, params: M::Params) -> Result<Self, JsonError>
    where
        M: crate::method::Method,
    {
        Ok(Self {
            jsonrpc: Version::V2,
            method: Method(method.name()),
            params: Value::new(params, M::serialize_params)?,
            id: Id::next(),
        })
    }
}

/// Notification object.
#[derive(Debug, Deserialize, Serialize)]
pub struct Notification {
    pub jsonrpc: Version,
    pub method: Method,
    pub params: Value,
}

impl Notification {
    /// Creates a new notification for the specified method and paramters.
    pub fn new<M>(method: M, params: M::Params) -> Result<Self, JsonError>
    where
        M: crate::method::Method,
    {
        Ok(Self {
            jsonrpc: Version::V2,
            method: Method(method.name()),
            params: Value::new(params, M::serialize_params)?,
        })
    }
}

/// Response object.
#[derive(Debug)]
pub struct Response {
    pub jsonrpc: Version,
    pub result: Result<Value, Error>,
    pub id: Option<Id>,
}

impl Response {
    /// Extracts the response result for a method.
    pub fn result<M>(self) -> Result<Result<M::Result, Error>, JsonError>
    where
        M: crate::method::Method,
    {
        // TODO(nlordell): Might be some standard library function for this.
        match self.result {
            Ok(value) => value.result::<M>().map(Ok),
            Err(err) => Ok(Err(err)),
        }
    }
}

impl<'de> Deserialize<'de> for Response {
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

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Response;

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
                            result = Some(map.next_value()?);
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

                // Note that some RPC servers return **both** `result` and
                // `error` fields in some conditions (such as auto-mine nodes
                // returning both when mining a transaction that reverts). In
                // these cases, the `result` is what is expected by the Ethereum
                // RPC standard, so prefer it (even if it not strictly JSON RPC
                // standard compatible).
                Ok(Response {
                    jsonrpc: jsonrpc.ok_or_else(|| de::Error::missing_field("jsonrpc"))?,
                    result: match (result, error) {
                        (Some(result), _) => Ok(result),
                        (None, Some(error)) => Err(error),
                        (None, None) => {
                            return Err(de::Error::custom("missing 'result' or 'error' field"));
                        }
                    },
                    id,
                })
            }
        }

        deserializer.deserialize_struct("Response", &["jsonrpc", "result", "error", "id"], Visitor)
    }
}

impl Serialize for Response {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Response<'a> {
            jsonrpc: Version,
            #[serde(skip_serializing_if = "Option::is_none")]
            result: Option<&'a Value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            error: Option<&'a Error>,
            #[serde(skip_serializing_if = "Option::is_none")]
            id: Option<Id>,
        }

        let (result, error) = match &self.result {
            Ok(result) => (Some(result), None),
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

/// An RPC error that may be produced on a response.
#[derive(Clone, Debug, Deserialize, Error, Serialize)]
#[error("{code}: {message}")]
#[serde(deny_unknown_fields)]
pub struct Error {
    pub code: ErrorCode,
    pub message: String,
    #[serde(default)]
    pub data: Value,
}

impl Error {
    /// Creates an error with a custom error message.
    pub fn custom(message: impl Into<String>) -> Self {
        Self {
            code: ErrorCode::from(-32000),
            message: message.into(),
            data: Value::default(),
        }
    }
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
            -32768..=-32100 => Self::Reserved(code),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        eth, net,
        types::{BlockId, Empty, Transaction},
    };
    use ethprim::address;
    use hex_literal::hex;
    use serde_json::json;

    fn roundtrip(
        call: serde_json::Value,
        result: serde_json::Value,
    ) -> impl FnOnce(Request) -> Result<Response, Box<dyn std::error::Error>> {
        move |request| {
            assert_eq!(
                call,
                json!({
                    "method": request.method,
                    "params": request.params,
                }),
            );
            Ok(Response {
                jsonrpc: Version::V2,
                result: Ok(Value(result)),
                id: Some(request.id),
            })
        }
    }

    #[test]
    fn calls() {
        let version = call(
            net::Version,
            Empty,
            roundtrip(
                json!({
                    "method": "net_version",
                    "params": [],
                }),
                json!("42"),
            ),
        )
        .unwrap();
        assert_eq!(version, 42);

        let output = call(
            eth::Call,
            (
                Transaction {
                    to: Some(address!("0x9008D19f58AAbD9eD0D60971565AA8510560ab41")),
                    input: Some(hex!("f698da25").to_vec()),
                    ..Default::default()
                },
                BlockId::default(),
            ),
            roundtrip(
                json!({
                    "method": "eth_call",
                    "params": [
                        {
                            "to": "0x9008D19f58AAbD9eD0D60971565AA8510560ab41",
                            "input": "0xf698da25",
                        },
                        "latest",
                    ],
                }),
                json!("0xc078f884a2676e1345748b1feace7b0abee5d00ecadb6e574dcdd109a63e8943"),
            ),
        )
        .unwrap();
        assert_eq!(
            output,
            hex!("c078f884a2676e1345748b1feace7b0abee5d00ecadb6e574dcdd109a63e8943")
        );
    }
}
