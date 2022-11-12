use std::collections::{HashMap, HashSet, VecDeque};

use super::{date, sql_to_chrono_format, time, timestamp, Connection, Result};
use crate::models::{
    id::Id,
    order::{InsertOrder, Order},
};
use chrono::DurationRound;
use diesel::{
    delete,
    dsl::{count, IntervalDsl},
    insert_into,
    prelude::*,
    update,
};
use diesel_async::RunQueryDsl;

#[derive(Queryable, Clone, Copy, Debug)]
struct WorkerInfo {
    id: Id,
    specialty_count: i64,
    start_time: chrono::NaiveTime,
    end_time: chrono::NaiveTime,
}

#[derive(Queryable, Debug)]
struct OrderInfo {
    start_time: chrono::NaiveDateTime,
    duration: chrono::NaiveTime,
}

pub async fn get_available_time(
    service_id: Id,
    conn: &mut Connection,
) -> Result<Vec<(String, Vec<String>)>> {
    let service_duration = get_service_duration_by_id(service_id, conn).await?;
    let worker_info = get_worker_info_by_service_id(service_id, conn).await?;
    let mut results = HashSet::<chrono::NaiveDateTime>::new();
    for wi in worker_info {
        let worker_timestamps = get_available_time_for_worker(service_duration, wi, conn).await?;
        results = results.union(&worker_timestamps).copied().collect();
    }
    let mut hashmap = HashMap::<chrono::NaiveDate, Vec<chrono::NaiveTime>>::new();
    results.into_iter().for_each(|r| {
        hashmap
            .entry(r.date())
            .or_insert_with(Vec::new)
            .push(r.time());
    });
    let date_format = &sql_to_chrono_format(date::FORMAT);
    let time_format = &sql_to_chrono_format(time::FORMAT);
    let mut tuples: Vec<(String, Vec<String>)> = hashmap
        .into_iter()
        .map(|(d, mut tt)| {
            tt.sort();
            (
                d.format(date_format).to_string(),
                tt.into_iter()
                    .map(|t| t.format(time_format).to_string())
                    .collect(),
            )
        })
        .collect();
    tuples.sort_by_key(|(x, _)| x.to_owned());
    Ok(tuples)
}

pub async fn get_specialty_id(
    service_id: Id,
    start_time: String,
    conn: &mut Connection,
) -> Result<Id> {
    use crate::schema::specialties;
    let datetime_format = format!(
        "{} {}",
        &sql_to_chrono_format(date::FORMAT),
        &sql_to_chrono_format(time::FORMAT),
    );
    let timestamp = chrono::NaiveDateTime::parse_from_str(&start_time, &datetime_format).unwrap();
    let service_duration = get_service_duration_by_id(service_id, conn).await?;
    let worker_info = get_worker_info_by_service_id(service_id, conn).await?;
    let mut chosen_worker: Option<WorkerInfo> = None;
    for wi in worker_info {
        let available_time = get_available_time_for_worker(service_duration, wi, conn).await?;
        if !available_time.contains(&timestamp) {
            continue;
        }
        if chosen_worker.is_none() || chosen_worker.unwrap().specialty_count > wi.specialty_count {
            chosen_worker = Some(wi);
        }
    }
    specialties::table
        .select(specialties::id)
        .filter(specialties::service_id.eq(service_id))
        .filter(specialties::worker_id.eq(chosen_worker.unwrap().id))
        .first::<Id>(conn)
        .await
}

