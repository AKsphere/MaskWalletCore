mod account;
mod common;
mod sign;
mod stored_key;
mod validate;
use chain_common::api::mw_request::Request::*;
use chain_common::api::{mw_request, MwResponse};

pub fn dispatch_request(request: mw_request::Request) -> MwResponse {
    match request {
        ParamGetVersion(_) => common::get_lib_version(),
        ParamLoadStoredKey(param) => stored_key::load_stored_keys(param),
        ParamCreateStoredKey(param) => stored_key::create_stored_key(param),
        ParamImportPrivateKey(param) => stored_key::create_stored_key_with_private_key(param),
        ParamImportMnemonic(param) => stored_key::create_stored_key_with_mnemonic(param),
        ParamImportJson(param) => stored_key::create_stored_key_with_json(param),
        ParamExportPrivateKey(param) => stored_key::export_private_key(param),
        ParamExportPrivateKeyOfPath(param) => stored_key::export_private_key_of_path(param),
        ParamExportMnemonic(param) => stored_key::export_mnemonic(param),
        ParamExportKeyStoreJsonOfAddress(param) => {
            stored_key::export_key_store_json_of_address(param)
        }
        ParamExportKeyStoreJsonOfPath(param) => stored_key::export_key_store_json_of_path(param),
        ParamUpdateKeyStorePassword(param) => stored_key::update_key_store_password(param),
        ParamUpdateKeyStoreName(param) => stored_key::update_key_store_name(param),

        ParamGetStoredKeyAccountCount(param) => account::get_stored_key_account_count(param),
        ParamGetStoredKeyAccount(param) => account::get_store_key_account(param),
        ParamGetStoredKeyAllAccounts(param) => account::get_stored_key_all_accounts(param),
        ParamGetStoredKeyAccountsOfCoin(param) => account::get_stored_key_accounts_of_coin(param),
        ParamCreateAcccountOfCoinAtPath(param) => {
            account::create_stored_key_account_of_coin_at_path(param)
        }
        ParamRemoveAccountsOfCoin(param) => account::remove_stored_key_account_of_coin(param),
        ParamRemoveAccountOfAddress(param) => account::remove_account_of_address(param),
        ParamSignTransaction(param) => sign::sign_transaction(param),

        ParamValidation(param) => validate::validate(param),
    }
}
