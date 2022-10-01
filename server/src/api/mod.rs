pub mod orders;
pub mod services;

use actix_web::web::Data;
use diesel::result::Error;

use crate::{DbPool, PooledConnection};
use crate::errors::{ServerError, map::to_internal_error};

async fn retrieve_connection(pool: Data<DbPool>) -> Result<PooledConnection, ServerError> {
    pool.get().await
        .map_err(to_internal_error())
}

fn to_server_error() -> impl Fn(Error) -> ServerError {
    |e| {
        match e {
            Error::NotFound => ServerError::NotFoundError,
            _ => ServerError::InternalServerError(format!("{}", e))
        }
    }
}
