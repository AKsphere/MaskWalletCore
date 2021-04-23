use uuid::Uuid;
use serde::{ Serialize, Deserialize };

use crate::Error;
use crypto::Error as CryptoError;
use crypto::bip39::Mnemonic;
use super::account::Account;
use super::encryption_params::{ EncryptionParams };
use super::derivation_path::DerivationPath;
use super::coin_dispatcher::get_dispatcher;
use super::hd_wallet::HdWallet;
use chain_common::coin::Coin;
use chain_common::private_key::PrivateKey;

#[derive(Serialize, Deserialize, PartialEq)]
pub enum StoredKeyType {
    PrivateKey = 1,
    Mnemonic,
}

#[derive(Serialize, Deserialize)]
pub struct StoredKey {
    pub r#type: StoredKeyType,

    pub id: String,

    pub name: String,

    payload: EncryptionParams,

    accounts: Vec<Account>,
}

impl StoredKey {
    fn create_with_data(r#type: StoredKeyType, name: &str, password: &str, data: &[u8]) -> Result<StoredKey, Error> {
        let uuid = Uuid::new_v4();
        let payload = EncryptionParams::new(password.as_bytes(), &data)?;
        Ok(StoredKey {
            r#type,
            name: String::from(name),
            id: uuid.to_string(),
            payload,
            accounts: vec![]
        })
    }

    pub fn create_with_private_key(name: &str, password: &str, private_key: &str) -> Result<StoredKey, Error> {
        let priv_key_bytes = hex::decode(private_key).map_err(|_| CryptoError::InvalidPrivateKey)?;
        Self::create_with_data(StoredKeyType::PrivateKey, &name, &password, &priv_key_bytes)
    }

    pub fn create_with_private_key_and_default_address(name: &str, password: &str, private_key: &str, coin: Coin) -> Result<StoredKey, Error> {
        let priv_key_bytes = hex::decode(private_key).map_err(|_| CryptoError::InvalidPrivateKey)?;
        PrivateKey::is_valid(&priv_key_bytes, &coin.curve)?;
        let mut stored_key = StoredKey::create_with_private_key(name, password, private_key)?;

        let private_key_struct = PrivateKey::new(&priv_key_bytes)?;

        let public_key = private_key_struct.get_public_key(&coin.public_key_type)?;
        let derivation_path = DerivationPath::new(&coin.derivation_path)?;
        let address = get_dispatcher(&coin).derive_address(&coin, &public_key, &[], &[])?;
        
        stored_key.accounts.push(Account {
            address,
            coin,
            derivation_path,
            extended_public_key: "".to_owned(),
        });
        Ok(stored_key)
    }

    pub fn create_with_mnemonic(name: &str, password: &str, mnemonic: &str) -> Result<StoredKey, Error> {
        if !Mnemonic::is_valid(mnemonic) {
            return Err(Error::CryptoError(CryptoError::InvalidMnemonic));
        }
        Self::create_with_data(StoredKeyType::Mnemonic, &name, &password, &mnemonic.as_bytes())
    }

    pub fn create_with_mnemonic_random(name: &str, password: &str) -> Result<StoredKey, Error> {
        let wallet = HdWallet::new(12, "")?;
        Self::create_with_data(StoredKeyType::Mnemonic, &name, &password, &wallet.mnemonic.as_bytes())
    }

    pub fn create_with_mnemonic_random_add_default_address(name: &str, password: &str, mnemonic: &str, coin: Coin) -> Result<StoredKey, Error> {
        let mut stored_key = StoredKey::create_with_mnemonic(&name, &password, &mnemonic)?;

        let wallet = HdWallet::new_with_mnemonic(mnemonic, "")?;
        let derivation_path = DerivationPath::new(&coin.derivation_path)?;
        let address = wallet.get_address_for_coin(&coin)?;
        let extended_public_key = wallet.get_extended_public_key(&coin);
        stored_key.accounts.push(Account {
            address,
            coin,
            derivation_path,
            extended_public_key,
        });
        Ok(stored_key)
    }
}

impl StoredKey {
    pub fn get_wallet(&self, password: &str) -> Result<HdWallet, Error> {
        if self.r#type != StoredKeyType::Mnemonic {
            return Err(Error::InvalidAccountRequested);
        }
        let mnemonic_bytes = self.payload.decrypt(&password.as_bytes())?;
        let mnemonic = std::str::from_utf8(&mnemonic_bytes).map_err(|_| Error::CryptoError(CryptoError::PasswordIncorrect) )?;
        HdWallet::new_with_mnemonic(&mnemonic, "")
    }
}

