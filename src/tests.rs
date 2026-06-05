use crate::{eth, types::*};
use ethprim::{address, digest, uint};
use hex_literal::hex;
use serde_json::json;
use std::collections::HashMap;

macro_rules! assert_method_serialization {
    ($m:ty {
        $pv:expr => $pj:expr;
        $rv:expr => $rj:expr;
    }) => {{
        let pv = $pv;
        let pj = $pj;
        assert_eq!(
            <$m as $crate::method::Method>::serialize_params(&pv, ::serde_json::value::Serializer)
                .unwrap(),
            pj
        );
        assert_eq!(
            <$m as $crate::method::Method>::deserialize_params(pj).unwrap(),
            pv
        );
        let rv = $rv;
        let rj = $rj;
        assert_eq!(
            <$m as $crate::method::Method>::serialize_result(&rv, ::serde_json::value::Serializer)
                .unwrap(),
            rj
        );
        assert_eq!(
            <$m as $crate::method::Method>::deserialize_result(rj).unwrap(),
            rv
        );
    }};
}

#[test]
fn eth_accounts() {
    assert_method_serialization!(eth::Accounts {
        Empty => json!([]);
        vec![
            address!("0xD1F5279BE4B4dD94133A23deE1B23F5bfC0Db1d0"),
        ] => json!([
            "0xD1F5279BE4B4dD94133A23deE1B23F5bfC0Db1d0",
        ]);
    });
}

#[test]
fn eth_blob_base_fee() {
    assert_method_serialization!(eth::BlobBaseFee {
        Empty => json!([]);
        uint!("0x3f5694c1f") => json!("0x3f5694c1f");
    });
}

#[test]
fn eth_block_number() {
    assert_method_serialization!(eth::BlockNumber {
        Empty => json!([]);
        0x2377 => json!("0x2377");
    });
}

#[test]
fn eth_call() {
    assert_method_serialization!(eth::Call {
        (
            Transaction {
                to: Some(address!("0x69498dd54BD25AA0c886cF1f8B8aE0856d55fF13")),
                value: Some(uint!("0x1")),
                ..Default::default()
            },
            Some(BlockId::Tag(BlockTag::Latest)),
        ) => json!([
            {
                "to": "0x69498dd54BD25AA0c886cF1f8B8aE0856d55fF13",
                "value": "0x1",
            },
            "latest",
        ]);
        vec![] => json!("0x");
    });
}

#[test]
fn eth_chain_id() {
    assert_method_serialization!(eth::ChainId {
        Empty => json!([]);
        0x1 => json!("0x1");
    });
}

#[test]
fn eth_coinbase() {
    assert_method_serialization!(eth::Coinbase {
        Empty => json!([]);
        address!("0xFE3B557E8Fb62b89F4916B721be55cEb828dBd73")
            => json!("0xFE3B557E8Fb62b89F4916B721be55cEb828dBd73");
    });
}

#[test]
fn eth_config() {
    assert_method_serialization!(eth::Config {
        Empty => json!([]);
        Configuration {
            current: ForkConfiguration {
                activation_time: 1746612311,
                blob_schedule: BlobSchedule {
                    base_fee_update_fraction: 5007716,
                    max: 9,
                    target: 6,
                },
                chain_id: 0x1,
                fork_id: hex!("3ff0e375"),
                precompiles: HashMap::from([
                    ("ECREC".to_owned(),
                        address!("0x0000000000000000000000000000000000000001")),
                    ("SHA256".to_owned(),
                        address!("0x0000000000000000000000000000000000000002")),
                    ("RIPEMD160".to_owned(),
                        address!("0x0000000000000000000000000000000000000003")),
                    ("ID".to_owned(),
                        address!("0x0000000000000000000000000000000000000004")),
                    ("MODEXP".to_owned(),
                        address!("0x0000000000000000000000000000000000000005")),
                    ("BN254_ADD".to_owned(),
                        address!("0x0000000000000000000000000000000000000006")),
                    ("BN254_MUL".to_owned(),
                        address!("0x0000000000000000000000000000000000000007")),
                    ("BN254_PAIRING".to_owned(),
                        address!("0x0000000000000000000000000000000000000008")),
                    ("BLAKE2F".to_owned(),
                        address!("0x0000000000000000000000000000000000000009")),
                    ("KZG_POINT_EVALUATION".to_owned(),
                        address!("0x000000000000000000000000000000000000000A")),
                    ("BLS12_G1ADD".to_owned(),
                        address!("0x000000000000000000000000000000000000000b")),
                    ("BLS12_G1MSM".to_owned(),
                        address!("0x000000000000000000000000000000000000000C")),
                    ("BLS12_G2ADD".to_owned(),
                        address!("0x000000000000000000000000000000000000000d")),
                    ("BLS12_G2MSM".to_owned(),
                        address!("0x000000000000000000000000000000000000000E")),
                    ("BLS12_PAIRING_CHECK".to_owned(),
                        address!("0x000000000000000000000000000000000000000F")),
                    ("BLS12_MAP_FP_TO_G1".to_owned(),
                        address!("0x0000000000000000000000000000000000000010")),
                    ("BLS12_MAP_FP2_TO_G2".to_owned(),
                        address!("0x0000000000000000000000000000000000000011")),
                ]),
                system_contracts: HashMap::from([
                    ("BEACON_ROOTS_ADDRESS".to_owned(),
                        address!("0x000F3df6D732807Ef1319fB7B8bB8522d0Beac02")),
                    ("CONSOLIDATION_REQUEST_PREDEPLOY_ADDRESS".to_owned(),
                        address!("0x0000BBdDc7CE488642fb579F8B00f3a590007251")),
                    ("DEPOSIT_CONTRACT_ADDRESS".to_owned(),
                        address!("0x00000000219ab540356cBB839Cbe05303d7705Fa")),
                    ("HISTORY_STORAGE_ADDRESS".to_owned(),
                        address!("0x0000F90827F1C53a10cb7A02335B175320002935")),
                    ("WITHDRAWAL_REQUEST_PREDEPLOY_ADDRESS".to_owned(),
                        address!("0x00000961Ef480Eb55e80D19ad83579A64c007002")),
                ]),
            },
            next: None,
            last: None,
        } => json!({
            "current": {
                "activationTime": 1746612311,
                "blobSchedule": {
                    "baseFeeUpdateFraction": 5007716,
                    "max": 9,
                    "target": 6,
                },
                "chainId": 1,
                "forkId": "0x3ff0e375",
                "precompiles": {
                    "ECREC": "0x0000000000000000000000000000000000000001",
                    "SHA256": "0x0000000000000000000000000000000000000002",
                    "RIPEMD160": "0x0000000000000000000000000000000000000003",
                    "ID": "0x0000000000000000000000000000000000000004",
                    "MODEXP": "0x0000000000000000000000000000000000000005",
                    "BN254_ADD": "0x0000000000000000000000000000000000000006",
                    "BN254_MUL": "0x0000000000000000000000000000000000000007",
                    "BN254_PAIRING": "0x0000000000000000000000000000000000000008",
                    "BLAKE2F": "0x0000000000000000000000000000000000000009",
                    "KZG_POINT_EVALUATION": "0x000000000000000000000000000000000000000A",
                    "BLS12_G1ADD": "0x000000000000000000000000000000000000000b",
                    "BLS12_G1MSM": "0x000000000000000000000000000000000000000C",
                    "BLS12_G2ADD": "0x000000000000000000000000000000000000000d",
                    "BLS12_G2MSM": "0x000000000000000000000000000000000000000E",
                    "BLS12_PAIRING_CHECK": "0x000000000000000000000000000000000000000F",
                    "BLS12_MAP_FP_TO_G1": "0x0000000000000000000000000000000000000010",
                    "BLS12_MAP_FP2_TO_G2": "0x0000000000000000000000000000000000000011",
                },
                "systemContracts": {
                    "BEACON_ROOTS_ADDRESS": "0x000F3df6D732807Ef1319fB7B8bB8522d0Beac02",
                    "CONSOLIDATION_REQUEST_PREDEPLOY_ADDRESS": "0x0000BBdDc7CE488642fb579F8B00f3a590007251",
                    "DEPOSIT_CONTRACT_ADDRESS": "0x00000000219ab540356cBB839Cbe05303d7705Fa",
                    "HISTORY_STORAGE_ADDRESS": "0x0000F90827F1C53a10cb7A02335B175320002935",
                    "WITHDRAWAL_REQUEST_PREDEPLOY_ADDRESS": "0x00000961Ef480Eb55e80D19ad83579A64c007002",
                },
            },
            "next": null,
            "last": null,
        });
    });
}

#[test]
fn eth_create_access_list() {
    assert_method_serialization!(eth::CreateAccessList {
        (
            Transaction {
                from: Some(address!("0xaeA8F8f781326bfE6A7683C2BD48Dd6AA4d3Ba63")),
                input: Some(hex!("608060806080608155").to_vec()),
                ..Default::default()
            },
            Some(BlockSpec::Tag(BlockTag::Latest)),
        ) => json!([
            {
                "from": "0xaeA8F8f781326bfE6A7683C2BD48Dd6AA4d3Ba63",
                "input": "0x608060806080608155",
            },
            "latest",
        ]);
        AccessListResult {
            access_list: vec![AccessListEntry {
                address: address!("0xa02457E5Dfd32Bda5Fc7e1f1b008Aa5979568150"),
                storage_keys: vec![digest!("0x0000000000000000000000000000000000000000000000000000000000000081")],
            }],
            error: None,
            gas_used: 0x125f8,
        } => json!({
            "accessList": [{
                "address": "0xa02457E5Dfd32Bda5Fc7e1f1b008Aa5979568150",
                "storageKeys": ["0x0000000000000000000000000000000000000000000000000000000000000081"],
            }],
            "gasUsed": "0x125f8",
        });
    });
}

#[test]
fn eth_estimate_gas() {
    assert_method_serialization!(eth::EstimateGas {
        (
            Transaction {
                from: Some(address!("0xFE3B557E8Fb62b89F4916B721be55cEb828dBd73")),
                to: Some(address!("0x44Aa93095D6749A706051658B970b941c72c1D53")),
                value: Some(uint!("0x1")),
                ..Default::default()
            },
            None,
        ) => json!([{
            "from": "0xFE3B557E8Fb62b89F4916B721be55cEb828dBd73",
            "to": "0x44Aa93095D6749A706051658B970b941c72c1D53",
            "value": "0x1",
        }]);
        0x5208 => json!("0x5208");
    });
}

#[test]
fn eth_fee_history() {
    assert_method_serialization!(eth::FeeHistory {
        (0x5, BlockSpec::Tag(BlockTag::Latest), vec![20., 30.])
            => json!(["0x5", "latest", [20., 30.]]);
        FeeHistoryResult {
            oldest_block: 0x10b52f,
            base_fee_per_gas: vec![
                uint!("0x3fa63a3f"),
                uint!("0x37f999ee"),
                uint!("0x3e36f20a"),
                uint!("0x4099f79a"),
                uint!("0x430d532d"),
                uint!("0x46fcd4a4"),
            ],
            gas_used_ratio: vec![
                0.017712333333333333,
                0.9458865666666667,
                0.6534561,
                0.6517375666666667,
                0.7347769666666667,
            ],
            reward: Some(vec![
                vec![uint!("0x3b9aca00"), uint!("0x59682f00")],
                vec![uint!("0x3a13012"), uint!("0x3a13012")],
                vec![uint!("0x3a13012"), uint!("0x3a13012")],
                vec![uint!("0xf4240"), uint!("0xf4240")],
                vec![uint!("0xf4240"), uint!("0xf4240")],
            ]),
            base_fee_per_blob_gas: Some(vec![
                uint!("0x7b7609c19"),
                uint!("0x6dbe41789"),
                uint!("0x7223341d4"),
                uint!("0x6574a002c"),
                uint!("0x7223341d4"),
                uint!("0x6574a002c"),
            ]),
            blob_gas_used_ratio: Some(vec![0., 0.6666666666666666, 0., 1., 0.]),
        } => json!({
            "oldestBlock": "0x10b52f",
            "baseFeePerGas": [
                "0x3fa63a3f",
                "0x37f999ee",
                "0x3e36f20a",
                "0x4099f79a",
                "0x430d532d",
                "0x46fcd4a4",
            ],
            "gasUsedRatio": [
                0.017712333333333333,
                0.9458865666666667,
                0.6534561,
                0.6517375666666667,
                0.7347769666666667,
            ],
            "reward": [
                ["0x3b9aca00", "0x59682f00"],
                ["0x3a13012", "0x3a13012"],
                ["0x3a13012", "0x3a13012"],
                ["0xf4240", "0xf4240"],
                ["0xf4240", "0xf4240"],
            ],
            "baseFeePerBlobGas": [
                "0x7b7609c19",
                "0x6dbe41789",
                "0x7223341d4",
                "0x6574a002c",
                "0x7223341d4",
                "0x6574a002c",
            ],
            "blobGasUsedRatio": [0., 0.6666666666666666, 0., 1., 0.],
        });
    });
}

