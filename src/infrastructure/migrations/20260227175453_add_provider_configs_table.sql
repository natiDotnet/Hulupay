-- Add migration script here
CREATE TABLE payment_provider_configs (
   id UUID PRIMARY KEY,
   merchant_id UUID NOT NULL,
   provider_id UUID NOT NULL REFERENCES payment_providers(id),
   config JSONB NOT NULL,
   is_active BOOLEAN NOT NULL DEFAULT true,
   created_at TIMESTAMP NOT NULL,
   updated_at TIMESTAMP NOT NULL,

   UNIQUE (merchant_id, provider_id)
);