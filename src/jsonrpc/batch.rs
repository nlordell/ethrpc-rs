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
        types::{BlockSpec, BlockTag, Empty, Hydrated, TransactionReceiptKind},
    };
    use ethprim::U256;
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
                (eth::GetBlockByNumber, (BlockTag::Safe.into(), Hydrated::No)),
                (eth::GetBlockReceipts, (BlockSpec::Number(U256::new(18_460_382)), )),
            ),
            roundtrip(
                json!([
                    { "method": "eth_blockNumber", "params": [] },
                    { "method": "eth_getBlockByNumber", "params": ["safe", false] },
                    { "method": "eth_getBlockReceipts", "params": ["0x119aede"] },
                ]),
                serde_json::from_str(
                    r#"[
                      "0x1163fd1",
                      {
                        "baseFeePerGas": "0x7b1d8527b",
                        "difficulty": "0x0",
                        "extraData": "0x6265617665726275696c642e6f7267",
                        "gasLimit": "0x1c9c380",
                        "gasUsed": "0xb0c0f4",
                        "hash": "0xf5fcd85eaff36e6739cf6637279f11d41884b5a5eebbc4a8cec911a1f66b8c24",
                        "baseFeePerGas": "0x7b1d8527b",
                        "difficulty": "0x0",
                        "extraData": "0x6265617665726275696c642e6f7267",
                        "gasLimit": "0x1c9c380",
                        "gasUsed": "0xb0c0f4",
                        "hash": "0xf5fcd85eaff36e6739cf6637279f11d41884b5a5eebbc4a8cec911a1f66b8c24",
                        "logsBloom": "0x132905054780d3061010123a9b0652211623be014d8b2244085983188ca01544b260136c419c09116e123b16097501f1034760008827ae30329cb29c202a05905096ec4a4d74bb2d7a22ea2f4c2a10ae8682e5e04f661cc08951758e926207081a8a2b0c1e03b82930081b422029686106390cd91405f720500427bb306800088216d6d74e4007912ccde043673a174630c887e9990588a84621327480fee2308fd0a3407b886f522c4252fa50d08722ae1e840eda8382e223f84ce610980a31090a009282002d6019660a52ac3b10c6c04b2a01e0e1b2933824210e0090a0a4e933a82ac364c0ee166433af2b5a5a8ad42051b68b1a42c858c9d171696a74ed",
                        "miner": "0x95222290dd7278aa3ddd389cc1e1d165cc4bafe5",
                        "mixHash": "0xbf5d2f41d6d0cc3fe263de5cf4fa0926149f242560f7e97e88a0f364e3a157d0",
                        "nonce": "0x0000000000000000",
                        "number": "0x1163fa3",
                        "parentHash": "0xc9db34a41ed456258cc1e0b90398930a1989852e371ab90df7fb24aab1f3d9b6",
                        "receiptsRoot": "0x6064134c0ec4cefa61150f64505611e657ee51df68cd375aa655ba81f93a349a",
                        "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
                        "size": "0x1ca6a",
                        "stateRoot": "0x0e04c8d1034d2a8aa80bbd3cf6e9052e9793616781bd97d10db50aef60d26d25",
                        "timestamp": "0x6515a3d7",
                        "totalDifficulty": "0xc70d815d562d3cfa955",
                        "transactions": [
                          "0x3120833ee5bee8e09c5016617a9f6a6a29f81e51e6faf6802a890a98ad08e0f4",
                          "0x3cd260bb570df2cef90512950ca4dda0821a670f0cd9d41e93edae6886620f0d",
                          "0x9063ac6333afc903314c871d89f48d33da2c2c2f4919fff86e4d4824392e0ac8",
                          "0x0674f4b3252b1f61e093a62c6ba0c2cecea3eb175e78e8bae981103d7c0467e1",
                          "0xd65bcc2d97c5118a689d214c4a86f639e31c03879425837a255f8a8f95facf99",
                          "0x6c9cbcddb671131f9f70b098c890152ee3cd9b0e6a22859c8f4395a962dbb4d2",
                          "0x60ff69f5ad5c35e3db507d0f7d6d6e5d04d5b2f8f75b27bee02f10b18829d961",
                          "0xd266669a939ccd11f3931e8faf16b1be79aa4bd185cce02dd8f7c344fe857024",
                          "0xc36af8c7901fab78601e61803870465d0b3e0193189e9c637105fd53371b9f9f",
                          "0x993a137061ea6a988931927543b0abaa193af180aa1c49dbfcb5a218914c86ce",
                          "0x3e82af8eb0ff682b0a0d695cbca700beb260f398e6de1404b7a963a9a62c88ff",
                          "0x3590dcf1a53fcfcc3f323cf6e16fd638223e8ab90bfd0eeaaf2809170c030fb0",
                          "0x54f904dd4d1b696f0862e6acf124e1a69e878424b792b920e270f96ff30a8a0a",
                          "0xa45386d9a698c05a0d2eba95d85598e2a17029c1c2143e5e2839e623a52b783e",
                          "0xc14c51358655cf9052a23d3fa6a3f8f15a7377ca8d35fa4855bb333f6f505922",
                          "0x81b414b16cca2d81f69a3804e9ffda792a1865ee00ea7766c3230d9c23fa3437",
                          "0xf3faf9cdba7515e8b5d0b1315304dde32b6a8fe8838af7e42d135352e495fc0d",
                          "0x2bbb0078b85c3e2f55fda5f5c9a26dd5d1b0d9fe97b2e7a1d36edeef26f73d63",
                          "0x6a180ead05891e31ea72e9633e8db6f29c2865bd05aa3bdb682c8437bb014d28",
                          "0x6b3e1cab87b540fd43c6686e06d594a68ad2610d384320d6a15eef4b6451c825",
                          "0xb4feac7835d838fe174efc5989875a202d6808d00dde188d6e44937a8826f511",
                          "0x51902d5c42db0847d551ab582c922e73cc8c0e38270aa35e515d10766a613ff1",
                          "0x282f90361ea8a985f2ca67a4c641505fb66dcb67a4765bcd347a939932d6bd88",
                          "0x36a4f6309c7473c6cb6a56aa2954e4840bd31879b079077b112db7af431d689d",
                          "0xe649f0dfd24e2d8f4b2068eb715971541687b2fa3869adf3643b74efe3940e30",
                          "0x56c65fc07c411c12b35d86f6e2872aa5bc574c9b37f8aa11263830f75b6fa367",
                          "0x066ac5dcb8c96a10908466ceb2cf8905efb4d32671d2f7ff6c522146b4b7eef8",
                          "0x7b69567c5938e9ac518a8ac0e09b5626e4cfde4b5ba3a4ced9f27a4f8900cbee",
                          "0x5a3fdde4e45c65988b02949f13ee161dd55f604db27d6270a67c9b1ba5b595d3",
                          "0xbbc5745a56096076157995b70a27691fd761b1e7506bac18ed36dff1cf7a6ba0",
                          "0xd89a5823481047300235e5a77e3fd7cf33c6e94018c6b37320365811d6d7f804",
                          "0x33cd2fbf1fbe0a09baa221aede6debbe41f6be68b7d865425b46c087e931ae9a",
                          "0xad555f0a7c861a1a3826852367b36c57914fc62328ec0a466bfd70cf8b51afd5",
                          "0xe8173aa24de0f9223ad8cb266b08f10a40e5cf8ccff3ca3ff3339bb8c60647d3",
                          "0xe11b62ccb6709f32b5edf8e39d6573e09af9fbcf54da6df23320f754949fa18d",
                          "0xc4bfe51111516f43721672822cac33e87e3d79d58b49adebaf4fb40f569ef930",
                          "0xd177840c008b823d05311018c7c31de7ad8ad3cfe448c91d8759189cad32c06c",
                          "0x8341385d18befca291af72f0b45039682425e46d75cac2be803d62122eccea55",
                          "0x6eacce2ff4d507a00438f2fcd0bb11bc11f53fcfae58227344390e52e71a4a75",
                          "0x7c3d85514b28503a711c45dfcc0ff6dacc00d7632928c164408114cfb34f2178",
                          "0xf23bf719eda5085c8d89b805cfbc4fe453a45b52df0bc38394924029a880e28f",
                          "0x0c73d272f0683cf2e358be963ea7db703de7a8dae359ed1c5f9e78ed3f1e667c",
                          "0xadb6ae52f9efaaa425072ef6830d3b2f5d002a02732fd6a5721b6ffcf1124960",
                          "0x40a8bb589e4e0ce605c15c45532864137bb392ca145d2b405895b365a9ed1b90",
                          "0x0b5d7eaf62ec8a1a4aba19be70ced3ae10a7eece0fd97b5f8a9c786d40411739",
                          "0x61cdb01dd7595d47a041493099d60cf0756c613fc4149c2702be4efc097886d0",
                          "0xf412605b9b6ddb0f52de624e77e9b9c91cb5f9b1022ca20b44a9b283c724008e",
                          "0x3609b42ca2aae79ea5da2e2ff2ae5b78c2e40b804a00702f3d59f6fa9a188130",
                          "0x93ac1a9833f90a9066d41b24ed4123eb41883a8a4ca95c40390f85a48091d2da",
                          "0x27b68bbab90c3f4d0f80fb95e8fdc937d4fb8e051815c71bbd32fb644e7df1e5",
                          "0xf0803f270011cbe4bb2d123c32625017f8dec495f7e565687b3f8a639e6e79a0",
                          "0x8c191147cf779be567cea548cfa5763fe5cc388dd3d8d06b950af172c7c89f6c",
                          "0x808d29abb4d3f398ed1031cbe7b91d7831c72b7933465fb92316c45804d1e114",
                          "0xfb6fd6861a175d805e68b6ca46fd95e8c0cec123e69b4667d273bf5196a6056f",
                          "0x812592694e17c2a018281249a42af7bdfa315b5cb05652cc24c1811e098021d5",
                          "0xb5a34cedba9599d3518f658a0744efe5cec27164ec1df9302b1dc865fa61f23b",
                          "0x1a1ff9325d55831fb3850340115ece6085bfeb8693139b81d1943c2ba69622b3",
                          "0x34a6ab763f81ae65980690a8ea97a17529d5d6ff968500a9296d1051b8ac712f",
                          "0xed9df7aa2c52390be46566a6238d15db61e90652217ca24a224eacfdb27ce86c",
                          "0x689dd9f66dc201ea67c19c3f331844d64ede9a56232819c5f4ae16c3db4e7441",
                          "0x3bb096c83f947512fd17c151aec4ed9f6872bc92415bc2a12dbe06ab7a02c4fc",
                          "0x599cb5b39130943f2a7a6c42cb22d53e33886933fbaabae0facf24955405f2e0",
                          "0x93372f5019c4f2103dac8b70df57577305fb3482b970ff348d2826f23a67c0f1",
                          "0xeb29f66a5f3c22506eb812da36daff5aa40b43ecf3b111852ecc0d068f373e27",
                          "0xed16e388f9cce6130194daa16a9c05a955c9548c701c3efa729f42ed0c3ba1f1",
                          "0x27551436d16551d18c04daaff7665571531593055d590602231580fdc427cce8",
                          "0x2ab642051edce4008b12bf9ab5d9755f8b83e6604dc28bfd12c15c40d6bee23b",
                          "0x93e0c9bbb710df0e237e446b8c42da3e63c4a6367a3414ed241d49c735aa2fb4",
                          "0x0795ead0f24c765a79cd8e5f9305c9fb8d41367bd73164ecbf74929774dcb4cc",
                          "0x76f99018c70a2b2255951d80980f873f8a54c24d96ad4ac44a67cb678d02c2a7",
                          "0xc5502fe73d3ce819d85a1d4058262acb2b39f40943fce7e8c0f6f178d27281a4",
                          "0x728a9d414cc5fa09657a159ecac137164b0462cbeff1eb1d6580f74d1851f7c7",
                          "0xd02454ccf6e354b44fb3c446093a25158311dd951333b60d8c857ea37ef87cd8",
                          "0x1de8a95b2922c1d7cd7ac385e224f8ad7a0314834d730e4d6321516585d39cb5",
                          "0xd257b3da5b9b664beaba1ac2330d7235833f5b5761f401472f01da5b04d27e00",
                          "0xd6b5bc5b02573e75438806b17326144c320f409128da80f99d5a0eb48da49e55",
                          "0x50b251cd258ef8fa8e2c1280de483abab5c02a5cb5c623af8b50c38ddfcd827a",
                          "0x90a089804c6312f3cfe78cd691c9ef5f7b5a12982cf0c2c8abc4665d0f2bcdb7",
                          "0xb74666d9fee935786074e68cdf5e196eb31d5a3ffeb9c9eb91b1250e5b626f06",
                          "0x32a82b569cb51ab03ca3c52d17d903e644d7ebe75422e3ec5a730babce5835c7",
                          "0x734b84ad355cad510beda85682ea93adda08126cea493220e5ff54f33cad4352",
                          "0x6ebe493f0be13dedb67f3aacb1df9808e6d57848c9fbb1f6d0ea2b7b110b2a25",
                          "0xe8022d63926137e217f238742ddfbac9e4f0e061ab9bd86c4dcc702a6060b41d",
                          "0x0893a11a17071802ef7253c56dcbedccb3cfe4008c65863fa9f909adee0b2dc9",
                          "0xd605e918f76b40e303cc0289ccd80e7f7467ca2967feb691413da592933ee3ab",
                          "0x5091b6080238f8e3f6b79b1088cf949f168ef3bc1b90f973c27febe4ad44498a",
                          "0x23634c534cbd4f397b7046c332b70adaae8d358d81db475939fe7ff11e84c5c4",
                          "0x2e29f76391bc4aec109545fac423629565ce104032598d840471261b1b64537c",
                          "0x43ff617004b6c9267629d593d2b64ed1db7fe124edc50138352368f12aaacc4c",
                          "0x71535ed541f6480b7b0a9d6673371a568e301daff86b992cd20053c9e973bf16",
                          "0x1d60a5768cf035aa96bcdc21d7352f2099e7963e00c9ac52d4fd432d4f055e27",
                          "0x21e4beed11dd54d433c40fbff4c236cb17d46ef4ce61d4aaee8c0129f191b46f",
                          "0x39d49167e8e621c76610a6af0f18b1b47a27ad04d75daaee92a7485f427cf167",
                          "0xb1850499cdefc378d8c75808c09a65437126a5f71669a1c7a47c1bf3ce8f6cfc",
                          "0x77e33a9317837e70bfcf91b0524d67668b10bf2e782567cf8c88a611a3948d42",
                          "0xaa6cbc6c7268dc05900065c2df2b9f91c72026dcbbc9ba00c689198f37bd0018",
                          "0x8f2bf4e42e1a3108fbe1b518471894a28245b6cfd9aa96c32d33dc1dbee0db5f",
                          "0x0bffed1228cde2302faa57ad72e98a66c44909cefb508db8586f4e88c372c0e8",
                          "0x1dac9febf7ffefa2128efb563802c19d0f9339547935c9c3f3e1516fe86577a6",
                          "0x26e11a23842a6cf9d576d4583804b4a75add0f8256b73de303dcd51fc93fc1a2",
                          "0x85a3b57347f7f88ea84c8d8693f5252e1d772db79dffc73c2c27624e746c1cbd",
                          "0xbd385148609bac1680549e1bf01099c38c15a909b0bc4f9ecf25ef71df38852c",
                          "0xddadf9b1a4a1046a3a5150c2b0f0669d8a3ffc1a8d02670772effac175cea5e8",
                          "0x9c36f2ee9b80cc172a3eaf6ff7a91198ff6825a035548058203314122ef977d9",
                          "0x67f0193961cf43affb0b561595fcf41535e42de291c3b4737f3556f21e729669",
                          "0x9ce41a3ec42999dc1fb52a5482c75373eb761ac6fef4df7476de6c8f6deeb4b2",
                          "0x9695086db3090cbfcd5f7778496dcc95d7a7107a3c6463f9f425b98d9bf78f5e",
                          "0x16387ff82118ae21df82b72d4a678534173515d9f0434045b19459fb03139ec4",
                          "0x4beead072e7a04786b256e78f0d3833488e46f56c516a5043055491f6828fd43",
                          "0x1eb8ac0607ce392d5bd5546b67dab83f651ed48274f13e7b61dc355e05d4dfe1",
                          "0x53cf21301bec49a965e4f653355c686a47d400be9a65ec58c3a9bb454c3d2e0d",
                          "0x1efd95ea615eaacce916ba9c5c2c1d212e180cd311d2e6ebc6356e78f7e84d4a",
                          "0x518177b40fa27d02758f10e97fc2a28f59bbc41b99cc4f0a721bd54b73737869",
                          "0x6628b7fb8b73a0fd28caf99ee3bd3fe9db30ca373a343553412df6a3b16f322b",
                          "0x5ad6b6391574cde5df7b4270ceca534b0609f26b377f8e4c219e245fc2f7d1b9",
                          "0x83c279e8ee809e9cca41a4094e4f4acc98ea3713108c600871bd12c2c38e848b"
                        ],
                        "transactionsRoot": "0x5aa864f4f036cb563310bff686d1d3843500754a4def76b2c6b36a33e41057ec",
                        "uncles": [],
                        "withdrawals": [
                          {
                            "address": "0xb9d7934878b5fb9610b3fe8a5e441e8fad7e293f",
                            "amount": "0xfa118b",
                            "index": "0x1250d26",
                            "validatorIndex": "0x9e425"
                          },
                          {
                            "address": "0xb9d7934878b5fb9610b3fe8a5e441e8fad7e293f",
                            "amount": "0xf9d074",
                            "index": "0x1250d27",
                            "validatorIndex": "0x9e426"
                          },
                          {
                            "address": "0xb9d7934878b5fb9610b3fe8a5e441e8fad7e293f",
                            "amount": "0xf8c228",
                            "index": "0x1250d28",
                            "validatorIndex": "0x9e427"
                          },
                          {
                            "address": "0xb9d7934878b5fb9610b3fe8a5e441e8fad7e293f",
                            "amount": "0xf8c771",
                            "index": "0x1250d29",
                            "validatorIndex": "0x9e428"
                          },
                          {
                            "address": "0xb9d7934878b5fb9610b3fe8a5e441e8fad7e293f",
                            "amount": "0xfa234f",
                            "index": "0x1250d2a",
                            "validatorIndex": "0x9e429"
                          },
                          {
                            "address": "0xb9d7934878b5fb9610b3fe8a5e441e8fad7e293f",
                            "amount": "0xfa2ef1",
                            "index": "0x1250d2b",
                            "validatorIndex": "0x9e42a"
                          },
                          {
                            "address": "0xfbfb5b26f14cae1f6c90bd999fed943a78bf4f48",
                            "amount": "0xf9d503",
                            "index": "0x1250d2c",
                            "validatorIndex": "0x9e42b"
                          },
                          {
                            "address": "0x8306300ffd616049fd7e4b0354a64da835c1a81c",
                            "amount": "0xfa2eab",
                            "index": "0x1250d2d",
                            "validatorIndex": "0x9e42c"
                          },
                          {
                            "address": "0x8710fb877c3f09cbbf7004d304e7a310a2157e21",
                            "amount": "0xf65836",
                            "index": "0x1250d2e",
                            "validatorIndex": "0x9e42d"
                          },
                          {
                            "address": "0x400a390014e896f71ad40647b9f42f0712b9cbaf",
                            "amount": "0xf1b4a9",
                            "index": "0x1250d2f",
                            "validatorIndex": "0x9e42e"
                          },
                          {
                            "address": "0x636ac64caf1bc1fc19f9cecd9c023355354cc4ef",
                            "amount": "0xfa1b36",
                            "index": "0x1250d30",
                            "validatorIndex": "0x9e42f"
                          },
                          {
                            "address": "0xe2a8a0db2abbcdb61e7d05a52248f306ddc32370",
                            "amount": "0xf78062",
                            "index": "0x1250d31",
                            "validatorIndex": "0x9e430"
                          },
                          {
                            "address": "0xb541105909a1d80ba28cb4db425026a4731b6cb2",
                            "amount": "0xf9e573",
                            "index": "0x1250d32",
                            "validatorIndex": "0x9e431"
                          },
                          {
                            "address": "0xb541105909a1d80ba28cb4db425026a4731b6cb2",
                            "amount": "0xf9ef82",
                            "index": "0x1250d33",
                            "validatorIndex": "0x9e432"
                          },
                          {
                            "address": "0xb541105909a1d80ba28cb4db425026a4731b6cb2",
                            "amount": "0xf9fb31",
                            "index": "0x1250d34",
                            "validatorIndex": "0x9e433"
                          },
                          {
                            "address": "0xb541105909a1d80ba28cb4db425026a4731b6cb2",
                            "amount": "0xfa0554",
                            "index": "0x1250d35",
                            "validatorIndex": "0x9e434"
                          }
                        ],
                        "withdrawalsRoot": "0x0c6cfaf85519d8d390d3100202d6b9205482382d5f97caca01cf2dbb3a0365d9"
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
        assert_eq!(receipts.unwrap()[0].kind, TransactionReceiptKind::Eip1559);
        assert_eq!(latest, 0x1163fd1);
        assert_eq!(safe.unwrap().number, 0x1163fa3);
    }
}
