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
