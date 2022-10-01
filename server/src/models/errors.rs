use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::Display;
use serde::Serialize;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Display)]
pub enum ServerError {
    #[display(fmt = "Validation failed: {}", reason)]
    ValidationError { reason: String },
    #[display(fmt = "Requested {} was not found", what)]
    NotFoundError { what: String },
    #[display(fmt = "An unexpected error occurred")]
    InternalServerError(String),
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