#[test]
fn eth_gas_price() {
    assert_method_serialization!(eth::GasPrice {
        Empty => json!([]);
        uint!("0x3e8") => json!("0x3e8");
    });
}

#[test]
fn eth_get_balance() {
    assert_method_serialization!(eth::GetBalance {
        (
            address!("0xFE3B557E8Fb62b89F4916B721be55cEb828dBd73"),
            BlockId::Tag(BlockTag::Latest),
        ) => json!([
            "0xFE3B557E8Fb62b89F4916B721be55cEb828dBd73",
            "latest",
        ]);
        uint!("0x1cfe56f3795885980000") => json!("0x1cfe56f3795885980000");
    });
}

#[test]
fn eth_get_block_by_hash() {
    assert_method_serialization!(eth::GetBlockByHash {
        (
            digest!("0xd5f1812548be429cbdc6376b29611fc49e06f1359758c4ceaaa3b393e2239f9c"),
            Hydrated::No,
        ) => json!([
            "0xd5f1812548be429cbdc6376b29611fc49e06f1359758c4ceaaa3b393e2239f9c",
            false,
        ]);
        Some(Block {
            hash: digest!("0xd5f1812548be429cbdc6376b29611fc49e06f1359758c4ceaaa3b393e2239f9c"),
            parent_hash: digest!("0x1f68ac259155e2f38211ddad0f0a15394d55417b185a93923e2abe71bb7a4d6d"),
            sha3_uncles: digest!("0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"),
            miner: address!("0xB42b6c4A95406C78FF892D270AD20b22642e102D"),
            state_root: digest!("0x91309efa7e42c1f137f31fe9edbe88ae087e6620d0d59031324da3e2f4f93233"),
            transactions_root: digest!("0x4d0c8e91e16bdff538c03211c5c73632ed054d00a7e210c0eb25146c20048126"),
            receipts_root: digest!("0x68461ab700003503a305083630a8fb8d14927238f0bc8b6b3d246c0c64f21f4a"),
            logs_bloom: Bloom(hex!(
                "0000000000000010000000408000000000050000000000000002000010000000\
                 0800001000000004000001000000000000000800040010000020100000000400\
                 0000100000000000000000400000000000000400000000000000000000000000\
                 0000040000240000000000000000000000000000000400000400000000000084\
                 0000000800000080010004000000001000000800000000000000000000000000\
                 0000000008000000000000400000000200000000000000000008000004000000\
                 0000000000000000060000040000000000200000000000000000000000400000\
                 0000000000100000000000000000000000000000000000040000900010000000"
            )),
            difficulty: uint!("0x66e619a"),
            number: 0x68b3,
            gas_limit: 0x47e7c4,
            gas_used: 0x37993,
            timestamp: 0x5835c54d,
            extra_data: hex!("d583010502846765746885676f312e37856c696e7578").to_vec(),
            mix_hash: digest!("0x24900fb3da77674a861c428429dce0762707ecb6052325bbd9b3c64e74b5af9d"),
            nonce: BlockNonce(hex!("378da40ff335b070")),
            base_fee_per_gas: uint!("0x7"),
            withdrawals_root: digest!("0x7a4ecf19774d15cf9c15adf0dd8e8a250c128b26c9e2ab2a08d6c9c8ffbd104f"),
            blob_gas_used: 0x0,
            excess_blob_gas: 0x0,
            parent_beacon_block_root: digest!("0x95c4dbd5b19f6fe3cbc3183be85ff4e85ebe75c5b4fc911f1c91e5b7a554a685"),
            requests_hash: digest!("0x0000000000000000000000000000000000000000000000000000000000000000"),
            size: 0x334,
            transactions: BlockTransactions::Hash(vec![
                digest!("0xa0807e117a8dd124ab949f460f08c36c72b710188f01609595223b325e58e0fc"),
                digest!("0xeae6d797af50cb62a596ec3939114d63967c374fa57de9bc0f4e2b576ed6639d"),
            ]),
            withdrawals: vec![Withdrawal {
                address: address!("0xB9D7934878B5FB9610B3fE8A5e441e8fad7E293f"),
                amount: 0x11a33e3760,
                index: 0x0,
                validator_index: 0x9d8c0,
            }],
            uncles: vec![],
        }) => json!({
            "baseFeePerGas": "0x7",
            "blobGasUsed": "0x0",
            "difficulty": "0x66e619a",
            "excessBlobGas": "0x0",
            "extraData": "0xd583010502846765746885676f312e37856c696e7578",
            "gasLimit": "0x47e7c4",
            "gasUsed": "0x37993",
            "hash": "0xd5f1812548be429cbdc6376b29611fc49e06f1359758c4ceaaa3b393e2239f9c",
            "logsBloom":
                "0x0000000000000010000000408000000000050000000000000002000010000000\
                   0800001000000004000001000000000000000800040010000020100000000400\
                   0000100000000000000000400000000000000400000000000000000000000000\
                   0000040000240000000000000000000000000000000400000400000000000084\
                   0000000800000080010004000000001000000800000000000000000000000000\
                   0000000008000000000000400000000200000000000000000008000004000000\
                   0000000000000000060000040000000000200000000000000000000000400000\
                   0000000000100000000000000000000000000000000000040000900010000000",
            "miner": "0xB42b6c4A95406C78FF892D270AD20b22642e102D",
            "mixHash": "0x24900fb3da77674a861c428429dce0762707ecb6052325bbd9b3c64e74b5af9d",
            "nonce": "0x378da40ff335b070",
            "number": "0x68b3",
            "parentBeaconBlockRoot": "0x95c4dbd5b19f6fe3cbc3183be85ff4e85ebe75c5b4fc911f1c91e5b7a554a685",
            "parentHash": "0x1f68ac259155e2f38211ddad0f0a15394d55417b185a93923e2abe71bb7a4d6d",
            "receiptsRoot": "0x68461ab700003503a305083630a8fb8d14927238f0bc8b6b3d246c0c64f21f4a",
            "requestsHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
            "size": "0x334",
            "stateRoot": "0x91309efa7e42c1f137f31fe9edbe88ae087e6620d0d59031324da3e2f4f93233",
            "timestamp": "0x5835c54d",
            "transactions": [
                "0xa0807e117a8dd124ab949f460f08c36c72b710188f01609595223b325e58e0fc",
                "0xeae6d797af50cb62a596ec3939114d63967c374fa57de9bc0f4e2b576ed6639d",
            ],
            "transactionsRoot": "0x4d0c8e91e16bdff538c03211c5c73632ed054d00a7e210c0eb25146c20048126",
            "uncles": [],
            "withdrawals": [
                {
                    "address": "0xB9D7934878B5FB9610B3fE8A5e441e8fad7E293f",
                    "amount": "0x11a33e3760",
                    "index": "0x0",
                    "validatorIndex": "0x9d8c0",
                },
            ],
            "withdrawalsRoot": "0x7a4ecf19774d15cf9c15adf0dd8e8a250c128b26c9e2ab2a08d6c9c8ffbd104f",
        });
    });
}

#[test]
fn eth_get_block_by_number() {
    assert_method_serialization!(eth::GetBlockByNumber {
        (BlockSpec::Number(0x68b3), Hydrated::No) => json!(["0x68b3", false]);
        Some(Block {
            hash: digest!("0xd5f1812548be429cbdc6376b29611fc49e06f1359758c4ceaaa3b393e2239f9c"),
            parent_hash: digest!("0x1f68ac259155e2f38211ddad0f0a15394d55417b185a93923e2abe71bb7a4d6d"),
            sha3_uncles: digest!("0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"),
            miner: address!("0xB42b6c4A95406C78FF892D270AD20b22642e102D"),
            state_root: digest!("0x91309efa7e42c1f137f31fe9edbe88ae087e6620d0d59031324da3e2f4f93233"),
            transactions_root: digest!("0x4d0c8e91e16bdff538c03211c5c73632ed054d00a7e210c0eb25146c20048126"),
            receipts_root: digest!("0x68461ab700003503a305083630a8fb8d14927238f0bc8b6b3d246c0c64f21f4a"),
            logs_bloom: Bloom(hex!(
                "0000000000000010000000408000000000050000000000000002000010000000\
                 0800001000000004000001000000000000000800040010000020100000000400\
                 0000100000000000000000400000000000000400000000000000000000000000\
                 0000040000240000000000000000000000000000000400000400000000000084\
                 0000000800000080010004000000001000000800000000000000000000000000\
                 0000000008000000000000400000000200000000000000000008000004000000\
                 0000000000000000060000040000000000200000000000000000000000400000\
                 0000000000100000000000000000000000000000000000040000900010000000"
            )),
            difficulty: uint!("0x66e619a"),
            number: 0x68b3,
            gas_limit: 0x47e7c4,
            gas_used: 0x37993,
            timestamp: 0x5835c54d,
            extra_data: hex!("d583010502846765746885676f312e37856c696e7578").to_vec(),
            mix_hash: digest!("0x24900fb3da77674a861c428429dce0762707ecb6052325bbd9b3c64e74b5af9d"),
            nonce: BlockNonce(hex!("378da40ff335b070")),
            base_fee_per_gas: uint!("0x7"),
            withdrawals_root: digest!("0x7a4ecf19774d15cf9c15adf0dd8e8a250c128b26c9e2ab2a08d6c9c8ffbd104f"),
            blob_gas_used: 0x0,
            excess_blob_gas: 0x0,
            parent_beacon_block_root: digest!("0x95c4dbd5b19f6fe3cbc3183be85ff4e85ebe75c5b4fc911f1c91e5b7a554a685"),
            requests_hash: digest!("0x0000000000000000000000000000000000000000000000000000000000000000"),
            size: 0x334,
            transactions: BlockTransactions::Hash(vec![
                digest!("0xa0807e117a8dd124ab949f460f08c36c72b710188f01609595223b325e58e0fc"),
                digest!("0xeae6d797af50cb62a596ec3939114d63967c374fa57de9bc0f4e2b576ed6639d"),
            ]),
            withdrawals: vec![Withdrawal {
                address: address!("0xB9D7934878B5FB9610B3fE8A5e441e8fad7E293f"),
                amount: 0x11a33e3760,
                index: 0x0,
                validator_index: 0x9d8c0,
            }],
            uncles: vec![],
        }) => json!({
            "baseFeePerGas": "0x7",
            "blobGasUsed": "0x0",
            "difficulty": "0x66e619a",
            "excessBlobGas": "0x0",
            "extraData": "0xd583010502846765746885676f312e37856c696e7578",
            "gasLimit": "0x47e7c4",
            "gasUsed": "0x37993",
            "hash": "0xd5f1812548be429cbdc6376b29611fc49e06f1359758c4ceaaa3b393e2239f9c",
            "logsBloom":
                "0x0000000000000010000000408000000000050000000000000002000010000000\
                   0800001000000004000001000000000000000800040010000020100000000400\
                   0000100000000000000000400000000000000400000000000000000000000000\
                   0000040000240000000000000000000000000000000400000400000000000084\
                   0000000800000080010004000000001000000800000000000000000000000000\
                   0000000008000000000000400000000200000000000000000008000004000000\
                   0000000000000000060000040000000000200000000000000000000000400000\
                   0000000000100000000000000000000000000000000000040000900010000000",
            "miner": "0xB42b6c4A95406C78FF892D270AD20b22642e102D",
            "mixHash": "0x24900fb3da77674a861c428429dce0762707ecb6052325bbd9b3c64e74b5af9d",
            "nonce": "0x378da40ff335b070",
            "number": "0x68b3",
            "parentBeaconBlockRoot": "0x95c4dbd5b19f6fe3cbc3183be85ff4e85ebe75c5b4fc911f1c91e5b7a554a685",
            "parentHash": "0x1f68ac259155e2f38211ddad0f0a15394d55417b185a93923e2abe71bb7a4d6d",
            "receiptsRoot": "0x68461ab700003503a305083630a8fb8d14927238f0bc8b6b3d246c0c64f21f4a",
            "requestsHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
            "size": "0x334",
            "stateRoot": "0x91309efa7e42c1f137f31fe9edbe88ae087e6620d0d59031324da3e2f4f93233",
            "timestamp": "0x5835c54d",
            "transactions": [
                "0xa0807e117a8dd124ab949f460f08c36c72b710188f01609595223b325e58e0fc",
                "0xeae6d797af50cb62a596ec3939114d63967c374fa57de9bc0f4e2b576ed6639d",
            ],
            "transactionsRoot": "0x4d0c8e91e16bdff538c03211c5c73632ed054d00a7e210c0eb25146c20048126",
            "uncles": [],
            "withdrawals": [
                {
                    "address": "0xB9D7934878B5FB9610B3fE8A5e441e8fad7E293f",
                    "amount": "0x11a33e3760",
                    "index": "0x0",
                    "validatorIndex": "0x9d8c0",
                },
            ],
            "withdrawalsRoot": "0x7a4ecf19774d15cf9c15adf0dd8e8a250c128b26c9e2ab2a08d6c9c8ffbd104f",
        });
    });
}

