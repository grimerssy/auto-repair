use std::fmt::Display;

use super::ServerError;

pub fn to_internal_error<E>() -> impl Fn(E) -> ServerError
where
    E: Display,
{
    |e| ServerError::InternalServerError(format!("{}", e))
}

pub fn to_not_found_error<E>(what: String) -> impl Fn(E) -> ServerError {
    move |_| ServerError::NotFoundError(what.clone())
}
