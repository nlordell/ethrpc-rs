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

fn main() {
    println!("Hello, world!");
}

fn server(requests: mpsc::Receiver<String>, responses: mpsc::Sender<String>) {
    while let Ok(request) = requests.recv() {
        if let Ok(request) = serde_json::from_str::<jsonrpc::Request<custom::Time>>(&request) {
            let _ = responses.send(
                serde_json::to_string(&jsonrpc::Response::<custom::Time> {
                    jsonrpc: request.jsonrpc,
                    result: Ok(SystemTime::now()),
                    id: Some(request.id),
                })
                .unwrap(),
            );
        } else if let Ok(request) = serde_json::from_str::<jsonrpc::Request<custom::Add>>(&request)
        {
            let (a, b) = request.params;
            let sum = a.checked_add(b).
            let _ = responses.send(
                serde_json::to_string(&jsonrpc::Response::<custom::Add> {
                    jsonrpc: request.jsonrpc,
                    result: request.params.0.chc,
                    id: Some(request.id),
                })
                .unwrap(),
            );
        } else {
        };
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

/*
more concrete request/response types
error creation helpers
potnetially JSON RPC server helpers
*/
