pub mod map;

use core::fmt;
use std::fmt::Display;

use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use serde::Serialize;

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum ServerError {
    NotFoundError,
    ValidationError(String),
    InternalServerError(String),
}

impl Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::NotFoundError => write!(f, "Requested resource was not found"),
            ServerError::ValidationError(reason) => write!(f, "Validation failed: {}", reason),
            ServerError::InternalServerError(_) => write!(f, "An unexpected error occurred"),
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
            Self::ValidationError { .. } => StatusCode::BAD_REQUEST,
            Self::NotFoundError { .. } => StatusCode::NOT_FOUND,
            Self::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
