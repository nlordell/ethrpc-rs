//! Ethereum RPC types.

use crate::{debug, serialization};
use ethprim::AsU256 as _;
use serde::{
    Deserialize, Serialize,
    de::{self, Deserializer},
    ser::Serializer,
};
use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
};

pub use crate::bloom::Bloom;
pub use arrayvec::ArrayVec;
pub use ethprim::{Address, Digest, I256, U256};

/// Empty JSON RPC parameters.
pub struct Empty;

impl Serialize for Empty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        [(); 0].serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Empty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        <[(); 0]>::deserialize(deserializer)?;
        Ok(Empty)
    }
}

/// Block number or tag.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BlockSpec {
    /// Block by number.
    Number(U256),
    /// Block by tag.
    Tag(BlockTag),
}

impl Default for BlockSpec {
    fn default() -> Self {
        Self::Tag(Default::default())
    }
}

impl From<U256> for BlockSpec {
    fn from(number: U256) -> Self {
        Self::Number(number)
    }
}

impl From<u64> for BlockSpec {
    fn from(number: u64) -> Self {
        number.as_u256().into()
    }
}

impl From<BlockTag> for BlockSpec {
    fn from(tag: BlockTag) -> Self {
        Self::Tag(tag)
    }
}

/// Block number, tag, or block hash.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BlockId {
    /// Block by number.
    Number(U256),
    /// Block by hash.
    Hash(Digest),
    /// Block by tag.
    Tag(BlockTag),
}

impl Default for BlockId {
    fn default() -> Self {
        Self::Tag(Default::default())
    }
}

impl From<U256> for BlockId {
    fn from(number: U256) -> Self {
        Self::Number(number)
    }
}

impl From<u64> for BlockId {
    fn from(number: u64) -> Self {
        number.as_u256().into()
    }
}

impl From<BlockTag> for BlockId {
    fn from(tag: BlockTag) -> Self {
        Self::Tag(tag)
    }
}

impl From<BlockSpec> for BlockId {
    fn from(spec: BlockSpec) -> Self {
        match spec {
            BlockSpec::Number(number) => Self::Number(number),
            BlockSpec::Tag(tag) => Self::Tag(tag),
        }
    }
}

/// Block tag.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BlockTag {
    /// The lowest numbered block the client has available.
    Earliest,
    /// The most recent crypto-economically secure block, cannot be re-orged
    /// outside of manual intervention driven by community coordination.
    Finalized,
    /// The most recent block that is safe from re-orgs under honest majority
    /// and certain synchronicity assumptions.
    Safe,
    /// The most recent block in the canonical chain observed by the client,
    /// this block may be re-orged out of the canonical chain even under
    /// healthy/normal conditions.
    #[default]
    Latest,
    /// A sample next block built by the client on top of [`BlockTag::Latest`]
    /// and containing the set of transactions usually taken from local mempool.
    Pending,
}

/// A log, block, or transaction filter identifier.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct FilterId(U256);

impl FilterId {
    /// Creates a filter from a raw ID. The caller must make sure that this is a
    /// valid ID, otherwise filter ID RPC methods will fail.
    pub fn from_raw(value: U256) -> Self {
        Self(value)
    }

    /// Gets the raw underlying ID for the filter.
    pub fn into_raw(self) -> U256 {
        self.0
    }
}

/// Whether block transactions should be hydrated.
#[derive(Clone, Copy, Debug, Default)]
pub enum Hydrated {
    /// Only fetch transaction hashes for blocks.
    #[default]
    No,
    /// Fetch full transaction data for blocks.
    Yes,
}

impl Hydrated {
    /// Returns the value matching the boolean value used for encoding Ethereum RPC calls for this
    /// parameter.
    fn from_bool(value: bool) -> Self {
        match value {
            false => Self::No,
            true => Self::Yes,
        }
    }

    /// Returns the boolean value used for encoding Ethereum RPC calls for this
    /// parameter.
    fn as_bool(&self) -> bool {
        match self {
            Self::No => false,
            Self::Yes => true,
        }
    }
}

impl Serialize for Hydrated {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_bool().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Hydrated {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        bool::deserialize(deserializer).map(Self::from_bool)
    }
}

/// A block nonce.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct BlockNonce(pub [u8; 8]);

impl Debug for BlockNonce {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("BlockNonce")
            .field(&debug::Hex(&self.0))
            .finish()
    }
}

impl Serialize for BlockNonce {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialization::bytearray::serialize(&self.0, serializer)
    }
}

impl<'de> Deserialize<'de> for BlockNonce {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serialization::bytearray::deserialize(deserializer).map(Self)
    }
}

/// Transactions included in a block.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BlockTransactions {
    /// Transaction hashes that were part of a block.
    Hash(Vec<Digest>),
    /// Full transaction data.
    Full(Vec<SignedTransaction>),
}

/// A signed transaction.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum SignedTransaction {
    /// Signed legacy transaction.
    #[serde(rename = "0x0")]
    Legacy(SignedLegacyTransaction),
    /// Signed EIP-2930 transaction.
    #[serde(rename = "0x1")]
    Eip2930(SignedEip2930Transaction),
    /// Signed EIP-1559 transaction.
    #[serde(rename = "0x2")]
    Eip1559(SignedEip1559Transaction),
    /// Signed EIP-4844 transaction.
    #[serde(rename = "0x3")]
    Eip4844(SignedEip4844Transaction),
    /// Signed EIP-7702 transaction.
    #[serde(rename = "0x4")]
    Eip7702(SignedEip7702Transaction),
}

