-- Alter transactions table to change amount type from NUMERIC to BIGINT
ALTER TABLE transactions
    ALTER COLUMN amount TYPE BIGINT,
    ALTER COLUMN amount SET NOT NULL;
