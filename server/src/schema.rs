// @generated automatically by Diesel CLI.

diesel::table! {
    cars (vin) {
        vin -> Varchar,
        contact_id -> Int4,
        make -> Varchar,
        model -> Varchar,
        year -> Int2,
    }
}

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
        specialty_id -> Int4,
        car_vin -> Varchar,
        start_time -> Timestamp,
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
    sessions (key) {
        key -> Varchar,
        user_id -> Int4,
        id_address -> Varchar,
        exp -> Timestamp,
    }
}

diesel::table! {
    specialties (id) {
        id -> Int4,
        service_id -> Int4,
        worker_id -> Int4,
    }
}

diesel::table! {
    users (contact_id) {
        contact_id -> Int4,
        password_hash -> Varchar,
        role -> Varchar,
        first_name -> Varchar,
        middle_name -> Nullable<Varchar>,
        last_name -> Varchar,
        date_of_birth -> Date,
        registered_at -> Timestamp,
    }
}

diesel::table! {
    workers (id) {
        id -> Int4,
        first_name -> Varchar,
        middle_name -> Nullable<Varchar>,
        last_name -> Varchar,
        date_of_birth -> Date,
        start_time -> Time,
        end_time -> Time,
    }
}

diesel::joinable!(cars -> contacts (contact_id));
diesel::joinable!(orders -> cars (car_vin));
diesel::joinable!(orders -> specialties (specialty_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(specialties -> services (service_id));
diesel::joinable!(specialties -> workers (worker_id));
diesel::joinable!(users -> contacts (contact_id));

diesel::allow_tables_to_appear_in_same_query!(
    cars,
    contacts,
    orders,
    services,
    sessions,
    specialties,
    users,
    workers,
);
