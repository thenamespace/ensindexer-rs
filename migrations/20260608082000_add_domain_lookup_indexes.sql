create index if not exists domains_labelhash_idx on domains(labelhash);
create index if not exists domains_name_md5_idx on domains(md5(name));
create index if not exists domains_label_name_md5_idx on domains(md5(label_name));
