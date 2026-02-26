//! JSON serialization helpers.

#![allow(dead_code)]

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

/// Serialize a fixed size `[u8; N]`.
pub mod bytearray {
    use super::{bytes, *};
    use std::{borrow::Cow, str};

    #[doc(hidden)]
    pub fn serialize<const N: usize, S>(value: &[u8; N], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        bytes::serialize(value, serializer)
    }

    #[doc(hidden)]
    pub fn deserialize<'de, const N: usize, D>(deserializer: D) -> Result<[u8; N], D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut bytes = [0; N];
        bytes::decode_into(&Cow::<str>::deserialize(deserializer)?, &mut bytes)?;
        Ok(bytes)
    }
}

/// Serialize a `[u8]`
pub mod bytes {
    use super::*;
    use std::{borrow::Cow, fmt::Write as _, str};

    #[doc(hidden)]
    pub fn encode(bytes: &[u8]) -> String {
        let mut buffer = String::with_capacity(2 + bytes.len() * 2);
        buffer.push_str("0x");
        for byte in bytes {
            write!(&mut buffer, "{byte:02x}").unwrap();
        }
        buffer
    }

    #[doc(hidden)]
    pub fn decode<E>(hex: &str) -> Result<Vec<u8>, E>
    where
        E: de::Error,
    {
        let mut bytes = vec![0; (hex.len() / 2).saturating_sub(1)];
        decode_into(hex, &mut bytes)?;
        Ok(bytes)
    }

    #[doc(hidden)]
    pub fn decode_into<E>(hex: &str, bytes: &mut [u8]) -> Result<(), E>
    where
        E: de::Error,
    {
        let hex = hex
            .strip_prefix("0x")
            .ok_or_else(|| de::Error::custom("bytes missing '0x' prefix"))?;

        if hex.len() % 2 != 0 {
            return Err(de::Error::custom("odd number of characters in hex string"));
        }

        let nibble = |x: u8| -> Result<u8, E> {
            match x {
                b'0'..=b'9' => Ok(x - b'0'),
                b'a'..=b'f' => Ok(x - b'a' + 0xa),
                b'A'..=b'F' => Ok(x - b'A' + 0xa),
                _ => Err(de::Error::custom("invalid hex ASCII digit {x:02#x}")),
            }
        };

        for (byte, chunk) in bytes.iter_mut().zip(hex.as_bytes().chunks_exact(2)) {
            *byte = (nibble(chunk[0])? << 4) + nibble(chunk[1])?;
        }

        Ok(())
    }

    #[doc(hidden)]
    pub fn serialize<T, S>(value: T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: AsRef<[u8]>,
        S: Serializer,
    {
        encode(value.as_ref()).serialize(serializer)
    }

    #[doc(hidden)]
    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: From<Vec<u8>>,
        D: Deserializer<'de>,
    {
        Ok(decode(&Cow::<str>::deserialize(deserializer)?)?.into())
    }
}

/// Serialize an `Option<[u8]>`
pub mod option_bytes {
    use super::{
        bytes::{decode, encode},
        *,
    };
    use std::{borrow::Cow, str};

    #[doc(hidden)]
    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: AsRef<[u8]>,
        S: Serializer,
    {
        match value {
            Some(bytes) => serializer.serialize_some(&encode(bytes.as_ref())),
            None => serializer.serialize_none(),
        }
    }

    #[doc(hidden)]
    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: From<Vec<u8>>,
        D: Deserializer<'de>,
    {
        match Option::<Cow<str>>::deserialize(deserializer)? {
            Some(hex) => Ok(Some(decode(&hex)?.into())),
            None => Ok(None),
        }
    }
}

/// Serialize a `Vec<Vec<u8>>`
pub mod vec_bytes {
    use super::{
        bytes::{decode, encode},
        *,
    };
    use std::{borrow::Cow, str};

    #[doc(hidden)]
    pub fn serialize<S>(value: &[Vec<u8>], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value
            .iter()
            .map(|bytes| encode(bytes.as_ref()))
            .collect::<Vec<_>>()
            .serialize(serializer)
    }

    #[doc(hidden)]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Vec::<Cow<str>>::deserialize(deserializer)?
            .into_iter()
            .map(|item| decode(&item))
            .collect()
    }
}

/// Serialize an `Option<Vec<Vec<u8>>>`
pub mod option_vec_bytes {
    use super::{
        bytes::{decode, encode},
        *,
    };
    use std::{borrow::Cow, str};

    #[doc(hidden)]
    pub fn serialize<S>(value: &Option<Vec<Vec<u8>>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => serializer.serialize_some(
                &value
                    .iter()
                    .map(|bytes| encode(bytes.as_ref()))
                    .collect::<Vec<_>>(),
            ),
            None => serializer.serialize_none(),
        }
    }

    #[doc(hidden)]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<Vec<u8>>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match Option::<Vec<Cow<str>>>::deserialize(deserializer)? {
            Some(value) => Ok(Some(
                value
                    .into_iter()
                    .map(|item| decode(&item))
                    .collect::<Result<Vec<_>, D::Error>>()?,
            )),
            None => Ok(None),
        }
    }
}

/// Serialize `0x` prefixed hex numbers.
pub mod num {
    use super::*;
    use std::borrow::Cow;

    #[doc(hidden)]
    pub trait Num: Sized {
        fn to_hex(&self) -> String;
        fn from_hex<E>(s: &str) -> Result<Self, E>
        where
            E: de::Error;
    }

    macro_rules! impl_num {
        ($($t:ty,)*) => {$(
            impl Num for $t {
                fn to_hex(&self) -> String {
                    format!("{self:#x}")
                }

                fn from_hex<E>(s: &str) -> Result<Self, E>
                where
                    E: de::Error,
                {
                    let hex = s
                        .strip_prefix("0x")
                        .ok_or_else(|| E::custom("missing 0x prefix"))?;
                    Self::from_str_radix(hex, 16).map_err(E::custom)
                }
            }
        )*};
    }

    impl_num! {
        u8,
        u16,
        u32,
        u64,
        u128,
    }

    #[doc(hidden)]
    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Num,
        S: Serializer,
    {
        value.to_hex().serialize(serializer)
    }

    #[doc(hidden)]
    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: Num,
        D: Deserializer<'de>,
    {
        T::from_hex(&Cow::<str>::deserialize(deserializer)?)
    }
}

/// Serialize optional `0x` prefixed hex numbers.
pub mod option_num {
    use super::{num::Num, *};
    use std::borrow::Cow;

    #[doc(hidden)]
    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Num,
        S: Serializer,
    {
        match value {
            Some(value) => serializer.serialize_some(&value.to_hex()),
            None => serializer.serialize_none(),
        }
    }

    #[doc(hidden)]
    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: Num,
        D: Deserializer<'de>,
    {
        match Option::<Cow<str>>::deserialize(deserializer)? {
            Some(value) => Ok(Some(T::from_hex(&value)?)),
            None => Ok(None),
        }
    }
}

/// Serialize a single bytes parameter as a one-item JSON RPC params array.
pub mod param_eth_send_raw_transaction {
    use super::{
        bytes::{decode, encode},
        *,
    };
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
pub mod param_eth_sign {
    use super::{
        bytes::{decode, encode},
        *,
    };
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
