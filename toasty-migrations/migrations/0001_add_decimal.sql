ALTER TYPE "provider" ADD VALUE 'SIMULATOR';

ALTER TABLE "payment_orders"
    ALTER COLUMN "amount" TYPE NUMERIC
        USING "amount"::NUMERIC;

ALTER TABLE "payment_transactions"
    ALTER COLUMN "amount" TYPE NUMERIC
        USING "amount"::NUMERIC;

ALTER TABLE "payment_items"
    ALTER COLUMN "unit_price" TYPE NUMERIC
        USING "unit_price"::NUMERIC;

ALTER TABLE "payment_items"
    ALTER COLUMN "total_price" TYPE NUMERIC
        USING "total_price"::NUMERIC;

ALTER TABLE "merchant_webhooks"
    ALTER COLUMN "amount" TYPE NUMERIC
        USING "amount"::NUMERIC;

ALTER TABLE "merchant_webhooks"
    ALTER COLUMN "charge" TYPE NUMERIC
        USING "charge"::NUMERIC;