async fn get_available_time_for_worker(
    service_duration: chrono::Duration,
    worker_info: WorkerInfo,
    conn: &mut Connection,
) -> Result<HashSet<chrono::NaiveDateTime>> {
    use crate::schema::{orders, services, specialties};
    let now = chrono::offset::Utc::now().naive_utc();
    let now_ceil = (now + chrono::Duration::seconds(15 * 60 / 2))
        .duration_round(chrono::Duration::minutes(15))
        .unwrap();
    let mut worker_timestamps = VecDeque::<chrono::NaiveDateTime>::new();
    for i in 0..7 {
        let current_date = now.date() + chrono::Duration::days(i);
        if current_date.and_time(worker_info.end_time) < now_ceil {
            continue;
        }
        worker_timestamps.push_back(current_date.and_time(worker_info.start_time).max(now_ceil));
        worker_timestamps.push_back(current_date.and_time(worker_info.end_time));
    }
    let order_info = orders::table
        .inner_join(specialties::table.on(orders::specialty_id.eq(specialties::id)))
        .inner_join(services::table.on(specialties::service_id.eq(services::id)))
        .select((orders::start_time, services::duration))
        .filter(specialties::worker_id.eq(worker_info.id))
        .filter(orders::start_time.gt(diesel::dsl::now))
        .filter(orders::start_time.lt(diesel::dsl::now + 7.days()))
        .order(orders::start_time)
        .load::<OrderInfo>(conn)
        .await?;

    let mut timestamps = Vec::<chrono::NaiveDateTime>::new();
    for oi in order_info {
        while !worker_timestamps.is_empty() && *worker_timestamps.front().unwrap() < oi.start_time {
            timestamps.push(worker_timestamps.pop_front().unwrap());
        }
        timestamps.push(oi.start_time);
        let end_time = oi.start_time
            + oi.duration
                .signed_duration_since(chrono::NaiveTime::default());
        timestamps.push(end_time);
    }
    while !worker_timestamps.is_empty() {
        timestamps.push(worker_timestamps.pop_front().unwrap());
    }
    let mut results = HashSet::<chrono::NaiveDateTime>::new();
    for i in 0..(timestamps.len() / 2) {
        let start = timestamps[i * 2];
        let end = timestamps[i * 2 + 1];
        let time_gap = end - start;
        if time_gap < service_duration {
            continue;
        }
        let diff_minutes = (time_gap - service_duration).num_minutes();
        for j in 0..=(diff_minutes / 15) {
            let ts = start + chrono::Duration::minutes(15 * j);
            results.insert(ts);
        }
    }
    Ok(results)
}

async fn get_worker_info_by_service_id(
    service_id: Id,
    conn: &mut Connection,
) -> Result<Vec<WorkerInfo>> {
    use crate::schema::{specialties, workers};
    let test = diesel::alias!(specialties as test);
    workers::table
        .inner_join(specialties::table.on(workers::id.eq(specialties::worker_id)))
        .filter(
            workers::id.eq_any(
                test.select(test.field(crate::schema::specialties::worker_id))
                    .filter(
                        test.field(crate::schema::specialties::service_id)
                            .eq(service_id),
                    ),
            ),
        )
        .group_by((workers::id, workers::start_time, workers::end_time))
        .select((
            workers::id,
            count(specialties::service_id),
            workers::start_time,
            workers::end_time,
        ))
        .load::<WorkerInfo>(conn)
        .await
}

async fn get_service_duration_by_id(
    service_id: Id,
    conn: &mut Connection,
) -> Result<chrono::Duration> {
    use crate::schema::services;
    services::table
        .select(services::duration)
        .filter(services::id.eq(service_id))
        .first::<chrono::NaiveTime>(conn)
        .await
        .map(|d| d.signed_duration_since(chrono::NaiveTime::default()))
}

pub async fn insert(order: InsertOrder, conn: &mut Connection) -> Result<()> {
    use crate::schema::*;

    insert_into(orders::table)
        .values((
            orders::specialty_id.eq(order.specialty_id),
            orders::car_vin.eq(order.car_vin),
            orders::start_time.eq(timestamp::to_timestamp(order.start_time, timestamp::FORMAT)),
        ))
        .execute(conn)
        .await
        .map(|_| ())
}

