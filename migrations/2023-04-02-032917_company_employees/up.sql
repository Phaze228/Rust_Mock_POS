-- Your SQL goes here

DO $$ BEGIN
    CREATE TYPE role as ENUM('CEO', 'SUPERVISOR','ANALYST', 'HR', 'UNEMPLOYED');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;


CREATE TABLE employees (
    id SERIAL PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT NOT NULL,
    position Role NOT NULL,
    pay      INT NOT NULL,
    employed BOOLEAN NOT NULL DEFAULT TRUE
)