create index if not exists registrations_label_name_md5_expiry_idx on registrations(md5(label_name), expiry_date desc, id);

analyze registrations;
