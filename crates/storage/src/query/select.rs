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
        DomainOrderField::SubdomainCount => "subdomain_count",
        DomainOrderField::CreatedAt => "created_at",
        DomainOrderField::ExpiryDate => "expiry_date",
    }
}

pub(crate) fn registration_order_column(order_by: RegistrationOrderField) -> &'static str {
    match order_by {
        RegistrationOrderField::Id => "id",
        RegistrationOrderField::RegistrationDate => "registration_date",
        RegistrationOrderField::ExpiryDate => "expiry_date",
        RegistrationOrderField::Cost => "cost",
        RegistrationOrderField::LabelName => "label_name",
    }
}

pub(crate) fn wrapped_domain_order_column(order_by: WrappedDomainOrderField) -> &'static str {
    match order_by {
        WrappedDomainOrderField::Id => "id",
        WrappedDomainOrderField::ExpiryDate => "expiry_date",
        WrappedDomainOrderField::Fuses => "fuses",
        WrappedDomainOrderField::Name => "name",
    }
}

pub(crate) fn resolver_order_column(order_by: ResolverOrderField) -> &'static str {
    match order_by {
        ResolverOrderField::Id => "id",
        ResolverOrderField::Address => "address",
    }
}

pub(crate) fn event_order_column(order_by: EventOrderField) -> &'static str {
    match order_by {
        EventOrderField::Id => "id",
        EventOrderField::BlockNumber => "block_number",
    }
}
