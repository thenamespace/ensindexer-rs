create table if not exists account_snapshots (
  id text not null,
  block_number integer not null,
  deleted boolean not null default false,
  primary key (id, block_number)
);

create table if not exists domain_snapshots (
  id text not null,
  block_number integer not null,
  deleted boolean not null default false,
  name text,
  label_name text,
  labelhash text,
  parent_id text,
  subdomain_count integer,
  resolved_address_id text,
  resolver_id text,
  ttl numeric,
  is_migrated boolean,
  created_at numeric,
  owner_id text,
  registrant_id text,
  wrapped_owner_id text,
  expiry_date numeric,
  primary key (id, block_number)
);

create table if not exists registration_snapshots (
  id text not null,
  block_number integer not null,
  deleted boolean not null default false,
  domain_id text,
  registration_date numeric,
  expiry_date numeric,
  cost numeric,
  registrant_id text,
  label_name text,
  primary key (id, block_number)
);

create table if not exists wrapped_domain_snapshots (
  id text not null,
  block_number integer not null,
  deleted boolean not null default false,
  domain_id text,
  expiry_date numeric,
  fuses integer,
  owner_id text,
  name text,
  primary key (id, block_number)
);

create table if not exists resolver_snapshots (
  id text not null,
  block_number integer not null,
  deleted boolean not null default false,
  domain_id text,
  address text,
  addr_id text,
  content_hash text,
  texts text[],
  coin_types numeric[],
  primary key (id, block_number)
);

create or replace function latest_account_snapshots(target_block integer)
returns table (id text)
language sql
stable
as $$
  select id
  from (
    select distinct on (id) id, deleted
    from account_snapshots
    where block_number <= target_block
    order by id, block_number desc
  ) latest
  where not deleted
$$;

create or replace function latest_domain_snapshots(target_block integer)
returns table (
  id text,
  name text,
  label_name text,
  labelhash text,
  parent_id text,
  subdomain_count integer,
  resolved_address_id text,
  resolver_id text,
  ttl numeric,
  is_migrated boolean,
  created_at numeric,
  owner_id text,
  registrant_id text,
  wrapped_owner_id text,
  expiry_date numeric
)
language sql
stable
as $$
  select id, name, label_name, labelhash, parent_id, subdomain_count,
         resolved_address_id, resolver_id, ttl, is_migrated, created_at,
         owner_id, registrant_id, wrapped_owner_id, expiry_date
  from (
    select distinct on (id) *
    from domain_snapshots
    where block_number <= target_block
    order by id, block_number desc
  ) latest
  where not deleted
$$;

create or replace function latest_registration_snapshots(target_block integer)
returns table (
  id text,
  domain_id text,
  registration_date numeric,
  expiry_date numeric,
  cost numeric,
  registrant_id text,
  label_name text
)
language sql
stable
as $$
  select id, domain_id, registration_date, expiry_date, cost, registrant_id, label_name
  from (
    select distinct on (id) *
    from registration_snapshots
    where block_number <= target_block
    order by id, block_number desc
  ) latest
  where not deleted
$$;

create or replace function latest_wrapped_domain_snapshots(target_block integer)
returns table (
  id text,
  domain_id text,
  expiry_date numeric,
  fuses integer,
  owner_id text,
  name text
)
language sql
stable
as $$
  select id, domain_id, expiry_date, fuses, owner_id, name
  from (
    select distinct on (id) *
    from wrapped_domain_snapshots
    where block_number <= target_block
    order by id, block_number desc
  ) latest
  where not deleted
$$;

create or replace function latest_resolver_snapshots(target_block integer)
returns table (
  id text,
  domain_id text,
  address text,
  addr_id text,
  content_hash text,
  texts text[],
  coin_types numeric[]
)
language sql
stable
as $$
  select id, domain_id, address, addr_id, content_hash, texts, coin_types
  from (
    select distinct on (id) *
    from resolver_snapshots
    where block_number <= target_block
    order by id, block_number desc
  ) latest
  where not deleted
$$;

create index if not exists account_snapshots_block_idx on account_snapshots(block_number);
create index if not exists domain_snapshots_block_idx on domain_snapshots(block_number);
create index if not exists registration_snapshots_block_idx on registration_snapshots(block_number);
create index if not exists wrapped_domain_snapshots_block_idx on wrapped_domain_snapshots(block_number);
create index if not exists resolver_snapshots_block_idx on resolver_snapshots(block_number);
