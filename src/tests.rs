use crate::{eth, types::*};
use serde_json::json;

macro_rules! assert_method_serialization {
    ($m:ty {
        $pv:expr => $pj:expr;
        $rv:expr => $rj:expr;
    }) => {{
        let pv = $pv;
        let pj = $pj;
        assert_eq!(<$m as $crate::method::Method>::serialize_params(&pv, ::serde_json::value::Serializer).unwrap(), pj);
        assert_eq!(<$m as $crate::method::Method>::deserialize_params(pj).unwrap(), pv);
    }};
}

#[test]
fn eth_accounts() {
    assert_method_serialization!(eth::Accounts {
        Empty => json!([]);
        vec![
            address!("0xd1f5279be4b4dd94133a23dee1b23f5bfc0db1d0"),
        ] => json!([
            "0xd1f5279be4b4dd94133a23dee1b23f5bfc0db1d0",
        ]);
    })
}
