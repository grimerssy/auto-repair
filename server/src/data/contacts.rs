use diesel::{prelude::*, insert_into, update};
use diesel_async::RunQueryDsl;
use diesel::result::Error;

use super::Connection;
use crate::models::{contact::{Contact, InsertContact}, id::Id};

pub async fn get_contact_id_by_pn_create_if_absent(contact: InsertContact, conn: &mut Connection)
-> Result<Id, Error>
{
        let contact_result = get_contact_by_phone_number(
            contact.phone_number.clone(), conn).await;

        match contact_result {
            Ok(mut db_contact) => {
                let id = db_contact.id;
                if contact.email.clone() != None {
                    db_contact.email = contact.email.clone();
                    update_contact_email(db_contact, conn).await?;
                }
                Ok(id)
            },
            Err(_) => {
                let insert_contact = InsertContact {
                    phone_number: contact.phone_number.clone(),
                    email: contact.email.clone(),
                };
                insert_contact_returning_id(insert_contact, conn).await
            }
        }
}

async fn get_contact_by_phone_number(phone: String, conn: &mut Connection)
-> Result<Contact, Error>
{
    use crate::schema::contacts::dsl::*;

    contacts
        .filter(phone_number.eq(phone))
        .first::<Contact>(conn)
        .await
}

async fn insert_contact_returning_id(contact: InsertContact, conn: &mut Connection)
-> Result<Id, Error>
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

async fn update_contact_email(contact: Contact, conn: &mut Connection)
-> Result<(), Error>
{
    use crate::schema::contacts::dsl::*;

    update(contacts.filter(id.eq(contact.id)))
        .set(email.eq(contact.email))
        .execute(conn)
        .await
        .map(|_| ())
}
