ALTER TYPE "provider" ADD VALUE 'SIMULATOR';
ALTER TABLE "payment_orders" ALTER COLUMN "amount" TYPE NUMERIC;
ALTER TABLE "payment_transactions" ALTER COLUMN "amount" TYPE NUMERIC;
ALTER TABLE "payment_items" ALTER COLUMN "unit_price" TYPE NUMERIC;
ALTER TABLE "payment_items" ALTER COLUMN "total_price" TYPE NUMERIC;
ALTER TABLE "merchant_webhooks" ALTER COLUMN "amount" TYPE NUMERIC;
ALTER TABLE "merchant_webhooks" ALTER COLUMN "charge" TYPE NUMERIC;
