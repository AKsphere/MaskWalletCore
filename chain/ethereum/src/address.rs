use std::string::ToString;
use chain_common::public_key::PublicKey;
use crypto::Error;
use crypto::public_key::PublicKeyType;
use crypto::hash::Keccak256;
use super::address_checksum::{ ChecksumType, checksum };

const ADDRESS_SIZE: usize = 20;

pub struct EthereumAddress {
    pub data: Vec<u8>,
}

impl EthereumAddress {
    pub fn new(public_key: &PublicKey) -> Result<Self, Error> {
        if public_key.r#type != PublicKeyType::SECP256k1Extended {
            return Err(Error::NotSupportedPublicKeyType);
        }
        let hash = public_key.hash(&[], Keccak256, true)?;
        let begin = hash.len() - ADDRESS_SIZE;
        Ok(EthereumAddress {
            data: hash[begin..].to_vec()
        })
    }
}

impl ToString for EthereumAddress {
    fn to_string(&self) -> String {
        checksum(&self, ChecksumType::EIP55)
    }
}

#[cfg(test)]
mod tests {
    use chain_common::public_key::PublicKey;
    use crypto::public_key::PublicKeyType;
    use crate::address::EthereumAddress;
    #[test]
    fn test_derive_from_pub_key() {
        
        let pub_key_str = "0499c6f51ad6f98c9c583f8e92bb7758ab2ca9a04110c0a1126ec43e5453d196c166b489a4b7c491e7688e6ebea3a71fc3a1a48d60f98d5ce84c93b65e423fde91";

        let pub_key_data = hex::decode(pub_key_str).unwrap();

        let public_key = PublicKey {
            r#type: PublicKeyType::SECP256k1Extended,
            data: pub_key_data.to_vec(),
        };
        let address = EthereumAddress::new(&public_key);
        assert_eq!(address.is_ok(), true);
        let address_str = address.unwrap().to_string();
        assert_eq!(address_str, "0xAc1ec44E4f0ca7D172B7803f6836De87Fb72b309");
    }
}
