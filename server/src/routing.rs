use crate::api::*;
use actix_web::web;

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/orders")
                    .service(orders::create)
                    .service(orders::get_all)
                    .service(orders::get_by_service_id),
            )
            .service(
                web::scope("/services")
                    .service(services::create)
                    .service(services::get_all)
                    .service(services::get_by_id)
                    .service(services::delete_by_id)
                    .service(services::update_by_id),
            )
            .service(
                web::scope("/auth")
                    .service(users::signup)
                    .service(users::login),
            ),
    );
}