/// The signature parity.
#[derive(Clone, Copy, Debug, Eq, Ord, Hash, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum YParity {
    /// Even parity (0).
    Even = 0,
    /// Odd parity (1).
    Odd = 1,
}

impl Serialize for YParity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).as_u256().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for YParity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = U256::deserialize(deserializer)?;
        match u8::try_from(value) {
            Ok(0) => Ok(Self::Even),
            Ok(1) => Ok(Self::Odd),
            _ => Err(de::Error::custom(format!("invalid y-parity value {value}"))),
        }
    }
}

/// Signed legacy transaction.
#[derive(Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedLegacyTransaction {
    /// The hash of the block containing the transaction.
    pub block_hash: Digest,
    /// The height of the block containing the transaction.
    pub block_number: U256,
    /// The timestamp of the block containing the transaction.
    #[serde(with = "serialization::num")]
    pub block_timestamp: u64,
    /// Address of transaction sender.
    pub from: Address,
    /// The limit in gas units for the transaction.
    #[serde(with = "serialization::num")]
    pub gas: u64,
    /// Gas price willing to be paid by the sender.
    pub gas_price: U256,
    /// The hash of the transaction.
    pub hash: Digest,
    /// The calldata associated with the transaction.
    #[serde(with = "serialization::bytes")]
    pub input: Vec<u8>,
    /// The transaction nonce.
    #[serde(with = "serialization::num")]
    pub nonce: u64,
    /// The transaction recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Address>,
    /// The index of the transaction within the block it was included.
    #[serde(with = "serialization::num")]
    pub transaction_index: u64,
    /// The Ether value associated with the transaction.
    pub value: U256,
    /// Chain ID that the transaction is valid on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<U256>,
    /// V
    pub v: U256,
    /// R
    pub r: U256,
    /// S
    pub s: U256,
}

impl Debug for SignedLegacyTransaction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("SignedLegacyTransaction")
            .field("block_hash", &self.block_hash)
            .field("block_number", &self.block_number)
            .field("block_timestamp", &self.block_timestamp)
            .field("from", &self.from)
            .field("gas", &self.gas)
            .field("gas_price", &self.gas_price)
            .field("hash", &self.hash)
            .field("input", &debug::Hex(&self.input))
            .field("nonce", &self.nonce)
            .field("to", &self.to)
            .field("transaction_index", &self.transaction_index)
            .field("value", &self.value)
            .field("chain_id", &self.chain_id)
            .field("v", &self.v)
            .field("r", &self.r)
            .field("s", &self.s)
            .finish()
    }
}

/// Signed EIP-2930 transaction.
#[derive(Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedEip2930Transaction {
    /// The hash of the block containing the transaction.
    pub block_hash: Digest,
    /// The height of the block containing the transaction.
    pub block_number: U256,
    /// The timestamp of the block containing the transaction.
    #[serde(with = "serialization::num")]
    pub block_timestamp: u64,
    /// Address of transaction sender.
    pub from: Address,
    /// The limit in gas units for the transaction.
    #[serde(with = "serialization::num")]
    pub gas: u64,
    /// Gas price willing to be paid by the sender.
    pub gas_price: U256,
    /// The hash of the transaction.
    pub hash: Digest,
    /// The calldata associated with the transaction.
    #[serde(with = "serialization::bytes")]
    pub input: Vec<u8>,
    /// The transaction nonce.
    #[serde(with = "serialization::num")]
    pub nonce: u64,
    /// The transaction recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Address>,
    /// The index of the transaction within the block it was included.
    #[serde(with = "serialization::num")]
    pub transaction_index: u64,
    /// The Ether value associated with the transaction.
    pub value: U256,
    /// State access list.
    pub access_list: AccessList,
    /// Chain ID that the transaction is valid on.
    pub chain_id: U256,
    /// R
    pub r: U256,
    /// S
    pub s: U256,
    /// Y parity of the signature.
    #[serde(alias = "v")]
    pub y_parity: YParity,
}

impl Debug for SignedEip2930Transaction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("SignedEip2930Transaction")
            .field("block_hash", &self.block_hash)
            .field("block_number", &self.block_number)
            .field("block_timestamp", &self.block_timestamp)
            .field("from", &self.from)
            .field("gas", &self.gas)
            .field("gas_price", &self.gas_price)
            .field("hash", &self.hash)
            .field("input", &debug::Hex(&self.input))
            .field("nonce", &self.nonce)
            .field("to", &self.to)
            .field("transaction_index", &self.transaction_index)
            .field("value", &self.value)
            .field("access_list", &self.access_list)
            .field("chain_id", &self.chain_id)
            .field("r", &self.r)
            .field("s", &self.s)
            .field("y_parity", &self.y_parity)
            .finish()
    }
}

