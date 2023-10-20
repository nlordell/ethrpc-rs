//! Module containing concept of an Ethereum RPC method.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::borrow::Cow;

/// A trait defining an Ethereum RPC method.
pub trait Method {
    type Params;
    type Result;

    fn name(&self) -> Cow<'static, str>;

    fn deserialize_params<'de, D>(deserializer: D) -> Result<Self::Params, D::Error>
    where
        D: Deserializer<'de>;
    fn serialize_params<S>(value: &Self::Params, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;

    fn deserialize_result<'de, D>(deserializer: D) -> Result<Self::Result, D::Error>
    where
        D: Deserializer<'de>;
    fn serialize_result<S>(value: &Self::Result, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

macro_rules! impl_method_for_stringlike {
    ($($str:ty,)*) => {$(
        impl Method for $str {
            type Params = Value;
            type Result = Value;

            fn name(&self) -> Cow<'static, str> {
                Cow::Owned(self.to_string())
            }

            fn deserialize_params<'de, D>(deserializer: D) -> Result<Self::Params, D::Error>
            where
                D: Deserializer<'de>,
            {
                Value::deserialize(deserializer)
            }

            fn serialize_params<S>(value: &Self::Params, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                value.serialize(serializer)
            }

            fn deserialize_result<'de, D>(deserializer: D) -> Result<Self::Result, D::Error>
            where
                D: Deserializer<'de>,
            {
                Value::deserialize(deserializer)
            }

            fn serialize_result<S>(value: &Self::Result, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                value.serialize(serializer)
            }
        }
    )*};
}

impl_method_for_stringlike! {
    str,
    &'_ str,
    String,
    Cow<'_, str>,
}

#[macro_export]
macro_rules! method {
    (
        $(#[$attr:meta])*
        $pub:vis struct $type:ident as $name:literal $params:ty => $result:ty;
    ) => {
        $crate::method! {
            $(#[$attr])* $pub struct $type as $name
                $params [<$params>] => $result [<$result>];
        }
    };

    (
        $(#[$attr:meta])*
        $pub:vis struct $type:ident as $name:literal
            $params:ty => $result:ty [$($resultas:tt)*];
    ) => {
        $crate::method! {
            $(#[$attr])* $pub struct $type as $name
                $params [<$params>] => $result [$($resultas)*];
        }
    };

    (
        $(#[$attr:meta])*
        $pub:vis struct $type:ident as $name:literal
            $params:ty [$($paramsas:tt)*] => $result:ty;
    ) => {
        $crate::method! {
            $(#[$attr])* $pub struct $type as $name
                $params [$($paramsas)*] => $result [<$result>];
        }
    };

    (
        $(#[$attr:meta])*
        $pub:vis struct $type:ident as $name:literal
            $params:ty [$($paramsas:tt)*] => $result:ty [$($resultas:tt)*];
    ) => {
        $(#[$attr])*
        $pub struct $type;

        impl ::std::fmt::Debug for $type {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.debug_tuple(stringify!($type))
                    .field(&$name)
                    .finish()
            }
        }

        #[allow(unused_imports)]
        impl $crate::method::Method for $type {
            type Params = $params;
            type Result = $result;

            fn name(&self) -> ::std::borrow::Cow<'static, str> {
                ::std::borrow::Cow::Borrowed($name)
            }

            fn deserialize_params<'de, D>(deserializer: D) -> Result<Self::Params, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                use ::serde::Deserialize as _;
                $($paramsas)*::deserialize(deserializer)
            }

            fn serialize_params<S>(value: &Self::Params, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                use ::serde::Serialize as _;
                $($paramsas)*::serialize(value, serializer)
            }

            fn deserialize_result<'de, D>(deserializer: D) -> Result<Self::Result, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                use ::serde::Deserialize as _;
                $($resultas)*::deserialize(deserializer)
            }

            fn serialize_result<S>(value: &Self::Result, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                use ::serde::Serialize as _;
                $($resultas)*::serialize(value, serializer)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $type {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                let value = ::std::borrow::Cow::<str>::deserialize(deserializer)?;
                if value != $name {
                    return Err(::serde::de::Error::custom(format!(
                        "expected {:?} but got {:?}",
                        $name,
                        value,
                    )));
                }
                Ok(Self)
            }
        }

        impl ::serde::Serialize for $type {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                use $crate::method::Method as _;
                self.name().serialize(serializer)
            }
        }
    };
}

#[macro_export]
macro_rules! module {
    (
        $(#[$attr:meta])*
        $pub:vis mod $mod:ident {
            $(
                $(#[$ma:meta])*
                $mv:vis struct $mt:ident as $mn:literal
                    $mp:ty $([$($mpp:tt)*])? => $mr:ty $([$($mrr:tt)*])?;
            )*
        }
    ) => {
        $(#[$attr])*
        $pub mod $mod {
            #[allow(unused_imports)]
            use super::*;

            $(
                $crate::method! {
                    $(#[$ma])* $mv struct $mt as $mn
                        $mp $([$($mpp)*])* => $mr $([$($mrr)*])*;
                }
            )*
        }
    };
}
