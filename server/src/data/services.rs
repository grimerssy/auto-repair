use diesel::result::Error;
use diesel_async::RunQueryDsl;
use crate::{Connection, models::service::Service};

pub async fn get_all_services(conn: &mut Connection) -> Result<Vec<Service>, Error> {
    use crate::schema::services::dsl::*;

    services.load::<Service>(conn).await
}
