use super::api::{ MwResponse, mw_request};
use super::api::mw_request::Request::*;

use super::param::*;

use wallet::stored_key::*;
// use wallet::encryption::scrypt_parameters::ScryptParameters;

pub fn dispatch_request(request: mw_request::Request) -> MwResponse {
    let response = match request {
        ParamImportPrivateKey(param) => {
            create_stored_key(param)
        }
    };
    return MwResponse {
        is_success: true, 
        error: String::from(""),
        data: String::from("")
    };
}

fn create_stored_key(parma: PrivateKeyStoreImportParam) -> MwResponse {
    // let stored_key = StoredKey::<ScryptParameters>::create_with_private_key("test1", "password", "tt");

    MwResponse {
        is_success: true,
        error: String::from(""),
        data: String::from("")
    }
}