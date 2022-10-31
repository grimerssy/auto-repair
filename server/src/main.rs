mod api;
mod data;
mod errors;
mod models;
mod routing;
mod schema;

use actix_cors::Cors;
use actix_web::{middleware, web::Data, App, HttpServer};
use data::get_connection_pool;
use models::id::keys::{Key, Keys};
use routing::configuration;
use std::env;

#[derive(Clone, Copy)]
pub struct BcryptCfg {
    cost: u32,
}

#[derive(Clone)]
pub struct JwtCfg {
    access_sec_ttl: i64,
    secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let client_host = env::var("CLIENT_HOST").unwrap();
    let database_url = env::var("DATABASE_URL").unwrap();
    let bcrypt_cfg = BcryptCfg {
        cost: env::var("BCRYPT_COST")
            .unwrap_or_else(|_| bcrypt::DEFAULT_COST.to_string())
            .parse::<u32>()
            .unwrap(),
    };
    let jwt_cfg = JwtCfg {
        access_sec_ttl: env::var("ACCESS_SECONDS_TTL")
            .unwrap()
            .parse::<i64>()
            .unwrap(),
        secret: env::var("JWT_SECRET").unwrap(),
    };
    let keys = Keys {
        contacts: Key::new(
            env::var("CONTACTS_PRIME").unwrap().parse::<i32>().unwrap(),
            env::var("CONTACTS_RANDOM").unwrap().parse::<i32>().unwrap(),
        ),
        orders: Key::new(
            env::var("ORDERS_PRIME").unwrap().parse::<i32>().unwrap(),
            env::var("ORDERS_RANDOM").unwrap().parse::<i32>().unwrap(),
        ),
        services: Key::new(
            env::var("SERVICES_PRIME").unwrap().parse::<i32>().unwrap(),
            env::var("SERVICES_RANDOM").unwrap().parse::<i32>().unwrap(),
        ),
    };

    HttpServer::new(move || {
        let database_url = database_url.clone();

        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(&client_host.clone())
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allow_any_header(),
            )
            .wrap(middleware::Logger::default())
            .app_data(Data::new(keys))
            .app_data(Data::new(jwt_cfg.clone()))
            .app_data(Data::new(bcrypt_cfg))
            .data_factory(move || get_connection_pool(database_url.clone()))
            .configure(configuration)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
