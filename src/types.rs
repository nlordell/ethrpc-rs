//! Ethereum RPC types.

use crate::{debug, serialization};
use ethprim::AsU256 as _;
use serde::{
    de::{self, Deserializer},
    ser::Serializer,
    Deserialize, Serialize,
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

impl<'de> Deserialize<'de> for Empty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        <[(); 0]>::deserialize(deserializer)?;
        Ok(Empty)
    }
}

impl Serialize for Empty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        [(); 0].serialize(serializer)
    }
}

/// An Ethereum block specifier.
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

/// An Ethereum block id.
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

/// An Ethereum block tag.
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

/// Transaction information to include with a block.
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
    /// Signed ERC-2930 transaction.
    #[serde(rename = "0x1")]
    Erc2930(SignedErc2930Transaction),
    /// Signed ERC-1559 transaction.
    #[serde(rename = "0x2")]
    Erc1559(SignedErc1559Transaction),
    /// Signed EIP-4844 transaction.
    #[serde(rename = "0x3")]
    Eip4844(SignedEip4844Transaction),
}

/// The signature parity.
#[derive(Clone, Copy, Debug, Eq, Ord, Hash, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum YParity {
    /// Even parity (0).
    #[serde(rename = "0x0")]
    Even = 0,
    /// Odd parity (1).
    #[serde(rename = "0x1")]
    Odd = 1,
}

/// Signed legacy transaction.
#[derive(Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedLegacyTransaction {
    /// The transaction nonce.
    pub nonce: U256,
    /// The transaction recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Address>,
    /// The limit in gas units for the transaction.
    pub gas: U256,
    /// The Ether value associated with the transaction.
    pub value: U256,
    /// The calldata associated with the transaction.
    #[serde(with = "serialization::bytes")]
    pub input: Vec<u8>,
    /// Gas price willing to be paid by the sender.
    pub gas_price: U256,
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
            .field("nonce", &self.nonce)
            .field("to", &self.to)
            .field("gas", &self.gas)
            .field("value", &self.value)
            .field("input", &debug::Hex(&self.input))
            .field("gas_price", &self.gas_price)
            .field("chain_id", &self.chain_id)
            .field("v", &self.v)
            .field("r", &self.r)
            .field("s", &self.s)
            .finish()
    }
}

/// Signed ERC-2930 transaction.
#[derive(Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedErc2930Transaction {
    /// The transaction nonce.
    pub nonce: U256,
    /// The transaction recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Address>,
    /// The limit in gas units for the transaction.
    pub gas: U256,
    /// The Ether value associated with the transaction.
    pub value: U256,
    /// The calldata associated with the transaction.
    #[serde(with = "serialization::bytes")]
    pub input: Vec<u8>,
    /// Gas price willing to be paid by the sender.
    pub gas_price: U256,
    /// State access list.
    pub access_list: AccessList,
    /// Chain ID that the transaction is valid on.
    pub chain_id: U256,
    /// Y parity of the signature.
    #[serde(alias = "v")]
    pub y_parity: YParity,
    /// R
    pub r: U256,
    /// S
    pub s: U256,
}

impl Debug for SignedErc2930Transaction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("SignedErc2930Transaction")
            .field("nonce", &self.nonce)
            .field("to", &self.to)
            .field("gas", &self.gas)
            .field("value", &self.value)
            .field("input", &debug::Hex(&self.input))
            .field("gas_price", &self.gas_price)
            .field("access_list", &self.access_list)
            .field("chain_id", &self.chain_id)
            .field("y_parity", &self.y_parity)
            .field("r", &self.r)
            .field("s", &self.s)
            .finish()
    }
}

/// Signed ERC-1559 transaction.
#[derive(Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedErc1559Transaction {
    /// The transaction nonce.
    pub nonce: U256,
    /// The transaction recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Address>,
    /// The limit in gas units for the transaction.
    pub gas: U256,
    /// The Ether value associated with the transaction.
    pub value: U256,
    /// The calldata associated with the transaction.
    #[serde(with = "serialization::bytes")]
    pub input: Vec<u8>,
    /// Maximum fee per gas the sender is willing to pay to miners in wei
    pub max_priority_fee_per_gas: U256,
    /// The maximum total fee per gas the sender is willing to pay, including
    /// the network (A.K.A. base) fee and miner (A.K.A priority) fee.
    pub max_fee_per_gas: U256,
    /// State access list.
    pub access_list: AccessList,
    /// Chain ID that the transaction is valid on.
    pub chain_id: U256,
    /// Y parity of the signature.
    #[serde(alias = "v")]
    pub y_parity: YParity,
    /// R
    pub r: U256,
    /// S
    pub s: U256,
}

