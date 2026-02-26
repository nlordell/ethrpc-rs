//! A simple Ethereum RPC implementation.
//!
//! Documentation for the APIs can be found here:
//! <https://ethereum.github.io/execution-apis/>

#[cfg(feature = "curl")]
pub mod curl;
pub mod ext;
#[cfg(feature = "http")]
pub mod http;
pub mod jsonrpc;
#[macro_use]
pub mod method;
mod bloom;
mod debug;
mod serialization;
pub mod types;

use self::types::*;

module! {
    /// The `eth` namespace.
    pub mod eth {
        /// Returns the number of most recent block.
        pub struct BlockNumber as "eth_blockNumber"
            Empty => U256;

        /// Simulates a transaction without adding it to the blockchain.
        pub struct Call as "eth_call"
            (TransactionCall, BlockId) => Vec<u8> [serialization::bytes];

        /// Returns the chain ID of the current network
        pub struct ChainId as "eth_chainId"
            Empty => U256;

        /// Generates and returns an estimate of how much gas is necessary to
        /// allow the transaction to complete.
        pub struct EstimateGas as "eth_estimateGas"
            (TransactionCall, BlockSpec) => U256;

        /// Returns the current price per gas in wei.
        pub struct GasPrice as "eth_gasPrice"
            Empty => U256;

        /// Returns the balance of the account of given address.
        pub struct GetBalance as "eth_getBalance"
            (Address, Option<BlockId>) => U256;

        /// Returns information about a block by hash.
        pub struct GetBlockByHash as "eth_getBlockByHash"
            (Digest, Hydrated) => Option<Block>;

        /// Returns information about a block by number.
        pub struct GetBlockByNumber as "eth_getBlockByNumber"
            (BlockSpec, Hydrated) => Option<Block>;

        /// Returns the receipts of a block by number or hash.
        pub struct GetBlockReceipts as "eth_getBlockReceipts"
            (BlockSpec,) => Option<Vec<TransactionReceipt>>;

        /// Returns the number of transactions in a block from a block matching
        /// the given block hash.
        pub struct GetBlockTransactionCountByHash as "eth_getBlockTransactionCountByHash"
            (Digest,) => Option<U256>;

        /// Returns the number of transactions in a block matching the given
        /// block number.
        pub struct GetBlockTransactionCountByNumber as "eth_getBlockTransactionCountByNumber"
            (BlockSpec,) => Option<U256>;

        /// Returns code at a given address.
        pub struct GetCode as "eth_getCode"
            (Address, BlockId) => Vec<u8> [serialization::bytes];

        /// Returns a collection of all logs matching the given filter.
        pub struct GetLogs as "eth_getLogs"
            (LogFilter,) => Vec<Log>;

        /// Returns the value from a storage position at a given address.
        pub struct GetStorageAt as "eth_getStorageAt"
            (Address, U256, Option<BlockId>) => [u8; 32] [serialization::bytearray];

        /// Returns information about a transaction by block hash and
        /// transaction index position.
        pub struct GetTransactionByBlockHashAndIndex as "eth_getTransactionByBlockHashAndIndex"
            (Digest, U256) => Option<SignedTransaction>;

        /// Returns information about a transaction by block number and
        /// transaction index position.
        pub struct GetTransactionByBlockNumberAndIndex as "eth_getTransactionByBlockNumberAndIndex"
            (BlockSpec, U256) => Option<SignedTransaction>;

        /// Returns information about a transaction requested by transaction
        /// hash.
        pub struct GetTransactionByHash as "eth_getTransactionByHash"
            (Digest,) => Option<SignedTransaction>;

        /// Returns the value from a storage position at a given address.
        pub struct GetTransactionCount as "eth_getTransactionCount"
            (Address, Option<BlockId>) => U256;

        /// Returns the receipt of a transaction by transaction hash.
        pub struct GetTransactionReceipt as "eth_getTransactionReceipt"
            (Digest,) => Option<TransactionReceipt>;

        /// Returns the number of uncles in a block from a block matching the
        /// given block hash.
        pub struct GetUncleCountByBlockHash as "eth_getUncleCountByBlockHash"
            (Digest,) => Option<U256>;

        /// Returns the number of uncles in a block from a block matching the
        /// given block number.
        pub struct GetUncleCountByBlockNumber as "eth_getUncleCountByBlockNumber"
            (BlockSpec,) => Option<U256>;

        /// Returns the current maxPriorityFeePerGas per gas in wei.
        pub struct MaxPriorityFeePerGas as "eth_maxPriorityFeePerGas"
            Empty => U256;

        /// Creates a filter in the node, to notify when a new block arrives.
        pub struct NewBlockFilter as "eth_newBlockFilter"
            Empty => U256;

        /// Creates a filter in the node, to notify when new pending
        /// transactions arrive.
        pub struct NewPendingTransactionFilter as "eth_newPendingTransactionFilter"
            Empty => U256;
    }
}

module! {
    /// The `net` namespace.
    pub mod net {
        /// Returns the current network ID. This is usually equivalent to the
        /// chainID, but may differ from it for some legacy networks or special
        /// testnets.
        pub struct Version as "net_version"
            Empty => U256 [ethprim::num::serde::decimal];
    }
}
