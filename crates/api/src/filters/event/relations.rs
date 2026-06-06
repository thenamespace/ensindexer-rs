use async_graphql::InputObject;

use super::common::{ApplyEventFilter, EventFilter};
use crate::filters::{AccountFilter, DomainFilter, RegistrationFilter, ResolverFilter};

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct DomainRelationFilter {
    #[graphql(name = "domain")]
    pub domain: Option<String>,
    #[graphql(name = "domain_not")]
    pub domain_not: Option<String>,
    #[graphql(name = "domain_gt")]
    pub domain_gt: Option<String>,
    #[graphql(name = "domain_lt")]
    pub domain_lt: Option<String>,
    #[graphql(name = "domain_gte")]
    pub domain_gte: Option<String>,
    #[graphql(name = "domain_lte")]
    pub domain_lte: Option<String>,
    #[graphql(name = "domain_in")]
    pub domain_in: Option<Vec<String>>,
    #[graphql(name = "domain_not_in")]
    pub domain_not_in: Option<Vec<String>>,
    #[graphql(name = "domain_contains")]
    pub domain_contains: Option<String>,
    #[graphql(name = "domain_contains_nocase")]
    pub domain_contains_nocase: Option<String>,
    #[graphql(name = "domain_not_contains")]
    pub domain_not_contains: Option<String>,
    #[graphql(name = "domain_not_contains_nocase")]
    pub domain_not_contains_nocase: Option<String>,
    #[graphql(name = "domain_starts_with")]
    pub domain_starts_with: Option<String>,
    #[graphql(name = "domain_starts_with_nocase")]
    pub domain_starts_with_nocase: Option<String>,
    #[graphql(name = "domain_not_starts_with")]
    pub domain_not_starts_with: Option<String>,
    #[graphql(name = "domain_not_starts_with_nocase")]
    pub domain_not_starts_with_nocase: Option<String>,
    #[graphql(name = "domain_ends_with")]
    pub domain_ends_with: Option<String>,
    #[graphql(name = "domain_ends_with_nocase")]
    pub domain_ends_with_nocase: Option<String>,
    #[graphql(name = "domain_not_ends_with")]
    pub domain_not_ends_with: Option<String>,
    #[graphql(name = "domain_not_ends_with_nocase")]
    pub domain_not_ends_with_nocase: Option<String>,
    #[graphql(name = "domain_")]
    pub domain_filter: Option<Box<DomainFilter>>,
}

impl ApplyEventFilter for DomainRelationFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.parent_id = self.domain;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct ParentDomainRelationFilter {
    #[graphql(name = "parentDomain")]
    pub parent_domain: Option<String>,
    #[graphql(name = "parentDomain_not")]
    pub parent_domain_not: Option<String>,
    #[graphql(name = "parentDomain_gt")]
    pub parent_domain_gt: Option<String>,
    #[graphql(name = "parentDomain_lt")]
    pub parent_domain_lt: Option<String>,
    #[graphql(name = "parentDomain_gte")]
    pub parent_domain_gte: Option<String>,
    #[graphql(name = "parentDomain_lte")]
    pub parent_domain_lte: Option<String>,
    #[graphql(name = "parentDomain_in")]
    pub parent_domain_in: Option<Vec<String>>,
    #[graphql(name = "parentDomain_not_in")]
    pub parent_domain_not_in: Option<Vec<String>>,
    #[graphql(name = "parentDomain_contains")]
    pub parent_domain_contains: Option<String>,
    #[graphql(name = "parentDomain_contains_nocase")]
    pub parent_domain_contains_nocase: Option<String>,
    #[graphql(name = "parentDomain_not_contains")]
    pub parent_domain_not_contains: Option<String>,
    #[graphql(name = "parentDomain_not_contains_nocase")]
    pub parent_domain_not_contains_nocase: Option<String>,
    #[graphql(name = "parentDomain_starts_with")]
    pub parent_domain_starts_with: Option<String>,
    #[graphql(name = "parentDomain_starts_with_nocase")]
    pub parent_domain_starts_with_nocase: Option<String>,
    #[graphql(name = "parentDomain_not_starts_with")]
    pub parent_domain_not_starts_with: Option<String>,
    #[graphql(name = "parentDomain_not_starts_with_nocase")]
    pub parent_domain_not_starts_with_nocase: Option<String>,
    #[graphql(name = "parentDomain_ends_with")]
    pub parent_domain_ends_with: Option<String>,
    #[graphql(name = "parentDomain_ends_with_nocase")]
    pub parent_domain_ends_with_nocase: Option<String>,
    #[graphql(name = "parentDomain_not_ends_with")]
    pub parent_domain_not_ends_with: Option<String>,
    #[graphql(name = "parentDomain_not_ends_with_nocase")]
    pub parent_domain_not_ends_with_nocase: Option<String>,
    #[graphql(name = "parentDomain_")]
    pub parent_domain_filter: Option<Box<DomainFilter>>,
}

