CREATE TABLE contacts (
    id SERIAL PRIMARY KEY,
    phone_number BIGINT NOT NULL,
    email VARCHAR(64)
);

CREATE INDEX idx_contacts_phone ON contacts(phone_number);
