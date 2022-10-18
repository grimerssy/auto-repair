pub mod orders;
pub mod services;
pub mod users;

use crate::{
    data::{DbPool, PooledConnection},
    errors::{Error, map::to_internal_error},
};
use actix_web::web::Data;

type Result<T> = std::result::Result<T, Error>;

async fn retrieve_connection(pool: Data<DbPool>) -> Result<PooledConnection> {
    pool.get().await
        .map_err(to_internal_error())
}
