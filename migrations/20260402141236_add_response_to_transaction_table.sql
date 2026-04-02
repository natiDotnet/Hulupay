-- Add provider_id and response columns to transactions table
ALTER TABLE transactions
ADD COLUMN provider_id UUID REFERENCES payment_providers(id) ON DELETE SET NULL;

ALTER TABLE transactions
ADD COLUMN response JSONB NULL;

-- Create index on provider_id
CREATE INDEX idx_transactions_provider_id ON transactions(provider_id);