//! Module containing common extensions to the standard Ethereum RPC methods.

use crate::{module, serialization, types::*};

module! {
    /// Extensions to the `eth` namespace.
    pub mod eth {
        /// Simulates a transaction without adding it to the blockchain with
        /// support for state overrides.
        pub struct Call as "eth_call"
            (Transaction, BlockId, StateOverrides) => Vec<u8> [serialization::bytes];
    }
}
