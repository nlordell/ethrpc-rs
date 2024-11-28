//! Module containing concept of an Ethereum RPC batches.
//!
//! Currently, this implementation is a bit hacky, and I'm not 100% satisfied
//! with how the typing turned out, but it is somewhat ergonomic to use.
//! Specifically, the generic types here make quite a few assumptions and aren't
//! entirely generic.

use crate::{
    jsonrpc::{self, Id, JsonError, Request, Response},
    method::Method,
};
use std::future::Future;
use thiserror::Error;

/// Executes a batch of JSON RPC calls with the provided roundtrip
/// implementation.
pub fn call<B, F, E>(batch: B, roundtrip: F) -> Result<B::Values, E>
where
    B: Batch,
    F: FnOnce(Vec<Request>) -> Result<Vec<Response>, E>,
    E: From<Error> + From<jsonrpc::Error> + From<JsonError>,
{
    let results = try_call(batch, roundtrip)?;
    let values = B::values(results)?;
    Ok(values)
}

/// Executes a batch of JSON RPC calls with the provided `async` roundtrip
/// implementation.
pub async fn call_async<B, F, Fut, E>(batch: B, roundtrip: F) -> Result<B::Values, E>
where
    B: Batch,
    F: FnOnce(Vec<Request>) -> Fut,
    Fut: Future<Output = Result<Vec<Response>, E>>,
    E: From<Error> + From<jsonrpc::Error> + From<JsonError>,
{
    let results = try_call_async(batch, roundtrip).await?;
    let values = B::values(results)?;
    Ok(values)
}

/// Executes a batch of JSON RPC calls with the provided roundtrip
/// implementation.
///
/// Returns the individual JSON RPC results for each request. This allows fine-
/// grained error handing for individual calls.
pub fn try_call<B, F, E>(batch: B, roundtrip: F) -> Result<B::Results, E>
where
    B: Batch,
    F: FnOnce(Vec<Request>) -> Result<Vec<Response>, E>,
    E: From<Error> + From<JsonError>,
{
    let (requests, ids) = requests(batch)?;
    let responses = roundtrip(requests)?;
    results::<B, E>(ids, responses)
}

/// Executes a batch of JSON RPC calls with the provided `async` roundtrip
/// implementation.
///
/// Returns the individual JSON RPC results for each request. This allows fine-
/// grained error handing for individual calls.
pub async fn try_call_async<B, F, Fut, E>(batch: B, roundtrip: F) -> Result<B::Results, E>
where
    B: Batch,
    F: FnOnce(Vec<Request>) -> Fut,
    Fut: Future<Output = Result<Vec<Response>, E>>,
    E: From<Error> + From<JsonError>,
{
    let (requests, ids) = requests(batch)?;
    let responses = roundtrip(requests).await?;
    results::<B, E>(ids, responses)
}

fn requests<B>(batch: B) -> Result<(Vec<Request>, Vec<Id>), JsonError>
where
    B: Batch,
{
    let requests = batch.into_requests()?;
    let ids = requests.iter().map(|request| request.id).collect();
    Ok((requests, ids))
}

fn results<B, E>(ids: Vec<Id>, mut responses: Vec<Response>) -> Result<B::Results, E>
where
    B: Batch,
    E: From<Error> + From<JsonError>,
{
    if ids.len() != responses.len() || responses.iter().any(|response| response.id.is_none()) {
        return Err(Error.into());
    }

    responses.sort_unstable_by_key(|response| response.id.unwrap().0);
    if responses
        .iter()
        .zip(ids)
        .any(|(response, id)| response.id.unwrap() != id)
    {
        return Err(Error.into());
    }

    let results = B::from_responses(responses)?;
    Ok(results)
}

/// A trait defining a batch Ethereum RPC requests.
pub trait Batch {
    type Results;
    type Values;

    fn into_requests(self) -> Result<Vec<Request>, JsonError>;
    fn from_responses(responses: Vec<Response>) -> Result<Self::Results, JsonError>;
    fn values(results: Self::Results) -> Result<Self::Values, jsonrpc::Error>;
}

macro_rules! impl_batch_for_tuple {
    ($($m:ident),*) => {
        impl<$($m,)*> Batch for ($(($m, <$m>::Params),)*)
        where
            $($m: Method,)*
        {
            type Results = ($(Result<<$m>::Result, jsonrpc::Error>,)*);
            type Values = ($(<$m>::Result,)*);

            fn into_requests(self) -> Result<Vec<Request>, JsonError> {
                #[allow(non_snake_case)]
                let ($($m,)*) = self;
                Ok(vec![
                    $(Request::new($m.0, $m.1)?,)*
                ])
            }

            fn from_responses(responses: Vec<Response>) -> Result<Self::Results, JsonError> {
                #[allow(unused_mut, unused_variables)]
                let mut responses = responses.into_iter();
                Ok((
                    $(responses.next().unwrap().result::<$m>()?,)*
                ))
            }

            fn values(results: Self::Results) -> Result<Self::Values, jsonrpc::Error> {
                #[allow(non_snake_case)]
                let ($($m,)*) = results;
                Ok(($($m?,)*))
            }
        }
    };
}

