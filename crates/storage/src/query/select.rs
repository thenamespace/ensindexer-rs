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
    }
}

pub(crate) fn domain_order_column(order_by: DomainOrderField) -> &'static str {
    match order_by {
        DomainOrderField::Id => "id",
        DomainOrderField::Name => "name",
        DomainOrderField::LabelName => "label_name",
        DomainOrderField::Labelhash => "labelhash",
        DomainOrderField::Parent => "parent_id",
        DomainOrderField::SubdomainCount => "subdomain_count",
        DomainOrderField::ResolvedAddress => "resolved_address_id",
        DomainOrderField::Resolver => "resolver_id",
        DomainOrderField::Ttl => "ttl",
        DomainOrderField::IsMigrated => "is_migrated",
        DomainOrderField::CreatedAt => "created_at",
        DomainOrderField::Owner => "owner_id",
        DomainOrderField::Registrant => "registrant_id",
        DomainOrderField::WrappedOwner => "wrapped_owner_id",
        DomainOrderField::ExpiryDate => "expiry_date",
    }
}

pub(crate) fn registration_order_column(order_by: RegistrationOrderField) -> &'static str {
    match order_by {
        RegistrationOrderField::Id => "id",
        RegistrationOrderField::Domain => "domain_id",
        RegistrationOrderField::RegistrationDate => "registration_date",
        RegistrationOrderField::ExpiryDate => "expiry_date",
        RegistrationOrderField::Cost => "cost",
        RegistrationOrderField::Registrant => "registrant_id",
        RegistrationOrderField::LabelName => "label_name",
    }
}

pub(crate) fn wrapped_domain_order_column(order_by: WrappedDomainOrderField) -> &'static str {
    match order_by {
        WrappedDomainOrderField::Id => "id",
        WrappedDomainOrderField::Domain => "domain_id",
        WrappedDomainOrderField::ExpiryDate => "expiry_date",
        WrappedDomainOrderField::Fuses => "fuses",
        WrappedDomainOrderField::Owner => "owner_id",
        WrappedDomainOrderField::Name => "name",
    }
}

pub(crate) fn resolver_order_column(order_by: ResolverOrderField) -> &'static str {
    match order_by {
        ResolverOrderField::Id => "id",
        ResolverOrderField::Domain => "domain_id",
        ResolverOrderField::Address => "address",
        ResolverOrderField::Addr => "addr_id",
        ResolverOrderField::ContentHash => "content_hash",
    }
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