#[test]
fn eth_get_block_receipts() {
    assert_method_serialization!(eth::GetBlockReceipts {
        (BlockId::Tag(BlockTag::Latest),) => json!(["latest"]);
        Some(vec![
            TransactionReceipt {
                kind: TransactionReceiptKind::Legacy,
                transaction_hash: digest!("0x4a481e4649da999d92db0585c36cba94c18a33747e95dc235330e6c737c6f975"),
                transaction_index: 0x0,
                block_hash: digest!("0x19514ce955c65e4dd2cd41f435a75a46a08535b8fc16bc660f8092b32590b182"),
                block_number: 0x6f55,
                from: address!("0x22896Bfc68814BFD855b1a167255eE497006e730"),
                to: Some(address!("0xfd584430cAfa2F451b4e2eBCF3986a21FFf04350")),
                effective_gas_price: uint!("0x9502f907"),
                cumulative_gas_used: 0x18c36,
                gas_used: 0x18c36,
                contract_address: None,
                logs: vec![Log {
                    removed: false,
                    log_index: 0x0,
                    transaction_index: 0x0,
                    transaction_hash: digest!("0x4a481e4649da999d92db0585c36cba94c18a33747e95dc235330e6c737c6f975"),
                    block_hash: digest!("0x19514ce955c65e4dd2cd41f435a75a46a08535b8fc16bc660f8092b32590b182"),
                    block_number: 0x6f55,
                    block_timestamp: None,
                    address: address!("0xfd584430cAfa2F451b4e2eBCF3986a21FFf04350"),
                    data: vec![],
                    topics: [
                        digest!("0x2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d"),
                        digest!("0x4be29e0e4eb91f98f709d98803cba271592782e293b84a625e025cbb40197ba8"),
                        digest!("0x000000000000000000000000835281a2563db4ebf1b626172e085dc406bfc7d2"),
                        digest!("0x00000000000000000000000022896bfc68814bfd855b1a167255ee497006e730"),
                    ].into_iter().collect(),
                }],
                logs_bloom: Bloom(hex!(
                    "0000000400000000000000000000000000000000000000000000000000000000\
                     0000800000000000000000000000000000000000000000000000000000000000\
                     0800000000000000000000000000000000000000000000000000000800000000\
                     0000000000000000000000000000000000000000000000000000000000000000\
                     0000000000000000000000000000000000000000000000000200000000000000\
                     0000000800200000000000002000100000000000000000000010000008000000\
                     0000000000000000000000000000000000000000000000010010000000000000\
                     0000000008000000000000000000000000000000002000000000000000000000"
                )),
                root: None,
                status: Some(TransactionReceiptStatus::Success),
            },
            TransactionReceipt {
                kind: TransactionReceiptKind::Eip1559,
                transaction_hash: digest!("0xefb83b4e3f1c317e8da0f8e2fbb2fe964f34ee184466032aeecac79f20eacaf6"),
                transaction_index: 0x1,
                block_hash: digest!("0x19514ce955c65e4dd2cd41f435a75a46a08535b8fc16bc660f8092b32590b182"),
                block_number: 0x6f55,
                from: address!("0x712e3A792c974B3E3dbE41229Ad4290791C75A82"),
                to: Some(address!("0xd42e2B1c14D02F1Df5369A9827CB8E6f3f75F338")),
                effective_gas_price: uint!("0x9502f907"),
                cumulative_gas_used: 0x1de3e,
                gas_used: 0x5208,
                contract_address: None,
                logs: vec![],
                logs_bloom: Bloom::zero(),
                root: None,
                status: Some(TransactionReceiptStatus::Success),
            },
        ]) => json!([
            {
                "type": "0x0",
                "transactionHash": "0x4a481e4649da999d92db0585c36cba94c18a33747e95dc235330e6c737c6f975",
                "transactionIndex": "0x0",
                "blockHash": "0x19514ce955c65e4dd2cd41f435a75a46a08535b8fc16bc660f8092b32590b182",
                "blockNumber": "0x6f55",
                "from": "0x22896Bfc68814BFD855b1a167255eE497006e730",
                "to": "0xfd584430cAfa2F451b4e2eBCF3986a21FFf04350",
                "effectiveGasPrice": "0x9502f907",
                "cumulativeGasUsed": "0x18c36",
                "gasUsed": "0x18c36",
                "contractAddress": null,
                "logs": [
                    {
                        "removed": false,
                        "logIndex": "0x0",
                        "transactionIndex": "0x0",
                        "transactionHash": "0x4a481e4649da999d92db0585c36cba94c18a33747e95dc235330e6c737c6f975",
                        "blockHash": "0x19514ce955c65e4dd2cd41f435a75a46a08535b8fc16bc660f8092b32590b182",
                        "blockNumber": "0x6f55",
                        "address": "0xfd584430cAfa2F451b4e2eBCF3986a21FFf04350",
                        "data": "0x",
                        "topics": [
                            "0x2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d",
                            "0x4be29e0e4eb91f98f709d98803cba271592782e293b84a625e025cbb40197ba8",
                            "0x000000000000000000000000835281a2563db4ebf1b626172e085dc406bfc7d2",
                            "0x00000000000000000000000022896bfc68814bfd855b1a167255ee497006e730"
                        ]
                    }
                ],
                "logsBloom":
                    "0x0000000400000000000000000000000000000000000000000000000000000000\
                       0000800000000000000000000000000000000000000000000000000000000000\
                       0800000000000000000000000000000000000000000000000000000800000000\
                       0000000000000000000000000000000000000000000000000000000000000000\
                       0000000000000000000000000000000000000000000000000200000000000000\
                       0000000800200000000000002000100000000000000000000010000008000000\
                       0000000000000000000000000000000000000000000000010010000000000000\
                       0000000008000000000000000000000000000000002000000000000000000000",
                "status": "0x1"
            },
            {
                "type": "0x2",
                "transactionHash": "0xefb83b4e3f1c317e8da0f8e2fbb2fe964f34ee184466032aeecac79f20eacaf6",
                "transactionIndex": "0x1",
                "blockHash": "0x19514ce955c65e4dd2cd41f435a75a46a08535b8fc16bc660f8092b32590b182",
                "blockNumber": "0x6f55",
                "from": "0x712e3A792c974B3E3dbE41229Ad4290791C75A82",
                "to": "0xd42e2B1c14D02F1Df5369A9827CB8E6f3f75F338",
                "effectiveGasPrice": "0x9502f907",
                "cumulativeGasUsed": "0x1de3e",
                "gasUsed": "0x5208",
                "contractAddress": null,
                "logs": [],
                "logsBloom":
                    "0x0000000000000000000000000000000000000000000000000000000000000000\
                       0000000000000000000000000000000000000000000000000000000000000000\
                       0000000000000000000000000000000000000000000000000000000000000000\
                       0000000000000000000000000000000000000000000000000000000000000000\
                       0000000000000000000000000000000000000000000000000000000000000000\
                       0000000000000000000000000000000000000000000000000000000000000000\
                       0000000000000000000000000000000000000000000000000000000000000000\
                       0000000000000000000000000000000000000000000000000000000000000000",
                "status": "0x1"
            }
        ]);
    });
}

#[test]
fn eth_get_block_transaction_count_by_hash() {
    assert_method_serialization!(eth::GetBlockTransactionCountByHash {
        (digest!("0xb903239f8543d04b5dc1ba6579132b143087c68db1b2168786408fcbce568238"),)
            => json!(["0xb903239f8543d04b5dc1ba6579132b143087c68db1b2168786408fcbce568238"]);
        Some(0x8) => json!("0x8");
    });
}

#[test]
fn eth_get_block_transaction_count_by_number() {
    assert_method_serialization!(eth::GetBlockTransactionCountByNumber {
        (BlockSpec::Number(0xe8),) => json!(["0xe8"]);
        Some(0x8) => json!("0x8");
    });
}

#[test]
fn eth_get_code() {
    assert_method_serialization!(eth::GetCode {
        (
            address!("0xa50a51c09a5c451C52BB714527E1974b686D8e77"),
            BlockId::Tag(BlockTag::Latest),
        ) => json!([
            "0xa50a51c09a5c451C52BB714527E1974b686D8e77",
            "latest",
        ]);
        hex!(
            "60806040526004361060485763ffffffff7c0100000000000000000000000000\
             0000000000000000000000000000006000350416633fa4f2458114604d578063\
             55241077146071575b600080fd5b348015605857600080fd5b50605f6088565b\
             60408051918252519081900360200190f35b348015607c57600080fd5b506086\
             600435608e565b005b60005481565b60008190556040805182815290517f199c\
             d93e851e4c78c437891155e2112093f8f15394aa89dab09e38d6ca0727879181\
             900360200190a1505600a165627a7a723058209d8929142720a69bde2ab3bfa2\
             da6217674b984899b62753979743c0470a2ea70029"
        )
        .to_vec() => json!(
            "0x60806040526004361060485763ffffffff7c0100000000000000000000000000\
               0000000000000000000000000000006000350416633fa4f2458114604d578063\
               55241077146071575b600080fd5b348015605857600080fd5b50605f6088565b\
               60408051918252519081900360200190f35b348015607c57600080fd5b506086\
               600435608e565b005b60005481565b60008190556040805182815290517f199c\
               d93e851e4c78c437891155e2112093f8f15394aa89dab09e38d6ca0727879181\
               900360200190a1505600a165627a7a723058209d8929142720a69bde2ab3bfa2\
               da6217674b984899b62753979743c0470a2ea70029"
        );
    });
}

