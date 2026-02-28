-- Add migration script here
CREATE TABLE users (
                       id UUID PRIMARY KEY,
                       merchant_id UUID NULL REFERENCES merchants(id),
                       email TEXT NOT NULL UNIQUE,
                       password_hash TEXT NOT NULL,
                       role TEXT NOT NULL,
                       created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                       updated_at TIMESTAMPTZ NOT NULL
);