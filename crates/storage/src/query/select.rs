use crate::filters::{
    AccountOrderField, DomainOrderField, EventOrderField, RegistrationOrderField,
    ResolverOrderField, WrappedDomainOrderField,
};

pub(crate) fn domain_select_sql() -> &'static str {
    r#"
    select id, name, label_name, labelhash, parent_id, subdomain_count,
           resolved_address_id, resolver_id, ttl, is_migrated, created_at,
           owner_id, registrant_id, wrapped_owner_id, expiry_date
    from domains
    "#
}

pub(crate) fn account_order_column(order_by: AccountOrderField) -> &'static str {
    match order_by {
        AccountOrderField::Id => "id",
        AccountOrderField::Domains => "(select count(*) from domains where owner_id = accounts.id)",
        AccountOrderField::WrappedDomains => {
            "(select count(*) from wrapped_domains where owner_id = accounts.id)"
        }
        AccountOrderField::Registrations => {
            "(select count(*) from registrations where registrant_id = accounts.id)"
        }
    }
}

pub(crate) fn domain_order_column(order_by: DomainOrderField) -> &'static str {
    match order_by {
        DomainOrderField::Id => "id",
        DomainOrderField::Name => "name",
        DomainOrderField::LabelName => "label_name",
        DomainOrderField::Labelhash => "labelhash",
        DomainOrderField::Parent => "parent_id",
        DomainOrderField::ParentId => "(select p.id from domains p where p.id = domains.parent_id)",
        DomainOrderField::ParentName => {
            "(select p.name from domains p where p.id = domains.parent_id)"
        }
        DomainOrderField::ParentLabelName => {
            "(select p.label_name from domains p where p.id = domains.parent_id)"
        }
        DomainOrderField::ParentLabelhash => {
            "(select p.labelhash from domains p where p.id = domains.parent_id)"
        }
        DomainOrderField::ParentSubdomainCount => {
            "(select p.subdomain_count from domains p where p.id = domains.parent_id)"
        }
        DomainOrderField::ParentTtl => {
            "(select p.ttl from domains p where p.id = domains.parent_id)"
        }
        DomainOrderField::ParentIsMigrated => {
            "(select p.is_migrated from domains p where p.id = domains.parent_id)"
        }
        DomainOrderField::ParentCreatedAt => {
            "(select p.created_at from domains p where p.id = domains.parent_id)"
        }
        DomainOrderField::ParentExpiryDate => {
            "(select p.expiry_date from domains p where p.id = domains.parent_id)"
        }
        DomainOrderField::Subdomains => {
            "(select count(*) from domains s where s.parent_id = domains.id)"
        }
        DomainOrderField::SubdomainCount => "subdomain_count",
        DomainOrderField::ResolvedAddress => "resolved_address_id",
        DomainOrderField::ResolvedAddressId => "resolved_address_id",
        DomainOrderField::Resolver => "resolver_id",
        DomainOrderField::ResolverId => "resolver_id",
        DomainOrderField::ResolverAddress => {
            "(select r.address from resolvers r where r.id = domains.resolver_id)"
        }
        DomainOrderField::ResolverContentHash => {
            "(select r.content_hash from resolvers r where r.id = domains.resolver_id)"
        }
        DomainOrderField::Ttl => "ttl",
        DomainOrderField::IsMigrated => "is_migrated",
        DomainOrderField::CreatedAt => "created_at",
        DomainOrderField::Owner => "owner_id",
        DomainOrderField::OwnerId => "owner_id",
        DomainOrderField::Registrant => "registrant_id",
        DomainOrderField::RegistrantId => "registrant_id",
        DomainOrderField::WrappedOwner => "wrapped_owner_id",
        DomainOrderField::WrappedOwnerId => "wrapped_owner_id",
        DomainOrderField::ExpiryDate => "expiry_date",
        DomainOrderField::Registration => {
            "(select r.id from registrations r where r.domain_id = domains.id)"
        }
        DomainOrderField::RegistrationId => {
            "(select r.id from registrations r where r.domain_id = domains.id)"
        }
        DomainOrderField::RegistrationRegistrationDate => {
            "(select r.registration_date from registrations r where r.domain_id = domains.id)"
        }
        DomainOrderField::RegistrationExpiryDate => {
            "(select r.expiry_date from registrations r where r.domain_id = domains.id)"
        }
        DomainOrderField::RegistrationCost => {
            "(select r.cost from registrations r where r.domain_id = domains.id)"
        }
        DomainOrderField::RegistrationLabelName => {
            "(select r.label_name from registrations r where r.domain_id = domains.id)"
        }
        DomainOrderField::WrappedDomain => {
            "(select w.id from wrapped_domains w where w.domain_id = domains.id)"
        }
        DomainOrderField::WrappedDomainId => {
            "(select w.id from wrapped_domains w where w.domain_id = domains.id)"
        }
        DomainOrderField::WrappedDomainExpiryDate => {
            "(select w.expiry_date from wrapped_domains w where w.domain_id = domains.id)"
        }
        DomainOrderField::WrappedDomainFuses => {
            "(select w.fuses from wrapped_domains w where w.domain_id = domains.id)"
        }
        DomainOrderField::WrappedDomainName => {
            "(select w.name from wrapped_domains w where w.domain_id = domains.id)"
        }
        DomainOrderField::Events => domain_event_count_sql(),
    }
}

