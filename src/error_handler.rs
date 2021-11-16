use actix_web::{
    error::{self, Error, JsonPayloadError},
    http::StatusCode,
    HttpRequest, HttpResponseBuilder, ResponseError,
};
use serde_json::json;

use crate::actix_web_error::{ActixWebError, ErrorInfo};

pub fn json_error_handler(err: JsonPayloadError, _req: &HttpRequest) -> Error {
    let awe = ActixWebError {
        error_info: ErrorInfo {
            message: err.to_string(),
            explaination: Some("Invalid json body".to_owned()),
            action: None,
        },
        status_code: StatusCode::BAD_REQUEST,
    };

    error::InternalError::from_response(
        "",
        HttpResponseBuilder::new(awe.status_code())
            .content_type(mime::APPLICATION_JSON)
            .body(json!(awe.to_string()).to_string()),
    )
    .into()
}