impl_batch_for_tuple!();
impl_batch_for_tuple!(M0);
impl_batch_for_tuple!(M0, M1);
impl_batch_for_tuple!(M0, M1, M2);
impl_batch_for_tuple!(M0, M1, M2, M3);
impl_batch_for_tuple!(M0, M1, M2, M3, M4);
impl_batch_for_tuple!(M0, M1, M2, M3, M4, M5);
impl_batch_for_tuple!(M0, M1, M2, M3, M4, M5, M6);
impl_batch_for_tuple!(M0, M1, M2, M3, M4, M5, M6, M7);
impl_batch_for_tuple!(M0, M1, M2, M3, M4, M5, M6, M7, M8);
impl_batch_for_tuple!(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9);
impl_batch_for_tuple!(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, Ma);
impl_batch_for_tuple!(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, Ma, Mb);
impl_batch_for_tuple!(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, Ma, Mb, Mc);
impl_batch_for_tuple!(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, Ma, Mb, Mc, Md);
impl_batch_for_tuple!(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, Ma, Mb, Mc, Md, Me);
impl_batch_for_tuple!(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, Ma, Mb, Mc, Md, Me, Mf);

impl<M, const N: usize> Batch for [(M, M::Params); N]
where
    M: Method,
{
    type Results = [Result<M::Result, jsonrpc::Error>; N];
    type Values = [M::Result; N];

    fn into_requests(self) -> Result<Vec<Request>, JsonError> {
        self.into_iter()
            .map(|(method, params)| Request::new(method, params))
            .collect()
    }

    fn from_responses(responses: Vec<Response>) -> Result<Self::Results, JsonError> {
        Ok(responses
            .into_iter()
            .map(Response::result::<M>)
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .unwrap_or_else(|_| panic!("incorrect length")))
    }

    fn values(results: Self::Results) -> Result<Self::Values, jsonrpc::Error> {
        Ok(results
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .unwrap_or_else(|_| panic!("incorrect length")))
    }
}

impl<M> Batch for Vec<(M, M::Params)>
where
    M: Method,
{
    type Results = Vec<Result<M::Result, jsonrpc::Error>>;
    type Values = Vec<M::Result>;

    fn into_requests(self) -> Result<Vec<Request>, JsonError> {
        self.into_iter()
            .map(|(method, params)| Request::new(method, params))
            .collect()
    }

    fn from_responses(responses: Vec<Response>) -> Result<Self::Results, JsonError> {
        responses.into_iter().map(Response::result::<M>).collect()
    }

    fn values(results: Self::Results) -> Result<Self::Values, jsonrpc::Error> {
        results.into_iter().collect()
    }
}