#[test]
fn eth_get_filter_changes() {
    assert_method_serialization!(eth::GetFilterChanges {
        (FilterId::from_raw(uint!("0x1")),) => json!(["0x1"]);
        FilterChanges::Logs(vec![
            Log {
                removed: false,
                log_index: 0x0,
                transaction_index: 0x0,
                transaction_hash: digest!("0x66e7a140c8fa27fe98fde923defea7562c3ca2d6bb89798aabec65782c08f63d"),
                block_hash: digest!("0xfc139f5e2edee9e9c888d8df9a2d2226133a9bd87c88ccbd9c930d3d4c9f9ef5"),
                block_number: 0x233,
                block_timestamp: Some(0x11),
                address: address!("0x42699A7612A82f1d9C36148af9C77354759b210b"),
                data: hex!("0000000000000000000000000000000000000000000000000000000000000004").to_vec(),
                topics: [
                    digest!("0x04474795f5b996ff80cb47c148d4c5ccdbe09ef27551820caa9c2f8ed149cce3"),
                ].into_iter().collect(),
            },
            Log {
                removed: false,
                log_index: 0x0,
                transaction_index: 0x0,
                transaction_hash: digest!("0xdb17aa1c2ce609132f599155d384c0bc5334c988a6c368056d7e167e23eee058"),
                block_hash: digest!("0x98b0ec0f9fea0018a644959accbe69cd046a8582e89402e1ab0ada91cad644ed"),
                block_number: 0x238,
                block_timestamp: Some(0x22),
                address: address!("0x42699A7612A82f1d9C36148af9C77354759b210b"),
                data: hex!("0000000000000000000000000000000000000000000000000000000000000007").to_vec(),
                topics: [
                    digest!("0x04474795f5b996ff80cb47c148d4c5ccdbe09ef27551820caa9c2f8ed149cce3"),
                ].into_iter().collect(),
            },
        ]) => json!([
            {
                "removed": false,
                "logIndex": "0x0",
                "transactionIndex": "0x0",
                "transactionHash": "0x66e7a140c8fa27fe98fde923defea7562c3ca2d6bb89798aabec65782c08f63d",
                "blockHash": "0xfc139f5e2edee9e9c888d8df9a2d2226133a9bd87c88ccbd9c930d3d4c9f9ef5",
                "blockNumber": "0x233",
                "blockTimestamp": "0x11",
                "address": "0x42699A7612A82f1d9C36148af9C77354759b210b",
                "data": "0x0000000000000000000000000000000000000000000000000000000000000004",
                "topics": [
                    "0x04474795f5b996ff80cb47c148d4c5ccdbe09ef27551820caa9c2f8ed149cce3"
                ]
            },
            {
                "removed": false,
                "logIndex": "0x0",
                "transactionIndex": "0x0",
                "transactionHash": "0xdb17aa1c2ce609132f599155d384c0bc5334c988a6c368056d7e167e23eee058",
                "blockHash": "0x98b0ec0f9fea0018a644959accbe69cd046a8582e89402e1ab0ada91cad644ed",
                "blockNumber": "0x238",
                "blockTimestamp": "0x22",
                "address": "0x42699A7612A82f1d9C36148af9C77354759b210b",
                "data": "0x0000000000000000000000000000000000000000000000000000000000000007",
                "topics": [
                    "0x04474795f5b996ff80cb47c148d4c5ccdbe09ef27551820caa9c2f8ed149cce3"
                ]
            }
        ]);
    });
}

#[test]
fn eth_get_filter_logs() {
    assert_method_serialization!(eth::GetFilterLogs {
        (FilterId::from_raw(uint!("0x1")),) => json!(["0x1"]);
        vec![
            Log {
                removed: false,
                log_index: 0x0,
                transaction_index: 0x0,
                transaction_hash: digest!("0x66e7a140c8fa27fe98fde923defea7562c3ca2d6bb89798aabec65782c08f63d"),
                block_hash: digest!("0xfc139f5e2edee9e9c888d8df9a2d2226133a9bd87c88ccbd9c930d3d4c9f9ef5"),
                block_number: 0x233,
                block_timestamp: Some(0x11),
                address: address!("0x42699A7612A82f1d9C36148af9C77354759b210b"),
                data: hex!("0000000000000000000000000000000000000000000000000000000000000004").to_vec(),
                topics: [
                    digest!("0x04474795f5b996ff80cb47c148d4c5ccdbe09ef27551820caa9c2f8ed149cce3"),
                ].into_iter().collect(),
            },
            Log {
                removed: false,
                log_index: 0x0,
                transaction_index: 0x0,
                transaction_hash: digest!("0xdb17aa1c2ce609132f599155d384c0bc5334c988a6c368056d7e167e23eee058"),
                block_hash: digest!("0x98b0ec0f9fea0018a644959accbe69cd046a8582e89402e1ab0ada91cad644ed"),
                block_number: 0x238,
                block_timestamp: Some(0x22),
                address: address!("0x42699A7612A82f1d9C36148af9C77354759b210b"),
                data: hex!("0000000000000000000000000000000000000000000000000000000000000007").to_vec(),
                topics: [
                    digest!("0x04474795f5b996ff80cb47c148d4c5ccdbe09ef27551820caa9c2f8ed149cce3"),
                ].into_iter().collect(),
            },
        ] => json!([
            {
                "removed": false,
                "logIndex": "0x0",
                "transactionIndex": "0x0",
                "transactionHash": "0x66e7a140c8fa27fe98fde923defea7562c3ca2d6bb89798aabec65782c08f63d",
                "blockHash": "0xfc139f5e2edee9e9c888d8df9a2d2226133a9bd87c88ccbd9c930d3d4c9f9ef5",
                "blockNumber": "0x233",
                "blockTimestamp": "0x11",
                "address": "0x42699A7612A82f1d9C36148af9C77354759b210b",
                "data": "0x0000000000000000000000000000000000000000000000000000000000000004",
                "topics": [
                    "0x04474795f5b996ff80cb47c148d4c5ccdbe09ef27551820caa9c2f8ed149cce3"
                ]
            },
            {
                "removed": false,
                "logIndex": "0x0",
                "transactionIndex": "0x0",
                "transactionHash": "0xdb17aa1c2ce609132f599155d384c0bc5334c988a6c368056d7e167e23eee058",
                "blockHash": "0x98b0ec0f9fea0018a644959accbe69cd046a8582e89402e1ab0ada91cad644ed",
                "blockNumber": "0x238",
                "blockTimestamp": "0x22",
                "address": "0x42699A7612A82f1d9C36148af9C77354759b210b",
                "data": "0x0000000000000000000000000000000000000000000000000000000000000007",
                "topics": [
                    "0x04474795f5b996ff80cb47c148d4c5ccdbe09ef27551820caa9c2f8ed149cce3"
                ]
            }
        ]);
    });
}

#[test]
fn eth_get_logs() {
    assert_method_serialization!(eth::GetLogs {
        (LogFilter {
            blocks: LogFilterBlocks::Range {
                from: BlockSpec::Number(0x137d3c2),
                to: BlockSpec::Number(0x137d3c3),
            },
            address: LogFilterValue::Exact(address!("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2")),
            topics: ArrayVec::new(),
        },) => json!([{
            "fromBlock": "0x137d3c2",
            "toBlock": "0x137d3c3",
            "address": "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
            "topics": [],
        }]);
        vec![
            Log {
                removed: false,
                log_index: 0x0,
                transaction_index: 0x0,
                transaction_hash: digest!("0x66e7a140c8fa27fe98fde923defea7562c3ca2d6bb89798aabec65782c08f63d"),
                block_hash: digest!("0xfc139f5e2edee9e9c888d8df9a2d2226133a9bd87c88ccbd9c930d3d4c9f9ef5"),
                block_number: 0x233,
                block_timestamp: Some(0x11),
                address: address!("0x42699A7612A82f1d9C36148af9C77354759b210b"),
                data: hex!("0000000000000000000000000000000000000000000000000000000000000004").to_vec(),
                topics: [
                    digest!("0x04474795f5b996ff80cb47c148d4c5ccdbe09ef27551820caa9c2f8ed149cce3"),
                ].into_iter().collect(),
            },
            Log {
                removed: false,
                log_index: 0x0,
                transaction_index: 0x0,
                transaction_hash: digest!("0xdb17aa1c2ce609132f599155d384c0bc5334c988a6c368056d7e167e23eee058"),
                block_hash: digest!("0x98b0ec0f9fea0018a644959accbe69cd046a8582e89402e1ab0ada91cad644ed"),
                block_number: 0x238,
                block_timestamp: Some(0x22),
                address: address!("0x42699A7612A82f1d9C36148af9C77354759b210b"),
                data: hex!("0000000000000000000000000000000000000000000000000000000000000007").to_vec(),
                topics: [
                    digest!("0x04474795f5b996ff80cb47c148d4c5ccdbe09ef27551820caa9c2f8ed149cce3"),
                ].into_iter().collect(),
            },
        ] => json!([
            {
                "removed": false,
                "logIndex": "0x0",
                "transactionIndex": "0x0",
                "transactionHash": "0x66e7a140c8fa27fe98fde923defea7562c3ca2d6bb89798aabec65782c08f63d",
                "blockHash": "0xfc139f5e2edee9e9c888d8df9a2d2226133a9bd87c88ccbd9c930d3d4c9f9ef5",
                "blockNumber": "0x233",
                "blockTimestamp": "0x11",
                "address": "0x42699A7612A82f1d9C36148af9C77354759b210b",
                "data": "0x0000000000000000000000000000000000000000000000000000000000000004",
                "topics": [
                    "0x04474795f5b996ff80cb47c148d4c5ccdbe09ef27551820caa9c2f8ed149cce3"
                ]
            },
            {
                "removed": false,
                "logIndex": "0x0",
                "transactionIndex": "0x0",
                "transactionHash": "0xdb17aa1c2ce609132f599155d384c0bc5334c988a6c368056d7e167e23eee058",
                "blockHash": "0x98b0ec0f9fea0018a644959accbe69cd046a8582e89402e1ab0ada91cad644ed",
                "blockNumber": "0x238",
                "blockTimestamp": "0x22",
                "address": "0x42699A7612A82f1d9C36148af9C77354759b210b",
                "data": "0x0000000000000000000000000000000000000000000000000000000000000007",
                "topics": [
                    "0x04474795f5b996ff80cb47c148d4c5ccdbe09ef27551820caa9c2f8ed149cce3"
                ]
            }
        ]);
    });
}

