-- Add migration script here
CREATE TABLE IF NOT EXISTS datalog (
    id SERIAL PRIMARY KEY,
    collector_id varchar(255) NOT NULL,
    total_memory BIGINT NOT NULL,
    used_memory BIGINT NOT NULL,
    average_cpu REAL NOT NULL
);