-- Add migration script here
CREATE TABLE IF NOT EXISTS image (
    id SERIAL PRIMARY KEY,
    name varchar(255) NOT NULL
);