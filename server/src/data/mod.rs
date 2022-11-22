pub mod cars;
pub mod contacts;
pub mod orders;
pub mod services;
pub mod users;
pub mod workers;

pub mod reports;

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

pub async fn get_connection_pool(url: String) -> Result<DbPool> {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);
    Ok(Pool::builder(manager).max_size(5).build().unwrap())
}

pub fn sql_to_chrono_format(sql_format: &str) -> String {
    let result = sql_format.replace("DD", "%d");
    let result = &result.replace("MM", "%m");
    let result = &result.replace("YYYY", "%Y");
    let result = &result.replace("HH24", "%H");
    let result = &result.replace("MI", "%M");
    result.to_owned()
}

pub mod date {
    use diesel::{
        sql_function,
        sql_types::{Date, Text},
    };

    pub static FORMAT: &str = "DD.MM.YYYY";

    sql_function! {
        fn to_char(t: Date, f: Text) -> Text;
    }

    sql_function! {
        fn to_date(t: Text, f: Text) -> Date;
    }
}

pub mod time {
    use diesel::{
        sql_function,
        sql_types::{Text, Time},
    };

    pub static FORMAT: &str = "HH24:MI";

    sql_function! {
        fn to_char(t: Time, f: Text) -> Text;
    }

    sql_function! {
        fn to_timestamp(t: Text, f: Text) -> Time;
    }
}

pub mod timestamp {
    use diesel::{
        sql_function,
        sql_types::{Text, Timestamp},
    };

    pub static FORMAT: &str = "DD.MM.YYYY HH24:MI";

    sql_function! {
        fn to_char(t: Timestamp, f: Text) -> Text;
    }

    sql_function! {
        fn to_timestamp(t: Text, f: Text) -> Timestamp;
    }
}
