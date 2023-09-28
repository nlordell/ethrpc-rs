//! A log bloom filter.
//!
//! TODO(nlordell): This should live in its own crate and implement an actual
//! bloom filter.

use crate::{debug, serialization};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{self, Debug, Formatter};

/// A bloom filter.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Bloom(pub [u8; 256]);

impl Default for Bloom {
    fn default() -> Self {
        Self([0; 256])
    }
}

impl Debug for Bloom {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("Bloom").field(&debug::Hex(&self.0)).finish()
    }
}

impl Serialize for Bloom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialization::bytearray::serialize(&self.0, serializer)
    }
}

impl<'de> Deserialize<'de> for Bloom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serialization::bytearray::deserialize(deserializer).map(Self)
    }
}
