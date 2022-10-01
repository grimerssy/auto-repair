mod api;
mod data;
mod models;
mod errors;
mod schema;
mod routing;

use data::get_connection_pool;
use std::env;
use diesel_async::{pooled_connection::deadpool::{Object, Pool}, AsyncPgConnection};
use routing::configuration;
use actix_web::{HttpServer, App};

type Connection = AsyncPgConnection;
type DbPool = Pool<Connection>;
type PooledConnection = Object<Connection>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap();

    HttpServer::new(move || {
        let database_url = database_url.clone();

        App::new()
            .data_factory(move || get_connection_pool(database_url.clone()))
            .configure(configuration)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
