use async_graphql::InputObject;
use storage::RegistrationFilter as StorageRegistrationFilter;

use super::{AccountFilter, DomainFilter};

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
}

impl From<RegistrationFilter> for StorageRegistrationFilter {
    fn from(value: RegistrationFilter) -> Self {
        Self {
            id: value.id,
            id_not: value.id_not,
            id_in: value.id_in,
            id_not_in: value.id_not_in,
            domain_id: value.domain,
            domain_filter: value.domain_filter.map(|filter| Box::new((*filter).into())),
            registrant_id: value.registrant,
            registrant_filter: value
                .registrant_filter
                .map(|filter| Box::new((*filter).into())),
            label_name: value.label_name,
            label_name_contains: value.label_name_contains,
            label_name_contains_nocase: value.label_name_contains_nocase,
            label_name_starts_with: value.label_name_starts_with,
            label_name_ends_with: value.label_name_ends_with,
            registration_date: value.registration_date,
            registration_date_gt: value.registration_date_gt,
            registration_date_lt: value.registration_date_lt,
            registration_date_gte: value.registration_date_gte,
            registration_date_lte: value.registration_date_lte,
            expiry_date: value.expiry_date,
            expiry_date_gt: value.expiry_date_gt,
            expiry_date_lt: value.expiry_date_lt,
            expiry_date_gte: value.expiry_date_gte,
            expiry_date_lte: value.expiry_date_lte,
            cost: value.cost,
            cost_gt: value.cost_gt,
            cost_lt: value.cost_lt,
            cost_gte: value.cost_gte,
            cost_lte: value.cost_lte,
        }
    }
}
