use crate::api::*;
use actix_web::web;

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("orders")
                    .service(orders::make_order)
                    .service(orders::get_all),
            )
            .service(
                web::scope("/services")
                    .service(services::get_all)
                    .service(services::get_by_id),
            ),
    );
}