impl ApplyEventFilter for ParentDomainRelationFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.parent_domain_id = self.parent_domain;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct RegistrationRelationFilter {
    #[graphql(name = "registration")]
    pub registration: Option<String>,
    #[graphql(name = "registration_not")]
    pub registration_not: Option<String>,
    #[graphql(name = "registration_gt")]
    pub registration_gt: Option<String>,
    #[graphql(name = "registration_lt")]
    pub registration_lt: Option<String>,
    #[graphql(name = "registration_gte")]
    pub registration_gte: Option<String>,
    #[graphql(name = "registration_lte")]
    pub registration_lte: Option<String>,
    #[graphql(name = "registration_in")]
    pub registration_in: Option<Vec<String>>,
    #[graphql(name = "registration_not_in")]
    pub registration_not_in: Option<Vec<String>>,
    #[graphql(name = "registration_contains")]
    pub registration_contains: Option<String>,
    #[graphql(name = "registration_contains_nocase")]
    pub registration_contains_nocase: Option<String>,
    #[graphql(name = "registration_not_contains")]
    pub registration_not_contains: Option<String>,
    #[graphql(name = "registration_not_contains_nocase")]
    pub registration_not_contains_nocase: Option<String>,
    #[graphql(name = "registration_starts_with")]
    pub registration_starts_with: Option<String>,
    #[graphql(name = "registration_starts_with_nocase")]
    pub registration_starts_with_nocase: Option<String>,
    #[graphql(name = "registration_not_starts_with")]
    pub registration_not_starts_with: Option<String>,
    #[graphql(name = "registration_not_starts_with_nocase")]
    pub registration_not_starts_with_nocase: Option<String>,
    #[graphql(name = "registration_ends_with")]
    pub registration_ends_with: Option<String>,
    #[graphql(name = "registration_ends_with_nocase")]
    pub registration_ends_with_nocase: Option<String>,
    #[graphql(name = "registration_not_ends_with")]
    pub registration_not_ends_with: Option<String>,
    #[graphql(name = "registration_not_ends_with_nocase")]
    pub registration_not_ends_with_nocase: Option<String>,
    #[graphql(name = "registration_")]
    pub registration_filter: Option<Box<RegistrationFilter>>,
}

impl ApplyEventFilter for RegistrationRelationFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.parent_id = self.registration;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct ResolverRelationFilter {
    #[graphql(name = "resolver")]
    pub resolver: Option<String>,
    #[graphql(name = "resolver_not")]
    pub resolver_not: Option<String>,
    #[graphql(name = "resolver_gt")]
    pub resolver_gt: Option<String>,
    #[graphql(name = "resolver_lt")]
    pub resolver_lt: Option<String>,
    #[graphql(name = "resolver_gte")]
    pub resolver_gte: Option<String>,
    #[graphql(name = "resolver_lte")]
    pub resolver_lte: Option<String>,
    #[graphql(name = "resolver_in")]
    pub resolver_in: Option<Vec<String>>,
    #[graphql(name = "resolver_not_in")]
    pub resolver_not_in: Option<Vec<String>>,
    #[graphql(name = "resolver_contains")]
    pub resolver_contains: Option<String>,
    #[graphql(name = "resolver_contains_nocase")]
    pub resolver_contains_nocase: Option<String>,
    #[graphql(name = "resolver_not_contains")]
    pub resolver_not_contains: Option<String>,
    #[graphql(name = "resolver_not_contains_nocase")]
    pub resolver_not_contains_nocase: Option<String>,
    #[graphql(name = "resolver_starts_with")]
    pub resolver_starts_with: Option<String>,
    #[graphql(name = "resolver_starts_with_nocase")]
    pub resolver_starts_with_nocase: Option<String>,
    #[graphql(name = "resolver_not_starts_with")]
    pub resolver_not_starts_with: Option<String>,
    #[graphql(name = "resolver_not_starts_with_nocase")]
    pub resolver_not_starts_with_nocase: Option<String>,
    #[graphql(name = "resolver_ends_with")]
    pub resolver_ends_with: Option<String>,
    #[graphql(name = "resolver_ends_with_nocase")]
    pub resolver_ends_with_nocase: Option<String>,
    #[graphql(name = "resolver_not_ends_with")]
    pub resolver_not_ends_with: Option<String>,
    #[graphql(name = "resolver_not_ends_with_nocase")]
    pub resolver_not_ends_with_nocase: Option<String>,
    #[graphql(name = "resolver_")]
    pub resolver_filter: Option<Box<ResolverFilter>>,
}

