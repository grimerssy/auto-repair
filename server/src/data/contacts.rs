use super::{Connection, Result};
use crate::models::{
    contact::{Contact, InsertContact},
    id::Id,
};
use diesel::{insert_into, prelude::*, update};
use diesel_async::RunQueryDsl;

pub async fn get_id_by_phone_number(phone_number: String, conn: &mut Connection) -> Result<Id> {
    use crate::schema::contacts;

    contacts::table
        .select(contacts::id)
        .filter(contacts::phone_number.eq(phone_number))
        .first::<Id>(conn)
        .await
}

pub async fn get_id_by_email(email: String, conn: &mut Connection) -> Result<Id> {
    use crate::schema::contacts;

    contacts::table
        .select(contacts::id)
        .filter(contacts::email.eq(email))
        .first::<Id>(conn)
        .await
}

pub async fn get_id_by_phone_number_create_if_absent(
    contact: InsertContact,
    conn: &mut Connection,
) -> Result<Id> {
    let contact_result = get_by_phone_number(contact.phone_number.clone(), conn).await;

    match contact_result {
        Ok(mut db_contact) => {
            let id = db_contact.id;
            if contact.email.clone() != None {
                db_contact.email = contact.email.clone();
                update_email(db_contact, conn).await?;
            }
            Ok(id)
        }
        Err(_) => {
            let insert_contact = InsertContact {
                phone_number: contact.phone_number.clone(),
                email: contact.email.clone(),
            };
            insert_returning_id(insert_contact, conn).await
        }
    }
}

async fn get_by_phone_number(phone: String, conn: &mut Connection) -> Result<Contact> {
    use crate::schema::contacts::dsl::*;

    contacts
        .filter(phone_number.eq(phone))
        .first::<Contact>(conn)
        .await
}

async fn insert_returning_id(contact: InsertContact, conn: &mut Connection) -> Result<Id> {
    use crate::schema::contacts::dsl::*;

    let contact = insert_into(contacts)
        .values((
            phone_number.eq(contact.phone_number),
            email.eq(contact.email),
        ))
        .get_result::<Contact>(conn)
        .await?;

    Ok(contact.id)
}

async fn update_email(contact: Contact, conn: &mut Connection) -> Result<()> {
    use crate::schema::contacts::dsl::*;

    update(contacts.filter(id.eq(contact.id)))
        .set(email.eq(contact.email))
        .execute(conn)
        .await
        .map(|_| ())
}
