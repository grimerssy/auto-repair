// @generated automatically by Diesel CLI.

diesel::table! {
    contacts (id) {
        id -> Int4,
        phone_number -> Int8,
        email -> Nullable<Varchar>,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        contact_id -> Int4,
        service_id -> Int4,
        start_time -> Timestamp,
        car_make -> Varchar,
        car_model -> Varchar,
        car_year -> Int2,
    }
}

diesel::table! {
    services (id) {
        id -> Int4,
        title -> Varchar,
        price -> Money,
        duration -> Time,
    }
}

diesel::joinable!(orders -> contacts (contact_id));
diesel::joinable!(orders -> services (service_id));

diesel::allow_tables_to_appear_in_same_query!(
    contacts,
    orders,
    services,
);
