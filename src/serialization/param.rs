//! Parameter serialization.
//!
//! Parameter tuples that require custom serialization need `serde` methods for
//! going to and from JSON. This is a bit of a smell related to the fact that
//! we cannot tag specific tuple fields with serialization handlers, and need
//! to tag the whole tuple.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Serialize `eth_feeHistory` parameters.
pub mod fee_history {
    use super::*;
    use crate::{serialization::num::Quantity, types::BlockSpec};

    #[doc(hidden)]
    pub fn serialize<S>(
        (block_count, newest_block, reward_percentiles): &(u64, BlockSpec, Vec<f64>),
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (
            Quantity::from_ref(block_count),
            newest_block,
            reward_percentiles,
        )
            .serialize(serializer)
    }

    #[doc(hidden)]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<(u64, BlockSpec, Vec<f64>), D::Error>
    where
        D: Deserializer<'de>,
    {
        let (Quantity(block_count), newest_block, reward_percentiles) =
            Deserialize::deserialize(deserializer)?;
        Ok((block_count, newest_block, reward_percentiles))
    }
}

/// Serialize a `(_, u64)` parameter pair where the number is a hex quantity.
pub mod get_transaction_by_block_and_index {
    use super::*;
    use crate::serialization::num::Quantity;

    #[doc(hidden)]
    pub fn serialize<T, S>((block, index): &(T, u64), serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
        S: Serializer,
    {
        (block, Quantity::from_ref(index)).serialize(serializer)
    }

    #[doc(hidden)]
    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<(T, u64), D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        let (block, Quantity(index)) = Deserialize::deserialize(deserializer)?;
        Ok((block, index))
    }
}

/// Serialize tuple with an optional last value.
pub mod call_like {
    use super::*;

    #[doc(hidden)]
    pub fn serialize<S, A, B>((a, b): &(A, Option<B>), serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        A: Serialize,
        B: Serialize,
    {
        if let Some(b) = b {
            (a, b).serialize(serializer)
        } else {
            (a,).serialize(serializer)
        }
    }

    #[doc(hidden)]
    pub fn deserialize<'de, D, A, B>(deserializer: D) -> Result<(A, Option<B>), D::Error>
    where
        D: Deserializer<'de>,
        A: Deserialize<'de>,
        B: Deserialize<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Inner<A, B> {
            Both((A, B)),
            Single((A,)),
        }

        match Inner::deserialize(deserializer)? {
            Inner::Both((a, b)) => Ok((a, Some(b))),
            Inner::Single((a,)) => Ok((a, None)),
        }
    }
}

/// Serialize a single bytes parameter as a one-item JSON RPC params array.
pub mod eth_send_raw_transaction {
    use super::*;
    use crate::serialization::bytes::{decode, encode};
    use std::{borrow::Cow, str};

    #[doc(hidden)]
    pub fn serialize<S>(value: &(Vec<u8>,), serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (encode(&value.0),).serialize(serializer)
    }

    #[doc(hidden)]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<(Vec<u8>,), D::Error>
    where
        D: Deserializer<'de>,
    {
        let (hex,): (Cow<str>,) = Deserialize::deserialize(deserializer)?;
        let bytes = decode(&hex)?;
        Ok((bytes,))
    }
}

/// Serialize `(address, bytes)` as JSON RPC params.
pub mod eth_sign {
    use super::*;
    use crate::serialization::bytes::{decode, encode};
    use ethprim::Address;
    use std::{borrow::Cow, str};

    #[doc(hidden)]
    pub fn serialize<S>(value: &(Address, Vec<u8>), serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (address, bytes) = value;
        (address, encode(bytes)).serialize(serializer)
    }

    #[doc(hidden)]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<(Address, Vec<u8>), D::Error>
    where
        D: Deserializer<'de>,
    {
        let (address, hex): (Address, Cow<str>) = Deserialize::deserialize(deserializer)?;
        Ok((address, decode(&hex)?))
    }
}
