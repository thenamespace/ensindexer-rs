create extension if not exists btree_gin;

create index if not exists domains_parent_name_trgm_idx
  on domains using gin (parent_id, name gin_trgm_ops);

create index if not exists domains_parent_name_lower_trgm_idx
  on domains using gin (parent_id, lower(name) gin_trgm_ops);

create index if not exists domains_parent_label_name_trgm_idx
  on domains using gin (parent_id, label_name gin_trgm_ops);

create index if not exists domains_parent_label_name_lower_trgm_idx
  on domains using gin (parent_id, lower(label_name) gin_trgm_ops);