impl ApplyEventFilter for ResolverRelationFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.parent_id = self.resolver;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct NewResolverRelationFilter {
    #[graphql(name = "resolver")]
    pub resolver: Option<String>,
    #[graphql(name = "resolver_not")]
    pub resolver_not: Option<String>,
    #[graphql(name = "resolver_gt")]
    pub resolver_gt: Option<String>,
    #[graphql(name = "resolver_lt")]
    pub resolver_lt: Option<String>,
    #[graphql(name = "resolver_gte")]
    pub resolver_gte: Option<String>,
    #[graphql(name = "resolver_lte")]
    pub resolver_lte: Option<String>,
    #[graphql(name = "resolver_in")]
    pub resolver_in: Option<Vec<String>>,
    #[graphql(name = "resolver_not_in")]
    pub resolver_not_in: Option<Vec<String>>,
    #[graphql(name = "resolver_contains")]
    pub resolver_contains: Option<String>,
    #[graphql(name = "resolver_contains_nocase")]
    pub resolver_contains_nocase: Option<String>,
    #[graphql(name = "resolver_not_contains")]
    pub resolver_not_contains: Option<String>,
    #[graphql(name = "resolver_not_contains_nocase")]
    pub resolver_not_contains_nocase: Option<String>,
    #[graphql(name = "resolver_starts_with")]
    pub resolver_starts_with: Option<String>,
    #[graphql(name = "resolver_starts_with_nocase")]
    pub resolver_starts_with_nocase: Option<String>,
    #[graphql(name = "resolver_not_starts_with")]
    pub resolver_not_starts_with: Option<String>,
    #[graphql(name = "resolver_not_starts_with_nocase")]
    pub resolver_not_starts_with_nocase: Option<String>,
    #[graphql(name = "resolver_ends_with")]
    pub resolver_ends_with: Option<String>,
    #[graphql(name = "resolver_ends_with_nocase")]
    pub resolver_ends_with_nocase: Option<String>,
    #[graphql(name = "resolver_not_ends_with")]
    pub resolver_not_ends_with: Option<String>,
    #[graphql(name = "resolver_not_ends_with_nocase")]
    pub resolver_not_ends_with_nocase: Option<String>,
    #[graphql(name = "resolver_")]
    pub resolver_filter: Option<Box<ResolverFilter>>,
}

