use super::{Connection, Result};
use crate::models::{
    id::Id,
    service::{InsertService, Service},
};
use diesel::{delete, insert_into, prelude::*, update};
use diesel_async::RunQueryDsl;

pub async fn get_all(conn: &mut Connection) -> Result<Vec<Service>> {
    use crate::schema::services;

    services::table
        .order(services::id)
        .load::<Service>(conn)
        .await
}

pub async fn get_by_id(service_id: Id, conn: &mut Connection) -> Result<Service> {
    use crate::schema::services::dsl::*;
    services
        .filter(id.eq(service_id))
        .first::<Service>(conn)
        .await
}

pub async fn get_for_worker(worker_id: Id, conn: &mut Connection) -> Result<Vec<Service>> {
    use crate::schema::{services, specialties};
    services::table
        .select((
            services::id,
            services::title,
            services::price,
            services::duration,
        ))
        .inner_join(specialties::table.on(services::id.eq(specialties::service_id)))
        .filter(specialties::worker_id.eq(worker_id))
        .load::<Service>(conn)
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

pub async fn add_to_worker(worker_id: Id, service_id: Id, conn: &mut Connection) -> Result<()> {
    use crate::schema::specialties;

    insert_into(specialties::table)
        .values((
            specialties::service_id.eq(service_id),
            specialties::worker_id.eq(worker_id),
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
    delete(services.filter(id.eq(service_id)))
        .execute(conn)
        .await
        .map(|_| ())
}

pub async fn remove_for_worker(worker_id: Id, service_id: Id, conn: &mut Connection) -> Result<()> {
    use crate::schema::specialties;
    delete(
        specialties::table
            .filter(specialties::worker_id.eq(worker_id))
            .filter(specialties::service_id.eq(service_id)),
    )
    .execute(conn)
    .await
    .map(|_| ())
}