impl Debug for SignedErc1559Transaction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("SignedErc1559Transaction")
            .field("nonce", &self.nonce)
            .field("to", &self.to)
            .field("gas", &self.gas)
            .field("value", &self.value)
            .field("input", &debug::Hex(&self.input))
            .field("max_priority_fee_per_gas", &self.max_priority_fee_per_gas)
            .field("max_fee_per_gas", &self.max_fee_per_gas)
            .field("access_list", &self.access_list)
            .field("chain_id", &self.chain_id)
            .field("y_parity", &self.y_parity)
            .field("r", &self.r)
            .field("s", &self.s)
            .finish()
    }
}

/// Signed EIP-4844 transaction.
#[derive(Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedEip4844Transaction {
    /// The transaction nonce.
    pub nonce: U256,
    /// The transaction recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Address>,
    /// The limit in gas units for the transaction.
    pub gas: U256,
    /// The Ether value associated with the transaction.
    pub value: U256,
    /// The calldata associated with the transaction.
    #[serde(with = "serialization::bytes")]
    pub input: Vec<u8>,
    /// Maximum fee per gas the sender is willing to pay to miners in wei
    pub max_priority_fee_per_gas: U256,
    /// The maximum total fee per gas the sender is willing to pay, including
    /// the network (A.K.A. base) fee and miner (A.K.A priority) fee.
    pub max_fee_per_gas: U256,
    /// The maximum total fee per gas the sender is willing to pay for blob gas in wei.
    pub max_fee_per_blob_gas: U256,
    /// State access list.
    pub access_list: AccessList,
    /// List of versioned blob hashes associated with the transaction's EIP-4844 data blobs.
    pub blob_versioned_hashes: Vec<Digest>,
    /// Chain ID that the transaction is valid on.
    pub chain_id: U256,
    /// Y parity of the signature.
    #[serde(alias = "v")]
    pub y_parity: YParity,
    /// R
    pub r: U256,
    /// S
    pub s: U256,
}

impl Debug for SignedEip4844Transaction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("SignedEip4844Transaction")
            .field("nonce", &self.nonce)
            .field("to", &self.to)
            .field("gas", &self.gas)
            .field("value", &self.value)
            .field("input", &debug::Hex(&self.input))
            .field("max_priority_fee_per_gas", &self.max_priority_fee_per_gas)
            .field("max_fee_per_gas", &self.max_fee_per_gas)
            .field("max_fee_per_blob_gas", &self.max_fee_per_blob_gas)
            .field("access_list", &self.access_list)
            .field("blob_versioned_hashes", &self.blob_versioned_hashes)
            .field("chain_id", &self.chain_id)
            .field("y_parity", &self.y_parity)
            .field("r", &self.r)
            .field("s", &self.s)
            .finish()
    }
}

/// A validator withdrawal.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Withdrawal {
    #[serde(with = "serialization::num")]
    pub index: u64,
    #[serde(with = "serialization::num")]
    pub validator_index: u64,
    #[serde(with = "serialization::num")]
    pub amount: u128,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub difficulty: Option<U256>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_difficulty: Option<U256>,
    /// The base fee per gas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_fee_per_gas: Option<U256>,
    /// The withdrawals root.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdrawals_root: Option<Digest>,
    /// The size of the block.
    pub size: U256,
    /// Block transactions.
    pub transactions: BlockTransactions,
    /// Withdrawals.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdrawals: Option<Vec<Withdrawal>>,
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
            .field("size", &self.size)
            .field("transactions", &self.transactions)
            .field("withdrawals", &self.withdrawals)
            .field("uncles", &self.uncles)
            .finish()
    }
}

/// An Ethereum transaction call object.
#[derive(Clone, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionCall {
    /// The account sending the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Address>,
    /// The transaction type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TransactionKind>,
    /// The transaction nonce.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<U256>,
    /// The transaction recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Address>,
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
    /// Gas price willing to be paid by the sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<U256>,
    /// Maximum fee per gas the sender is willing to pay to miners in wei
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_priority_fee_per_gas: Option<U256>,
    /// The maximum total fee per gas the sender is willing to pay, including
    /// the network (A.K.A. base) fee and miner (A.K.A priority) fee.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee_per_gas: Option<U256>,
    /// State access list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_list: Option<AccessList>,
    /// Chain ID that the transaction is valid on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<U256>,
}

impl Debug for TransactionCall {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("TransactionCall")
            .field("from", &self.from)
            .field("kind", &self.kind)
            .field("nonce", &self.nonce)
            .field("to", &self.to)
            .field("gas", &self.gas)
            .field("value", &self.value)
            .field("input", &self.input.as_deref().map(debug::Hex))
            .field("gas_price", &self.gas_price)
            .field("max_priority_fee_per_gas", &self.max_priority_fee_per_gas)
            .field("max_fee_per_gas", &self.max_fee_per_gas)
            .field("access_list", &self.access_list)
            .field("chain_id", &self.chain_id)
            .finish()
    }
}