#[test]
fn eth_get_proof() {
    assert_method_serialization!(eth::GetProof {
        (
            address!("0xe5cB067E90D5Cd1F8052B83562Ae670bA4A211a8"),
            vec![digest!("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421")],
            BlockId::Tag(BlockTag::Latest),
        ) => json!([
            "0xe5cB067E90D5Cd1F8052B83562Ae670bA4A211a8",
            ["0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"],
            "latest",
        ]);
        AccountProof {
            address: address!("0xe5cB067E90D5Cd1F8052B83562Ae670bA4A211a8"),
            account_proof: vec![
                hex!(
                    "f90211a06a718c2c9da77c253b12d7b2569657901e37bb691718f5dda1b86157\
                     ab1dd5eda0e7f19ed5e21bccc8d3260236b24f80ad88b3634f5d005f37b83888\
                     1f0e12f1bda0abb301291704e4d92686c0f5f8ebb1734185321559b8d717ffdc\
                     a95c99591976a0d0c2026bfab65c3b95276bfa82af9dec860b485f8857f293c1\
                     48d63a2182128fa0c98044ec9a1273a218bed58b478277dd39173ad7b8edb95c\
                     200423a6bc8fc25fa056e5a55d9ddccdbf49362857200bbb1f042d61187c9f5f\
                     9ddcff5d2f1fc984a2a02a5b7200af424114f99a4b5f0a21c19aac82209e431e\
                     d80bfde177adb1004bdfa0026e4374f0518ff44a80fa374838ecb86cc64ac93b\
                     b710fea6dff4198f947b27a03fea341d87984673ad523177ed52f278bf4d8f97\
                     e6531c8ece932aeede4802f4a0bfe2f4a7fcb78f7e9f080dea7b6977fb1d88c4\
                     41696e4456dad92b9d34ff0f43a02a3eb5c0edb14626c9c629601027bd60178b\
                     b2b688a67cea4d179fc432436615a0747355b8e02f3b884b4ffe5cea1619e325\
                     15fea064cca98208591af8c744e894a0874253737bae37f020ad3bb7e3292c7c\
                     4a63cdc158af6b33aaa4deaef016dccba03d8192bc1fc6aa1548912e763a0b50\
                     13a94399cefad7b47cf388873b2b794068a09b67f9737c6028d796bfd1c5da57\
                     a6f45824dc891f848ea0e1f8019d1fb5fba8a0aa871f9de8da85960fcd8a22cd\
                     f21c27f11e3966c14a6737ffd414b98dda00b280"
                )
                .to_vec(),
                hex!(
                    "f90211a0d360be1e1da1a0c32bc4c105833bd531e59d110684007b7c50fb2709\
                     002973eca0cf6dd1e350a7031b4e2ab49c899fd8bd47551c8565d8fd8d1d7796\
                     c83820c3b1a0eb0a88c29bb33989a589156f7bf07d9efc74034dd9d3f5b73385\
                     c3b45c3249bea02783c25f97a6ddb8dc07adf4b176991836d39184b1f678aded\
                     a832fff15e3664a00a4e288060045e587774d8a64993a7add73068b16863145e\
                     1e8eeb4602e18e19a0340851f4046ad1298962d6e47d05c66329549c839c1587\
                     48aaad7ae00b943aefa085b127bc2a3bd17604283de21b2b3c9aa8f1d4b7b85c\
                     94d8105a46fe32c77688a00f531d62b3c5435324c01009c284fe31277e8d3830\
                     2b75ea01be89f09e205969a00011c8351c0e3d639ac54b9d3a59de630b16a67d\
                     e8270d7d6064d0a67e93f9cca048780d32b7f2db88650b51c46f46fd0a68795e\
                     dee1fd5ecee6eb3595741d9669a0c91afd74eaf8e08a997061a62b354e2516fd\
                     c494e8e26cc50ceeb8f4a175608ba0e2c07f1b48fab80eecb340f5882e8c7b32\
                     ee416e4045c61f1df646a133487303a01a1eff78435a7a29a29463bdc3486ae8\
                     1364b00bea82ba0fdf67a110770f2261a04f2eb440ba71c72da5fd7f0e439018\
                     d6671dc809f747213a1ea755848124e994a074ff9f37fce99daa3ed01dd76307\
                     6450022996fc729be2cc43c61ec5182c2366a0b80b36b7b621112592f52390b8\
                     9748d422e9b1517c4b0203b8176a53f89d4a6680"
                )
                .to_vec(),
                hex!(
                    "f90211a0b25f283bd01a8c8b2418049f9585bc37ff2c1e2e12eab4b7f64ae1f2\
                     6647389aa02ad96c150d7c3c9c194d30315456852cf6a0a940e0191ae5d04007\
                     454823d4e9a0b220cf7a855e2dbcc0b973134e2e119b982d7d40dbb1b27d9981\
                     6c41f40e829aa049224431da84cbf1b7ae813abcc9ef4c1dfc1760f6ddc5d57f\
                     7354bf3cbf6cc4a015191f879ac115b362f0257fd3eedb789537e836574a5b1a\
                     bf1c9982ebe3bdfea07913c1b6e7282569d2d421e9fa2257f5d1698e93303bc4\
                     9b941704287d7aaefea0a526576981ce6fd9f2bd48dd2ca6d5272f2fbdc85f0e\
                     e35a295f6ccd97ae8765a0313fad407f0c737c29024c02a890c4ecc12d7771c0\
                     5ab7b435e5087a7cdef4d9a0d2044603cba9d4afdaf6fd2470e729ef3a65242d\
                     e71276f20d59accfa6b53a7ca0457caacb9370c09b15f7d904adefd2308be94e\
                     23669ba5f43241ffff5f438a0aa09fb2dd45a383a0cc088a72b14117e1e9b7d6\
                     889218f3ac7631e8de644c5cb76da0c675dcd4d3fb692b514851c6106e2b09e6\
                     f5661d56a0a32ae02e2efc1515c235a074949a59ff1bdba87548510d6e404ec4\
                     532f4456dfdec8e753d92fda11a3088ba0a328c6ab1ab8f70db4d23e95bb163c\
                     13ba0c508f063a5b1393a4efd7ff375f05a0c722fe3ce796998269373cbb2fc2\
                     29b2bdf2c43c6c2df003309422e043ce6c03a024e69343286eec44fa4744f690\
                     7209116e5383cff3fa98fe81ba06e7e8d4366680"
                )
                .to_vec(),
                hex!(
                    "f90211a00e99ba2198124b8241ea304551fe973215829e2fbc0438d67922707a\
                     2a847432a0bb9ce24fd527879c5fe6dbbec1ef5a05ed9d1ca88e921d140bafbe\
                     c1112f6a6aa099787fd6c7a1989229c4291ef5267335e66152ce417daea46e66\
                     d19cb6f81d1ca0e430ff4b8d5621baa5978673344e78b4d8b4df51431b6e6378\
                     5267c98a24ce18a0bb3e91a825fe3d42ed270a93e9ad1aabd566c40cb28e622f\
                     7f1d7ee967c8afd6a0aa364b0056870c6507bc3262a5f851ecb13684088bdb13\
                     996d3cb2db401ce3ffa0a3732eba4c7a6e062665ab5be08acb986c3db87556fb\
                     138548cc900ff1e56995a026b088e90c9738b8ce16e853107a937a50d52726a2\
                     4f9f6ce60f587762eb45a2a006c9d5bc3c064b5c1fb565bff91cace9161c64ae\
                     653a329610c1dcf34d434429a06c16df2edc70656d322d0c2403bad7d45bc790\
                     ffc3e7adeef856d98ea6afc91ba0ae05ed5d6c34b5da29c2e94d7880aeba0906\
                     f95f4ec10b132a1d4766a0701c98a01470a86aa350d1ada0c082eac75de828a8\
                     51f9c8c7c4aa49b1556fe3a5574966a0334eef025100a6da1033710dd98e0475\
                     f29d3d7e397caf618ca71c336c5f4f49a0ef0b3abbebcff34d6a8a8f5cdbfbd1\
                     54ab3452b58dcb09de58ec983644963675a041857e865ec38e200a13bc1a3cb7\
                     1c7d69aeef7ffdee8be515c9a5b691ce091fa059edd0eb3bbec36bbf38a19802\
                     d4646c00ba821ab55fdeea12e15bab62c4e1e580"
                )
                .to_vec(),
                hex!(
                    "f90211a0af0c7fa65ffcb84c31e68c1cf00e1a20bf8bb497c39883e19b66a999\
                     75b03431a0c492cab3623eb7926069794c3c718733e16c5fd0d4a13fb7c752ee\
                     9809aac7ada05003cea7132aa70d6f36731d60640a90bcd8f4fd493e4540d5ab\
                     1b4943679c0ca0fd700683405b1d2306b586dd3b5b2f92f1692fae20d17cd8b8\
                     e59d09b9c6670da01db8683910e46e56e8afeb9fe2b7c35382e5a0914d7b0dd8\
                     f0e8cb9981ba7435a0fa7f75d73aa73c35824387bec81388315caa4aee3f4f55\
                     62f971beb256c62d49a0ee478e420d83f413e8568dacfd5d83f83a5dd7c45f49\
                     4b504828e5dc962f0e3ea094b95444a917ac94a675681f6bf851172ad0969801\
                     a783a63a71edafed45e7a7a0a0c46586e109abe80fe50361dd582e3f143cb416\
                     828239faa43bb2b890869501a0ae051d5d43634c68bf9c97823256cc68580f19\
                     4dfdbd0c301140c7ca5853430ca0660b9365bb77ec9cdc6eb95516c162dca207\
                     27c6f828dbbeb1ae110dde4d3134a09feb1b75e84ff6722e4d837bfb6d207b6e\
                     e3b21b86844a01140ce293813b49a1a0ed58a70b04efa3bdc0babe2abfa20824\
                     a75d61d52291bfdb5cf08597800764d6a020a2d5d3a83f9e35ad9fd1c448626d\
                     90af0eb3efefaa4f2f93207b4096ef5507a0fc8efc4484dcf0a54f0574de9aaa\
                     de0dcff6ec3599edb9f82efb26b6566dcaeaa032f7e79856db3fd984f72bb2c9\
                     3d4dab328198d355a61c975fab1f08bdb2046580"
                )
                .to_vec(),
            ],
            balance: uint!("0x0"),
            code_hash: digest!("0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"),
            nonce: 0,
            storage_hash: digest!("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"),
            storage_proof: vec![StorageProof {
                key: uint!("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"),
                value: uint!("0x1"),
                proof: vec![
                        hex!(
                            "f90211a0c87222cccea2bf32759fcee9dbaacbe3ea4165dd6184af6773651c5e\
                             00e34a8ba0be90e6e5d1a67ab5587779c60ac136d6a96db62b84c04998a5f03a\
                             367346abd6a05344aa1c9ca2e3e56bf98fd718ec43728578d148e1967fbaf8bf\
                             17a2a073a0bda011a2f9312c3308640a0d6ceeae218747290f23806067456da1\
                             d444c65abae437a0b3097a108bfce79af6699da4ae3003cd4929f0b4576aad65\
                             5c31cb725bde84c7a0c133d3c637e174f36a73c22b1039eb003da6374bc09293\
                             21241badb3efa3c4a9a0f13059f2301ad9862ce02e3f7f3f2c9ab78eb3058376\
                             4d73654f7f1f8b1e86fda06544e3915748b18204e09df75ff20d2fa6bd8121e2\
                             e669699012d54590383d6fa070e3a8e093691581d58fadb560b510262a758037\
                             632cd8670d3a36df828976b7a062a88a2900544dc76a32255a6b2b2a2eef8fa6\
                             8279700c00adc7508286702552a0a474aeebd5603dfce46a6ecd1ecd519068dc\
                             034a544fde03ac42d4018e60a334a0b7d528fc41c8fdc8ea18c6e7d0099270c7\
                             77ec1403cf879d1f5134bdc12a6c6ca04440f1242e42c5bfa7c536591ab89c8e\
                             84bea417435871c32eef1e25295b20daa06a5dcfe3cc84cff9d3e3c3ae868cfb\
                             a8f0dd111a90c3f85869dab5b893f96643a026b2fb9dd7d08b0ed2f1c44fbf87\
                             5011412a384f86f751c92e1013248d4aa371a0c75597b2b789fc4e939b719373\
                             90ce9d7d53159431328ac52180eef08ef200f280"
                        )
                        .to_vec(),
                        hex!(
                            "f90191a0f0c5b800b542001597f2b7a8e106ac0e2849d2cc1df1727ac35c4ea3\
                             965f1c9180a08537f2e248702a6ae2a57e9110a5740f5772c876389739ac90de\
                             bd6a0692713ea00b3a26a05b5494fb3ff6f0b3897688a5581066b20b07ebab92\
                             52d169d928717fa0a9a54d84976d134d6dba06a65064c7f3a964a75947d452db\
                             6f6bb4b6c47b43aaa01e2a1ed3d1572b872bbf09ee44d2ed737da31f01de3c0f\
                             4b4e1f046740066461a076f251d160b9a02eb0b5c1d83b61c9cdd4f37361705e\
                             79a45529bf49801fb824a0774a01a624cb14a50d17f2fe4b7ae6af8a67bbb029\
                             177ccc3dd729a734484d3ea05921b8a19aebe4fff5a36071e311778f9b934591\
                             83fdf7f6d870b401fa25dcbba0c8d71dd13d2806e2865a5c2cfa447f626471bf\
                             0b66182a8fd07230434e1cad2680a0e9864fdfaf3693b2602f56cd938ccd494b\
                             8634b1f91800ef02203a3609ca4c21a0c69d174ad6b6e58b0bd05914352839ec\
                             60915cd066dd2bee2a48016139687f21a0513dd5514fd6bad56871711441d38d\
                             e2821cc6913cb192416b0385f025650731808080"
                        )
                        .to_vec(),
                ],
            }],
        } => json!({
            "address": "0xe5cB067E90D5Cd1F8052B83562Ae670bA4A211a8",
            "accountProof": [
                "0xf90211a06a718c2c9da77c253b12d7b2569657901e37bb691718f5dda1b86157\
                   ab1dd5eda0e7f19ed5e21bccc8d3260236b24f80ad88b3634f5d005f37b83888\
                   1f0e12f1bda0abb301291704e4d92686c0f5f8ebb1734185321559b8d717ffdc\
                   a95c99591976a0d0c2026bfab65c3b95276bfa82af9dec860b485f8857f293c1\
                   48d63a2182128fa0c98044ec9a1273a218bed58b478277dd39173ad7b8edb95c\
                   200423a6bc8fc25fa056e5a55d9ddccdbf49362857200bbb1f042d61187c9f5f\
                   9ddcff5d2f1fc984a2a02a5b7200af424114f99a4b5f0a21c19aac82209e431e\
                   d80bfde177adb1004bdfa0026e4374f0518ff44a80fa374838ecb86cc64ac93b\
                   b710fea6dff4198f947b27a03fea341d87984673ad523177ed52f278bf4d8f97\
                   e6531c8ece932aeede4802f4a0bfe2f4a7fcb78f7e9f080dea7b6977fb1d88c4\
                   41696e4456dad92b9d34ff0f43a02a3eb5c0edb14626c9c629601027bd60178b\
                   b2b688a67cea4d179fc432436615a0747355b8e02f3b884b4ffe5cea1619e325\
                   15fea064cca98208591af8c744e894a0874253737bae37f020ad3bb7e3292c7c\
                   4a63cdc158af6b33aaa4deaef016dccba03d8192bc1fc6aa1548912e763a0b50\
                   13a94399cefad7b47cf388873b2b794068a09b67f9737c6028d796bfd1c5da57\
                   a6f45824dc891f848ea0e1f8019d1fb5fba8a0aa871f9de8da85960fcd8a22cd\
                   f21c27f11e3966c14a6737ffd414b98dda00b280",
                "0xf90211a0d360be1e1da1a0c32bc4c105833bd531e59d110684007b7c50fb2709\
                   002973eca0cf6dd1e350a7031b4e2ab49c899fd8bd47551c8565d8fd8d1d7796\
                   c83820c3b1a0eb0a88c29bb33989a589156f7bf07d9efc74034dd9d3f5b73385\
                   c3b45c3249bea02783c25f97a6ddb8dc07adf4b176991836d39184b1f678aded\
                   a832fff15e3664a00a4e288060045e587774d8a64993a7add73068b16863145e\
                   1e8eeb4602e18e19a0340851f4046ad1298962d6e47d05c66329549c839c1587\
                   48aaad7ae00b943aefa085b127bc2a3bd17604283de21b2b3c9aa8f1d4b7b85c\
                   94d8105a46fe32c77688a00f531d62b3c5435324c01009c284fe31277e8d3830\
                   2b75ea01be89f09e205969a00011c8351c0e3d639ac54b9d3a59de630b16a67d\
                   e8270d7d6064d0a67e93f9cca048780d32b7f2db88650b51c46f46fd0a68795e\
                   dee1fd5ecee6eb3595741d9669a0c91afd74eaf8e08a997061a62b354e2516fd\
                   c494e8e26cc50ceeb8f4a175608ba0e2c07f1b48fab80eecb340f5882e8c7b32\
                   ee416e4045c61f1df646a133487303a01a1eff78435a7a29a29463bdc3486ae8\
                   1364b00bea82ba0fdf67a110770f2261a04f2eb440ba71c72da5fd7f0e439018\
                   d6671dc809f747213a1ea755848124e994a074ff9f37fce99daa3ed01dd76307\
                   6450022996fc729be2cc43c61ec5182c2366a0b80b36b7b621112592f52390b8\
                   9748d422e9b1517c4b0203b8176a53f89d4a6680",
                "0xf90211a0b25f283bd01a8c8b2418049f9585bc37ff2c1e2e12eab4b7f64ae1f2\
                   6647389aa02ad96c150d7c3c9c194d30315456852cf6a0a940e0191ae5d04007\
                   454823d4e9a0b220cf7a855e2dbcc0b973134e2e119b982d7d40dbb1b27d9981\
                   6c41f40e829aa049224431da84cbf1b7ae813abcc9ef4c1dfc1760f6ddc5d57f\
                   7354bf3cbf6cc4a015191f879ac115b362f0257fd3eedb789537e836574a5b1a\
                   bf1c9982ebe3bdfea07913c1b6e7282569d2d421e9fa2257f5d1698e93303bc4\
                   9b941704287d7aaefea0a526576981ce6fd9f2bd48dd2ca6d5272f2fbdc85f0e\
                   e35a295f6ccd97ae8765a0313fad407f0c737c29024c02a890c4ecc12d7771c0\
                   5ab7b435e5087a7cdef4d9a0d2044603cba9d4afdaf6fd2470e729ef3a65242d\
                   e71276f20d59accfa6b53a7ca0457caacb9370c09b15f7d904adefd2308be94e\
                   23669ba5f43241ffff5f438a0aa09fb2dd45a383a0cc088a72b14117e1e9b7d6\
                   889218f3ac7631e8de644c5cb76da0c675dcd4d3fb692b514851c6106e2b09e6\
                   f5661d56a0a32ae02e2efc1515c235a074949a59ff1bdba87548510d6e404ec4\
                   532f4456dfdec8e753d92fda11a3088ba0a328c6ab1ab8f70db4d23e95bb163c\
                   13ba0c508f063a5b1393a4efd7ff375f05a0c722fe3ce796998269373cbb2fc2\
                   29b2bdf2c43c6c2df003309422e043ce6c03a024e69343286eec44fa4744f690\
                   7209116e5383cff3fa98fe81ba06e7e8d4366680",
                "0xf90211a00e99ba2198124b8241ea304551fe973215829e2fbc0438d67922707a\
                   2a847432a0bb9ce24fd527879c5fe6dbbec1ef5a05ed9d1ca88e921d140bafbe\
                   c1112f6a6aa099787fd6c7a1989229c4291ef5267335e66152ce417daea46e66\
                   d19cb6f81d1ca0e430ff4b8d5621baa5978673344e78b4d8b4df51431b6e6378\
                   5267c98a24ce18a0bb3e91a825fe3d42ed270a93e9ad1aabd566c40cb28e622f\
                   7f1d7ee967c8afd6a0aa364b0056870c6507bc3262a5f851ecb13684088bdb13\
                   996d3cb2db401ce3ffa0a3732eba4c7a6e062665ab5be08acb986c3db87556fb\
                   138548cc900ff1e56995a026b088e90c9738b8ce16e853107a937a50d52726a2\
                   4f9f6ce60f587762eb45a2a006c9d5bc3c064b5c1fb565bff91cace9161c64ae\
                   653a329610c1dcf34d434429a06c16df2edc70656d322d0c2403bad7d45bc790\
                   ffc3e7adeef856d98ea6afc91ba0ae05ed5d6c34b5da29c2e94d7880aeba0906\
                   f95f4ec10b132a1d4766a0701c98a01470a86aa350d1ada0c082eac75de828a8\
                   51f9c8c7c4aa49b1556fe3a5574966a0334eef025100a6da1033710dd98e0475\
                   f29d3d7e397caf618ca71c336c5f4f49a0ef0b3abbebcff34d6a8a8f5cdbfbd1\
                   54ab3452b58dcb09de58ec983644963675a041857e865ec38e200a13bc1a3cb7\
                   1c7d69aeef7ffdee8be515c9a5b691ce091fa059edd0eb3bbec36bbf38a19802\
                   d4646c00ba821ab55fdeea12e15bab62c4e1e580",
                "0xf90211a0af0c7fa65ffcb84c31e68c1cf00e1a20bf8bb497c39883e19b66a999\
                   75b03431a0c492cab3623eb7926069794c3c718733e16c5fd0d4a13fb7c752ee\
                   9809aac7ada05003cea7132aa70d6f36731d60640a90bcd8f4fd493e4540d5ab\
                   1b4943679c0ca0fd700683405b1d2306b586dd3b5b2f92f1692fae20d17cd8b8\
                   e59d09b9c6670da01db8683910e46e56e8afeb9fe2b7c35382e5a0914d7b0dd8\
                   f0e8cb9981ba7435a0fa7f75d73aa73c35824387bec81388315caa4aee3f4f55\
                   62f971beb256c62d49a0ee478e420d83f413e8568dacfd5d83f83a5dd7c45f49\
                   4b504828e5dc962f0e3ea094b95444a917ac94a675681f6bf851172ad0969801\
                   a783a63a71edafed45e7a7a0a0c46586e109abe80fe50361dd582e3f143cb416\
                   828239faa43bb2b890869501a0ae051d5d43634c68bf9c97823256cc68580f19\
                   4dfdbd0c301140c7ca5853430ca0660b9365bb77ec9cdc6eb95516c162dca207\
                   27c6f828dbbeb1ae110dde4d3134a09feb1b75e84ff6722e4d837bfb6d207b6e\
                   e3b21b86844a01140ce293813b49a1a0ed58a70b04efa3bdc0babe2abfa20824\
                   a75d61d52291bfdb5cf08597800764d6a020a2d5d3a83f9e35ad9fd1c448626d\
                   90af0eb3efefaa4f2f93207b4096ef5507a0fc8efc4484dcf0a54f0574de9aaa\
                   de0dcff6ec3599edb9f82efb26b6566dcaeaa032f7e79856db3fd984f72bb2c9\
                   3d4dab328198d355a61c975fab1f08bdb2046580",
            ],
            "balance": "0x0",
            "codeHash": "0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470",
            "nonce": "0x0",
            "storageHash": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
            "storageProof": [{
                "key": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
                "value": "0x1",
                "proof": [
                    "0xf90211a0c87222cccea2bf32759fcee9dbaacbe3ea4165dd6184af6773651c5e\
                       00e34a8ba0be90e6e5d1a67ab5587779c60ac136d6a96db62b84c04998a5f03a\
                       367346abd6a05344aa1c9ca2e3e56bf98fd718ec43728578d148e1967fbaf8bf\
                       17a2a073a0bda011a2f9312c3308640a0d6ceeae218747290f23806067456da1\
                       d444c65abae437a0b3097a108bfce79af6699da4ae3003cd4929f0b4576aad65\
                       5c31cb725bde84c7a0c133d3c637e174f36a73c22b1039eb003da6374bc09293\
                       21241badb3efa3c4a9a0f13059f2301ad9862ce02e3f7f3f2c9ab78eb3058376\
                       4d73654f7f1f8b1e86fda06544e3915748b18204e09df75ff20d2fa6bd8121e2\
                       e669699012d54590383d6fa070e3a8e093691581d58fadb560b510262a758037\
                       632cd8670d3a36df828976b7a062a88a2900544dc76a32255a6b2b2a2eef8fa6\
                       8279700c00adc7508286702552a0a474aeebd5603dfce46a6ecd1ecd519068dc\
                       034a544fde03ac42d4018e60a334a0b7d528fc41c8fdc8ea18c6e7d0099270c7\
                       77ec1403cf879d1f5134bdc12a6c6ca04440f1242e42c5bfa7c536591ab89c8e\
                       84bea417435871c32eef1e25295b20daa06a5dcfe3cc84cff9d3e3c3ae868cfb\
                       a8f0dd111a90c3f85869dab5b893f96643a026b2fb9dd7d08b0ed2f1c44fbf87\
                       5011412a384f86f751c92e1013248d4aa371a0c75597b2b789fc4e939b719373\
                       90ce9d7d53159431328ac52180eef08ef200f280",
                    "0xf90191a0f0c5b800b542001597f2b7a8e106ac0e2849d2cc1df1727ac35c4ea3\
                       965f1c9180a08537f2e248702a6ae2a57e9110a5740f5772c876389739ac90de\
                       bd6a0692713ea00b3a26a05b5494fb3ff6f0b3897688a5581066b20b07ebab92\
                       52d169d928717fa0a9a54d84976d134d6dba06a65064c7f3a964a75947d452db\
                       6f6bb4b6c47b43aaa01e2a1ed3d1572b872bbf09ee44d2ed737da31f01de3c0f\
                       4b4e1f046740066461a076f251d160b9a02eb0b5c1d83b61c9cdd4f37361705e\
                       79a45529bf49801fb824a0774a01a624cb14a50d17f2fe4b7ae6af8a67bbb029\
                       177ccc3dd729a734484d3ea05921b8a19aebe4fff5a36071e311778f9b934591\
                       83fdf7f6d870b401fa25dcbba0c8d71dd13d2806e2865a5c2cfa447f626471bf\
                       0b66182a8fd07230434e1cad2680a0e9864fdfaf3693b2602f56cd938ccd494b\
                       8634b1f91800ef02203a3609ca4c21a0c69d174ad6b6e58b0bd05914352839ec\
                       60915cd066dd2bee2a48016139687f21a0513dd5514fd6bad56871711441d38d\
                       e2821cc6913cb192416b0385f025650731808080",
                ],
            }],
        });
    });
}

