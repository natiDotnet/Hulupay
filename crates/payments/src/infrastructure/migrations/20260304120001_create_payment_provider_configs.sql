-- Payments crate migration: Create payment_provider_configs table
CREATE TABLE IF NOT EXISTS payment_provider_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_id UUID NOT NULL REFERENCES merchants(id) ON DELETE CASCADE,
    provider_id UUID NOT NULL REFERENCES payment_providers(id) ON DELETE CASCADE,
    is_test_mode BOOLEAN NOT NULL DEFAULT false,
    config JSONB NOT NULL DEFAULT '{}',
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT unique_merchant_provider UNIQUE (merchant_id, provider_id)
);

CREATE INDEX idx_provider_configs_merchant_id ON payment_provider_configs(merchant_id);
CREATE INDEX idx_provider_configs_is_active ON payment_provider_configs(is_active);
CREATE INDEX idx_provider_configs_test_mode ON payment_provider_configs(is_test_mode);
