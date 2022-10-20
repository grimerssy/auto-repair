CREATE TABLE services (
    id SERIAL PRIMARY KEY,
    title VARCHAR(32) UNIQUE NOT NULL,
    price MONEY NOT NULL,
    duration TIME NOT NULL
);
