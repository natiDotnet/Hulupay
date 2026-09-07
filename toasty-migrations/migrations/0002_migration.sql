ALTER TYPE "provider" ADD VALUE 'LAKIPAY';
ALTER TYPE "provider" ADD VALUE 'STARPAY';
ALTER TABLE "payment_transactions" ADD COLUMN "response" JSONB;
