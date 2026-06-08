create index if not exists domains_parent_label_name_sort_idx on domains(parent_id, left(label_name, 256), id);
create index if not exists domains_parent_name_sort_idx on domains(parent_id, left(name, 256), id);
create index if not exists domains_parent_created_idx on domains(parent_id, created_at desc, id);
create index if not exists domains_parent_expiry_idx on domains(parent_id, expiry_date desc, id);
