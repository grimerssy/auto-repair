use diesel::sql_query;
use diesel_async::RunQueryDsl;

use crate::models::report_data::*;

use super::{Connection, Result};

pub async fn get_most_profitable_services_for_month(
    conn: &mut Connection,
) -> Result<Vec<ServiceReport>> {
    sql_query(
        r"
select services.*, price_sum as revenue from services
inner join (
    select services.id, sum(services.price) as price_sum from services
    inner join specialties on specialties.service_id = services.id
    inner join orders on orders.specialty_id = specialties.id
    group by services.id
    having sum(services.price) = (
        select max(price_sum) as max_sum from (
            select sum(services.price) as price_sum from services
            inner join specialties on specialties.service_id = services.id
            inner join orders on orders.specialty_id = specialties.id
            group by services.id
        ) _
    )
) subq on subq.id = services.id;
",
    )
    .load::<ServiceReport>(conn)
    .await
}

pub async fn get_most_valuable_clients_for_month(
    conn: &mut Connection,
) -> Result<Vec<ClientsReport>> {
    sql_query(format!(
        r"
select
    contacts.*,
    users.first_name,
    users.middle_name,
    users.last_name,
    to_char(users.date_of_birth, '{}') as date_of_birth,
    count(orders.id) as order_count
from
    contacts
    left join users on contacts.id = users.contact_id
    inner join cars on cars.contact_id = contacts.id
    inner join orders on cars.vin = orders.car_vin
group by
    contacts.id,
    contacts.phone_number,
    contacts.email,
    users.first_name,
    users.middle_name,
    users.last_name,
    users.date_of_birth
having count(orders.id) >= (
  select c from (
    select count(orders.id) as c from contacts
    inner join cars on cars.contact_id = contacts.id
    inner join orders on orders.car_vin = cars.vin
    group by contacts.id
    order by count(orders.id) desc
    limit 10
  ) subq
  order by c asc
  limit 1
);
",
        super::date::FORMAT
    ))
    .load::<ClientsReport>(conn)
    .await
}

pub async fn get_most_frequent_cars_for_month(conn: &mut Connection) -> Result<Vec<CarsReport>> {
    sql_query(
        r"
select cars.make, cars.model, cars.year, count(orders.id) as order_count from cars
inner join orders on cars.vin = orders.car_vin
group by cars.make, cars.model, cars.year
having count(orders.id) >= (
  select c from (
    select count(orders.id) as c from cars
    inner join orders on orders.car_vin = cars.vin
    group by cars.make, cars.model, cars.year
    order by count(orders.id) desc
    limit 10
  ) subq
  order by c asc
  limit 1
);
",
    )
    .load::<CarsReport>(conn)
    .await
}

pub async fn get_total_work_hours_for_month(conn: &mut Connection) -> Result<WorkHoursReport> {
    sql_query(
        r"
select cast(to_char(sum(services.duration), 'HH24') as integer) as hours from orders
inner join specialties on specialties.id = orders.specialty_id
inner join services on services.id = specialties.service_id
where orders.start_time between now() - interval '1 month' and now();
",
    )
    .load::<WorkHoursReport>(conn)
    .await
    .map(|vec| vec.into_iter().next().unwrap())
}
