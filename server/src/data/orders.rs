use diesel_async::RunQueryDsl;
use diesel::{prelude::*, insert_into, result::Error};
use super::{Connection, TIMESTAMP_FORMAT, to_char, to_timestamp};
use crate::models::{order::{InsertOrder, Order}, id::Id};

pub async fn insert_order(order: InsertOrder, conn: &mut Connection) -> Result<(), Error> {
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

pub async fn get_orders_by_contact_id(contact_id: Id, conn: &mut Connection)
-> Result<Vec<Order>, Error>
{
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

pub async fn get_all_orders(conn: &mut Connection)
-> Result<Vec<Order>, Error>
{
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
