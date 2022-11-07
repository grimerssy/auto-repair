CREATE TABLE workers (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(32) NOT NULL,
    middle_name VARCHAR(32),
    last_name VARCHAR(32) NOT NULL,
    date_of_birth DATE NOT NULL,
    start_time TIME NOT NULL,
    end_time TIME NOT NULL
);
