use std::fmt::Display;

use super::ServerError;

pub fn to_internal_error<E>() -> impl Fn(E) -> ServerError
where
    E: Display,
{
    |e| ServerError::Internal(e.to_string())
}
