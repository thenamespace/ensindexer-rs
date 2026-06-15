create table if not exists accounts (
  id text primary key
);

create table if not exists domains (
  id text primary key,
  name text,
  label_name text,
  labelhash text,
  parent_id text references domains(id),
  subdomain_count integer not null default 0,
  resolved_address_id text references accounts(id),
  resolver_id text,
  ttl numeric,
  is_migrated boolean not null default false,
  created_at numeric not null default 0,
  owner_id text not null references accounts(id),
  registrant_id text references accounts(id),
  wrapped_owner_id text references accounts(id),
  expiry_date numeric
);

create table if not exists registrations (
  id text primary key,
  domain_id text not null references domains(id),
  registration_date numeric not null,
  expiry_date numeric not null,
  cost numeric,
  registrant_id text not null references accounts(id),
  label_name text
);

create table if not exists wrapped_domains (
  id text primary key,
  domain_id text not null references domains(id),
  expiry_date numeric not null,
  fuses integer not null,
  owner_id text not null references accounts(id),
  name text
);

create table if not exists resolvers (
  id text primary key,
  domain_id text references domains(id),
  address text not null,
  addr_id text references accounts(id),
  content_hash text,
  texts text[] not null default '{}',
  coin_types numeric[] not null default '{}'
);

create table if not exists blocks (
  number bigint primary key,
  hash text not null unique,
  parent_hash text,
  timestamp bigint not null
);

create table if not exists source_checkpoints (
  source text primary key,
  block_number bigint not null,
  block_hash text not null,
  updated_at timestamptz not null default now()
);

create table if not exists entity_changes (
  entity_type text not null,
  entity_id text not null,
  block_number integer not null,
  primary key (entity_type, entity_id, block_number)
);

create table if not exists transfer_events (
  id text primary key,
  domain_id text not null references domains(id),
  block_number integer not null,
  transaction_id text not null,
  owner_id text not null references accounts(id)
);

create table if not exists new_owner_events (
  id text primary key,
  domain_id text not null references domains(id),
  block_number integer not null,
  transaction_id text not null,
  parent_domain_id text not null references domains(id),
  owner_id text not null references accounts(id)
);

create table if not exists new_resolver_events (
  id text primary key,
  domain_id text not null references domains(id),
  block_number integer not null,
  transaction_id text not null,
  resolver_id text not null
);

create table if not exists new_ttl_events (
  id text primary key,
  domain_id text not null references domains(id),
  block_number integer not null,
  transaction_id text not null,
  ttl numeric not null
);

create table if not exists wrapped_transfer_events (
  id text primary key,
  domain_id text not null references domains(id),
  block_number integer not null,
  transaction_id text not null,
  owner_id text not null references accounts(id)
);

create table if not exists name_wrapped_events (
  id text primary key,
  domain_id text not null references domains(id),
  block_number integer not null,
  transaction_id text not null,
  name text,
  fuses integer not null,
  owner_id text not null references accounts(id),
  expiry_date numeric not null
);

create table if not exists name_unwrapped_events (
  id text primary key,
  domain_id text not null references domains(id),
  block_number integer not null,
  transaction_id text not null,
  owner_id text not null references accounts(id)
);

create table if not exists fuses_set_events (
  id text primary key,
  domain_id text not null references domains(id),
  block_number integer not null,
  transaction_id text not null,
  fuses integer not null
);

create table if not exists expiry_extended_events (
  id text primary key,
  domain_id text not null references domains(id),
  block_number integer not null,
  transaction_id text not null,
  expiry_date numeric not null
);

create table if not exists name_registered_events (
  id text primary key,
  registration_id text not null references registrations(id),
  block_number integer not null,
  transaction_id text not null,
  registrant_id text not null references accounts(id),
  expiry_date numeric not null
);

create table if not exists name_renewed_events (
  id text primary key,
  registration_id text not null references registrations(id),
  block_number integer not null,
  transaction_id text not null,
  expiry_date numeric not null
);

