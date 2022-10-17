CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    password_hash VARCHAR(64) NOT NULL,
    contact_id INTEGER NOT NULL REFERENCES contacts(id),
    first_name VARCHAR(32) NOT NULL,
    middle_name VARCHAR(32),
    last_name VARCHAR(32) NOT NULL,
    dob Date NOT NULL,
    registered_at TIMESTAMP NOT NULL DEFAULT NOW()
);
