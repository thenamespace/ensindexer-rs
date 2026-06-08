create index if not exists domains_name_md5_id_idx on domains(md5(name), id);
create index if not exists domains_label_name_md5_id_idx on domains(md5(label_name), id);

analyze domains;
