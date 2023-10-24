//! A simple Ethereum RPC implementation.

#[cfg(feature = "curl")]
pub mod curl;
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
    /// The `web3` namespace.
    pub mod web3 {
        /// Gets the current client version.
        pub struct ClientVersion as "web3_clientVersion"
            Empty => String;
    }
}

module! {
    /// The `eth` namespace.
    ///
    /// Documentation for the APIs can be found here:
    /// <https://ethereum.github.io/execution-apis/api-documentation/>
    pub mod eth {
        /// Gets the current client version.
        pub struct BlockNumber as "eth_blockNumber"
            Empty => U256;

        /// Simulates a transaction without adding it to the blockchain.
        pub struct Call as "eth_call"
            (TransactionCall, BlockId) => Vec<u8> [serialization::bytes];

        /// Returns information about a block by hash.
        pub struct GetBlockByHash as "eth_getBlockByHash"
            (Digest, Hydrated) => Option<Block>;

        /// Returns information about a block by number.
        pub struct GetBlockByNumber as "eth_getBlockByNumber"
            (BlockSpec, Hydrated) => Option<Block>;

        /// Returns the number of transactions in a block from a block matching the given block hash.
        pub struct GetBlockTransactionCountByHash as "eth_getBlockTransactionCountByHash"
            (Digest,) => Option<U256>;

        /// Returns the number of transactions in a block matching the given block number.
        pub struct GetBlockTransactionCountByNumber as "eth_getBlockTransactionCountByNumber"
            (BlockSpec,) => Option<U256>;

        /// Returns code at a given address.
        pub struct GetCode as "eth_getCode"
            (Address, BlockId) => Vec<u8> [serialization::bytes];

        /// Returns a collection of all logs matching the given filter.
        pub struct GetLogs as "eth_getLogs"
            (LogFilter,) => Vec<Log>;

        /// Returns information about a transaction by block hash and transaction index position.
        pub struct GetTransactionByBlockHashAndIndex as "eth_getTransactionByBlockHashAndIndex"
            (Digest, U256) => Option<SignedTransaction>;

        /// Returns information about a transaction by block number and transaction index position.
        pub struct GetTransactionByBlockNumberAndIndex as "eth_getTransactionByBlockNumberAndIndex"
            (BlockId, U256) => Option<SignedTransaction>;

        /// Returns information about a transaction requested by transaction hash.
        pub struct GetTransactionByHash as "eth_getTransactionByHash"
            (Digest,) => Option<SignedTransaction>;
    }
}

/// Module containing common extensions to the standard Ethereum RPC methods.
pub mod ext {
    use crate::{serialization, types::*};

    module! {
        /// Extensions to the `eth` namespace.
        pub mod eth {
            /// Simulates a transaction without adding it to the blockchain with
            /// support for state overrides.
            pub struct Call as "eth_call"
                (TransactionCall, BlockId, StateOverrides) => Vec<u8> [serialization::bytes];
        }
    }
}
