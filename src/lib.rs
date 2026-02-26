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
        /// Returns a list of addresses owned by client.
        pub struct Accounts as "eth_accounts"
            Empty => Vec<Address>;

        /// Returns the base fee per blob gas in wei.
        pub struct BlobBaseFee as "eth_blobBaseFee"
            Empty => U256;

        /// Returns the number of most recent block.
        pub struct BlockNumber as "eth_blockNumber"
            Empty => U256;

        /// Executes a new message call immediately without creating a
        /// transaction on the block chain.
        pub struct Call as "eth_call"
            (Transaction, BlockId) => Vec<u8> [serialization::bytes];

        /// Returns the chain ID of the current network.
        pub struct ChainId as "eth_chainId"
            Empty => U256;

        /// Returns the client coinbase address.
        pub struct Coinbase as "eth_coinbase"
            Empty => Address;

        /// Generates an access list for a transaction.
        pub struct CreateAccessList as "eth_createAccessList"
            (Transaction, Option<BlockSpec>) => AccessListResult;

        /// Generates and returns an estimate of how much gas is necessary to
        /// allow the transaction to complete.
        pub struct EstimateGas as "eth_estimateGas"
            (Transaction, BlockSpec) => U256;

        /// Returns transaction base fee per gas and effective priority fee per
        /// gas for the requested/supported block range.
        pub struct FeeHistory as "eth_feeHistory"
            (U256, BlockSpec, Vec<f64>) => FeeHistoryResult;

        /// Returns the current price per gas in wei.
        pub struct GasPrice as "eth_gasPrice"
            Empty => U256;

        /// Returns the balance of the account of given address.
        pub struct GetBalance as "eth_getBalance"
            (Address, BlockId) => U256;

        /// Returns information about a block by hash.
        pub struct GetBlockByHash as "eth_getBlockByHash"
            (Digest, Hydrated) => Option<Block>;

        /// Returns information about a block by number.
        pub struct GetBlockByNumber as "eth_getBlockByNumber"
            (BlockSpec, Hydrated) => Option<Block>;

        /// Returns the receipts of a block by number or hash.
        pub struct GetBlockReceipts as "eth_getBlockReceipts"
            (BlockId,) => Option<Vec<TransactionReceipt>>;

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

        /// Polling method for the filter with the given ID (created using
        /// `eth_newFilter`). Returns an array of logs, block hashes, or
        /// transaction hashes since last poll, depending on the installed
        /// filter.
        pub struct GetFilterChanges as "eth_getFilterChanges"
            (FilterId,) => FilterChanges;

        /// Returns an array of all logs matching the filter with the given ID
        /// (created using `eth_newFilter`).
        pub struct GetFilterLogs as "eth_getFilterLogs"
            (FilterId,) => Vec<Log>;

        /// Returns an array of all logs matching the specified filter.
        pub struct GetLogs as "eth_getLogs"
            (LogFilter,) => Vec<Log>;

        /// Returns the merkle proof for a given account and optionally some
        /// storage keys.
        pub struct GetProof as "eth_getProof"
            (Address, Vec<Digest>, BlockId) => AccountProof;

        /// Returns the value from a storage position at a given address.
        pub struct GetStorageAt as "eth_getStorageAt"
            (Address, U256, BlockId) => [u8; 32] [serialization::bytearray];

        /// Returns information about a transaction by block hash and
        /// transaction index position.
        pub struct GetTransactionByBlockHashAndIndex as "eth_getTransactionByBlockHashAndIndex"
            (Digest, U256) => Option<SignedTransaction>;

        /// Returns information about a transaction by block number and
        /// transaction index position.
        pub struct GetTransactionByBlockNumberAndIndex as "eth_getTransactionByBlockNumberAndIndex"
            (BlockSpec, U256) => Option<SignedTransaction>;

        /// Returns the information about a transaction requested by transaction
        /// hash.
        pub struct GetTransactionByHash as "eth_getTransactionByHash"
            (Digest,) => Option<SignedTransaction>;

        /// Returns the nonce of an account in the state.
        ///
        /// NOTE: The name eth_getTransactionCount reflects the historical fact
        /// that an account's nonce and sent transaction count were the same.
        /// After the Pectra fork, with the inclusion of EIP-7702, this is no
        /// longer true.
        pub struct GetTransactionCount as "eth_getTransactionCount"
            (Address, BlockId) => U256;

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

        /// Creates a filter in the node, allowing for later polling.
        /// Registers client interest in new blocks, and returns an identifier.
        pub struct NewBlockFilter as "eth_newBlockFilter"
            Empty => FilterId;

        /// Install a log filter in the server, allowing for later polling.
        /// Registers client interest in logs matching the filter, and returns
        /// an identifier.
        pub struct NewFilter as "eth_newFilter"
            (LogFilter,) => FilterId;

        /// Creates a filter in the node, allowing for later polling.
        /// Registers client interest in new transactions, and returns an
        /// identifier.
        pub struct NewPendingTransactionFilter as "eth_newPendingTransactionFilter"
            Empty => FilterId;

        /// Submits a raw transaction.
        pub struct SendRawTransaction as "eth_sendRawTransaction"
            (Vec<u8>,) [serialization::param_eth_send_raw_transaction] => Digest;

        /// Signs and submits a transaction.
        pub struct SendTransaction as "eth_sendTransaction"
            (Transaction,) => Digest;

        /// Returns an EIP-191 signature over the provided data.
        pub struct Sign as "eth_sign"
            (Address, Vec<u8>) [serialization::param_eth_sign] => Vec<u8> [serialization::bytes];

        /// Returns an RLP encoded transaction signed by the specified account.
        pub struct SignTransaction as "eth_signTransaction"
            (Transaction,) => Vec<u8> [serialization::bytes];

        /// Executes a sequence of message calls building on each other's state
        /// without creating transactions on the block chain, optionally
        /// overriding block and state data.
        pub struct SimulateV1 as "eth_simulateV1"
            (SimulatePayload, Option<BlockSpec>) => Vec<BlockResult>;

        /// Returns an object with data about the sync status or false.
        pub struct Syncing as "eth_syncing"
            Empty => SyncingStatus;

        /// Uninstalls a filter with given id.
        pub struct UninstallFilter as "eth_uninstallFilter"
            (FilterId,) => bool;
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