create table if not exists name_transferred_events (
  id text primary key,
  registration_id text not null references registrations(id),
  block_number integer not null,
  transaction_id text not null,
  new_owner_id text not null references accounts(id)
);

create table if not exists addr_changed_events (
  id text primary key,
  resolver_id text not null references resolvers(id),
  block_number integer not null,
  transaction_id text not null,
  addr_id text not null references accounts(id)
);

create table if not exists multicoin_addr_changed_events (
  id text primary key,
  resolver_id text not null references resolvers(id),
  block_number integer not null,
  transaction_id text not null,
  coin_type numeric not null,
  addr text not null
);

create table if not exists name_changed_events (
  id text primary key,
  resolver_id text not null references resolvers(id),
  block_number integer not null,
  transaction_id text not null,
  name text not null
);

create table if not exists abi_changed_events (
  id text primary key,
  resolver_id text not null references resolvers(id),
  block_number integer not null,
  transaction_id text not null,
  content_type numeric not null
);

create table if not exists pubkey_changed_events (
  id text primary key,
  resolver_id text not null references resolvers(id),
  block_number integer not null,
  transaction_id text not null,
  x text not null,
  y text not null
);

create table if not exists text_changed_events (
  id text primary key,
  resolver_id text not null references resolvers(id),
  block_number integer not null,
  transaction_id text not null,
  key text not null,
  value text
);

create table if not exists contenthash_changed_events (
  id text primary key,
  resolver_id text not null references resolvers(id),
  block_number integer not null,
  transaction_id text not null,
  hash text not null
);

create table if not exists interface_changed_events (
  id text primary key,
  resolver_id text not null references resolvers(id),
  block_number integer not null,
  transaction_id text not null,
  interface_id text not null,
  implementer text not null
);

create table if not exists authorisation_changed_events (
  id text primary key,
  resolver_id text not null references resolvers(id),
  block_number integer not null,
  transaction_id text not null,
  owner text not null,
  target text not null,
  is_authorized boolean not null
);

create table if not exists version_changed_events (
  id text primary key,
  resolver_id text not null references resolvers(id),
  block_number integer not null,
  transaction_id text not null,
  version numeric not null
);

