use super::{date, time, timestamp, Connection, Result};
use crate::models::{
    id::Id,
    order::{InsertOrder, Order},
};
use diesel::{delete, insert_into, prelude::*, update};
use diesel_async::RunQueryDsl;

pub async fn insert(order: InsertOrder, conn: &mut Connection) -> Result<()> {
    use crate::schema::*;

    insert_into(orders::table)
        .values((
            orders::specialty_id.eq(order.specialty_id),
            orders::car_vin.eq(order.car_vin),
            orders::start_time.eq(timestamp::to_timestamp(order.start_time, timestamp::FORMAT)),
        ))
        .execute(conn)
        .await
        .map(|_| ())
}

pub async fn get_all(conn: &mut Connection) -> Result<Vec<Order>> {
    use crate::schema::*;

    orders::table
        .inner_join(specialties::table.on(orders::specialty_id.eq(specialties::id)))
        .inner_join(services::table.on(specialties::service_id.eq(services::id)))
        .inner_join(workers::table.on(specialties::worker_id.eq(workers::id)))
        .inner_join(cars::table.on(orders::car_vin.eq(cars::vin)))
        .inner_join(contacts::table.on(cars::contact_id.eq(contacts::id)))
        .select((
            orders::id,
            (
                services::id,
                services::title,
                services::price,
                services::duration,
            ),
            (
                workers::id,
                workers::first_name,
                workers::middle_name,
                workers::last_name,
                date::to_char(workers::date_of_birth, date::FORMAT),
                time::to_char(workers::start_time, time::FORMAT),
                time::to_char(workers::end_time, time::FORMAT),
            ),
            (
                cars::vin,
                (contacts::id, contacts::phone_number, contacts::email),
                cars::make,
                cars::model,
                cars::year,
            ),
            timestamp::to_char(orders::start_time, timestamp::FORMAT),
        ))
        .load::<Order>(conn)
        .await
}

pub async fn get_by_service_id(service_id: Id, conn: &mut Connection) -> Result<Vec<Order>> {
    use crate::schema::*;

    orders::table
        .inner_join(specialties::table.on(orders::specialty_id.eq(specialties::id)))
        .inner_join(services::table.on(specialties::service_id.eq(services::id)))
        .inner_join(workers::table.on(specialties::worker_id.eq(workers::id)))
        .inner_join(cars::table.on(orders::car_vin.eq(cars::vin)))
        .inner_join(contacts::table.on(cars::contact_id.eq(contacts::id)))
        .select((
            orders::id,
            (
                services::id,
                services::title,
                services::price,
                services::duration,
            ),
            (
                workers::id,
                workers::first_name,
                workers::middle_name,
                workers::last_name,
                date::to_char(workers::date_of_birth, date::FORMAT),
                time::to_char(workers::start_time, time::FORMAT),
                time::to_char(workers::end_time, time::FORMAT),
            ),
            (
                cars::vin,
                (contacts::id, contacts::phone_number, contacts::email),
                cars::make,
                cars::model,
                cars::year,
            ),
            timestamp::to_char(orders::start_time, timestamp::FORMAT),
        ))
        .filter(services::id.eq(service_id))
        .load::<Order>(conn)
        .await
}

pub async fn get_by_id(id: Id, conn: &mut Connection) -> Result<Order> {
    use crate::schema::*;

    orders::table
        .inner_join(specialties::table.on(orders::specialty_id.eq(specialties::id)))
        .inner_join(services::table.on(specialties::service_id.eq(services::id)))
        .inner_join(workers::table.on(specialties::worker_id.eq(workers::id)))
        .inner_join(cars::table.on(orders::car_vin.eq(cars::vin)))
        .inner_join(contacts::table.on(cars::contact_id.eq(contacts::id)))
        .select((
            orders::id,
            (
                services::id,
                services::title,
                services::price,
                services::duration,
            ),
            (
                workers::id,
                workers::first_name,
                workers::middle_name,
                workers::last_name,
                date::to_char(workers::date_of_birth, date::FORMAT),
                time::to_char(workers::start_time, time::FORMAT),
                time::to_char(workers::end_time, time::FORMAT),
            ),
            (
                cars::vin,
                (contacts::id, contacts::phone_number, contacts::email),
                cars::make,
                cars::model,
                cars::year,
            ),
            timestamp::to_char(orders::start_time, timestamp::FORMAT),
        ))
        .filter(orders::id.eq(id))
        .first::<Order>(conn)
        .await
}

pub async fn update_by_id(id: Id, order: InsertOrder, conn: &mut Connection) -> Result<()> {
    use crate::schema::orders;

    update(orders::table.filter(orders::id.eq(id)))
        .set((
            orders::specialty_id.eq(order.specialty_id),
            orders::car_vin.eq(order.car_vin),
            orders::start_time.eq(timestamp::to_timestamp(order.start_time, timestamp::FORMAT)),
        ))
        .execute(conn)
        .await
        .map(|_| ())
}

pub async fn delete_by_id(id: Id, conn: &mut Connection) -> Result<()> {
    use crate::schema::orders;

    delete(orders::table.filter(orders::id.eq(id)))
        .execute(conn)
        .await
        .map(|_| ())
}
