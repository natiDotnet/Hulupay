-- Add migration script here
CREATE TABLE payment_providers (
   id UUID PRIMARY KEY,
   code VARCHAR(50) NOT NULL UNIQUE, 
   name VARCHAR(100) NOT NULL,
   is_active BOOLEAN NOT NULL DEFAULT true,
   created_at TIMESTAMP NOT NULL
);