impl ApplyEventFilter for NewResolverRelationFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.resolver_id = self.resolver;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct OwnerRelationFilter {
    #[graphql(name = "owner")]
    pub owner: Option<String>,
    #[graphql(name = "owner_not")]
    pub owner_not: Option<String>,
    #[graphql(name = "owner_gt")]
    pub owner_gt: Option<String>,
    #[graphql(name = "owner_lt")]
    pub owner_lt: Option<String>,
    #[graphql(name = "owner_gte")]
    pub owner_gte: Option<String>,
    #[graphql(name = "owner_lte")]
    pub owner_lte: Option<String>,
    #[graphql(name = "owner_in")]
    pub owner_in: Option<Vec<String>>,
    #[graphql(name = "owner_not_in")]
    pub owner_not_in: Option<Vec<String>>,
    #[graphql(name = "owner_contains")]
    pub owner_contains: Option<String>,
    #[graphql(name = "owner_contains_nocase")]
    pub owner_contains_nocase: Option<String>,
    #[graphql(name = "owner_not_contains")]
    pub owner_not_contains: Option<String>,
    #[graphql(name = "owner_not_contains_nocase")]
    pub owner_not_contains_nocase: Option<String>,
    #[graphql(name = "owner_starts_with")]
    pub owner_starts_with: Option<String>,
    #[graphql(name = "owner_starts_with_nocase")]
    pub owner_starts_with_nocase: Option<String>,
    #[graphql(name = "owner_not_starts_with")]
    pub owner_not_starts_with: Option<String>,
    #[graphql(name = "owner_not_starts_with_nocase")]
    pub owner_not_starts_with_nocase: Option<String>,
    #[graphql(name = "owner_ends_with")]
    pub owner_ends_with: Option<String>,
    #[graphql(name = "owner_ends_with_nocase")]
    pub owner_ends_with_nocase: Option<String>,
    #[graphql(name = "owner_not_ends_with")]
    pub owner_not_ends_with: Option<String>,
    #[graphql(name = "owner_not_ends_with_nocase")]
    pub owner_not_ends_with_nocase: Option<String>,
    #[graphql(name = "owner_")]
    pub owner_filter: Option<Box<AccountFilter>>,
}

impl ApplyEventFilter for OwnerRelationFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.owner_id = self.owner;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct RegistrantRelationFilter {
    #[graphql(name = "registrant")]
    pub registrant: Option<String>,
    #[graphql(name = "registrant_not")]
    pub registrant_not: Option<String>,
    #[graphql(name = "registrant_gt")]
    pub registrant_gt: Option<String>,
    #[graphql(name = "registrant_lt")]
    pub registrant_lt: Option<String>,
    #[graphql(name = "registrant_gte")]
    pub registrant_gte: Option<String>,
    #[graphql(name = "registrant_lte")]
    pub registrant_lte: Option<String>,
    #[graphql(name = "registrant_in")]
    pub registrant_in: Option<Vec<String>>,
    #[graphql(name = "registrant_not_in")]
    pub registrant_not_in: Option<Vec<String>>,
    #[graphql(name = "registrant_contains")]
    pub registrant_contains: Option<String>,
    #[graphql(name = "registrant_contains_nocase")]
    pub registrant_contains_nocase: Option<String>,
    #[graphql(name = "registrant_not_contains")]
    pub registrant_not_contains: Option<String>,
    #[graphql(name = "registrant_not_contains_nocase")]
    pub registrant_not_contains_nocase: Option<String>,
    #[graphql(name = "registrant_starts_with")]
    pub registrant_starts_with: Option<String>,
    #[graphql(name = "registrant_starts_with_nocase")]
    pub registrant_starts_with_nocase: Option<String>,
    #[graphql(name = "registrant_not_starts_with")]
    pub registrant_not_starts_with: Option<String>,
    #[graphql(name = "registrant_not_starts_with_nocase")]
    pub registrant_not_starts_with_nocase: Option<String>,
    #[graphql(name = "registrant_ends_with")]
    pub registrant_ends_with: Option<String>,
    #[graphql(name = "registrant_ends_with_nocase")]
    pub registrant_ends_with_nocase: Option<String>,
    #[graphql(name = "registrant_not_ends_with")]
    pub registrant_not_ends_with: Option<String>,
    #[graphql(name = "registrant_not_ends_with_nocase")]
    pub registrant_not_ends_with_nocase: Option<String>,
    #[graphql(name = "registrant_")]
    pub registrant_filter: Option<Box<AccountFilter>>,
}