#[test]
fn eth_get_storage_at() {
    assert_method_serialization!(eth::GetStorageAt {
        (
            address!("0xFE3B557E8Fb62b89F4916B721be55cEb828dBd73"),
            uint!("0x0"),
            BlockId::Tag(BlockTag::Latest),
        ) => json!([
            "0xFE3B557E8Fb62b89F4916B721be55cEb828dBd73",
            "0x0",
            "latest",
        ]);
        [0; 32] => json!("0x0000000000000000000000000000000000000000000000000000000000000000");
    });
}

#[test]
fn eth_get_storage_values() {
    assert_method_serialization!(eth::GetStorageValues {
        (
            HashMap::from([
                (
                    address!("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"),
                    vec![digest!("0x0000000000000000000000000000000000000000000000000000000000000003")],
                ),
                (
                    address!("0xdAC17F958D2ee523a2206206994597C13D831ec7"),
                    vec![
                        digest!("0x0000000000000000000000000000000000000000000000000000000000000002"),
                        digest!("0x0000000000000000000000000000000000000000000000000000000000000006"),
                    ],
                ),
            ]),
            BlockId::Tag(BlockTag::Latest),
        ) => json!([
            {
                "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48": [
                    "0x0000000000000000000000000000000000000000000000000000000000000003"
                ],
                "0xdAC17F958D2ee523a2206206994597C13D831ec7": [
                    "0x0000000000000000000000000000000000000000000000000000000000000002",
                    "0x0000000000000000000000000000000000000000000000000000000000000006"
                ],
            },
            "latest",
        ]);
        HashMap::from([
            (
                address!("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"),
                vec![digest!("0x0000000000000000000000000000000000000000000000000000000000000001")],
            ),
            (
                address!("0xdAC17F958D2ee523a2206206994597C13D831ec7"),
                vec![
                    digest!("0x00000000000000000000000000000000000000000000000000000000000f4240"),
                    digest!("0x0000000000000000000000000000000000000000000000000000000000000012"),
                ],
            ),
        ]) => json!({
            "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48": [
                "0x0000000000000000000000000000000000000000000000000000000000000001"
            ],
            "0xdAC17F958D2ee523a2206206994597C13D831ec7": [
                "0x00000000000000000000000000000000000000000000000000000000000f4240",
                "0x0000000000000000000000000000000000000000000000000000000000000012"
            ],
        });
    });
}

