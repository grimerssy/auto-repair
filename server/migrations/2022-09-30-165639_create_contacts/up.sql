CREATE TABLE contacts (
    id SERIAL PRIMARY KEY,
    phone_number BIGINT NOT NULL,
    email VARCHAR(64)
);
