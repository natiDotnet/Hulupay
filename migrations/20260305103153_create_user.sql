-- Add migration script here
-- Auth crate migration: Create users table
CREATE TABLE IF NOT EXISTS users (
     id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
     merchant_id UUID NULL REFERENCES merchants(id) ON DELETE SET NULL,
     email TEXT NOT NULL UNIQUE,
     password_hash TEXT NOT NULL,
     role TEXT NOT NULL,
     created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
     updated_at TIMESTAMPTZ NULL default NULL
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_merchant_id ON users(merchant_id);
CREATE INDEX idx_users_role ON users(role);