#[test]
fn eth_get_transaction_by_block_hash_and_index() {
    assert_method_serialization!(eth::GetTransactionByBlockHashAndIndex {
        (
            digest!("0xbf137c3a7a1ebdfac21252765e5d7f40d115c2757e4a4abee929be88c624fdb7"),
            0x2,
        ) => json!([
            "0xbf137c3a7a1ebdfac21252765e5d7f40d115c2757e4a4abee929be88c624fdb7",
            "0x2",
        ]);
        Some(SignedTransaction::Legacy(SignedLegacyTransaction {
            block_hash: digest!("0x510efccf44a192e6e34bcb439a1947e24b86244280762cbb006858c237093fda"),
            block_number: 0x422,
            block_timestamp: None,
            from: address!("0xFE3B557E8Fb62b89F4916B721be55cEb828dBd73"),
            gas: 0x5208,
            gas_price: uint!("0x3b9aca00"),
            hash: digest!("0xa52be92809541220ee0aaaede6047d9a6c5d0cd96a517c854d944ee70a0ebb44"),
            input: vec![],
            nonce: 0x1,
            to: Some(address!("0x627306090abaB3A6e1400e9345bC60c78a8BEf57")),
            transaction_index: 0x0,
            value: uint!("0x4e1003b28d9280000"),
            chain_id: Some(0x7e2),
            v: 0xfe7,
            r: uint!("0x84caf09aefbd5e539295acc67217563438a4efb224879b6855f56857fa2037d3"),
            s: uint!("0x5e863be3829812c81439f0ae9d8ecb832b531d651fb234c848d1bf45e62be8b9"),
        })) => json!({
            "type": "0x0",
            "blockHash": "0x510efccf44a192e6e34bcb439a1947e24b86244280762cbb006858c237093fda",
            "blockNumber": "0x422",
            "from": "0xFE3B557E8Fb62b89F4916B721be55cEb828dBd73",
            "gas": "0x5208",
            "gasPrice": "0x3b9aca00",
            "hash": "0xa52be92809541220ee0aaaede6047d9a6c5d0cd96a517c854d944ee70a0ebb44",
            "input": "0x",
            "nonce": "0x1",
            "to": "0x627306090abaB3A6e1400e9345bC60c78a8BEf57",
            "transactionIndex": "0x0",
            "value": "0x4e1003b28d9280000",
            "chainId": "0x7e2",
            "v": "0xfe7",
            "r": "0x84caf09aefbd5e539295acc67217563438a4efb224879b6855f56857fa2037d3",
            "s": "0x5e863be3829812c81439f0ae9d8ecb832b531d651fb234c848d1bf45e62be8b9",
        });
    });
}

#[test]
fn eth_get_transaction_by_block_number_and_index() {
    assert_method_serialization!(eth::GetTransactionByBlockNumberAndIndex {
        (BlockSpec::Number(0x1442e), 0x2)
            => json!(["0x1442e", "0x2"]);
        Some(SignedTransaction::Legacy(SignedLegacyTransaction {
            block_hash: digest!("0x510efccf44a192e6e34bcb439a1947e24b86244280762cbb006858c237093fda"),
            block_number: 0x422,
            block_timestamp: None,
            from: address!("0xFE3B557E8Fb62b89F4916B721be55cEb828dBd73"),
            gas: 0x5208,
            gas_price: uint!("0x3b9aca00"),
            hash: digest!("0xa52be92809541220ee0aaaede6047d9a6c5d0cd96a517c854d944ee70a0ebb44"),
            input: vec![],
            nonce: 0x1,
            to: Some(address!("0x627306090abaB3A6e1400e9345bC60c78a8BEf57")),
            transaction_index: 0x0,
            value: uint!("0x4e1003b28d9280000"),
            chain_id: Some(0x7e2),
            v: 0xfe7,
            r: uint!("0x84caf09aefbd5e539295acc67217563438a4efb224879b6855f56857fa2037d3"),
            s: uint!("0x5e863be3829812c81439f0ae9d8ecb832b531d651fb234c848d1bf45e62be8b9"),
        })) => json!({
            "type": "0x0",
            "blockHash": "0x510efccf44a192e6e34bcb439a1947e24b86244280762cbb006858c237093fda",
            "blockNumber": "0x422",
            "from": "0xFE3B557E8Fb62b89F4916B721be55cEb828dBd73",
            "gas": "0x5208",
            "gasPrice": "0x3b9aca00",
            "hash": "0xa52be92809541220ee0aaaede6047d9a6c5d0cd96a517c854d944ee70a0ebb44",
            "input": "0x",
            "nonce": "0x1",
            "to": "0x627306090abaB3A6e1400e9345bC60c78a8BEf57",
            "transactionIndex": "0x0",
            "value": "0x4e1003b28d9280000",
            "chainId": "0x7e2",
            "v": "0xfe7",
            "r": "0x84caf09aefbd5e539295acc67217563438a4efb224879b6855f56857fa2037d3",
            "s": "0x5e863be3829812c81439f0ae9d8ecb832b531d651fb234c848d1bf45e62be8b9",
        });
    });
}

#[test]
fn eth_get_transaction_by_hash() {
    assert_method_serialization!(eth::GetTransactionByHash {
        (digest!("0xa52be92809541220ee0aaaede6047d9a6c5d0cd96a517c854d944ee70a0ebb44"),)
            => json!(["0xa52be92809541220ee0aaaede6047d9a6c5d0cd96a517c854d944ee70a0ebb44"]);
        Some(SignedTransaction::Legacy(SignedLegacyTransaction {
            block_hash: digest!("0x510efccf44a192e6e34bcb439a1947e24b86244280762cbb006858c237093fda"),
            block_number: 0x422,
            block_timestamp: None,
            from: address!("0xFE3B557E8Fb62b89F4916B721be55cEb828dBd73"),
            gas: 0x5208,
            gas_price: uint!("0x3b9aca00"),
            hash: digest!("0xa52be92809541220ee0aaaede6047d9a6c5d0cd96a517c854d944ee70a0ebb44"),
            input: vec![],
            nonce: 0x1,
            to: Some(address!("0x627306090abaB3A6e1400e9345bC60c78a8BEf57")),
            transaction_index: 0x0,
            value: uint!("0x4e1003b28d9280000"),
            chain_id: Some(0x7e2),
            v: 0xfe7,
            r: uint!("0x84caf09aefbd5e539295acc67217563438a4efb224879b6855f56857fa2037d3"),
            s: uint!("0x5e863be3829812c81439f0ae9d8ecb832b531d651fb234c848d1bf45e62be8b9"),
        })) => json!({
            "type": "0x0",
            "blockHash": "0x510efccf44a192e6e34bcb439a1947e24b86244280762cbb006858c237093fda",
            "blockNumber": "0x422",
            "from": "0xFE3B557E8Fb62b89F4916B721be55cEb828dBd73",
            "gas": "0x5208",
            "gasPrice": "0x3b9aca00",
            "hash": "0xa52be92809541220ee0aaaede6047d9a6c5d0cd96a517c854d944ee70a0ebb44",
            "input": "0x",
            "nonce": "0x1",
            "to": "0x627306090abaB3A6e1400e9345bC60c78a8BEf57",
            "transactionIndex": "0x0",
            "value": "0x4e1003b28d9280000",
            "chainId": "0x7e2",
            "v": "0xfe7",
            "r": "0x84caf09aefbd5e539295acc67217563438a4efb224879b6855f56857fa2037d3",
            "s": "0x5e863be3829812c81439f0ae9d8ecb832b531d651fb234c848d1bf45e62be8b9",
        });
    });
}

#[test]
fn eth_get_transaction_count() {
    assert_method_serialization!(eth::GetTransactionCount {
        (
            address!("0xc94770007dda54cF92009BFF0dE90c06F603a09f"),
            BlockId::Tag(BlockTag::Latest),
        ) => json!([
            "0xc94770007dda54cF92009BFF0dE90c06F603a09f",
            "latest",
        ]);
        0x1 => json!("0x1");
    });
}

#[test]
fn eth_get_transaction_receipt() {
    assert_method_serialization!(eth::GetTransactionReceipt {
        (digest!("0x504ce587a65bdbdb6414a0c6c16d86a04dd79bfcc4f2950eec9634b30ce5370f"),)
            => json!(["0x504ce587a65bdbdb6414a0c6c16d86a04dd79bfcc4f2950eec9634b30ce5370f"]);
        Some(TransactionReceipt {
            kind: TransactionReceiptKind::Eip4844 {
                blob_gas_used: 0x20000,
                blob_gas_price: uint!("0x3"),
            },
            transaction_hash: digest!("0xc00e97af59c6f88de163306935f7682af1a34c67245e414537d02e422815efc3"),
            transaction_index: 0x0,
            block_hash: digest!("0xe7212a92cfb9b06addc80dec2a0dfae9ea94fd344efeb157c41e12994fcad60a"),
            block_number: 0x50,
            from: address!("0x627306090abaB3A6e1400e9345bC60c78a8BEf57"),
            to: Some(address!("0xf17f52151EbEF6C7334FAD080c5704D77216b732")),
            effective_gas_price: uint!("0x1"),
            cumulative_gas_used: 0x5208,
            gas_used: 0x5208,
            contract_address: None,
            logs: vec![],
            logs_bloom: Bloom([0; 256]),
            root: None,
            status: Some(TransactionReceiptStatus::Success),
        }) => json!({
            "type": "0x3",
            "blobGasUsed": "0x20000",
            "blobGasPrice": "0x3",
            "transactionHash": "0xc00e97af59c6f88de163306935f7682af1a34c67245e414537d02e422815efc3",
            "transactionIndex": "0x0",
            "blockHash": "0xe7212a92cfb9b06addc80dec2a0dfae9ea94fd344efeb157c41e12994fcad60a",
            "blockNumber": "0x50",
            "from": "0x627306090abaB3A6e1400e9345bC60c78a8BEf57",
            "to": "0xf17f52151EbEF6C7334FAD080c5704D77216b732",
            "effectiveGasPrice": "0x1",
            "cumulativeGasUsed": "0x5208",
            "gasUsed": "0x5208",
            "contractAddress": null,
            "logs": [],
            "logsBloom":
                "0x0000000000000000000000000000000000000000000000000000000000000000\
                   0000000000000000000000000000000000000000000000000000000000000000\
                   0000000000000000000000000000000000000000000000000000000000000000\
                   0000000000000000000000000000000000000000000000000000000000000000\
                   0000000000000000000000000000000000000000000000000000000000000000\
                   0000000000000000000000000000000000000000000000000000000000000000\
                   0000000000000000000000000000000000000000000000000000000000000000\
                   0000000000000000000000000000000000000000000000000000000000000000",
            "status": "0x1",
        });
    });
}

