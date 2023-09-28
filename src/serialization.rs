//! JSON serialization helpers.

#![allow(dead_code)]

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

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
        let bytes = match value {
            Some(value) => value.as_ref(),
            None => return serializer.serialize_none(),
        };

        serializer.serialize_some(&encode(bytes))
    }

    #[doc(hidden)]
    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: From<Vec<u8>>,
        D: Deserializer<'de>,
    {
        let hex = match Option::<Cow<str>>::deserialize(deserializer)? {
            Some(value) => value,
            None => return Ok(None),
        };

        Ok(Some(decode(&hex)?.into()))
    }
}

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