pub(crate) fn registration_order_column(order_by: RegistrationOrderField) -> &'static str {
    match order_by {
        RegistrationOrderField::Id => "id",
        RegistrationOrderField::Domain => "domain_id",
        RegistrationOrderField::DomainId => "domain_id",
        RegistrationOrderField::DomainName => {
            "(select d.name from domains d where d.id = registrations.domain_id)"
        }
        RegistrationOrderField::DomainLabelName => {
            "(select d.label_name from domains d where d.id = registrations.domain_id)"
        }
        RegistrationOrderField::DomainLabelhash => {
            "(select d.labelhash from domains d where d.id = registrations.domain_id)"
        }
        RegistrationOrderField::DomainSubdomainCount => {
            "(select d.subdomain_count from domains d where d.id = registrations.domain_id)"
        }
        RegistrationOrderField::DomainTtl => {
            "(select d.ttl from domains d where d.id = registrations.domain_id)"
        }
        RegistrationOrderField::DomainIsMigrated => {
            "(select d.is_migrated from domains d where d.id = registrations.domain_id)"
        }
        RegistrationOrderField::DomainCreatedAt => {
            "(select d.created_at from domains d where d.id = registrations.domain_id)"
        }
        RegistrationOrderField::DomainExpiryDate => {
            "(select d.expiry_date from domains d where d.id = registrations.domain_id)"
        }
        RegistrationOrderField::RegistrationDate => "registration_date",
        RegistrationOrderField::ExpiryDate => "expiry_date",
        RegistrationOrderField::Cost => "cost",
        RegistrationOrderField::Registrant => "registrant_id",
        RegistrationOrderField::RegistrantId => "registrant_id",
        RegistrationOrderField::LabelName => "label_name",
        RegistrationOrderField::Events => registration_event_count_sql(),
    }
}

pub(crate) fn wrapped_domain_order_column(order_by: WrappedDomainOrderField) -> &'static str {
    match order_by {
        WrappedDomainOrderField::Id => "id",
        WrappedDomainOrderField::Domain => "domain_id",
        WrappedDomainOrderField::DomainId => "domain_id",
        WrappedDomainOrderField::DomainName => {
            "(select d.name from domains d where d.id = wrapped_domains.domain_id)"
        }
        WrappedDomainOrderField::DomainLabelName => {
            "(select d.label_name from domains d where d.id = wrapped_domains.domain_id)"
        }
        WrappedDomainOrderField::DomainLabelhash => {
            "(select d.labelhash from domains d where d.id = wrapped_domains.domain_id)"
        }
        WrappedDomainOrderField::DomainSubdomainCount => {
            "(select d.subdomain_count from domains d where d.id = wrapped_domains.domain_id)"
        }
        WrappedDomainOrderField::DomainTtl => {
            "(select d.ttl from domains d where d.id = wrapped_domains.domain_id)"
        }
        WrappedDomainOrderField::DomainIsMigrated => {
            "(select d.is_migrated from domains d where d.id = wrapped_domains.domain_id)"
        }
        WrappedDomainOrderField::DomainCreatedAt => {
            "(select d.created_at from domains d where d.id = wrapped_domains.domain_id)"
        }
        WrappedDomainOrderField::DomainExpiryDate => {
            "(select d.expiry_date from domains d where d.id = wrapped_domains.domain_id)"
        }
        WrappedDomainOrderField::ExpiryDate => "expiry_date",
        WrappedDomainOrderField::Fuses => "fuses",
        WrappedDomainOrderField::Owner => "owner_id",
        WrappedDomainOrderField::OwnerId => "owner_id",
        WrappedDomainOrderField::Name => "name",
    }
}

