use ethrpc::{eth, types::*};

fn main() {
    let client = ethrpc::curl::Client::from_env();
    let (block_number, block) = client
        .batch((
            (eth::BlockNumber, Empty),
            (
                eth::GetBlockByNumber,
                (BlockTag::Latest.into(), Hydrated::No),
            ),
        ))
        .unwrap();

    assert_eq!(block_number, block.unwrap().number);
}