create index if not exists domains_parent_idx on domains(parent_id);
create index if not exists domains_owner_idx on domains(owner_id);
create index if not exists domains_resolver_idx on domains(resolver_id);
create index if not exists registrations_domain_idx on registrations(domain_id);
create index if not exists registrations_registrant_idx on registrations(registrant_id);
create index if not exists wrapped_domains_owner_idx on wrapped_domains(owner_id);
create index if not exists resolvers_domain_idx on resolvers(domain_id);
create index if not exists resolvers_address_idx on resolvers(address);
create index if not exists transfer_events_domain_idx on transfer_events(domain_id);
create index if not exists transfer_events_block_idx on transfer_events(block_number);
create index if not exists new_owner_events_domain_idx on new_owner_events(domain_id);
create index if not exists new_owner_events_block_idx on new_owner_events(block_number);
create index if not exists new_resolver_events_domain_idx on new_resolver_events(domain_id);
create index if not exists new_resolver_events_block_idx on new_resolver_events(block_number);
create index if not exists new_ttl_events_domain_idx on new_ttl_events(domain_id);
create index if not exists new_ttl_events_block_idx on new_ttl_events(block_number);
create index if not exists wrapped_transfer_events_domain_idx on wrapped_transfer_events(domain_id);
create index if not exists wrapped_transfer_events_block_idx on wrapped_transfer_events(block_number);
create index if not exists name_wrapped_events_domain_idx on name_wrapped_events(domain_id);
create index if not exists name_wrapped_events_block_idx on name_wrapped_events(block_number);
create index if not exists name_unwrapped_events_domain_idx on name_unwrapped_events(domain_id);
create index if not exists name_unwrapped_events_block_idx on name_unwrapped_events(block_number);
create index if not exists fuses_set_events_domain_idx on fuses_set_events(domain_id);
create index if not exists fuses_set_events_block_idx on fuses_set_events(block_number);
create index if not exists expiry_extended_events_domain_idx on expiry_extended_events(domain_id);
create index if not exists expiry_extended_events_block_idx on expiry_extended_events(block_number);
create index if not exists name_registered_events_registration_idx on name_registered_events(registration_id);
create index if not exists name_registered_events_block_idx on name_registered_events(block_number);
create index if not exists name_renewed_events_registration_idx on name_renewed_events(registration_id);
create index if not exists name_renewed_events_block_idx on name_renewed_events(block_number);
create index if not exists name_transferred_events_registration_idx on name_transferred_events(registration_id);
create index if not exists name_transferred_events_block_idx on name_transferred_events(block_number);
create index if not exists addr_changed_events_resolver_idx on addr_changed_events(resolver_id);
create index if not exists addr_changed_events_block_idx on addr_changed_events(block_number);
create index if not exists multicoin_addr_changed_events_resolver_idx on multicoin_addr_changed_events(resolver_id);
create index if not exists multicoin_addr_changed_events_block_idx on multicoin_addr_changed_events(block_number);
create index if not exists name_changed_events_resolver_idx on name_changed_events(resolver_id);
create index if not exists name_changed_events_block_idx on name_changed_events(block_number);
create index if not exists abi_changed_events_resolver_idx on abi_changed_events(resolver_id);
create index if not exists abi_changed_events_block_idx on abi_changed_events(block_number);
create index if not exists pubkey_changed_events_resolver_idx on pubkey_changed_events(resolver_id);
create index if not exists pubkey_changed_events_block_idx on pubkey_changed_events(block_number);
create index if not exists text_changed_events_resolver_idx on text_changed_events(resolver_id);
create index if not exists text_changed_events_block_idx on text_changed_events(block_number);
create index if not exists contenthash_changed_events_resolver_idx on contenthash_changed_events(resolver_id);
create index if not exists contenthash_changed_events_block_idx on contenthash_changed_events(block_number);
create index if not exists interface_changed_events_resolver_idx on interface_changed_events(resolver_id);
create index if not exists interface_changed_events_block_idx on interface_changed_events(block_number);
create index if not exists authorisation_changed_events_resolver_idx on authorisation_changed_events(resolver_id);
create index if not exists authorisation_changed_events_block_idx on authorisation_changed_events(block_number);
create index if not exists version_changed_events_resolver_idx on version_changed_events(resolver_id);
create index if not exists version_changed_events_block_idx on version_changed_events(block_number);
create index if not exists entity_changes_lookup_idx on entity_changes(entity_type, entity_id, block_number);

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

