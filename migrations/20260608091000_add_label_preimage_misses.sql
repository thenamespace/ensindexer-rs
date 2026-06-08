create table if not exists label_preimage_misses (
  labelhash text primary key,
  checked_at timestamptz not null default now()
);
