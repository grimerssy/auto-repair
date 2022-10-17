CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    contact_id INTEGER NOT NULL REFERENCES contacts(id),
    password_hash VARCHAR(64) NOT NULL,
    first_name VARCHAR(32) NOT NULL,
    middle_name VARCHAR(32),
    last_name VARCHAR(32) NOT NULL,
    date_of_birth Date NOT NULL,
    registered_at TIMESTAMP NOT NULL DEFAULT NOW()
);
