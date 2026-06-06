use async_graphql::InputObject;
use storage::RegistrationFilter as StorageRegistrationFilter;

use super::{AccountFilter, DomainFilter, extras::RegistrationFilterExtras};

#[derive(Debug, Clone, InputObject, Default)]
#[graphql(name = "Registration_filter")]
pub struct RegistrationFilter {
    pub id: Option<String>,
    #[graphql(name = "id_not")]
    pub id_not: Option<String>,
    #[graphql(name = "id_in")]
    pub id_in: Option<Vec<String>>,
    #[graphql(name = "id_not_in")]
    pub id_not_in: Option<Vec<String>>,
    pub domain: Option<String>,
    #[graphql(name = "domain_")]
    pub domain_filter: Option<Box<DomainFilter>>,
    pub registrant: Option<String>,
    #[graphql(name = "registrant_")]
    pub registrant_filter: Option<Box<AccountFilter>>,
    #[graphql(name = "labelName")]
    pub label_name: Option<String>,
    #[graphql(name = "labelName_contains")]
    pub label_name_contains: Option<String>,
    #[graphql(name = "labelName_contains_nocase")]
    pub label_name_contains_nocase: Option<String>,
    #[graphql(name = "labelName_starts_with")]
    pub label_name_starts_with: Option<String>,
    #[graphql(name = "labelName_ends_with")]
    pub label_name_ends_with: Option<String>,
    #[graphql(name = "registrationDate")]
    pub registration_date: Option<String>,
    #[graphql(name = "registrationDate_gt")]
    pub registration_date_gt: Option<String>,
    #[graphql(name = "registrationDate_lt")]
    pub registration_date_lt: Option<String>,
    #[graphql(name = "registrationDate_gte")]
    pub registration_date_gte: Option<String>,
    #[graphql(name = "registrationDate_lte")]
    pub registration_date_lte: Option<String>,
    #[graphql(name = "expiryDate")]
    pub expiry_date: Option<String>,
    #[graphql(name = "expiryDate_gt")]
    pub expiry_date_gt: Option<String>,
    #[graphql(name = "expiryDate_lt")]
    pub expiry_date_lt: Option<String>,
    #[graphql(name = "expiryDate_gte")]
    pub expiry_date_gte: Option<String>,
    #[graphql(name = "expiryDate_lte")]
    pub expiry_date_lte: Option<String>,
    pub cost: Option<String>,
    #[graphql(name = "cost_gt")]
    pub cost_gt: Option<String>,
    #[graphql(name = "cost_lt")]
    pub cost_lt: Option<String>,
    #[graphql(name = "cost_gte")]
    pub cost_gte: Option<String>,
    #[graphql(name = "cost_lte")]
    pub cost_lte: Option<String>,
    #[graphql(flatten)]
    extras: RegistrationFilterExtras,
}

