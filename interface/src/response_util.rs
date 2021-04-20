use super::api::{ MwResponse, MwResponseError };
use super::api::mw_response::Response;
use wallet::Error;

pub fn get_json_response_error() -> Option<Response> {
    Some(Response::Error(MwResponseError{
        error_code: "-1".to_owned(),
        error_msg: "Invalid Data".to_owned(),
    }))
}

pub fn get_json_error_response() -> MwResponse {
    MwResponse {
        is_success: false,
        response: get_json_response_error()
    }
}

fn get_error_response(error: Error) -> Response {
    Response::Error(MwResponseError{
        error_code: "-1".to_owned(), // TODO: error to error code
        error_msg: "Invalid Data".to_owned(),  // TODO: error to error message
    })
}

pub fn get_error_response_by_error(error: Error) -> MwResponse {
    MwResponse {
        is_success: false,
        response: Some(get_error_response(error))
    }
}