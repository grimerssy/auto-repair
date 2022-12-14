pub mod cars;
pub mod contacts;
pub mod orders;
pub mod reports;
pub mod services;
pub mod sql;
pub mod users;
pub mod workers;

use crate::{
    data::{DbPool, PooledConnection},
    errors::{map::to_internal_error, Error},
    models::id::Id,
};
use actix_web::{web::Data, HttpRequest};
use jsonwebtoken::{DecodingKey, Validation};
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Error>;

async fn retrieve_connection(pool: Data<DbPool>) -> Result<PooledConnection> {
    pool.get().await.map_err(to_internal_error())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    sub: Id,
    role: String,
    exp: i64,
}

async fn get_claims<'a>(req: &'a HttpRequest, secret: &'a str) -> Result<Claims> {
    let header = req
        .headers()
        .get("Authorization")
        .ok_or(Error::InvalidAuth)?
        .to_str()
        .map_err(|_| Error::InvalidAuth)?;
    let split = header.split(' ').collect::<Vec<&str>>();
    if split.len() != 2 || split[0] != "Bearer" {
        return Err(Error::InvalidAuth);
    }
    jsonwebtoken::decode::<Claims>(
        split[1],
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|t| t.claims)
    .map_err(|_| Error::InvalidAuth)
}

fn check_if_admin(claims: Claims) -> Result<()> {
    if claims.role != "admin" {
        return Err(Error::AccessDenied);
    }
    Ok(())
}
