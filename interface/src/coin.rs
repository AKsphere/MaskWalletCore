
use std::collections::HashMap;
use std::string::ToString;
use chain_common::coin::Coin;

use super::param::Coin as CoinType;

lazy_static! {
    static ref COINS_MAP: HashMap<String, Coin> = {
        let coin_json = include_str!("../resource/coin.json");
        let coins: Vec<Coin> = serde_json::from_str(coin_json).expect("fail to get coins info from json");

        // Since each chain may contains different key-values, we extend a HashMap<String, serde_json::Value>
        // to each chain with its whole key-values
        let mut coins_info_hashmaps: Vec<HashMap<String, serde_json::Value>> = serde_json::from_str(coin_json).expect("fail to get coins info from json");
        let mut coins_map: HashMap<String, Coin> = HashMap::new();

        coins.into_iter().for_each(|mut coin| {
            coin.all_info = Some(coins_info_hashmaps.remove(0));
            coins_map.insert(coin.name.to_owned(), coin);
        });
        coins_map
    };
}

impl ToString for CoinType {
    fn to_string(&self) -> String {
        match self {
            CoinType::Ethereum => "ethereum".to_owned(),
            CoinType::Polkadot => "polkadot".to_owned()
        }
    }
}

pub fn get_coin_info<'a>(coin_type: i32) -> Option<&'a Coin> {
    match CoinType::from_i32(coin_type) {
        Some(coin) => COINS_MAP.get(&coin.to_string()),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::coin::COINS_MAP;
    #[test]
    fn test_get_coin_info() {
        assert_eq!(COINS_MAP.len(), 2);
        let coin_info = COINS_MAP.get("Ethereum").unwrap();
        
        assert_eq!(coin_info.curve, "secp256k1");
    }
}