#[test]
fn eth_max_priority_fee_per_gas() {
    assert_method_serialization!(eth::MaxPriorityFeePerGas {
        Empty => json!([]);
        uint!("0x773c23ba") => json!("0x773c23ba");
    });
}

#[test]
fn eth_new_block_filter() {
    assert_method_serialization!(eth::NewBlockFilter {
        Empty => json!([]);
        FilterId::from_raw(uint!("0x1")) => json!("0x1");
    });
}

#[test]
fn eth_new_filter() {
    assert_method_serialization!(eth::NewFilter {
        (LogFilter {
            blocks: LogFilterBlocks::Range {
                from: BlockSpec::Number(0x137d3c2),
                to: BlockSpec::Number(0x137d3c3),
            },
            address: LogFilterValue::Exact(address!("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2")),
            topics: ArrayVec::new(),
        },) => json!([{
            "fromBlock": "0x137d3c2",
            "toBlock": "0x137d3c3",
            "address": "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
            "topics": [],
        }]);
        FilterId::from_raw(uint!("0x1")) => json!("0x1");
    });
}

#[test]
fn eth_new_pending_transaction_filter() {
    assert_method_serialization!(eth::NewPendingTransactionFilter {
        Empty => json!([]);
        FilterId::from_raw(uint!("0x1")) => json!("0x1");
    });
}

#[test]
fn eth_send_raw_transaction() {
    assert_method_serialization!(eth::SendRawTransaction {
        (
            hex!(
                "f869018203e882520894f17f52151ebef6c7334fad080c5704d77216b732881b\
                 c16d674ec80000801ba02da1c48b670996dcb1f447ef9ef00b33033c48a4fe93\
                 8f420bec3e56bfd24071a062e0aa78a81bf0290afbc3a9d8e9a068e6d74caa66\
                 c5e0fa8a46deaae96b0833"
            )
            .to_vec(),
        ) => json!([
            "0xf869018203e882520894f17f52151ebef6c7334fad080c5704d77216b732881b\
               c16d674ec80000801ba02da1c48b670996dcb1f447ef9ef00b33033c48a4fe93\
               8f420bec3e56bfd24071a062e0aa78a81bf0290afbc3a9d8e9a068e6d74caa66\
               c5e0fa8a46deaae96b0833",
        ]);
        digest!("0x0e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527331") => json!("0x0e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527331");
    });
}

#[test]
fn eth_send_transaction() {
    assert_method_serialization!(eth::SendTransaction {
        (Transaction {
            from: Some(address!("0xb60E8dD61C5d32be8058BB8eb970870F07233155")),
            to: Some(address!("0xd46E8dD67C5d32be8058Bb8Eb970870F07244567")),
            gas: Some(0x76c0),
            gas_price: Some(uint!("0x9184e72a000")),
            value: Some(uint!("0x9184e72a")),
            input: Some(
                hex!(
                    "d46e8dd67c5d32be8d46e8dd67c5d32be8058bb8eb970870f072445675058bb8\
                     eb970870f072445675"
                )
                .to_vec(),
            ),
            ..Default::default()
        },) => json!([{
            "from": "0xb60E8dD61C5d32be8058BB8eb970870F07233155",
            "to": "0xd46E8dD67C5d32be8058Bb8Eb970870F07244567",
            "gas": "0x76c0",
            "gasPrice": "0x9184e72a000",
            "value": "0x9184e72a",
            "input":
                "0xd46e8dd67c5d32be8d46e8dd67c5d32be8058bb8eb970870f072445675058bb8\
                   eb970870f072445675",
        }]);
        digest!("0x0e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527331") => json!("0x0e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527331");
    });
}

#[test]
fn eth_sign() {
    assert_method_serialization!(eth::Sign {
        (
            address!("0x9B2055d370F73eC7d8a03E965129118dC8F5bf83"),
            hex!("deadbeaf").to_vec(),
        ) => json!([
            "0x9B2055d370F73eC7d8a03E965129118dC8F5bf83",
            "0xdeadbeaf",
        ]);
        hex!(
            "a3f20717a250c2b0b729b7e5becbff67fdaef7e0699da4de7ca5895b02a170a1\
             2d887fd3b17bfdce3481f10bea41f45ba9f709d39ce8325427b57afcfc994cee\
             1b"
        )
        .to_vec() => json!(
            "0xa3f20717a250c2b0b729b7e5becbff67fdaef7e0699da4de7ca5895b02a170a1\
               2d887fd3b17bfdce3481f10bea41f45ba9f709d39ce8325427b57afcfc994cee\
               1b"
        );
    });
}

#[test]
fn eth_sign_transaction() {
    assert_method_serialization!(eth::SignTransaction {
        (Transaction {
            from: Some(address!("0xb60E8dD61C5d32be8058BB8eb970870F07233155")),
            to: Some(address!("0xd46E8dD67C5d32be8058Bb8Eb970870F07244567")),
            gas: Some(0x76c0),
            gas_price: Some(uint!("0x9184e72a000")),
            value: Some(uint!("0x9184e72a")),
            input: Some(
                hex!(
                    "d46e8dd67c5d32be8d46e8dd67c5d32be8058bb8eb970870f072445675058bb8\
                     eb970870f072445675"
                )
                .to_vec(),
            ),
            ..Default::default()
        },) => json!([{
            "from": "0xb60E8dD61C5d32be8058BB8eb970870F07233155",
            "to": "0xd46E8dD67C5d32be8058Bb8Eb970870F07244567",
            "gas": "0x76c0",
            "gasPrice": "0x9184e72a000",
            "value": "0x9184e72a",
            "input":
                "0xd46e8dd67c5d32be8d46e8dd67c5d32be8058bb8eb970870f072445675058bb8\
                   eb970870f072445675",
        }]);
        hex!(
            "a3f20717a250c2b0b729b7e5becbff67fdaef7e0699da4de7ca5895b02a170a1\
             2d887fd3b17bfdce3481f10bea41f45ba9f709d39ce8325427b57afcfc994cee\
             1b"
        )
        .to_vec() => json!(
            "0xa3f20717a250c2b0b729b7e5becbff67fdaef7e0699da4de7ca5895b02a170a1\
               2d887fd3b17bfdce3481f10bea41f45ba9f709d39ce8325427b57afcfc994cee\
               1b"
        );
    });
}

#[test]
fn eth_simulate_v1() {
    assert_method_serialization!(eth::SimulateV1 {
        (
            SimulatePayload {
                block_state_calls: vec![BlockStateCall {
                    block_overrides: None,
                    state_overrides: None,
                    calls: vec![Transaction {
                        from: Some(address!("0xc0ffee254729296a45a3885639AC7E10F9d54979")),
                        to: Some(address!("0xdAC17F958D2ee523a2206206994597C13D831ec7")),
                        value: Some(uint!("0x0")),
                        ..Default::default()
                    }],
                }],
                trace_transfers: Some(true),
                validation: Some(false),
                return_full_transactions: None,
            },
            Some(BlockSpec::Tag(BlockTag::Latest)),
        ) => json!([
            {
                "blockStateCalls": [{
                    "calls": [{
                        "from": "0xc0ffee254729296a45a3885639AC7E10F9d54979",
                        "to": "0xdAC17F958D2ee523a2206206994597C13D831ec7",
                        "value": "0x0",
                    }],
                }],
                "traceTransfers": true,
                "validation": false,
            },
            "latest",
        ]);
        vec![BlockResult {
            block: Block {
                hash: digest!("0x920189344ddf2bfadb7c8cf4362b8e8c18bfb021a5f6630f84d17460ca05f58c"),
                parent_hash: digest!("0x73b3825a878af74a1b0a0c4a8233afa743ac6f6b8027361a53afaaa0f45265b0"),
                sha3_uncles: digest!("0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"),
                miner: address!("0x4838B106FCe9647Bdf1E7877BF73cE8B0BAD5f97"),
                state_root: digest!("0xf8bed8e8aecd51364cb47cc175117bb204d4b539d70294c8c8127fe8b46191f2"),
                transactions_root: digest!("0xcd79f05db4ef640f2af2f569f5f953680f23558073542ba174e61df8169a0b2a"),
                receipts_root: digest!("0xf78dfb743fbd92ade140711c8bbc542b5e307f0ab7984eff35d751969fe57efa"),
                logs_bloom: Bloom([0; 256]),
                difficulty: uint!("0x0"),
                number: 0x1798e67,
                gas_limit: 0x3938700,
                gas_used: 0x5208,
                timestamp: 0x69c57aeb,
                extra_data: vec![],
                mix_hash: digest!("0x0000000000000000000000000000000000000000000000000000000000000000"),
                nonce: BlockNonce([0; 8]),
                base_fee_per_gas: uint!("0x0"),
                withdrawals_root: digest!("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"),
                blob_gas_used: 0x0,
                excess_blob_gas: 0xb5eba70,
                parent_beacon_block_root: digest!("0x0000000000000000000000000000000000000000000000000000000000000000"),
                requests_hash: digest!("0xe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
                size: 0x295,
                transactions: BlockTransactions::Hash(vec![
                    digest!("0xaa0f891967daa09f2813be3cfec01338649a13d24accee47331cfd801647a0bb"),
                ]),
                withdrawals: vec![],
                uncles: vec![],
            },
            calls: vec![CallResult::Success(CallResultSuccess {
                return_data: vec![],
                gas_used: 0x5208,
                logs: vec![],
            })],
        }] => json!([{
            "baseFeePerGas": "0x0",
            "blobGasUsed": "0x0",
            "difficulty": "0x0",
            "excessBlobGas": "0xb5eba70",
            "extraData": "0x",
            "gasLimit": "0x3938700",
            "gasUsed": "0x5208",
            "hash": "0x920189344ddf2bfadb7c8cf4362b8e8c18bfb021a5f6630f84d17460ca05f58c",
            "logsBloom":
                "0x0000000000000000000000000000000000000000000000000000000000000000\
                   0000000000000000000000000000000000000000000000000000000000000000\
                   0000000000000000000000000000000000000000000000000000000000000000\
                   0000000000000000000000000000000000000000000000000000000000000000\
                   0000000000000000000000000000000000000000000000000000000000000000\
                   0000000000000000000000000000000000000000000000000000000000000000\
                   0000000000000000000000000000000000000000000000000000000000000000\
                   0000000000000000000000000000000000000000000000000000000000000000",
            "miner": "0x4838B106FCe9647Bdf1E7877BF73cE8B0BAD5f97",
            "mixHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "nonce": "0x0000000000000000",
            "number": "0x1798e67",
            "parentBeaconBlockRoot": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "parentHash": "0x73b3825a878af74a1b0a0c4a8233afa743ac6f6b8027361a53afaaa0f45265b0",
            "receiptsRoot": "0xf78dfb743fbd92ade140711c8bbc542b5e307f0ab7984eff35d751969fe57efa",
            "requestsHash": "0xe3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
            "size": "0x295",
            "stateRoot": "0xf8bed8e8aecd51364cb47cc175117bb204d4b539d70294c8c8127fe8b46191f2",
            "timestamp": "0x69c57aeb",
            "transactions": [
                "0xaa0f891967daa09f2813be3cfec01338649a13d24accee47331cfd801647a0bb"
            ],
            "transactionsRoot": "0xcd79f05db4ef640f2af2f569f5f953680f23558073542ba174e61df8169a0b2a",
            "uncles": [],
            "withdrawals": [],
            "withdrawalsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
            "calls": [{
                "status": "0x1",
                "returnData": "0x",
                "gasUsed": "0x5208",
                "logs": [],
            }],
        }]);
    });
}

#[test]
fn eth_syncing() {
    assert_method_serialization!(eth::Syncing {
        Empty => json!([]);
        SyncingStatus::Syncing(SyncingProgress {
            starting_block: 0x0,
            current_block: 0x1518,
            highest_block: 0x9567a3,
        }) => json!({
            "startingBlock": "0x0",
            "currentBlock": "0x1518",
            "highestBlock": "0x9567a3",
        });
    });

    assert_method_serialization!(eth::Syncing {
        Empty => json!([]);
        SyncingStatus::NotSyncing => json!(false);
    });
}

#[test]
fn eth_uninstall_filter() {
    assert_method_serialization!(eth::UninstallFilter {
        (FilterId::from_raw(uint!("0x1")),) => json!(["0x1"]);
        true => json!(true);
    })
}
