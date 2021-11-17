use actix_web::{
    error::{Error, JsonPayloadError},
    http::StatusCode,
    HttpRequest,
};

use crate::actix_web_error::{ActixWebError, ErrorInfo};

/// transform json playload error to actix error
pub fn json_error_handler(err: JsonPayloadError, _req: &HttpRequest) -> Error {
    let actix_web_error = ActixWebError {
        error_info: ErrorInfo {
            message: "Invalid json body".to_string(),
            explaination: Some(err.to_string()),
            action: None,
        },
        status_code: StatusCode::BAD_REQUEST,
    };

    actix_web_error.to_error()
}
