use super::{Connection, Result};
use crate::models::{
    id::Id,
    service::{InsertService, Service},
};
use diesel::{delete, insert_into, prelude::*, update};
use diesel_async::RunQueryDsl;

pub async fn get_all(conn: &mut Connection) -> Result<Vec<Service>> {
    crate::schema::services::table.load::<Service>(conn).await
}

pub async fn get_by_id(service_id: Id, conn: &mut Connection) -> Result<Service> {
    use crate::schema::services::dsl::*;
    services
        .filter(id.eq(service_id))
        .first::<Service>(conn)
        .await
}

pub async fn insert(service: InsertService, conn: &mut Connection) -> Result<()> {
    use crate::schema::services::dsl::*;

    insert_into(services)
        .values((
            title.eq(service.title),
            price.eq(service.price),
            duration.eq(service.duration),
        ))
        .execute(conn)
        .await
        .map(|_| ())
}

pub async fn update_by_id(service: Service, conn: &mut Connection) -> Result<()> {
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

pub async fn delete_by_id(service_id: Id, conn: &mut Connection) -> Result<()> {
    use crate::schema::services::dsl::*;
    let res = delete(services.filter(id.eq(service_id)))
        .execute(conn)
        .await
        .map(|_| ());
    println!("result: {:?}", res);
    res
}
