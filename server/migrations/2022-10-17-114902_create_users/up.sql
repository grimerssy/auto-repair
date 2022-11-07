CREATE TABLE users (
    contact_id INTEGER PRIMARY KEY,
    password_hash VARCHAR(64) NOT NULL,
    role VARCHAR(5) NOT NULL,
    first_name VARCHAR(32) NOT NULL,
    middle_name VARCHAR(32),
    last_name VARCHAR(32) NOT NULL,
    date_of_birth DATE NOT NULL,
    registered_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_contact_id FOREIGN KEY (
        contact_id
    ) REFERENCES contacts(id) ON DELETE CASCADE
);
