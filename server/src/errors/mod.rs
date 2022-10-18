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
pub enum Error {
    BadRequest(String),
    NotFound,
    InvalidPassword,
    Internal(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            Self::NotFound => write!(f, "Requested resource was not found"),
            Self::InvalidPassword => write!(f, "Invalid password"),
            Self::Internal(_) => write!(f, "An unexpected error occurred"),
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}

impl error::ResponseError for Error {
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
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::InvalidPassword => StatusCode::UNAUTHORIZED,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
