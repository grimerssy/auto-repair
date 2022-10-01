CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    contact_id INTEGER NOT NULL REFERENCES contacts(id),
    service_id INTEGER NOT NULL REFERENCES services(id),
    start_time TIMESTAMP NOT NULL,
    car_make VARCHAR(32) NOT NULL,
    car_model VARCHAR(32) NOT NULL,
    car_year SMALLINT NOT NULL
);