/// Signed EIP-1559 transaction.
#[derive(Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedEip1559Transaction {
    /// The hash of the block containing the transaction.
    pub block_hash: Digest,
    /// The height of the block containing the transaction.
    pub block_number: U256,
    /// The timestamp of the block containing the transaction.
    #[serde(with = "serialization::num")]
    pub block_timestamp: u64,
    /// Address of transaction sender.
    pub from: Address,
    /// The limit in gas units for the transaction.
    #[serde(with = "serialization::num")]
    pub gas: u64,
    /// The maximum total fee per gas the sender is willing to pay, including
    /// the network (A.K.A. base) fee and miner (A.K.A priority) fee.
    pub max_fee_per_gas: U256,
    /// Maximum fee per gas the sender is willing to pay to miners in wei
    pub max_priority_fee_per_gas: U256,
    /// The hash of the transaction.
    pub hash: Digest,
    /// The calldata associated with the transaction.
    #[serde(with = "serialization::bytes")]
    pub input: Vec<u8>,
    /// The transaction nonce.
    #[serde(with = "serialization::num")]
    pub nonce: u64,
    /// The transaction recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Address>,
    /// The index of the transaction within the block it was included.
    #[serde(with = "serialization::num")]
    pub transaction_index: u64,
    /// The Ether value associated with the transaction.
    pub value: U256,
    /// State access list.
    pub access_list: AccessList,
    /// Chain ID that the transaction is valid on.
    pub chain_id: U256,
    /// R
    pub r: U256,
    /// S
    pub s: U256,
    /// Y parity of the signature.
    #[serde(alias = "v")]
    pub y_parity: YParity,
}

impl Debug for SignedEip1559Transaction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("SignedEip1559Transaction")
            .field("block_hash", &self.block_hash)
            .field("block_number", &self.block_number)
            .field("block_timestamp", &self.block_timestamp)
            .field("from", &self.from)
            .field("gas", &self.gas)
            .field("max_fee_per_gas", &self.max_fee_per_gas)
            .field("max_priority_fee_per_gas", &self.max_priority_fee_per_gas)
            .field("hash", &self.hash)
            .field("input", &debug::Hex(&self.input))
            .field("nonce", &self.nonce)
            .field("to", &self.to)
            .field("transaction_index", &self.transaction_index)
            .field("value", &self.value)
            .field("access_list", &self.access_list)
            .field("chain_id", &self.chain_id)
            .field("r", &self.r)
            .field("s", &self.s)
            .field("y_parity", &self.y_parity)
            .finish()
    }
}

/// Signed EIP-4844 transaction.
#[derive(Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedEip4844Transaction {
    /// The hash of the block containing the transaction.
    pub block_hash: Digest,
    /// The height of the block containing the transaction.
    pub block_number: U256,
    /// The timestamp of the block containing the transaction.
    #[serde(with = "serialization::num")]
    pub block_timestamp: u64,
    /// Address of transaction sender.
    pub from: Address,
    /// The limit in gas units for the transaction.
    #[serde(with = "serialization::num")]
    pub gas: u64,
    /// The maximum total fee per gas the sender is willing to pay, including
    /// the network (A.K.A. base) fee and miner (A.K.A priority) fee.
    pub max_fee_per_gas: U256,
    /// Maximum fee per gas the sender is willing to pay to miners in wei
    pub max_priority_fee_per_gas: U256,
    /// The maximum total fee per gas the sender is willing to pay for blob gas
    /// in wei.
    pub max_fee_per_blob_gas: U256,
    /// The hash of the transaction.
    pub hash: Digest,
    /// The calldata associated with the transaction.
    #[serde(with = "serialization::bytes")]
    pub input: Vec<u8>,
    /// The transaction nonce.
    #[serde(with = "serialization::num")]
    pub nonce: u64,
    /// The transaction recipient.
    pub to: Address,
    /// The index of the transaction within the block it was included.
    #[serde(with = "serialization::num")]
    pub transaction_index: u64,
    /// The Ether value associated with the transaction.
    pub value: U256,
    /// State access list.
    pub access_list: AccessList,
    /// Chain ID that the transaction is valid on.
    pub chain_id: U256,
    /// List of versioned blob hashes associated with the transaction's EIP-4844
    /// data blobs.
    pub blob_versioned_hashes: Vec<Digest>,
    /// R
    pub r: U256,
    /// S
    pub s: U256,
    /// Y parity of the signature.
    #[serde(alias = "v")]
    pub y_parity: YParity,
}

impl Debug for SignedEip4844Transaction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("SignedEip4844Transaction")
            .field("block_hash", &self.block_hash)
            .field("block_number", &self.block_number)
            .field("block_timestamp", &self.block_timestamp)
            .field("from", &self.from)
            .field("gas", &self.gas)
            .field("max_fee_per_gas", &self.max_fee_per_gas)
            .field("max_priority_fee_per_gas", &self.max_priority_fee_per_gas)
            .field("max_fee_per_blob_gas", &self.max_fee_per_blob_gas)
            .field("hash", &self.hash)
            .field("input", &debug::Hex(&self.input))
            .field("nonce", &self.nonce)
            .field("to", &self.to)
            .field("transaction_index", &self.transaction_index)
            .field("value", &self.value)
            .field("access_list", &self.access_list)
            .field("chain_id", &self.chain_id)
            .field("blob_versioned_hashes", &self.blob_versioned_hashes)
            .field("r", &self.r)
            .field("s", &self.s)
            .field("y_parity", &self.y_parity)
            .finish()
    }
}

