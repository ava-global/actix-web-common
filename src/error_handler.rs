use actix_web::{
    error::{self, Error, JsonPayloadError},
    http::StatusCode,
    HttpRequest, ResponseError,
};

use crate::actix_web_error::{ActixWebError, ErrorInfo};

pub fn json_error_handler(err: JsonPayloadError, _req: &HttpRequest) -> Error {
    let awe = ActixWebError {
        error_info: ErrorInfo {
            message: "Invalid json body".to_string(),
            explaination: Some(err.to_string()),
            action: None,
        },
        status_code: StatusCode::BAD_REQUEST,
    };

    error::InternalError::from_response("", awe.error_response()).into()
}
