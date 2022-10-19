use super::{Result, Connection};
use crate::models::{id::Id, service::Service};
use diesel::{prelude::*, update};
use diesel_async::RunQueryDsl;

pub async fn get_all_services(conn: &mut Connection) -> Result<Vec<Service>> {
    crate::schema::services::table.load::<Service>(conn).await
}

pub async fn get_service_by_id(service_id: Id, conn: &mut Connection) -> Result<Service> {
    use crate::schema::services::dsl::*;
    services.filter(id.eq(service_id)).first::<Service>(conn).await
}

pub async fn update_service_by_id(service: Service, conn: &mut Connection,
) -> Result<()> {
    use crate::schema::services::dsl::*;

    update(services.filter(id.eq(service.id)))
        .set((
            title.eq(service.title),
            price.eq(service.price),
            duration.eq(service.duration),
        ))
        .execute(conn)
        .await
        .map(|_| ())
}
