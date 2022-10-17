use diesel::{prelude::*, insert_into};
use diesel_async::RunQueryDsl;
use diesel::result::Error;

use crate::models::user::InsertUser;

use super::{Connection, to_date, DATE_FORMAT};

pub async fn insert_user(user: InsertUser, conn: &mut Connection) -> Result<(), Error> {
    use crate::schema::users::dsl::*;

    insert_into(users)
        .values((
            password_hash.eq(user.password_hash),
            contact_id.eq(user.contact_id),
            first_name.eq(user.first_name),
            middle_name.eq(user.middle_name),
            last_name.eq(user.last_name),
            date_of_birth.eq(to_date(user.date_of_birth, DATE_FORMAT)),
        ))
        .execute(conn)
        .await
        .map(|_| ())
}