create table if not exists label_preimages (
  labelhash text primary key,
  label_name text not null
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

create extension if not exists pg_trgm;

create index if not exists account_snapshots_block_idx on account_snapshots(block_number);
create index if not exists domain_snapshots_block_idx on domain_snapshots(block_number);
create index if not exists registration_snapshots_block_idx on registration_snapshots(block_number);
create index if not exists wrapped_domain_snapshots_block_idx on wrapped_domain_snapshots(block_number);
create index if not exists resolver_snapshots_block_idx on resolver_snapshots(block_number);

create index if not exists domains_labelhash_idx on domains(labelhash);
create index if not exists domains_name_md5_idx on domains(md5(name));
create index if not exists domains_label_name_md5_idx on domains(md5(label_name));
create index if not exists domains_bracketed_labelhash_idx
  on domains(labelhash)
  where label_name like '[%';
create index if not exists domains_registrant_idx on domains(registrant_id);
create index if not exists domains_wrapped_owner_idx on domains(wrapped_owner_id);
create index if not exists domains_resolved_address_idx on domains(resolved_address_id);
create index if not exists domains_name_trgm_idx on domains using gin (name gin_trgm_ops);
create index if not exists domains_name_lower_trgm_idx on domains using gin (lower(name) gin_trgm_ops);
create index if not exists domains_label_name_trgm_idx on domains using gin (label_name gin_trgm_ops);
create index if not exists domains_label_name_lower_trgm_idx on domains using gin (lower(label_name) gin_trgm_ops);
create index if not exists domains_expiry_date_idx on domains(expiry_date);
create index if not exists domains_created_at_idx on domains(created_at);
create index if not exists domains_owner_expiry_idx on domains(owner_id, expiry_date desc, id);
create index if not exists domains_registrant_expiry_idx on domains(registrant_id, expiry_date desc, id);
create index if not exists domains_wrapped_owner_expiry_idx on domains(wrapped_owner_id, expiry_date desc, id);
create index if not exists domains_resolved_address_expiry_idx on domains(resolved_address_id, expiry_date desc, id);
create index if not exists domains_owner_created_idx on domains(owner_id, created_at desc, id);
create index if not exists domains_registrant_created_idx on domains(registrant_id, created_at desc, id);
create index if not exists domains_wrapped_owner_created_idx on domains(wrapped_owner_id, created_at desc, id);
create index if not exists domains_resolved_address_created_idx on domains(resolved_address_id, created_at desc, id);
create index if not exists domains_parent_label_name_sort_idx on domains(parent_id, left(label_name, 256), id);
create index if not exists domains_parent_name_sort_idx on domains(parent_id, left(name, 256), id);
create index if not exists domains_parent_created_idx on domains(parent_id, created_at desc, id);
create index if not exists domains_parent_expiry_idx on domains(parent_id, expiry_date desc, id);
create index if not exists domains_name_md5_id_idx on domains(md5(name), id);
create index if not exists domains_label_name_md5_id_idx on domains(md5(label_name), id);

create index if not exists registrations_registration_date_idx on registrations(registration_date);
create index if not exists registrations_expiry_date_idx on registrations(expiry_date);
create index if not exists registrations_label_name_md5_expiry_idx on registrations(md5(label_name), expiry_date desc, id);
create index if not exists wrapped_domains_domain_idx on wrapped_domains(domain_id);

create index if not exists transfer_events_domain_id_idx on transfer_events(domain_id, id);
create index if not exists new_owner_events_domain_id_idx on new_owner_events(domain_id, id);
create index if not exists new_resolver_events_domain_id_idx on new_resolver_events(domain_id, id);
create index if not exists new_ttl_events_domain_id_idx on new_ttl_events(domain_id, id);
create index if not exists wrapped_transfer_events_domain_id_idx on wrapped_transfer_events(domain_id, id);
create index if not exists name_wrapped_events_domain_id_idx on name_wrapped_events(domain_id, id);
create index if not exists name_unwrapped_events_domain_id_idx on name_unwrapped_events(domain_id, id);
create index if not exists fuses_set_events_domain_id_idx on fuses_set_events(domain_id, id);
create index if not exists expiry_extended_events_domain_id_idx on expiry_extended_events(domain_id, id);

create index if not exists name_registered_events_registration_id_idx on name_registered_events(registration_id, id);
create index if not exists name_renewed_events_registration_id_idx on name_renewed_events(registration_id, id);
create index if not exists name_transferred_events_registration_id_idx on name_transferred_events(registration_id, id);

create index if not exists addr_changed_events_resolver_id_idx on addr_changed_events(resolver_id, id);
create index if not exists multicoin_addr_changed_events_resolver_id_idx on multicoin_addr_changed_events(resolver_id, id);
create index if not exists name_changed_events_resolver_id_idx on name_changed_events(resolver_id, id);
create index if not exists abi_changed_events_resolver_id_idx on abi_changed_events(resolver_id, id);
create index if not exists pubkey_changed_events_resolver_id_idx on pubkey_changed_events(resolver_id, id);
create index if not exists text_changed_events_resolver_id_idx on text_changed_events(resolver_id, id);
create index if not exists contenthash_changed_events_resolver_id_idx on contenthash_changed_events(resolver_id, id);
create index if not exists interface_changed_events_resolver_id_idx on interface_changed_events(resolver_id, id);
create index if not exists authorisation_changed_events_resolver_id_idx on authorisation_changed_events(resolver_id, id);
create index if not exists version_changed_events_resolver_id_idx on version_changed_events(resolver_id, id);
