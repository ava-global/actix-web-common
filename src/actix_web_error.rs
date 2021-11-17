use std::fmt::{Debug, Display};

use actix_web::{
    http::{self, StatusCode},
    HttpResponse, HttpResponseBuilder, ResponseError,
};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Default, Serialize)]
pub struct ErrorInfo {
    pub message: String,
    pub explaination: Option<String>,
    pub action: Option<String>,
}

/// Common actix web error struct
/// has 2 assoicated functions `new` and `to_error`
#[derive(Debug)]
pub struct ActixWebError {
    pub error_info: ErrorInfo,
    pub status_code: StatusCode,
}

impl ActixWebError {
    /// use to construct common new ActixWebError
    pub fn new(message: String, status_code: StatusCode) -> Self {
        Self {
            error_info: ErrorInfo {
                message,
                ..Default::default()
            },
            status_code,
        }
    }

    /// transform itself to actix error
    pub fn to_error(&self) -> actix_web::error::Error {
        actix_web::error::InternalError::from_response("", self.error_response()).into()
    }
}

impl Display for ActixWebError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl actix_web::error::ResponseError for ActixWebError {
    fn status_code(&self) -> http::StatusCode {
        self.status_code
    }

    /// for building common error response based on ActixWebError
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .insert_header(http::header::ContentType(mime::APPLICATION_JSON))
            .body(
                json!({
                    "error": &self.error_info
                })
                .to_string(),
            )
    }
}