/// Signed EIP-7702 transaction.
#[derive(Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedEip7702Transaction {
    /// The hash of the block containing the transaction.
    pub block_hash: Digest,
    /// The height of the block containing the transaction.
    pub block_number: U256,
    /// The timestamp of the block containing the transaction.
    #[serde(with = "serialization::num")]
    pub block_timestamp: u64,
    /// Address of transaction sender.
    pub from: Address,
    /// The limit in gas units for the transaction.
    #[serde(with = "serialization::num")]
    pub gas: u64,
    /// The effective gas price paid by the sender in wei.
    pub gas_price: U256,
    /// The maximum total fee per gas the sender is willing to pay, including
    /// the network (A.K.A. base) fee and miner (A.K.A priority) fee.
    pub max_fee_per_gas: U256,
    /// Maximum fee per gas the sender is willing to pay to miners in wei
    pub max_priority_fee_per_gas: U256,
    /// The hash of the transaction.
    pub hash: Digest,
    /// The calldata associated with the transaction.
    #[serde(with = "serialization::bytes")]
    pub input: Vec<u8>,
    /// The transaction nonce.
    #[serde(with = "serialization::num")]
    pub nonce: u64,
    /// The transaction recipient.
    pub to: Address,
    /// The index of the transaction within the block it was included.
    #[serde(with = "serialization::num")]
    pub transaction_index: u64,
    /// The Ether value associated with the transaction.
    pub value: U256,
    /// State access list.
    pub access_list: AccessList,
    /// Chain ID that the transaction is valid on.
    pub chain_id: U256,
    /// Transaction authorization list.
    pub authorization_list: AuthorizationList,
    /// R
    pub r: U256,
    /// S
    pub s: U256,
    /// Y parity of the signature.
    #[serde(alias = "v")]
    pub y_parity: YParity,
}

impl Debug for SignedEip7702Transaction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("SignedEip7702Transaction")
            .field("block_hash", &self.block_hash)
            .field("block_number", &self.block_number)
            .field("block_timestamp", &self.block_timestamp)
            .field("from", &self.from)
            .field("gas", &self.gas)
            .field("gas_price", &self.gas_price)
            .field("max_fee_per_gas", &self.max_fee_per_gas)
            .field("max_priority_fee_per_gas", &self.max_priority_fee_per_gas)
            .field("hash", &self.hash)
            .field("input", &debug::Hex(&self.input))
            .field("nonce", &self.nonce)
            .field("to", &self.to)
            .field("transaction_index", &self.transaction_index)
            .field("value", &self.value)
            .field("access_list", &self.access_list)
            .field("chain_id", &self.chain_id)
            .field("authorization_list", &self.authorization_list)
            .field("r", &self.r)
            .field("s", &self.s)
            .field("y_parity", &self.y_parity)
            .finish()
    }
}

/// A validator withdrawal.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Withdrawal {
    /// Recipient address.
    pub address: Address,
    /// Withdrawal amount.
    #[serde(with = "serialization::num")]
    pub amount: u64,
    /// Withdrawal index.
    #[serde(with = "serialization::num")]
    pub index: u64,
    /// Validator index.
    #[serde(with = "serialization::num")]
    pub validator_index: u64,
}

/// An Ethereum block object.
#[derive(Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    /// The block hash.
    pub hash: Digest,
    /// The parent block hash.
    pub parent_hash: Digest,
    /// The Ommer's hash.
    pub sha3_uncles: Digest,
    /// The coinbase. This is the address that received the block rewards.
    pub miner: Address,
    /// The state root.
    pub state_root: Digest,
    /// The transactions root.
    pub transactions_root: Digest,
    /// The transaction receipts root.
    pub receipts_root: Digest,
    /// The log bloom filter.
    pub logs_bloom: Bloom,
    /// The difficulty.
    pub difficulty: U256,
    /// The block height.
    pub number: U256,
    /// The gas limit.
    pub gas_limit: U256,
    /// The total gas used by all transactions.
    pub gas_used: U256,
    /// The timestamp (in second).
    pub timestamp: U256,
    /// Extra data.
    #[serde(with = "serialization::bytes")]
    pub extra_data: Vec<u8>,
    /// The mix hash.
    pub mix_hash: Digest,
    /// The nonce.
    pub nonce: BlockNonce,
    /// The total difficulty.
    pub total_difficulty: U256,
    /// The base fee per gas.
    #[serde(default)]
    pub base_fee_per_gas: U256,
    /// The withdrawals root.
    #[serde(default)]
    pub withdrawals_root: Digest,
    /// Blob gas used.
    #[serde(default)]
    pub blob_gas_used: U256,
    /// Excess blob gas.
    #[serde(default)]
    pub excess_blob_gas: U256,
    /// Parent beacon block root.
    #[serde(default)]
    pub parent_beacon_block_root: Digest,
    /// EIP-7685 requests hash.
    #[serde(default)]
    pub requests_hash: Digest,
    /// The size of the block.
    pub size: U256,
    /// Block transactions.
    pub transactions: BlockTransactions,
    /// Withdrawals.
    #[serde(default)]
    pub withdrawals: Vec<Withdrawal>,
    /// Uncle hashes.
    pub uncles: Vec<Digest>,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Block")
            .field("hash", &self.hash)
            .field("parent_hash", &self.parent_hash)
            .field("sha3_uncles", &self.sha3_uncles)
            .field("miner", &self.miner)
            .field("state_root", &self.state_root)
            .field("transactions_root", &self.transactions_root)
            .field("receipts_root", &self.receipts_root)
            .field("logs_bloom", &self.logs_bloom)
            .field("difficulty", &self.difficulty)
            .field("number", &self.number)
            .field("gas_limit", &self.gas_limit)
            .field("gas_used", &self.gas_used)
            .field("timestamp", &self.timestamp)
            .field("extra_data", &debug::Hex(&self.extra_data))
            .field("mix_hash", &self.mix_hash)
            .field("nonce", &self.nonce)
            .field("total_difficulty", &self.total_difficulty)
            .field("base_fee_per_gas", &self.base_fee_per_gas)
            .field("withdrawals_root", &self.withdrawals_root)
            .field("blob_gas_used", &self.blob_gas_used)
            .field("excess_blob_gas", &self.excess_blob_gas)
            .field("parent_beacon_block_root", &self.parent_beacon_block_root)
            .field("requests_hash", &self.requests_hash)
            .field("size", &self.size)
            .field("transactions", &self.transactions)
            .field("withdrawals", &self.withdrawals)
            .field("uncles", &self.uncles)
            .finish()
    }
}

