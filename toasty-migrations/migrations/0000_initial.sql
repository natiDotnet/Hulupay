CREATE TYPE "permission" AS ENUM ('merchant.read', 'merchant.update', 'merchant.delete', 'payment.create', 'payment.read', 'payment.refund', 'payment.export', 'provider.read', 'provider.update', 'provider.delete', 'routing.read', 'routing.update', 'apikey.create', 'apikey.rotate', 'apikey.delete', 'apikey.read', 'webhook.read', 'webhook.update', 'users.read', 'users.invite', 'users.delete', 'audit.read', 'platform.manage', 'platform.suspend', 'platform.analytics');
CREATE TYPE "tx_status" AS ENUM ('PENDING', 'SUCCESS', 'FAILED');
CREATE TYPE "payment_method" AS ENUM ('NONE', 'TELEBIRR', 'MPESA', 'CBEBIRR', 'AWASHBIRR', 'YAYA', 'COOPAYEBIRR', 'ZAMZAM', 'BINGET', 'KACHA', 'BOA', 'AMOLE', 'UNKNOWN');
CREATE TYPE "merchant_status" AS ENUM ('PENDING', 'ACTIVE', 'SUSPENDED', 'DELETED');
CREATE TYPE "routing_strategy" AS ENUM ('DEFAULT', 'PRIORITY', 'PAYMENTMETHOD', 'CURRENCY', 'AMOUNT', 'COUNTRY', 'LOWESTCOST', 'RULEBASED');
CREATE TYPE "environment" AS ENUM ('PRODUCTION', 'SANDBOX');
CREATE TYPE "condition_operator" AS ENUM ('EQUALS', 'GREATERTHAN', 'LESSTHAN');
CREATE TYPE "provider" AS ENUM ('STRIPE', 'HULU', 'CHAPA', 'ARIFPAY');
CREATE TYPE "condition_type" AS ENUM ('PAYMENTMETHOD', 'CURRENCY', 'COUNTRY', 'AMOUNTGREATERTHAN', 'AMOUNTLESSTHAN');
CREATE TYPE "tx_direction" AS ENUM ('CHARGE', 'REFUND');
CREATE TYPE "account_status" AS ENUM ('PENDING', 'ACTIVE', 'SUSPENDED', 'DELETED');
CREATE TYPE "role" AS ENUM ('master_admin', 'merchant_admin', 'owner', 'admin', 'developer', 'finance', 'viewer');
CREATE TYPE "provider_status" AS ENUM ('HEALTHY', 'DEGRADED', 'OFFLINE');
CREATE TYPE "payment_status" AS ENUM ('INITIATED', 'PENDING', 'PROCESSING', 'COMPLETED', 'FAILED', 'CANCELLED', 'REFUNDPENDING', 'REFUNDED');
CREATE TABLE "payment_transactions" (
    "id" UUID NOT NULL,
    "payment_order_id" UUID NOT NULL,
    "provider" provider NOT NULL,
    "provider_tx_id" TEXT,
    "direction" tx_direction NOT NULL,
    "amount" text NOT NULL,
    "currency" TEXT NOT NULL,
    "status" tx_status NOT NULL,
    "provider_response" JSONB,
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "updated_at" TIMESTAMPTZ(6) NOT NULL,
    PRIMARY KEY ("id")
);
CREATE TABLE "refresh_tokens" (
    "id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "token_hash" TEXT NOT NULL,
    "family_id" UUID NOT NULL,
    "expires_at" TIMESTAMPTZ(6) NOT NULL,
    "revoked_at" TIMESTAMPTZ(6),
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    PRIMARY KEY ("id")
);
CREATE TABLE "apikeys" (
    "id" UUID NOT NULL,
    "merchant_id" UUID NOT NULL,
    "name" TEXT NOT NULL,
    "prefix" TEXT NOT NULL,
    "hash" TEXT NOT NULL,
    "scopes" TEXT[] NOT NULL,
    "is_active" BOOLEAN NOT NULL,
    "expires_at" TIMESTAMPTZ(6) NOT NULL,
    "last_used_at" TIMESTAMPTZ(6),
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "updated_at" TIMESTAMPTZ(6),
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_apikeys_by_name" ON "apikeys" ("name");
CREATE TABLE "merchant_webhooks" (
    "id" UUID NOT NULL,
    "payment_order_id" UUID NOT NULL,
    "merchant_id" UUID NOT NULL,
    "status" payment_status NOT NULL,
    "provider_reference" TEXT NOT NULL,
    "payment_method" payment_method NOT NULL,
    "amount" text NOT NULL,
    "charge" text NOT NULL,
    "client_reference" TEXT NOT NULL,
    "txn_reference" TEXT NOT NULL,
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    PRIMARY KEY ("id")
);
CREATE TABLE "payment_providers" (
    "id" UUID NOT NULL,
    "code" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "logo" TEXT NOT NULL,
    "is_active" BOOLEAN NOT NULL,
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_payment_providers_by_code" ON "payment_providers" ("code");
CREATE TABLE "users" (
    "id" UUID NOT NULL,
    "email" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "password_hash" TEXT NOT NULL,
    "merchant_id" UUID NOT NULL,
    "role" role NOT NULL,
    "is_active" BOOLEAN NOT NULL,
    "status" account_status NOT NULL,
    "email_verified_at" TIMESTAMPTZ(6),
    "password_changed_at" TIMESTAMPTZ(6),
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "updated_at" TIMESTAMPTZ(6),
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_users_by_email" ON "users" ("email");
CREATE TABLE "role_permissions" (
    "id" UUID NOT NULL,
    "role_name" TEXT NOT NULL,
    "permission" permission NOT NULL,
    PRIMARY KEY ("id")
);
CREATE TABLE "merchant_configs" (
    "id" UUID NOT NULL,
    "merchant_id" UUID NOT NULL,
    "provider_id" UUID NOT NULL,
    "is_test_mode" BOOLEAN NOT NULL,
    "environment" environment NOT NULL,
    "priority" INTEGER NOT NULL,
    "is_default" BOOLEAN NOT NULL,
    "is_active" BOOLEAN NOT NULL,
    "config" JSONB NOT NULL,
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "updated_at" TIMESTAMPTZ(6) NOT NULL,
    PRIMARY KEY ("id")
);
CREATE INDEX "index_merchant_configs_by_merchant_id" ON "merchant_configs" ("merchant_id");
CREATE INDEX "index_merchant_configs_by_provider_id" ON "merchant_configs" ("provider_id");
CREATE TABLE "email_verifications" (
    "id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "token_hash" TEXT NOT NULL,
    "expires_at" TIMESTAMPTZ(6) NOT NULL,
    "verified_at" TIMESTAMPTZ(6),
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    PRIMARY KEY ("id")
);
CREATE TABLE "payment_callbacks" (
    "id" UUID NOT NULL,
    "payment_order_id" UUID NOT NULL,
    "success_url" TEXT NOT NULL,
    "error_url" TEXT NOT NULL,
    "cancel_url" TEXT NOT NULL,
    "notify_url" TEXT NOT NULL,
    PRIMARY KEY ("id")
);
CREATE TABLE "password_resets" (
    "id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "token_hash" TEXT NOT NULL,
    "expires_at" TIMESTAMPTZ(6) NOT NULL,
    "used_at" TIMESTAMPTZ(6),
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    PRIMARY KEY ("id")
);
CREATE TABLE "merchant_routing_strategies" (
    "id" UUID NOT NULL,
    "merchant_id" UUID NOT NULL,
    "strategy" routing_strategy NOT NULL,
    "enabled" BOOLEAN NOT NULL,
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "updated_at" TIMESTAMPTZ(6) NOT NULL,
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_merchant_routing_strategies_by_merchant_id" ON "merchant_routing_strategies" ("merchant_id");
CREATE TABLE "merchants" (
    "id" UUID NOT NULL,
    "name" TEXT NOT NULL,
    "slug" TEXT NOT NULL,
    "email" TEXT NOT NULL,
    "phone" TEXT NOT NULL,
    "website" TEXT NOT NULL,
    "is_active" BOOLEAN NOT NULL,
    "status" merchant_status NOT NULL,
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "updated_at" TIMESTAMPTZ(6),
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_merchants_by_name" ON "merchants" ("name");
CREATE UNIQUE INDEX "index_merchants_by_slug" ON "merchants" ("slug");
CREATE UNIQUE INDEX "index_merchants_by_email" ON "merchants" ("email");
CREATE TABLE "merchant_routing_rules" (
    "id" UUID NOT NULL,
    "merchant_id" UUID NOT NULL,
    "priority" INTEGER NOT NULL,
    "enabled" BOOLEAN NOT NULL,
    "condition_type" condition_type NOT NULL,
    "operator" condition_operator NOT NULL,
    "condition_value" TEXT NOT NULL,
    "target_provider_id" UUID NOT NULL,
    "fallback_provider_id" UUID,
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "updated_at" TIMESTAMPTZ(6) NOT NULL,
    PRIMARY KEY ("id")
);
CREATE INDEX "index_merchant_routing_rules_by_merchant_id" ON "merchant_routing_rules" ("merchant_id");
CREATE TABLE "provider_payment_methods" (
    "id" UUID NOT NULL,
    "provider_id" UUID NOT NULL,
    "payment_method_code" payment_method NOT NULL,
    "provider_method_code" TEXT NOT NULL,
    "provider_path_segment" TEXT,
    "is_active" BOOLEAN NOT NULL,
    PRIMARY KEY ("id")
);
CREATE INDEX "index_provider_payment_methods_by_payment_method_code" ON "provider_payment_methods" ("payment_method_code");
CREATE INDEX "index_provider_payment_methods_by_provider_method_code" ON "provider_payment_methods" ("provider_method_code");
CREATE TABLE "payment_items" (
    "id" UUID NOT NULL,
    "payment_order_id" UUID NOT NULL,
    "name" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "quantity" BIGINT NOT NULL,
    "image" TEXT,
    "unit_price" text NOT NULL,
    "total_price" text NOT NULL,
    PRIMARY KEY ("id")
);
CREATE TABLE "payment_orders" (
    "id" UUID NOT NULL,
    "merchant_id" UUID NOT NULL,
    "customer_id" UUID NOT NULL,
    "order_ref" TEXT NOT NULL,
    "amount" text NOT NULL,
    "currency" TEXT NOT NULL,
    "status" payment_status NOT NULL,
    "request_provider" provider NOT NULL,
    "provider" provider NOT NULL,
    "idempotency_key" TEXT NOT NULL,
    "retry_count" INTEGER NOT NULL,
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "updated_at" TIMESTAMPTZ(6) NOT NULL,
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_payment_orders_by_order_ref" ON "payment_orders" ("order_ref");
CREATE UNIQUE INDEX "index_payment_orders_by_idempotency_key" ON "payment_orders" ("idempotency_key");
CREATE TABLE "provider_metrics" (
    "id" UUID NOT NULL,
    "provider_id" UUID NOT NULL,
    "current_status" provider_status NOT NULL,
    "success_rate" DOUBLE PRECISION NOT NULL,
    "average_latency_ms" BIGINT NOT NULL,
    "error_rate" DOUBLE PRECISION NOT NULL,
    "last_health_check" TIMESTAMPTZ(6),
    "updated_at" TIMESTAMPTZ(6) NOT NULL,
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_provider_metrics_by_provider_id" ON "provider_metrics" ("provider_id");
CREATE TABLE "revoked_tokens" (
    "id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "expires_at" TIMESTAMPTZ(6) NOT NULL,
    "revoked_at" TIMESTAMPTZ(6) NOT NULL,
    PRIMARY KEY ("id")
);
CREATE TABLE "payment_customers" (
    "id" UUID NOT NULL,
    "payment_order_id" UUID NOT NULL,
    "name" TEXT NOT NULL,
    "email" TEXT NOT NULL,
    "phone" TEXT NOT NULL,
    "account_number" TEXT,
    PRIMARY KEY ("id")
);
