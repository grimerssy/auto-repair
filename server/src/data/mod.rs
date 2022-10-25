pub mod contacts;
pub mod orders;
pub mod services;
pub mod users;

use diesel_async::{
    pooled_connection::{
        deadpool::{Object, Pool},
        AsyncDieselConnectionManager,
    },
    AsyncPgConnection,
};

type Result<T> = std::result::Result<T, diesel::result::Error>;

pub type Connection = AsyncPgConnection;
pub type DbPool = Pool<Connection>;
pub type PooledConnection = Object<Connection>;

static TIMESTAMP_FORMAT: &str = "DD.MM.YYYY HH24:MI";
static DATE_FORMAT: &str = "DD.MM.YYYY";

pub async fn get_connection_pool(url: String) -> Result<DbPool> {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);
    Ok(Pool::builder(manager).max_size(5).build().unwrap())
}

pub mod timestamp {
    use diesel::{
        sql_function,
        sql_types::{Text, Timestamp},
    };

    sql_function! {
        fn to_char(t: Timestamp, f: Text) -> Text;
    }

    sql_function! {
        fn to_timestamp(t: Text, f: Text) -> Timestamp;
    }
}

pub mod date {
    use diesel::{
        sql_function,
        sql_types::{Date, Text},
    };

    sql_function! {
        fn to_char(t: Date, f: Text) -> Text;
    }

    sql_function! {
        fn to_date(t: Text, f: Text) -> Date;
    }
}
