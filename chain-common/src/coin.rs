use std::collections::HashMap;
use serde::{ Serialize, Deserialize };
use crate::param::Coin as ProtoCoin;

#[derive(Clone, Serialize, Deserialize)]
pub struct Coin {
    
    pub id: String,
    
    pub name: String,
    
    pub coin_id: i32,
    
    pub symbol: String,
    
    pub decimals: i32,
    
    pub blockchain: String,
    
    pub derivation_path: String,
    
    pub curve: String,
    
    pub public_key_type: String,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub all_info: HashMap<String, serde_json::Value>,
}

impl Coin {
    pub fn get_value(&self, key: &str) -> Option<String> {
        self.all_info.get(key).map(|x| x.to_string() )
    }

    pub fn get_xpub(&self) -> Option<String> {
        self.all_info.get("xpub").map(|x| x.to_string() )
    }
}

impl PartialEq for Coin {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Coin {}

impl ToString for ProtoCoin {
    fn to_string(&self) -> String {
        match self {
            ProtoCoin::Ethereum => "ethereum".to_owned(),
            ProtoCoin::Polkadot => "polkadot".to_owned(),
        }
    }
}