/// An Ethereum transaction object.
#[derive(Clone, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    /// The transaction type.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TransactionKind>,
    /// The transaction nonce.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<U256>,
    /// The transaction recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Address>,
    /// The account sending the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Address>,
    /// The limit in gas units for the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas: Option<U256>,
    /// The Ether value associated with the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<U256>,
    /// The calldata associated with the transaction.
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "serialization::option_bytes"
    )]
    pub input: Option<Vec<u8>>,
    /// The gas price willing to be paid by the sender in wei.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<U256>,
    /// Maximum fee per gas the sender is willing to pay to miners in wei.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_priority_fee_per_gas: Option<U256>,
    /// The maximum total fee per gas the sender is willing to pay (includes
    /// the network / base fee and miner / priority fee) in wei.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee_per_gas: Option<U256>,
    /// The maximum total fee per gas the sender is willing to pay for blob gas
    /// in wei.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee_per_blob_gas: Option<U256>,
    /// EIP-2930 access list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_list: Option<AccessList>,
    /// EIP-7702 authorization list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_list: Option<AuthorizationList>,
    /// List of versioned blob hashes associated with the transaction's EIP-4844
    /// data blobs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_versioned_hashes: Option<Vec<Digest>>,
    /// Raw blob data.
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "serialization::option_vec_bytes"
    )]
    pub blobs: Option<Vec<Vec<u8>>>,
    /// Chain ID that the transaction is valid on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<U256>,
}

impl Debug for Transaction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Transaction")
            .field("kind", &self.kind)
            .field("nonce", &self.nonce)
            .field("to", &self.to)
            .field("from", &self.from)
            .field("gas", &self.gas)
            .field("value", &self.value)
            .field("input", &self.input.as_deref().map(debug::Hex))
            .field("gas_price", &self.gas_price)
            .field("max_priority_fee_per_gas", &self.max_priority_fee_per_gas)
            .field("max_fee_per_gas", &self.max_fee_per_gas)
            .field("max_fee_per_blob_gas", &self.max_fee_per_blob_gas)
            .field("access_list", &self.access_list)
            .field("authorization_list", &self.authorization_list)
            .field("blob_versioned_hashes", &self.blob_versioned_hashes)
            .field("blobs", &self.blobs.as_deref().map(debug::HexSlice))
            .field("chain_id", &self.chain_id)
            .finish()
    }
}

/// Ethereum transaction kind.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
pub enum TransactionKind {
    /// Legacy transaction type.
    #[default]
    #[serde(rename = "0x0")]
    Legacy,
    /// An EIP-2930 transaction type.
    #[serde(rename = "0x1")]
    Eip2930,
    /// An EIP-1559 transaction type.
    #[serde(rename = "0x2")]
    Eip1559,
    /// An EIP-4844 transaction type.
    #[serde(rename = "0x3")]
    Eip4844,
    /// An EIP-7702 transaction type.
    #[serde(rename = "0x4")]
    Eip7702,
}

/// Access list.
pub type AccessList = Vec<AccessListEntry>;

/// Access list entry.
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessListEntry {
    /// The address.
    pub address: Address,
    /// The storage keys.
    pub storage_keys: Vec<U256>,
}

/// Created access list result.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessListResult {
    /// Access list.
    pub access_list: AccessList,
    /// Error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Gas used.
    pub gas_used: U256,
}

/// Fee history result.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeHistoryResult {
    /// Lowest number block of returned range.
    pub oldest_block: U256,
    /// An array of block base fees per gas. This includes the next block after
    /// the newest of the returned range, because this value can be derived from
    /// the newest block. Zeroes are returned for pre-EIP-1559 blocks.
    pub base_fee_per_gas: Vec<U256>,
    /// An array of block gas used ratios. These are calculated as the ratio of
    /// `gas-used` and `gas_limit`.
    pub gas_used_ratio: Vec<f64>,
    /// A two-dimensional array of effective priority fees per gas at the
    /// requested block percentiles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward: Option<Vec<Vec<U256>>>,
    /// An array of block base fees per blob gas. This includes the next block
    /// after the newest of the returned range, because this value can be
    /// derived from the newest block. Zeroes are returned for pre-EIP-4844
    /// blocks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_fee_per_blob_gas: Option<Vec<U256>>,
    /// An array of block blob gas used ratios. These are calculated as the
    /// ratio of `blob_gas_used` and the max blob gas per block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_gas_used_ratio: Option<Vec<f64>>,
}

/// Arguments for `eth_simulateV1`.
#[derive(Clone, Default, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulatePayload {
    /// Definition of blocks that can contain calls and overrides.
    pub block_state_calls: Vec<BlockStateCall>,
    /// Adds ETH transfers as ERC20 transfer events to the logs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_transfers: Option<bool>,
    /// Enables execution validation similar to full transaction execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation: Option<bool>,
    /// When true, full transaction objects are returned instead of hashes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_full_transactions: Option<bool>,
}