impl ApplyEventFilter for RegistrantRelationFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.registrant_id = self.registrant;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct NewOwnerRelationFilter {
    #[graphql(name = "newOwner")]
    pub new_owner: Option<String>,
    #[graphql(name = "newOwner_not")]
    pub new_owner_not: Option<String>,
    #[graphql(name = "newOwner_gt")]
    pub new_owner_gt: Option<String>,
    #[graphql(name = "newOwner_lt")]
    pub new_owner_lt: Option<String>,
    #[graphql(name = "newOwner_gte")]
    pub new_owner_gte: Option<String>,
    #[graphql(name = "newOwner_lte")]
    pub new_owner_lte: Option<String>,
    #[graphql(name = "newOwner_in")]
    pub new_owner_in: Option<Vec<String>>,
    #[graphql(name = "newOwner_not_in")]
    pub new_owner_not_in: Option<Vec<String>>,
    #[graphql(name = "newOwner_contains")]
    pub new_owner_contains: Option<String>,
    #[graphql(name = "newOwner_contains_nocase")]
    pub new_owner_contains_nocase: Option<String>,
    #[graphql(name = "newOwner_not_contains")]
    pub new_owner_not_contains: Option<String>,
    #[graphql(name = "newOwner_not_contains_nocase")]
    pub new_owner_not_contains_nocase: Option<String>,
    #[graphql(name = "newOwner_starts_with")]
    pub new_owner_starts_with: Option<String>,
    #[graphql(name = "newOwner_starts_with_nocase")]
    pub new_owner_starts_with_nocase: Option<String>,
    #[graphql(name = "newOwner_not_starts_with")]
    pub new_owner_not_starts_with: Option<String>,
    #[graphql(name = "newOwner_not_starts_with_nocase")]
    pub new_owner_not_starts_with_nocase: Option<String>,
    #[graphql(name = "newOwner_ends_with")]
    pub new_owner_ends_with: Option<String>,
    #[graphql(name = "newOwner_ends_with_nocase")]
    pub new_owner_ends_with_nocase: Option<String>,
    #[graphql(name = "newOwner_not_ends_with")]
    pub new_owner_not_ends_with: Option<String>,
    #[graphql(name = "newOwner_not_ends_with_nocase")]
    pub new_owner_not_ends_with_nocase: Option<String>,
    #[graphql(name = "newOwner_")]
    pub new_owner_filter: Option<Box<AccountFilter>>,
}

impl ApplyEventFilter for NewOwnerRelationFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.new_owner_id = self.new_owner;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct AddrAccountRelationFilter {
    #[graphql(name = "addr")]
    pub addr: Option<String>,
    #[graphql(name = "addr_not")]
    pub addr_not: Option<String>,
    #[graphql(name = "addr_gt")]
    pub addr_gt: Option<String>,
    #[graphql(name = "addr_lt")]
    pub addr_lt: Option<String>,
    #[graphql(name = "addr_gte")]
    pub addr_gte: Option<String>,
    #[graphql(name = "addr_lte")]
    pub addr_lte: Option<String>,
    #[graphql(name = "addr_in")]
    pub addr_in: Option<Vec<String>>,
    #[graphql(name = "addr_not_in")]
    pub addr_not_in: Option<Vec<String>>,
    #[graphql(name = "addr_contains")]
    pub addr_contains: Option<String>,
    #[graphql(name = "addr_contains_nocase")]
    pub addr_contains_nocase: Option<String>,
    #[graphql(name = "addr_not_contains")]
    pub addr_not_contains: Option<String>,
    #[graphql(name = "addr_not_contains_nocase")]
    pub addr_not_contains_nocase: Option<String>,
    #[graphql(name = "addr_starts_with")]
    pub addr_starts_with: Option<String>,
    #[graphql(name = "addr_starts_with_nocase")]
    pub addr_starts_with_nocase: Option<String>,
    #[graphql(name = "addr_not_starts_with")]
    pub addr_not_starts_with: Option<String>,
    #[graphql(name = "addr_not_starts_with_nocase")]
    pub addr_not_starts_with_nocase: Option<String>,
    #[graphql(name = "addr_ends_with")]
    pub addr_ends_with: Option<String>,
    #[graphql(name = "addr_ends_with_nocase")]
    pub addr_ends_with_nocase: Option<String>,
    #[graphql(name = "addr_not_ends_with")]
    pub addr_not_ends_with: Option<String>,
    #[graphql(name = "addr_not_ends_with_nocase")]
    pub addr_not_ends_with_nocase: Option<String>,
    #[graphql(name = "addr_")]
    pub addr_filter: Option<Box<AccountFilter>>,
}

impl ApplyEventFilter for AddrAccountRelationFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.addr_id = self.addr;
    }
}