pub(crate) fn resolver_order_column(order_by: ResolverOrderField) -> &'static str {
    match order_by {
        ResolverOrderField::Id => "id",
        ResolverOrderField::Domain => "domain_id",
        ResolverOrderField::DomainId => "domain_id",
        ResolverOrderField::DomainName => {
            "(select d.name from domains d where d.id = resolvers.domain_id)"
        }
        ResolverOrderField::DomainLabelName => {
            "(select d.label_name from domains d where d.id = resolvers.domain_id)"
        }
        ResolverOrderField::DomainLabelhash => {
            "(select d.labelhash from domains d where d.id = resolvers.domain_id)"
        }
        ResolverOrderField::DomainSubdomainCount => {
            "(select d.subdomain_count from domains d where d.id = resolvers.domain_id)"
        }
        ResolverOrderField::DomainTtl => {
            "(select d.ttl from domains d where d.id = resolvers.domain_id)"
        }
        ResolverOrderField::DomainIsMigrated => {
            "(select d.is_migrated from domains d where d.id = resolvers.domain_id)"
        }
        ResolverOrderField::DomainCreatedAt => {
            "(select d.created_at from domains d where d.id = resolvers.domain_id)"
        }
        ResolverOrderField::DomainExpiryDate => {
            "(select d.expiry_date from domains d where d.id = resolvers.domain_id)"
        }
        ResolverOrderField::Address => "address",
        ResolverOrderField::Addr => "addr_id",
        ResolverOrderField::AddrId => "addr_id",
        ResolverOrderField::ContentHash => "content_hash",
        ResolverOrderField::Texts => "cardinality(texts)",
        ResolverOrderField::CoinTypes => "cardinality(coin_types)",
        ResolverOrderField::Events => resolver_event_count_sql(),
    }
}

fn domain_event_count_sql() -> &'static str {
    "(select count(*) from (select id from transfer_events where domain_id = domains.id union all select id from new_owner_events where domain_id = domains.id union all select id from new_resolver_events where domain_id = domains.id union all select id from new_ttl_events where domain_id = domains.id union all select id from wrapped_transfer_events where domain_id = domains.id union all select id from name_wrapped_events where domain_id = domains.id union all select id from name_unwrapped_events where domain_id = domains.id union all select id from fuses_set_events where domain_id = domains.id union all select id from expiry_extended_events where domain_id = domains.id) e)"
}

fn registration_event_count_sql() -> &'static str {
    "(select count(*) from (select id from name_registered_events where registration_id = registrations.id union all select id from name_renewed_events where registration_id = registrations.id union all select id from name_transferred_events where registration_id = registrations.id) e)"
}

fn resolver_event_count_sql() -> &'static str {
    "(select count(*) from (select id from addr_changed_events where resolver_id = resolvers.id union all select id from multicoin_addr_changed_events where resolver_id = resolvers.id union all select id from name_changed_events where resolver_id = resolvers.id union all select id from abi_changed_events where resolver_id = resolvers.id union all select id from pubkey_changed_events where resolver_id = resolvers.id union all select id from text_changed_events where resolver_id = resolvers.id union all select id from contenthash_changed_events where resolver_id = resolvers.id union all select id from interface_changed_events where resolver_id = resolvers.id union all select id from authorisation_changed_events where resolver_id = resolvers.id union all select id from version_changed_events where resolver_id = resolvers.id) e)"
}

pub(crate) fn event_order_column(order_by: EventOrderField) -> &'static str {
    match order_by {
        EventOrderField::Id => "id",
        EventOrderField::BlockNumber => "block_number",
        EventOrderField::TransactionId => "transaction_id",
        EventOrderField::Domain => "domain_id",
        EventOrderField::ParentDomain => "parent_domain_id",
        EventOrderField::Registration => "registration_id",
        EventOrderField::Resolver => "resolver_id",
        EventOrderField::Owner => "owner_id",
        EventOrderField::Registrant => "registrant_id",
        EventOrderField::NewOwner => "new_owner_id",
        EventOrderField::Addr => "addr_id",
        EventOrderField::Name => "name",
        EventOrderField::Fuses => "fuses",
        EventOrderField::Ttl => "ttl",
        EventOrderField::ExpiryDate => "expiry_date",
        EventOrderField::CoinType => "coin_type",
        EventOrderField::ContentType => "content_type",
        EventOrderField::X => "x",
        EventOrderField::Y => "y",
        EventOrderField::Key => "key",
        EventOrderField::Value => "value",
        EventOrderField::Hash => "hash",
        EventOrderField::InterfaceId => "interface_id",
        EventOrderField::Implementer => "implementer",
        EventOrderField::Target => "target",
        EventOrderField::IsAuthorized => "is_authorized",
        EventOrderField::Version => "version",
    }
}
