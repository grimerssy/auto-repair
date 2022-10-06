CREATE TABLE services (
    id SERIAL PRIMARY KEY,
    title VARCHAR(32) NOT NULL,
    price MONEY NOT NULL,
    duration TIME NOT NULL
);
