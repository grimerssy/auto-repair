mod api;
mod data;
mod models;
mod errors;
mod schema;
mod routing;

use actix_cors::Cors;
use data::get_connection_pool;
use models::id::keys::{Keys, Key};
use std::env;
use routing::configuration;
use actix_web::{HttpServer, App, web::Data};

#[derive(Clone, Copy)]
pub struct BcryptCfg {
    cost: u32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let client_host = env::var("CLIENT_HOST").unwrap();
    let database_url = env::var("DATABASE_URL").unwrap();
    let bcrypt_cfg = BcryptCfg{
        cost: env::var("BCRYPT_COST")
            .unwrap_or(bcrypt::DEFAULT_COST.to_string())
            .parse::<u32>()
            .unwrap()
    };
    let keys = Keys{
        contacts: Key::new(
            env::var("CONTACTS_PRIME").unwrap().parse::<i32>().unwrap(),
            env::var("CONTACTS_RANDOM").unwrap().parse::<i32>().unwrap()
        ),
        orders: Key::new(
            env::var("ORDERS_PRIME").unwrap().parse::<i32>().unwrap(),
            env::var("ORDERS_RANDOM").unwrap().parse::<i32>().unwrap()
        ),
        services: Key::new(
            env::var("SERVICES_PRIME").unwrap().parse::<i32>().unwrap(),
            env::var("SERVICES_RANDOM").unwrap().parse::<i32>().unwrap()
        ),
    };

    HttpServer::new(move || {
        let database_url = database_url.clone();

        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(&client_host.clone())
                    .allowed_methods(vec!["GET", "POST"])
                    .allow_any_header()
            )
            .app_data(Data::new(keys))
            .app_data(Data::new(bcrypt_cfg))
            .data_factory(move || get_connection_pool(database_url.clone()))
            .configure(configuration)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