/// A block-state call group for `eth_simulateV1`.
#[derive(Clone, Default, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockStateCall {
    /// Block overrides for this simulated block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_overrides: Option<BlockOverrides>,
    /// State overrides for this simulated block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_overrides: Option<StateOverrides>,
    /// Transactions to execute at this simulated block/state.
    pub calls: Vec<Transaction>,
}

/// Context fields related to the block being executed.
#[derive(Clone, Default, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockOverrides {
    /// Block number.
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "serialization::option_num"
    )]
    pub number: Option<u64>,
    /// The previous value of randomness beacon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_randao: Option<U256>,
    /// Block timestamp.
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "serialization::option_num"
    )]
    pub time: Option<u64>,
    /// Gas limit.
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "serialization::option_num"
    )]
    pub gas_limit: Option<u64>,
    /// Fee recipient (also known as coinbase).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_recipient: Option<Address>,
    /// Withdrawals made by validators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdrawals: Option<Vec<Withdrawal>>,
    /// Base fee per unit of gas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_fee_per_gas: Option<U256>,
    /// Base fee per unit of blob gas.
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "serialization::option_num"
    )]
    pub blob_base_fee: Option<u64>,
}

/// Result of a simulated block.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockResult {
    /// Simulated block data.
    #[serde(flatten)]
    pub block: Block,
    /// Per-call results for this simulated block.
    pub calls: Vec<CallResult>,
}

/// A simulated call result.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(tag = "status")]
pub enum CallResult {
    /// A failed simulated call result.
    #[serde(rename = "0x0")]
    Failure(CallResultFailure),
    /// A successful simulated call result.
    #[serde(rename = "0x1")]
    Success(CallResultSuccess),
}

/// Result of a successful call in `eth_simulateV1`.
#[derive(Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CallResultSuccess {
    /// Return data.
    #[serde(with = "serialization::bytes")]
    pub return_data: Vec<u8>,
    /// Gas used.
    #[serde(with = "serialization::num")]
    pub gas_used: u64,
    /// Return logs.
    pub logs: Vec<Log>,
}

impl Debug for CallResultSuccess {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("CallResultSuccess")
            .field("return_data", &debug::Hex(&self.return_data))
            .field("gas_used", &self.gas_used)
            .field("logs", &self.logs)
            .finish()
    }
}

/// Result of a failed call in `eth_simulateV1`.
#[derive(Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CallResultFailure {
    /// Return data.
    #[serde(with = "serialization::bytes")]
    pub return_data: Vec<u8>,
    /// Gas used.
    #[serde(with = "serialization::num")]
    pub gas_used: u64,
    /// Failure details.
    pub error: CallResultError,
}

impl Debug for CallResultFailure {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("SimulatedCallFailure")
            .field("return_data", &debug::Hex(&self.return_data))
            .field("gas_used", &self.gas_used)
            .field("error", &self.error)
            .finish()
    }
}

/// Error details for a failed simulated call.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct CallResultError {
    /// Error code.
    pub code: i64,
    /// Error message.
    pub message: String,
}

/// Syncing progress.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncingProgress {
    /// Starting block.
    pub starting_block: U256,
    /// Current block.
    pub current_block: U256,
    /// Highest block.
    pub highest_block: U256,
}

/// Syncing status.
pub enum SyncingStatus {
    /// Syncing is in progress.
    Syncing(SyncingProgress),
    /// Not syncing.
    NotSyncing,
}

impl Serialize for SyncingStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Syncing(progress) => SyncingProgress::serialize(progress, serializer),
            Self::NotSyncing => serializer.serialize_bool(false),
        }
    }
}

impl<'de> Deserialize<'de> for SyncingStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Inner {
            Syncing(SyncingProgress),
            NotSyncing(bool),
        }

        match Inner::deserialize(deserializer)? {
            Inner::Syncing(progress) => Ok(Self::Syncing(progress)),
            Inner::NotSyncing(false) => Ok(Self::NotSyncing),
            Inner::NotSyncing(true) => Err(de::Error::custom("unexpected `true` value")),
        }
    }
}

/// A transaction authorization list.
pub type AuthorizationList = Vec<Authorization>;

/// A transaction authorization.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Authorization {
    /// Chain ID on which this authorization is valid.
    pub chain_id: U256,
    /// Authorization nonce.
    pub nonce: U256,
    /// Authorized address.
    pub address: Address,
    /// Y parity of the signature.
    pub y_parity: YParity,
    /// R
    pub r: U256,
    /// S
    pub s: U256,
}

/// State overrides.
pub type StateOverrides = HashMap<Address, AccountOverrides>;

/// Details of an account to be overridden.
#[derive(Clone, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountOverrides {
    /// Fake balance to set for the account before executing the call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<U256>,
    /// Fake nonce to set for the account before executing the call.
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "serialization::option_num"
    )]
    pub nonce: Option<u64>,
    /// Fake EVM bytecode to inject into the account before executing the call.
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "serialization::option_bytes"
    )]
    pub code: Option<Vec<u8>>,
    /// Fake key-value mapping to override **all** slots in the account storage
    /// before executing the call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<HashMap<U256, U256>>,
    /// Fake key-value mapping to override **individual** slots in the account
    /// storage before executing the call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_diff: Option<HashMap<U256, U256>>,
    /// Moves a precompile from its canonical address to this address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub move_precompile_to_address: Option<Address>,
}

