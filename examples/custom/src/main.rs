//! Example to demonstrate:
//! 1. Using `ethrpc` over something that is not HTTP. In this contrived
//!    example, we process requests in a separate "server" thread using MPSC
//!    channels.
//! 2. Custom RPC methods defined with the macro.

use ethrpc::{jsonrpc, types::Empty};
use std::{sync::mpsc, time::SystemTime};

ethrpc::module! {
    /// A custom namespace.
    pub mod custom {
        /// Gets the current time.
        pub struct Time as "custom_time"
            Empty => SystemTime [serialize_time];

        /// Adds two numbers.
        pub struct Add as "custom_add"
            (u64, u64) => u64;
    }
}

/// Custom serialization logic for the custom RPC module.
mod serialize_time {
    use serde::{de, ser, Deserializer, Serializer};
    use std::time::SystemTime;

    pub fn serialize<S>(value: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = value
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|_| ser::Error::custom("system time before epoch"))?
            .as_millis()
            .try_into()
            .map_err(|_| ser::Error::custom("too far in the future"))?;
        serializer.serialize_u64(timestamp)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}

fn server(requests: mpsc::Receiver<jsonrpc::Request>, responses: mpsc::Sender<jsonrpc::Response>) {
    macro_rules! handle {
        ($req:expr; $($t:expr => |$r:ident| $f:block)*) => {{
            let req: jsonrpc::Request = $req;
            match req.method.as_str() {
                $(m if m == ethrpc::method::Method::name(&$t) => {
                    let id = req.id;
                    let $r = param_of($t, req.params);
                    let result = { $f };
                    jsonrpc::Response {
                        jsonrpc: jsonrpc::Version::V2,
                        id: Some(id),
                        result: result.map(|value| result_of($t, value)),
                    }
                })*
                method => jsonrpc::Response {
                    jsonrpc: jsonrpc::Version::V2,
                    id: Some(req.id),
                    result: Err(jsonrpc::Error {
                        code: jsonrpc::ErrorCode::MethodNotFound,
                        message: format!("unknown method '{method}'"),
                        data: jsonrpc::Value::default(),
                    })
                }
            }
        }}
    }

    while let Ok(request) = requests.recv() {
        let _ = responses.send(handle! {
            request;
            custom::Time => |_params| {
                Ok(SystemTime::now())
            }
            custom::Add => |params| {
                let (a, b) = params;
                a.checked_add(b)
                    .ok_or_else(|| jsonrpc::Error::custom("overflow"))
            }
        });
    }
}

fn param_of<M>(_: M, value: jsonrpc::Value) -> M::Params
where
    M: ethrpc::method::Method,
{
    value.params::<M>().unwrap()
}

fn result_of<M>(_: M, result: M::Result) -> jsonrpc::Value
where
    M: ethrpc::method::Method,
{
    jsonrpc::Value::for_result::<M>(result).unwrap()
}
