use super::{Connection, Result};
use crate::models::{
    car::{Car, InsertCar},
    id::Id,
};
use diesel::{delete, insert_into, prelude::*, update};
use diesel_async::RunQueryDsl;

pub async fn get_all(conn: &mut Connection) -> Result<Vec<Car>> {
    use crate::schema::{cars, contacts};

    cars::table
        .inner_join(contacts::table.on(cars::contact_id.eq(contacts::id)))
        .select((
            cars::vin,
            (contacts::id, contacts::phone_number, contacts::email),
            cars::make,
            cars::model,
            cars::year,
        ))
        .load::<Car>(conn)
        .await
}

pub async fn get_by_contact_id(contact_id: Id, conn: &mut Connection) -> Result<Vec<Car>> {
    use crate::schema::{cars, contacts};

    cars::table
        .inner_join(contacts::table.on(cars::contact_id.eq(contacts::id)))
        .select((
            cars::vin,
            (contacts::id, contacts::phone_number, contacts::email),
            cars::make,
            cars::model,
            cars::year,
        ))
        .filter(contacts::id.eq(contact_id))
        .load::<Car>(conn)
        .await
}

pub async fn get_by_vin(car_vin: String, conn: &mut Connection) -> Result<Car> {
    use crate::schema::{cars, contacts};

    cars::table
        .inner_join(contacts::table.on(cars::contact_id.eq(contacts::id)))
        .select((
            cars::vin,
            (contacts::id, contacts::phone_number, contacts::email),
            cars::make,
            cars::model,
            cars::year,
        ))
        .filter(cars::vin.eq(car_vin))
        .first::<Car>(conn)
        .await
}

pub async fn insert(car: InsertCar, conn: &mut Connection) -> Result<()> {
    use crate::schema::cars::dsl::*;

    insert_into(cars)
        .values((
            vin.eq(car.vin),
            contact_id.eq(car.contact_id),
            make.eq(car.make),
            model.eq(car.model),
            year.eq(car.year),
        ))
        .execute(conn)
        .await
        .map(|_| ())
}

pub async fn update_by_vin(vin: String, car: InsertCar, conn: &mut Connection) -> Result<()> {
    use crate::schema::cars;

    update(cars::table.filter(cars::vin.eq(vin)))
        .set((
            cars::vin.eq(car.vin),
            cars::make.eq(car.make),
            cars::model.eq(car.model),
            cars::year.eq(car.year),
        ))
        .execute(conn)
        .await
        .map(|_| ())
}

pub async fn delete_by_vin(car_vin: String, conn: &mut Connection) -> Result<()> {
    use crate::schema::cars::dsl::*;
    delete(cars.filter(vin.eq(car_vin)))
        .execute(conn)
        .await
        .map(|_| ())
}
