-- Add migration script here
alter table merchants
add is_active boolean not null default true;