#[derive(Clone, Copy, Debug, Default, Error, PartialEq)]
#[error("JSON RPC batch responses do not match requests")]
pub struct Error;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        eth,
        types::{BlockTag, Empty, Hydrated, TransactionReceiptKind},
    };
    use serde_json::json;

    fn roundtrip(
        calls: serde_json::Value,
        results: serde_json::Value,
    ) -> impl FnOnce(Vec<Request>) -> Result<Vec<Response>, Box<dyn std::error::Error>> {
        move |requests| {
            for (call, request) in calls.as_array().unwrap().iter().zip(&requests) {
                assert_eq!(
                    call,
                    &json!({
                        "method": request.method,
                        "params": request.params,
                    }),
                );
            }
            let serde_json::Value::Array(results) = results else {
                panic!()
            };
            let responses = requests
                .iter()
                .zip(results)
                .map(|(request, result)| Response {
                    jsonrpc: jsonrpc::Version::V2,
                    result: Ok(jsonrpc::Value(result)),
                    id: Some(request.id),
                })
                .collect();
            Ok(responses)
        }
    }

    #[test]
    fn batch_request() {
        let (latest, safe, receipts) = call(
            (
                (eth::BlockNumber, Empty),
                (eth::GetBlockByNumber, (BlockTag::Safe.into(), Hydrated::Yes)),
                (eth::GetBlockReceipts, (18_460_382.into(),)),
            ),
            roundtrip(
                json!([
                    { "method": "eth_blockNumber", "params": [] },
                    { "method": "eth_getBlockByNumber", "params": ["safe", true] },
                    { "method": "eth_getBlockReceipts", "params": ["0x119aede"] },
                ]),
                serde_json::from_str(
                    r#"[
                      "0x1163fd1",
                      {
                        "baseFeePerGas": "0x1418f329",
                        "difficulty": "0x0",
                        "extraData": "0x",
                        "gasLimit": "0x1c9c380",
                        "gasUsed": "0xb76d",
                        "hash": "0xfc70e073ec6e1bb7f387698e4be418d7b1ff2216f625cdf41e1b80fb08029ef5",
                        "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
                        "miner": "0x4200000000000000000000000000000000000011",
                        "mixHash": "0x8bc8a48326d9309959f2452206c1c842be9fcc9840c789845caab9d290a9700c",
                        "nonce": "0x0000000000000000",
                        "number": "0x112",
                        "parentHash": "0xcfbc479d5f63476538db9ff15295167a6e91ecc9d6cf54be471eded689c251c9",
                        "receiptsRoot": "0x8c303881b3e408cc739f55ef3e5133075d552cbec9ee987aff6b71f58cf291ee",
                        "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
                        "size": "0x38a",
                        "stateRoot": "0x86be45fb7a22b8c12cbce8d538372b800377cc589de590b6d88f5f65dde97df8",
                        "timestamp": "0x65da607b",
                        "totalDifficulty": "0x0",
                        "transactions": [
                          {
                            "blockHash": "0xfc70e073ec6e1bb7f387698e4be418d7b1ff2216f625cdf41e1b80fb08029ef5",
                            "blockNumber": "0x112",
                            "from": "0xdeaddeaddeaddeaddeaddeaddeaddeaddead0001",
                            "gas": "0xf4240",
                            "gasPrice": "0x0",
                            "hash": "0x13a644af4f64ce801dcd57faa823e8952f2290b810fdd31995dda62977ed3df1",
                            "input": "0x015d8eb90000000000000000000000000000000000000000000000000000000001267f330000000000000000000000000000000000000000000000000000000065da6073000000000000000000000000000000000000000000000000000000075d3f861097c24796a4f639846a6a3ea3a59c11de8d89e11f551bae8feca9271a78926d420000000000000000000000000000000000000000000000000000000000000004000000000000000000000000415c8893d514f9bc5211d36eeda4183226b84aa700000000000000000000000000000000000000000000000000000000000000bc00000000000000000000000000000000000000000000000000000000000a6fe0",
                            "nonce": "0x111",
                            "to": "0x4200000000000000000000000000000000000015",
                            "transactionIndex": "0x0",
                            "value": "0x0",
                            "type": "0x7e",
                            "v": "0x0",
                            "r": "0x0",
                            "s": "0x0",
                            "sourceHash": "0x5a1b66228a26547e2ce6fe1aa734d2c34e062e42ccd5710463e6987890f1ad5d",
                            "mint": "0x0",
                            "depositReceiptVersion": "0x1"
                          }
                        ],
                        "transactionsRoot": "0xb01a2ad2c1ceb76f2edcdc02f2e0453c52e24c421fb04132cdfc0da5b70a593e",
                        "uncles": [],
                        "withdrawals": [],
                        "withdrawalsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"
                      },
                      [
                        {
                            "blockHash": "0x79313e7f7904f21e3e3f0abced0cd95b154bca0b4d0c4a5ddfbc70442c7f7205",
                            "blockNumber": "0x119aede",
                            "contractAddress": null,
                            "cumulativeGasUsed": "0xf0c1",
                            "effectiveGasPrice": "0x2bfe06c9d",
                            "from": "0xa3b458db8381dcc1fc4529a41ebe2804b07e7ef6",
                            "gasUsed": "0xf0c1",
                            "logs": [],
                            "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
                            "status": "0x0",
                            "to": "0x49048044d57e1c92a77f79988d21fa8faf74e97e",
                            "transactionHash": "0xdf3aa03889d1de2f198f31ccaeeb83019c2f9140cad911011a9d4d2849157393",
                            "transactionIndex": "0x0",
                            "type": "0x3",
                            "blobGasUsed": "0x123",
                            "blobGasPrice": "0x12345"
                        }
                      ]
                    ]"#,
                )
                .unwrap(),
            ),
        )
        .unwrap();
        assert_eq!(
            receipts.unwrap()[0].kind,
            TransactionReceiptKind::Eip4844 {
                blob_gas_used: 291_u64.into(),
                blob_gas_price: 74_565_u64.into(),
            }
        );
        assert_eq!(latest, 0x1163fd1);
        assert_eq!(safe.unwrap().number, 0x112);
    }
}