pub async fn get_all(conn: &mut Connection) -> Result<Vec<Order>> {
    use crate::schema::*;

    orders::table
        .inner_join(specialties::table.on(orders::specialty_id.eq(specialties::id)))
        .inner_join(services::table.on(specialties::service_id.eq(services::id)))
        .inner_join(workers::table.on(specialties::worker_id.eq(workers::id)))
        .inner_join(cars::table.on(orders::car_vin.eq(cars::vin)))
        .inner_join(contacts::table.on(cars::contact_id.eq(contacts::id)))
        .select((
            orders::id,
            (
                services::id,
                services::title,
                services::price,
                services::duration,
            ),
            (
                workers::id,
                workers::first_name,
                workers::middle_name,
                workers::last_name,
                date::to_char(workers::date_of_birth, date::FORMAT),
                time::to_char(workers::start_time, time::FORMAT),
                time::to_char(workers::end_time, time::FORMAT),
            ),
            (
                cars::vin,
                (contacts::id, contacts::phone_number, contacts::email),
                cars::make,
                cars::model,
                cars::year,
            ),
            timestamp::to_char(orders::start_time, timestamp::FORMAT),
        ))
        .load::<Order>(conn)
        .await
}

pub async fn get_by_service_id(service_id: Id, conn: &mut Connection) -> Result<Vec<Order>> {
    use crate::schema::*;

    orders::table
        .inner_join(specialties::table.on(orders::specialty_id.eq(specialties::id)))
        .inner_join(services::table.on(specialties::service_id.eq(services::id)))
        .inner_join(workers::table.on(specialties::worker_id.eq(workers::id)))
        .inner_join(cars::table.on(orders::car_vin.eq(cars::vin)))
        .inner_join(contacts::table.on(cars::contact_id.eq(contacts::id)))
        .select((
            orders::id,
            (
                services::id,
                services::title,
                services::price,
                services::duration,
            ),
            (
                workers::id,
                workers::first_name,
                workers::middle_name,
                workers::last_name,
                date::to_char(workers::date_of_birth, date::FORMAT),
                time::to_char(workers::start_time, time::FORMAT),
                time::to_char(workers::end_time, time::FORMAT),
            ),
            (
                cars::vin,
                (contacts::id, contacts::phone_number, contacts::email),
                cars::make,
                cars::model,
                cars::year,
            ),
            timestamp::to_char(orders::start_time, timestamp::FORMAT),
        ))
        .filter(services::id.eq(service_id))
        .load::<Order>(conn)
        .await
}

pub async fn get_by_id(id: Id, conn: &mut Connection) -> Result<Order> {
    use crate::schema::*;

    orders::table
        .inner_join(specialties::table.on(orders::specialty_id.eq(specialties::id)))
        .inner_join(services::table.on(specialties::service_id.eq(services::id)))
        .inner_join(workers::table.on(specialties::worker_id.eq(workers::id)))
        .inner_join(cars::table.on(orders::car_vin.eq(cars::vin)))
        .inner_join(contacts::table.on(cars::contact_id.eq(contacts::id)))
        .select((
            orders::id,
            (
                services::id,
                services::title,
                services::price,
                services::duration,
            ),
            (
                workers::id,
                workers::first_name,
                workers::middle_name,
                workers::last_name,
                date::to_char(workers::date_of_birth, date::FORMAT),
                time::to_char(workers::start_time, time::FORMAT),
                time::to_char(workers::end_time, time::FORMAT),
            ),
            (
                cars::vin,
                (contacts::id, contacts::phone_number, contacts::email),
                cars::make,
                cars::model,
                cars::year,
            ),
            timestamp::to_char(orders::start_time, timestamp::FORMAT),
        ))
        .filter(orders::id.eq(id))
        .first::<Order>(conn)
        .await
}

pub async fn update_by_id(id: Id, order: InsertOrder, conn: &mut Connection) -> Result<()> {
    use crate::schema::orders;

    update(orders::table.filter(orders::id.eq(id)))
        .set((
            orders::specialty_id.eq(order.specialty_id),
            orders::car_vin.eq(order.car_vin),
            orders::start_time.eq(timestamp::to_timestamp(order.start_time, timestamp::FORMAT)),
        ))
        .execute(conn)
        .await
        .map(|_| ())
}

pub async fn delete_by_id(id: Id, conn: &mut Connection) -> Result<()> {
    use crate::schema::orders;

    delete(orders::table.filter(orders::id.eq(id)))
        .execute(conn)
        .await
        .map(|_| ())
}
