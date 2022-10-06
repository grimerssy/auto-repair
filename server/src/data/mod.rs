pub mod orders;
pub mod contacts;
pub mod services;

use diesel::{sql_function, sql_types::{Timestamp, Text}};
use diesel_async::{pooled_connection::{AsyncDieselConnectionManager, deadpool::{Pool, Object}}, AsyncPgConnection};

pub type Connection = AsyncPgConnection;
pub type DbPool = Pool<Connection>;
pub type PooledConnection = Object<Connection>;

static TIMESTAMP_FORMAT: &str = "DD.MM.YYYY HH24:MI";

sql_function! {
    fn to_char(t: Timestamp, f: Text) -> Text;
}

sql_function! {
    fn to_timestamp(t: Text, f: Text) -> Timestamp;
}

pub async fn get_connection_pool(url: String) -> Result<DbPool, ()> {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);
    Ok(Pool::builder(manager).max_size(5).build().unwrap())
}
