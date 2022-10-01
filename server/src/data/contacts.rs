use diesel::{prelude::*, insert_into, update};
use diesel_async::RunQueryDsl;
use diesel::result::Error;

use crate::{models::contact::{Contact, InsertContact}, Connection};

pub async fn get_contact_by_phone_number(phone: String, conn: &mut Connection)
-> Result<Contact, Error>
{
    use crate::schema::contacts::dsl::*;

    contacts
        .filter(phone_number.eq(phone))
        .first::<Contact>(conn)
        .await
}

pub async fn insert_contact_returning_id(contact: InsertContact, conn: &mut Connection)
-> Result<i32, Error>
{
    use crate::schema::contacts::dsl::*;

    let contact = insert_into(contacts)
        .values((
            phone_number.eq(contact.phone_number),
            email.eq(contact.email)
        ))
        .get_result::<Contact>(conn)
        .await?;

    Ok(contact.id)
}

pub async fn update_contact_email(contact: Contact, conn: &mut Connection)
-> Result<(), Error>
{
    use crate::schema::contacts::dsl::*;

    update(contacts.filter(id.eq(contact.id)))
        .set(email.eq(contact.email))
        .execute(conn)
        .await
        .map(|_| ())
}
