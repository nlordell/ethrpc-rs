//! Module containing concept of an Ethereum RPC batches.
//!
//! Currently, this implementation is a bit hacky, and I'm not 100% satisfied
//! with how the typing turned out, but it is somewhat ergonomic to use.
//! Specifically, the generic types here make quite a few assumptions and aren't
//! entirely generic.

use crate::{jsonrpc, method::Method};

pub type Request = jsonrpc::Request<String>;
pub type Response = jsonrpc::Response<String>;

/// A trait defining a batch Ethereum RPC requests.
pub trait Batch {
    type Results;
    type Values;

    fn serialize_requests(self) -> Result<Vec<Request>, serde_json::Error>;
    fn deserialize_responses(responses: Vec<Response>) -> Result<Self::Results, serde_json::Error>;
    fn values(results: Self::Results) -> Result<Self::Values, jsonrpc::Error>;
}

fn serialize_request<M>(method: &M, params: &M::Params) -> Result<Request, serde_json::Error>
where
    M: Method,
{
    Ok(Request {
        jsonrpc: jsonrpc::Version::V2,
        method: method.name().to_string(),
        params: M::serialize_params(params, serde_json::value::Serializer)?,
        id: jsonrpc::Id(0),
    })
}

fn deserialize_response<M>(
    response: Response,
) -> Result<Result<M::Result, jsonrpc::Error>, serde_json::Error>
where
    M: Method,
{
    match response.result.map(|value| M::deserialize_result(value)) {
        Ok(Ok(value)) => Ok(Ok(value)),
        Ok(Err(err)) => Err(err),
        Err(err) => Ok(Err(err)),
    }
}

macro_rules! impl_batch_for_tuple {
    ($($m:ident),*) => {
        impl<$($m,)*> Batch for ($(($m, <$m>::Params),)*)
        where
            $($m: Method,)*
        {
            type Results = ($(Result<<$m>::Result, jsonrpc::Error>,)*);
            type Values = ($(<$m>::Result,)*);

            fn serialize_requests(self) -> Result<Vec<Request>, serde_json::Error> {
                #[allow(non_snake_case)]
                let ($($m,)*) = self;
                Ok(vec![
                    $(serialize_request(&$m.0, &$m.1)?,)*
                ])
            }

            fn deserialize_responses(responses: Vec<Response>) -> Result<Self::Results, serde_json::Error> {
                #[allow(unused_mut, unused_variables)]
                let mut responses = responses.into_iter();
                Ok((
                    $(deserialize_response::<$m>(responses.next().unwrap())?,)*
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

    fn serialize_requests(self) -> Result<Vec<Request>, serde_json::Error> {
        self.iter()
            .map(|(method, params)| serialize_request(method, params))
            .collect()
    }

    fn deserialize_responses(responses: Vec<Response>) -> Result<Self::Results, serde_json::Error> {
        Ok(responses
            .into_iter()
            .map(deserialize_response::<M>)
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

    fn serialize_requests(self) -> Result<Vec<Request>, serde_json::Error> {
        self.iter()
            .map(|(method, params)| serialize_request(method, params))
            .collect()
    }

    fn deserialize_responses(responses: Vec<Response>) -> Result<Self::Results, serde_json::Error> {
        responses
            .into_iter()
            .map(deserialize_response::<M>)
            .collect()
    }

    fn values(results: Self::Results) -> Result<Self::Values, jsonrpc::Error> {
        results.into_iter().collect()
    }
}
