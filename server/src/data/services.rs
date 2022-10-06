use diesel::prelude::*;
use diesel::result::Error;
use diesel_async::RunQueryDsl;
use super::Connection;
use crate::models::id::Id;
use crate::models::service::Service;

pub async fn get_all_services(conn: &mut Connection) -> Result<Vec<Service>, Error> {
    use crate::schema::services::dsl::*;

    services.load::<Service>(conn).await
}

pub async fn get_service_by_id(service_id: Id, conn: &mut Connection) -> Result<Service, Error> {
    use crate::schema::services::dsl::*;

    services.filter(id.eq(service_id)).first::<Service>(conn).await
}
