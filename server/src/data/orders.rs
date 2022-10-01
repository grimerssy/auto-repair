use diesel_async::RunQueryDsl;
use diesel::{prelude::*, insert_into, result::Error};
use crate::{models::order::InsertOrder, Connection};

pub async fn insert_order(order: InsertOrder, conn: &mut Connection) -> Result<(), Error> {
    use crate::schema::orders::dsl::*;

    insert_into(orders)
        .values((
            contact_id.eq(order.contact_id),
            service_id.eq(order.service_id),
            start_time.eq(order.start_time),
            car_make.eq(order.car_make),
            car_model.eq(order.car_model),
            car_year.eq(order.car_year),
        ))
        .execute(conn)
        .await
        .map(|_| ())
}
