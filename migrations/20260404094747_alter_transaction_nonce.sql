-- Add migration script here
ALTER TABLE transactions
    ADD COLUMN nonce TEXT not null default '';