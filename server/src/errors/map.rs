use std::fmt::Display;

use diesel::result::Error;

use super::ServerError;

pub fn from_diesel_error() -> impl Fn(Error) -> ServerError {
    |e| match e {
        Error::NotFound => ServerError::NotFound,
        Error::InvalidCString(_) => {
            ServerError::BadRequest("string contains null character".into())
        }
        _ => ServerError::Internal(e.to_string()),
    }
}

pub fn to_internal_error<E>() -> impl Fn(E) -> ServerError
where
    E: Display,
{
    |e| ServerError::Internal(e.to_string())
}
