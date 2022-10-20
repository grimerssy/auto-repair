use super::{Result, Connection, date, timestamp, DATE_FORMAT};
use crate::{
    data::TIMESTAMP_FORMAT,
    models::{user::{InsertUser, RawUser}, id::Id}
};
use diesel::{prelude::*, insert_into};
use diesel_async::RunQueryDsl;

pub async fn insert(user: InsertUser, conn: &mut Connection) -> Result<()> {
    use crate::schema::users::dsl::*;

    insert_into(users)
        .values((
            password_hash.eq(user.password_hash),
            role.eq(user.role),
            contact_id.eq(user.contact_id),
            first_name.eq(user.first_name),
            middle_name.eq(user.middle_name),
            last_name.eq(user.last_name),
            date_of_birth.eq(date::to_date(user.date_of_birth, DATE_FORMAT)),
        ))
        .execute(conn)
        .await
        .map(|_| ())
}

pub async fn get_by_contact_id(contact_id: Id, conn: &mut Connection) -> Result<RawUser> {
    use crate::schema::users;

    users::table
        .select((
            users::contact_id,
            users::password_hash,
            users::role,
            users::first_name,
            users::middle_name,
            users::last_name,
            date::to_char(users::date_of_birth, DATE_FORMAT),
            timestamp::to_char(users::registered_at, TIMESTAMP_FORMAT),
        ))
        .filter(users::contact_id.eq(contact_id))
        .first::<RawUser>(conn)
        .await
}