impl Debug for AccountOverrides {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("AccountOverrides")
            .field("balance", &self.balance)
            .field("nonce", &self.nonce)
            .field("code", &self.code.as_deref().map(debug::Hex))
            .field("state", &self.state)
            .field("state_diff", &self.state_diff)
            .field(
                "move_precompile_to_address",
                &self.move_precompile_to_address,
            )
            .finish()
    }
}

/// Filter block selector.
#[derive(Clone, Copy, Debug)]
pub enum LogFilterBlocks {
    /// An inclusive block range to include logs for.
    Range { from: BlockSpec, to: BlockSpec },
    /// An exact block hash to query logs for. See
    /// [EIP-234](https://eips.ethereum.org/EIPS/eip-234).
    Hash(Digest),
}

impl Default for LogFilterBlocks {
    fn default() -> Self {
        Self::Range {
            from: BlockSpec::default(),
            to: BlockSpec::default(),
        }
    }
}

/// A value used for filtering logs.
#[derive(Clone, Debug, Default)]
pub enum LogFilterValue<T> {
    /// A filter that accepts all values.
    #[default]
    Any,
    /// A filter that only accepts a single value.
    Exact(T),
    /// A filter that accepts any one of the specified values.
    OneOf(Vec<T>),
}

impl<T> Serialize for LogFilterValue<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Any => serializer.serialize_unit(),
            Self::Exact(value) => value.serialize(serializer),
            Self::OneOf(values) => values.serialize(serializer),
        }
    }
}

impl<'de, T> Deserialize<'de> for LogFilterValue<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Value<T> {
            Exact(T),
            OneOf(Vec<T>),
        }

        match <Option<Value<T>>>::deserialize(deserializer)? {
            None => Ok(Self::Any),
            Some(Value::Exact(value)) => Ok(Self::Exact(value)),
            Some(Value::OneOf(values)) => Ok(Self::OneOf(values)),
        }
    }
}

/// A filter for querying logs from a node.
#[derive(Clone, Debug, Default)]
pub struct LogFilter {
    /// The blocks to fetch logs for.
    pub blocks: LogFilterBlocks,
    /// The contract addresses to fetch logs for.
    pub address: LogFilterValue<Address>,
    /// The log topics to filter for.
    pub topics: ArrayVec<LogFilterValue<Digest>, 4>,
}

/// Filter changes.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FilterChanges {
    /// New block or transaction hashes.
    Hashes(Vec<Digest>),
    /// New logs.
    Logs(Vec<Log>),
}

/// Storage proof.
#[derive(Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageProof {
    /// Key.
    pub key: U256,
    /// Value.
    pub value: U256,
    /// Proof.
    #[serde(with = "serialization::vec_bytes")]
    pub proof: Vec<Vec<u8>>,
}

impl Debug for StorageProof {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("StorageProof")
            .field("key", &self.key)
            .field("value", &self.value)
            .field("proof", &debug::HexSlice(&self.proof))
            .finish()
    }
}

/// Account proof.
#[derive(Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountProof {
    /// Address.
    pub address: Address,
    /// Account proof.
    #[serde(with = "serialization::vec_bytes")]
    pub account_proof: Vec<Vec<u8>>,
    /// Balance.
    pub balance: U256,
    /// Code hash.
    pub code_hash: Digest,
    /// Nonce.
    #[serde(with = "serialization::num")]
    pub nonce: u64,
    /// Storage hash.
    pub storage_hash: Digest,
    /// Storage proofs.
    pub storage_proof: Vec<StorageProof>,
}

impl Debug for AccountProof {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("AccountProof")
            .field("address", &self.address)
            .field("account_proof", &debug::HexSlice(&self.account_proof))
            .field("balance", &self.balance)
            .field("code_hash", &self.code_hash)
            .field("nonce", &self.nonce)
            .field("storage_hash", &self.storage_hash)
            .field("storage_proof", &self.storage_proof)
            .finish()
    }
}

impl Serialize for LogFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        #[serde(untagged)]
        enum Value<'a> {
            #[serde(rename_all = "camelCase")]
            Range {
                from_block: BlockSpec,
                to_block: BlockSpec,
                address: &'a LogFilterValue<Address>,
                topics: &'a [LogFilterValue<Digest>],
            },
            #[serde(rename_all = "camelCase")]
            Hash {
                block_hash: Digest,
                address: &'a LogFilterValue<Address>,
                topics: &'a [LogFilterValue<Digest>],
            },
        }

        let value = match self.blocks {
            LogFilterBlocks::Range { from, to } => Value::Range {
                from_block: from,
                to_block: to,
                address: &self.address,
                topics: &self.topics,
            },
            LogFilterBlocks::Hash(hash) => Value::Hash {
                block_hash: hash,
                address: &self.address,
                topics: &self.topics,
            },
        };

        value.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for LogFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Value {
            #[serde(rename_all = "camelCase")]
            Range {
                from_block: BlockSpec,
                to_block: BlockSpec,
                address: LogFilterValue<Address>,
                topics: ArrayVec<LogFilterValue<Digest>, 4>,
            },
            #[serde(rename_all = "camelCase")]
            Hash {
                block_hash: Digest,
                address: LogFilterValue<Address>,
                topics: ArrayVec<LogFilterValue<Digest>, 4>,
            },
        }

        match Value::deserialize(deserializer)? {
            Value::Range {
                from_block,
                to_block,
                address,
                topics,
            } => Ok(Self {
                blocks: LogFilterBlocks::Range {
                    from: from_block,
                    to: to_block,
                },
                address,
                topics,
            }),
            Value::Hash {
                block_hash,
                address,
                topics,
            } => Ok(Self {
                blocks: LogFilterBlocks::Hash(block_hash),
                address,
                topics,
            }),
        }
    }
}

