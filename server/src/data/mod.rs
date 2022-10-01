pub mod orders;
pub mod contacts;
pub mod services;

use diesel_async::{pooled_connection::{AsyncDieselConnectionManager, deadpool::{Pool, Object}}, AsyncPgConnection};

pub type Connection = AsyncPgConnection;
pub type DbPool = Pool<Connection>;
pub type PooledConnection = Object<Connection>;

pub async fn get_connection_pool(url: String) -> Result<DbPool, ()> {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);
    Ok(Pool::builder(manager).max_size(5).build().unwrap())
}
