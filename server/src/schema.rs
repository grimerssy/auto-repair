// @generated automatically by Diesel CLI.

diesel::table! {
    contacts (id) {
        id -> Int4,
        phone_number -> Varchar,
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

diesel::table! {
    users (id) {
        id -> Int4,
        contact_id -> Int4,
        password_hash -> Varchar,
        first_name -> Varchar,
        middle_name -> Nullable<Varchar>,
        last_name -> Varchar,
        date_of_birth -> Date,
        registered_at -> Timestamp,
    }
}

diesel::joinable!(orders -> contacts (contact_id));
diesel::joinable!(orders -> services (service_id));
diesel::joinable!(users -> contacts (contact_id));

diesel::allow_tables_to_appear_in_same_query!(
    contacts,
    orders,
    services,
    users,
);
