use super::{date, time, Connection, Result};
use crate::models::{
    id::Id,
    worker::{InsertWorker, Worker},
};
use diesel::{delete, insert_into, prelude::*, update};
use diesel_async::RunQueryDsl;

pub async fn get_all(conn: &mut Connection) -> Result<Vec<Worker>> {
    use crate::schema::workers;

    workers::table
        .select((
            workers::id,
            workers::first_name,
            workers::middle_name,
            workers::last_name,
            date::to_char(workers::date_of_birth, date::FORMAT),
            time::to_char(workers::start_time, time::FORMAT),
            time::to_char(workers::end_time, time::FORMAT),
        ))
        .load::<Worker>(conn)
        .await
}

pub async fn get_by_id(worker_id: Id, conn: &mut Connection) -> Result<Worker> {
    use crate::schema::workers;
    workers::table
        .select((
            workers::id,
            workers::first_name,
            workers::middle_name,
            workers::last_name,
            date::to_char(workers::date_of_birth, date::FORMAT),
            time::to_char(workers::start_time, time::FORMAT),
            time::to_char(workers::end_time, time::FORMAT),
        ))
        .filter(workers::id.eq(worker_id))
        .first::<Worker>(conn)
        .await
}

pub async fn insert(worker: InsertWorker, conn: &mut Connection) -> Result<()> {
    use crate::schema::workers::dsl::*;

    insert_into(workers)
        .values((
            first_name.eq(worker.first_name),
            middle_name.eq(worker.middle_name),
            last_name.eq(worker.last_name),
            date_of_birth.eq(date::to_date(worker.date_of_birth, date::FORMAT)),
            start_time.eq(time::to_time(worker.start_time, time::FORMAT)),
            end_time.eq(time::to_time(worker.end_time, time::FORMAT)),
        ))
        .execute(conn)
        .await
        .map(|_| ())
}

pub async fn update_by_id(worker: Worker, conn: &mut Connection) -> Result<()> {
    use crate::schema::workers::dsl::*;

    update(workers.filter(id.eq(worker.id)))
        .set((
            first_name.eq(worker.first_name),
            middle_name.eq(worker.middle_name),
            last_name.eq(worker.last_name),
            date_of_birth.eq(date::to_date(worker.date_of_birth, date::FORMAT)),
            start_time.eq(time::to_time(worker.start_time, time::FORMAT)),
            end_time.eq(time::to_time(worker.end_time, time::FORMAT)),
        ))
        .execute(conn)
        .await
        .map(|_| ())
}

pub async fn delete_by_id(worker_id: Id, conn: &mut Connection) -> Result<()> {
    use crate::schema::workers::dsl::*;
    delete(workers.filter(id.eq(worker_id)))
        .execute(conn)
        .await
        .map(|_| ())
}
