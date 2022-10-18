use super::Error;
use diesel::result::Error as DieselError;
use std::fmt::Display;

pub fn from_diesel_error() -> impl Fn(DieselError) -> Error {
    |e| match e {
        DieselError::NotFound => Error::NotFound,
        DieselError::InvalidCString(_) => {
            Error::BadRequest("string contains null character".into())
        }
        _ => Error::Internal(e.to_string()),
    }
}

pub fn to_internal_error<E: Display>() -> impl Fn(E) -> Error {
    |e| Error::Internal(e.to_string())
}
