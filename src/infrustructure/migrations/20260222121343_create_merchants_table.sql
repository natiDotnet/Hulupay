-- Add migration script here
CREATE TABLE merchants (
   id UUID PRIMARY KEY,
   name TEXT NOT NULL
);

CREATE INDEX idx_merchants_name ON merchants(name);