impl From<RegistrationFilter> for StorageRegistrationFilter {
    fn from(value: RegistrationFilter) -> Self {
        let extras = value.extras;
        Self {
            id: value.id,
            id_not: value.id_not,
            id_gt: extras.id_gt,
            id_lt: extras.id_lt,
            id_gte: extras.id_gte,
            id_lte: extras.id_lte,
            id_in: value.id_in,
            id_not_in: value.id_not_in,
            domain_id: value.domain,
            domain_id_not: extras.domain_not,
            domain_id_gt: extras.domain_gt,
            domain_id_lt: extras.domain_lt,
            domain_id_gte: extras.domain_gte,
            domain_id_lte: extras.domain_lte,
            domain_id_in: extras.domain_in,
            domain_id_not_in: extras.domain_not_in,
            domain_id_contains: extras.domain_contains,
            domain_id_contains_nocase: extras.domain_contains_nocase,
            domain_id_not_contains: extras.domain_not_contains,
            domain_id_not_contains_nocase: extras.domain_not_contains_nocase,
            domain_id_starts_with: extras.domain_starts_with,
            domain_id_starts_with_nocase: extras.domain_starts_with_nocase,
            domain_id_not_starts_with: extras.domain_not_starts_with,
            domain_id_not_starts_with_nocase: extras.domain_not_starts_with_nocase,
            domain_id_ends_with: extras.domain_ends_with,
            domain_id_ends_with_nocase: extras.domain_ends_with_nocase,
            domain_id_not_ends_with: extras.domain_not_ends_with,
            domain_id_not_ends_with_nocase: extras.domain_not_ends_with_nocase,
            domain_filter: value.domain_filter.map(|filter| Box::new((*filter).into())),
            registrant_id: value.registrant,
            registrant_id_not: extras.registrant_not,
            registrant_id_gt: extras.registrant_gt,
            registrant_id_lt: extras.registrant_lt,
            registrant_id_gte: extras.registrant_gte,
            registrant_id_lte: extras.registrant_lte,
            registrant_id_in: extras.registrant_in,
            registrant_id_not_in: extras.registrant_not_in,
            registrant_id_contains: extras.registrant_contains,
            registrant_id_contains_nocase: extras.registrant_contains_nocase,
            registrant_id_not_contains: extras.registrant_not_contains,
            registrant_id_not_contains_nocase: extras.registrant_not_contains_nocase,
            registrant_id_starts_with: extras.registrant_starts_with,
            registrant_id_starts_with_nocase: extras.registrant_starts_with_nocase,
            registrant_id_not_starts_with: extras.registrant_not_starts_with,
            registrant_id_not_starts_with_nocase: extras.registrant_not_starts_with_nocase,
            registrant_id_ends_with: extras.registrant_ends_with,
            registrant_id_ends_with_nocase: extras.registrant_ends_with_nocase,
            registrant_id_not_ends_with: extras.registrant_not_ends_with,
            registrant_id_not_ends_with_nocase: extras.registrant_not_ends_with_nocase,
            registrant_filter: value
                .registrant_filter
                .map(|filter| Box::new((*filter).into())),
            label_name: value.label_name,
            label_name_not: extras.label_name_not,
            label_name_gt: extras.label_name_gt,
            label_name_lt: extras.label_name_lt,
            label_name_gte: extras.label_name_gte,
            label_name_lte: extras.label_name_lte,
            label_name_in: extras.label_name_in,
            label_name_not_in: extras.label_name_not_in,
            label_name_contains: value.label_name_contains,
            label_name_contains_nocase: value.label_name_contains_nocase,
            label_name_not_contains: extras.label_name_not_contains,
            label_name_not_contains_nocase: extras.label_name_not_contains_nocase,
            label_name_starts_with: value.label_name_starts_with,
            label_name_starts_with_nocase: extras.label_name_starts_with_nocase,
            label_name_not_starts_with: extras.label_name_not_starts_with,
            label_name_not_starts_with_nocase: extras.label_name_not_starts_with_nocase,
            label_name_ends_with: value.label_name_ends_with,
            label_name_ends_with_nocase: extras.label_name_ends_with_nocase,
            label_name_not_ends_with: extras.label_name_not_ends_with,
            label_name_not_ends_with_nocase: extras.label_name_not_ends_with_nocase,
            registration_date: value.registration_date,
            registration_date_not: extras.registration_date_not,
            registration_date_gt: value.registration_date_gt,
            registration_date_lt: value.registration_date_lt,
            registration_date_gte: value.registration_date_gte,
            registration_date_lte: value.registration_date_lte,
            registration_date_in: extras.registration_date_in,
            registration_date_not_in: extras.registration_date_not_in,
            expiry_date: value.expiry_date,
            expiry_date_not: extras.expiry_date_not,
            expiry_date_gt: value.expiry_date_gt,
            expiry_date_lt: value.expiry_date_lt,
            expiry_date_gte: value.expiry_date_gte,
            expiry_date_lte: value.expiry_date_lte,
            expiry_date_in: extras.expiry_date_in,
            expiry_date_not_in: extras.expiry_date_not_in,
            cost: value.cost,
            cost_not: extras.cost_not,
            cost_gt: value.cost_gt,
            cost_lt: value.cost_lt,
            cost_gte: value.cost_gte,
            cost_lte: value.cost_lte,
            cost_in: extras.cost_in,
            cost_not_in: extras.cost_not_in,
        }
    }
}