/// An Ethereum log.
#[derive(Clone, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    /// Whether or not the log was removed because of a re-org or not.
    pub removed: bool,
    /// The index of the log within the block.
    pub log_index: U256,
    /// The index of the transaction that emitted this log within the block.
    pub transaction_index: U256,
    /// The hash of the transaction that emitted this log.
    pub transaction_hash: Digest,
    /// The hash of the block containing the log.
    pub block_hash: Digest,
    /// The height of the block containing the log.
    pub block_number: U256,
    /// The timestamp of the block containing the log.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_timestamp: Option<U256>,
    /// The address of the contract that emitted the log.
    pub address: Address,
    /// The data emitted with the log.
    #[serde(with = "serialization::bytes")]
    pub data: Vec<u8>,
    /// The topics emitted with the log.
    pub topics: ArrayVec<Digest, 4>,
}

impl Debug for Log {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Log")
            .field("removed", &self.removed)
            .field("log_index", &self.log_index)
            .field("transaction_index", &self.transaction_index)
            .field("transaction_hash", &self.transaction_hash)
            .field("block_hash", &self.block_hash)
            .field("block_number", &self.block_number)
            .field("block_timestamp", &self.block_timestamp)
            .field("address", &self.address)
            .field("data", &debug::Hex(&self.data))
            .field("topics", &self.topics)
            .finish()
    }
}

/// An Ethereum transaction receipt.
#[derive(Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionReceipt {
    /// The type of the transaction. These have evolved over several EIPs;
    /// See [`TransactionReceiptKind`] for summaries and references.
    #[serde(flatten)]
    pub kind: TransactionReceiptKind,
    /// The hash of the transaction.
    pub transaction_hash: Digest,
    /// The index of the transaction within the block it was included.
    pub transaction_index: U256,
    /// The hash of the block containing the transaction.
    pub block_hash: Digest,
    /// The height of the block containing the transaction.
    pub block_number: U256,
    /// Address of transaction sender.
    pub from: Address,
    /// Transaction receipient ([`None`] for contract creation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Address>,
    /// The price paid post-execution by the transaction (i.e. base fee + priority fee).
    /// Both fields in 1559-style transactions are *maximums* (max fee + max priority fee), the
    /// amount that's actually paid by users can only be determined post-execution.
    pub effective_gas_price: U256,
    /// The sum of gas used by this transaction and all preceding transactions in the same block.
    pub cumulative_gas_used: U256,
    /// The amount of gas used for this specific transaction alone.
    pub gas_used: U256,
    /// Contract address created, or [`None`] if not a deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_address: Option<Address>,
    /// Logs emitted by the transaction.
    pub logs: Vec<Log>,
    /// The log bloom filter.
    pub logs_bloom: Bloom,
    /// The post-transaction state root. Only specified for transactions
    /// included before the Byzantium upgrade.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<Digest>,
    /// The transaction status, indicating whether it succeeded or reverted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TransactionReceiptStatus>,
}

/// The status of a `TransactionReceipt` (whether is succeeded or failed).
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum TransactionReceiptStatus {
    /// Status of a failed transaction.
    #[serde(rename = "0x0")]
    Failure,
    /// Status of a successful transaction.
    #[serde(rename = "0x1")]
    Success,
}

/// The type of a `TransactionReceipt`.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum TransactionReceiptKind {
    /// Legacy transaction type.
    #[serde(rename = "0x0")]
    Legacy,
    /// EIP-2930 transaction type.
    #[serde(rename = "0x1")]
    Eip2930,
    /// EIP-1559 transaction type.
    #[serde(rename = "0x2")]
    Eip1559,
    /// EIP-4844 transaction type.
    #[serde(rename = "0x3")]
    Eip4844 {
        /// The amount of blob gas used for this specific transaction.
        #[serde(rename = "blobGasUsed")]
        blob_gas_used: U256,
        /// The actual value per gas deducted from the sender's account for blob gas.
        #[serde(rename = "blobGasPrice")]
        blob_gas_price: U256,
    },
    /// EIP-7702 transaction type.
    #[serde(rename = "0x4")]
    Eip7702,
}

impl Debug for TransactionReceipt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("TransactionReceipt")
            .field("kind", &self.kind)
            .field("transaction_hash", &self.transaction_hash)
            .field("transaction_index", &self.transaction_index)
            .field("block_hash", &self.block_hash)
            .field("block_number", &self.block_number)
            .field("from", &self.from)
            .field("to", &self.to)
            .field("effective_gas_price", &self.effective_gas_price)
            .field("cumulative_gas_used", &self.cumulative_gas_used)
            .field("gas_used", &self.gas_used)
            .field("contract_address", &self.contract_address)
            .field("logs", &self.logs)
            .field("logs_bloom", &self.logs_bloom)
            .field("root", &self.root)
            .field("status", &self.status)
            .finish()
    }
}
