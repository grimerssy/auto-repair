pub mod map;

use core::fmt;
use std::fmt::Display;

use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use serde::Serialize;

#[derive(Debug)]
pub enum ServerError {
    FailToParse(String),
    NotFound,
    Internal(String),
}

impl Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FailToParse(msg) => write!(f, "Parsing error: {}", msg),
            Self::NotFound => write!(f, "Requested resource was not found"),
            Self::Internal(_) => write!(f, "An unexpected error occurred"),
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}

impl error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        let code = self.status_code();
        let message = self.to_string();
        let response = ErrorResponse {
            code: code.as_u16(),
            message,
        };
        HttpResponse::build(code)
            .insert_header(ContentType::json())
            .body(serde_json::to_string(&response).unwrap())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Self::FailToParse(_) => StatusCode::BAD_REQUEST,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
