-- Add migration script here
alter table payment_provider_configs
    add is_test_mode boolean not null default false;