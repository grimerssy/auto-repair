use super::{Result, Connection, TIMESTAMP_FORMAT, timestamp::{to_char, to_timestamp}};
use crate::models::{order::{InsertOrder, Order}, id::Id};
use diesel_async::RunQueryDsl;
use diesel::{prelude::*, insert_into};

pub async fn insert(order: InsertOrder, conn: &mut Connection) -> Result<()> {
    use crate::schema::orders::dsl::*;

    insert_into(orders)
        .values((
            contact_id.eq(order.contact_id),
            service_id.eq(order.service_id),
            start_time.eq(to_timestamp(order.start_time, TIMESTAMP_FORMAT)),
            car_make.eq(order.car_make),
            car_model.eq(order.car_model),
            car_year.eq(order.car_year),
        ))
        .execute(conn)
        .await
        .map(|_| ())
}

pub async fn get_all(conn: &mut Connection) -> Result<Vec<Order>> {
    use crate::schema::*;

    orders::table
        .inner_join(contacts::table.on(orders::contact_id.eq(contacts::id)))
        .inner_join(services::table.on(orders::service_id.eq(services::id)))
        .select((
            orders::id,
            (contacts::id, contacts::phone_number, contacts::email),
            (services::id, services::title, services::price, services::duration),
            to_char(orders::start_time, TIMESTAMP_FORMAT),
            orders::car_make,
            orders::car_model,
            orders::car_year))
            .load::<Order>(conn)
            .await
}

pub async fn get_by_service_id(service_id: Id, conn: &mut Connection,
) -> Result<Vec<Order>> {
    use crate::schema::*;

    orders::table
        .inner_join(contacts::table.on(orders::contact_id.eq(contacts::id)))
        .inner_join(services::table.on(orders::service_id.eq(services::id)))
        .select((
            orders::id,
            (contacts::id, contacts::phone_number, contacts::email),
            (services::id, services::title, services::price, services::duration),
            to_char(orders::start_time, TIMESTAMP_FORMAT),
            orders::car_make,
            orders::car_model,
            orders::car_year))
            .filter(orders::service_id.eq(service_id))
            .load::<Order>(conn)
            .await
}

pub async fn get_by_contact_id(contact_id: Id, conn: &mut Connection,
) -> Result<Vec<Order>> {
    use crate::schema::*;

    orders::table
        .inner_join(contacts::table.on(orders::contact_id.eq(contacts::id)))
        .inner_join(services::table.on(orders::service_id.eq(services::id)))
        .select((
            orders::id,
            (contacts::id, contacts::phone_number, contacts::email),
            (services::id, services::title, services::price, services::duration),
            to_char(orders::start_time, TIMESTAMP_FORMAT),
            orders::car_make,
            orders::car_model,
            orders::car_year))
            .filter(orders::contact_id.eq(contact_id))
            .load::<Order>(conn)
            .await
}
