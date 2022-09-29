use diesel_async::{pooled_connection::{AsyncDieselConnectionManager, deadpool::Pool}, AsyncPgConnection};

use crate::DbPool;

pub async fn get_connection_pool(url: String) -> Result<DbPool, ()> {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);
    Ok(Pool::builder(manager).max_size(5).build().unwrap())
}