impl StoredKey {
    pub fn get_accounts_count(&self) -> u32 {
        self.accounts.len() as u32
    }

    pub fn get_account(&self, index: u32) -> Result<&Account, Error> {
        let index = index as usize;
        if index >= self.accounts.len() {
            return Err(Error::IndexOutOfBounds);
        }
        Ok(&self.accounts[index as usize])
    }

    pub fn get_all_accounts(&self) -> &[Account] {
        &self.accounts
    }

    pub fn get_account_of_coin(&self, coin: &Coin) -> Option<&Account> {
        self.accounts.iter().find(|account| account.coin == *coin )
    }

    pub fn get_or_create_account_for_coin(&mut self, coin: &Coin, wallet: Option<HdWallet>) -> Result<Option<&Account>, Error> {
        let hd_wallet = match wallet {
            Some(wallet) => wallet,
            None => return Ok(self.get_account_of_coin(&coin)),
        };
        
        for (i, account) in self.accounts.iter_mut().enumerate() {
            if account.coin == *coin {
                // Found an account of required coin
                if account.address.is_empty() {
                    account.address = hd_wallet.get_address_for_coin(&coin)?;
                }
                return Ok(self.accounts.get(i))
            }
        } 
        // No valid account found for the coin, create a new one
        let derivation_path = DerivationPath::new(&coin.derivation_path)?;
        let address = hd_wallet.get_address_for_coin(&coin)?;
        let extended_public_key = hd_wallet.get_extended_public_key(&coin);
        let account = Account {
            address,
            coin: coin.clone(),
            derivation_path,
            extended_public_key,
        };
        self.accounts.push(account);
        Ok(self.accounts.last())
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;
    use chain_common::coin::Coin;
    #[test]
    fn test_create_stored_key_with_private_key() {
        let priv_key_str = "3a1076bf45ab87712ad64ccb3b10217737f7faacbf2872e88fdd9a537d8fe266";
        let password = "mask wallet";
        let derivation_path = "m/44'/60'/0'/0/0";
        let coin = Coin {
            id: "60".to_owned(),
            name: "ethereum".to_owned(),
            coin_id: 60,
            symbol: "ETH".to_owned(),
            decimals: 18,
            blockchain: "Ethereum".to_owned(),
            derivation_path: derivation_path.to_owned(),
            curve: "secp256k1".to_owned(),
            public_key_type: "secp256k1Extended".to_owned(),
            all_info: Some(HashMap::new())
        };
        
        let stored_key = StoredKey::create_with_private_key_and_default_address("mask", &password, priv_key_str, coin).unwrap();
        assert_eq!(stored_key.get_accounts_count(), 1);
        let account = stored_key.get_account(0).unwrap();
        assert_eq!(account.address, "0xC2D7CF95645D33006175B78989035C7c9061d3F9");
        assert_eq!(account.derivation_path.to_string(), derivation_path);
        assert_eq!(account.extended_public_key, "");
    }

    #[test]
    fn test_create_with_mnemonic() {
        let mnemonic = "team engine square letter hero song dizzy scrub tornado fabric divert saddle";
        let password = "";
        let derivation_path = "m/44'/60'/0'/0/0";
        let coin = Coin {
            id: "60".to_owned(),
            name: "ethereum".to_owned(),
            coin_id: 60,
            symbol: "ETH".to_owned(),
            decimals: 18,
            blockchain: "Ethereum".to_owned(),
            derivation_path: derivation_path.to_owned(),
            curve: "secp256k1".to_owned(),
            public_key_type: "secp256k1Extended".to_owned(),
            all_info: Some(HashMap::new())
        };
        
        let stored_key = StoredKey::create_with_mnemonic_random_add_default_address("mask", &password, &mnemonic, coin).unwrap();
        assert_eq!(stored_key.r#type == StoredKeyType::Mnemonic, true);
        assert_eq!(stored_key.get_accounts_count(), 1);
        let decrypted = stored_key.payload.decrypt(password.as_bytes()).unwrap();
        assert_eq!(&decrypted, mnemonic.as_bytes());
        let account = stored_key.get_account(0).unwrap();
        assert_eq!(account.address, "0x494f60cb6Ac2c8F5E1393aD9FdBdF4Ad589507F7");
        assert_eq!(account.derivation_path.to_string(), derivation_path);
        assert_eq!(account.coin.name, "ethereum");
        assert_eq!(account.extended_public_key, "");
    }
}