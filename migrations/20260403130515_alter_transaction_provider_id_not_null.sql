-- Add migration script here
ALTER TABLE transactions
    ALTER COLUMN provider_id SET NOT NULL;