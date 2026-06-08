create index if not exists domains_bracketed_labelhash_idx
  on domains(labelhash)
  where label_name like '[%';
