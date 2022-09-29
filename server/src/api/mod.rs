use std::fmt::Display;

use actix_web::web::Data;
use diesel_async::{pooled_connection::deadpool::Object, AsyncPgConnection};

use crate::{DbPool, errors::ServerError};

type Connection = Object<AsyncPgConnection>;

async fn retrieve_connection(pool: Data<DbPool>) -> Result<Connection, ServerError> {
    pool.get().await
        .map_err(to_server_error())
}

fn to_server_error<E>() -> impl Fn(E) -> ServerError
where
    E: Display
{
    |e| ServerError::InternalServerError(format!("{}", e))
}
