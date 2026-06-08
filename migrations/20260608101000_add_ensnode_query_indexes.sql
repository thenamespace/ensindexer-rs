create extension if not exists pg_trgm;

create index if not exists domains_registrant_idx on domains(registrant_id);
create index if not exists domains_wrapped_owner_idx on domains(wrapped_owner_id);
create index if not exists domains_resolved_address_idx on domains(resolved_address_id);
create index if not exists domains_name_trgm_idx on domains using gin (name gin_trgm_ops);
create index if not exists domains_name_lower_trgm_idx on domains using gin (lower(name) gin_trgm_ops);
create index if not exists domains_label_name_trgm_idx on domains using gin (label_name gin_trgm_ops);
create index if not exists domains_label_name_lower_trgm_idx on domains using gin (lower(label_name) gin_trgm_ops);

create index if not exists registrations_registration_date_idx on registrations(registration_date);
create index if not exists registrations_expiry_date_idx on registrations(expiry_date);
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