/// Ethereum transaction kind.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
pub enum TransactionKind {
    /// Legacy transaction type.
    #[default]
    Legacy = 0,
    /// An EIP-2930 transaction type.
    Erc2930 = 1,
    /// An EIP-1559 transaction type.
    Erc1559 = 2,
}

impl Serialize for TransactionKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).as_u256().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TransactionKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = U256::deserialize(deserializer)?;
        match u8::try_from(value) {
            Ok(0) => Ok(Self::Legacy),
            Ok(1) => Ok(Self::Erc2930),
            Ok(2) => Ok(Self::Erc1559),
            _ => Err(de::Error::custom(format!(
                "invalid transaction type {value}"
            ))),
        }
    }
}

/// An access list.
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

/// State overrides.
pub type StateOverrides = HashMap<Address, StateOverride>;

/// State override object.
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StateOverride {
    /// Fake balance to set for the account before executing the call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<U256>,
    /// Fake nonce to set for the account before executing the call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<U256>,
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
}

/// The blocks to fetch logs for.
#[derive(Clone, Copy, Debug)]
pub enum LogBlocks {
    /// An inclusive block range to include logs for.
    Range { from: BlockSpec, to: BlockSpec },
    /// An exact block hash to query logs for. See
    /// [EIP-234](https://eips.ethereum.org/EIPS/eip-234).
    Hash(Digest),
}

impl Default for LogBlocks {
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
    pub blocks: LogBlocks,
    /// The contract addresses to fetch logs for.
    pub address: LogFilterValue<Address>,
    /// The log topics to filter for.
    pub topics: ArrayVec<LogFilterValue<Digest>, 4>,
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
            LogBlocks::Range { from, to } => Value::Range {
                from_block: from,
                to_block: to,
                address: &self.address,
                topics: &self.topics,
            },
            LogBlocks::Hash(hash) => Value::Hash {
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
                blocks: LogBlocks::Range {
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
                blocks: LogBlocks::Hash(block_hash),
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
            .field("address", &self.address)
            .field("data", &debug::Hex(&self.data))
            .field("topics", &self.topics)
            .finish()
    }
}

/// An Ethereum Transaction Receipt
#[derive(Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionReceipt {
    #[serde(rename = "type", flatten)]
    pub kind: TransactionReceiptKind,

    /// The hash of the transaction that emitted this log.
    pub transaction_hash: Digest,
    /// The index of the transaction that emitted this log within the block.
    pub transaction_index: U256,
    /// The hash of the block containing the transaction.
    pub block_hash: Digest,
    /// The height of the block containing the transaction.
    pub block_number: U256,
    /// Address of transaction sender.
    pub from: Address,
    /// Transaction receipient (Null for contract creation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Address>,
    /// The sum of gas used by this transaction and all preceding transactions in the same block.
    pub cumulative_gas_used: U256,
    /// The amount of gas used for this specific transaction alone.
    pub gas_used: U256,
    /// Contract address created, or `None` if not a deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_address: Option<Address>,
    /// Logs emitted by the transaction.
    pub logs: Vec<Log>,
    /// The log bloom filter.
    pub logs_bloom: Bloom,
    /// State root. Only present before activation of [EIP-658](https://eips.ethereum.org/EIPS/eip-658).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<Digest>,
    /// Status: either 1 (success) or 0 (failure). Only present after activation of [EIP-658](https://eips.ethereum.org/EIPS/eip-658).
    pub status: Option<U256>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TransactionReceiptKind {
    #[serde(rename = "0x0")]
    Legacy,
    #[serde(rename = "0x1")]
    Eip2930,
    #[serde(rename = "0x2")]
    Eip1559 {
        /// The price paid post-execution by the transaction (i.e. base fee + priority fee).
        /// Both fields in 1559-style transactions are *maximums* (max fee + max priority fee), the
        /// amount that's actually paid by users can only be determined post-execution.
        effective_gas_price: Option<U256>,
    },
    #[serde(rename = "0x3")]
    Eip4844 {
        /// The amount of blob gas used for this specific transaction.
        /// Only specified for blob transactions as defined by EIP-4844.
        blob_gas_used: U256,
        /// The actual value per gas deducted from the sender's account for blob gas.
        /// Only specified for blob transactions as defined by [EIP-4844](https://eips.ethereum.org/EIPS/eip-4844).
        blob_gas_price: U256,
    },
}

impl Debug for TransactionReceipt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Log")
            .field("transaction_type", &self.kind)
            .field("transaction_hash", &self.transaction_hash)
            .field("transaction_index", &self.transaction_index)
            .field("block_hash", &self.block_hash)
            .field("block_number", &self.block_number)
            .field("from", &self.from)
            .field("to", &self.to)
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
