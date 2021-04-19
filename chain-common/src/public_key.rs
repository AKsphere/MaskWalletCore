use crypto::public_key::PublicKeyType;
use crypto::hash::Hashable;
use crypto::Error as CryptoError;

const SECP256K1_SIZE: usize = 33;

const SECP256K1EXTENDED_SIZE: usize = 65;

pub struct PublicKey {
    pub r#type: PublicKeyType,
    pub data: Vec<u8>,
}

impl PublicKey {

    pub fn is_valid_data(data: &[u8], r#type: &PublicKeyType) -> bool {
        if data.is_empty() {
            return false;
        }
        let size = data.len();
        match r#type {
            PublicKeyType::SECP256k1 => {
                return size == SECP256K1_SIZE && (data[0] == 0x02 || data[0] == 0x03)
            },
            PublicKeyType::SECP256k1Extended => {
                return size == SECP256K1EXTENDED_SIZE && data[0] == 0x04
            }
        }
    }

    pub fn new(r#type: PublicKeyType, data: &[u8]) -> Result<Self, CryptoError> {
        if !Self::is_valid_data(&data, &r#type) {
            return Err(CryptoError::InvalidPublicKey);
        }
        Ok(PublicKey {
            r#type: r#type,
            data: data.to_vec()
        })
    }

    pub fn hash<T: Hashable>(&self, prefix: &[u8], hasher: T, skip_type_byte: bool) -> Result<Vec<u8>, CryptoError> {
        let offset: usize = match skip_type_byte {
            true => 1,
            false => 0,
        };
        let hash = hasher.hash(&self.data[offset..])?;
        Ok([&prefix, &hash[..]].concat())
    }
}