-- Add migration script here
ALTER TABLE transactions
    ADD COLUMN webhook_body jsonb;
