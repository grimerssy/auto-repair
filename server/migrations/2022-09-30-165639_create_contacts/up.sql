CREATE TABLE contacts (
    id SERIAL PRIMARY KEY,
    phone_number VARCHAR(16) UNIQUE NOT NULL,
    email VARCHAR(64) UNIQUE
);

CREATE INDEX idx_contacts_phone ON contacts(phone_number);
