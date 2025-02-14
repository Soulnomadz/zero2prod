-- Add migration script here

create table subscriptions (
  id uuid not null,
  primary key (id),
  email TEXT not null unique,
  name text not null,
  subscribed_at timestamptz not null
);
