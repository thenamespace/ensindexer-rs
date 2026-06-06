use async_graphql::InputObject;

use crate::filters::event::common::{ApplyEventFilter, EventFilter};

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct FusesFieldFilter {
    #[graphql(name = "fuses")]
    pub fuses: Option<i32>,
    #[graphql(name = "fuses_not")]
    pub fuses_not: Option<i32>,
    #[graphql(name = "fuses_gt")]
    pub fuses_gt: Option<i32>,
    #[graphql(name = "fuses_lt")]
    pub fuses_lt: Option<i32>,
    #[graphql(name = "fuses_gte")]
    pub fuses_gte: Option<i32>,
    #[graphql(name = "fuses_lte")]
    pub fuses_lte: Option<i32>,
    #[graphql(name = "fuses_in")]
    pub fuses_in: Option<Vec<i32>>,
    #[graphql(name = "fuses_not_in")]
    pub fuses_not_in: Option<Vec<i32>>,
}

impl ApplyEventFilter for FusesFieldFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.fuses = self.fuses;
        filter.fuses_not = self.fuses_not;
        filter.fuses_gt = self.fuses_gt;
        filter.fuses_lt = self.fuses_lt;
        filter.fuses_gte = self.fuses_gte;
        filter.fuses_lte = self.fuses_lte;
        filter.fuses_in = self.fuses_in;
        filter.fuses_not_in = self.fuses_not_in;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct TtlFieldFilter {
    #[graphql(name = "ttl")]
    pub ttl: Option<String>,
    #[graphql(name = "ttl_not")]
    pub ttl_not: Option<String>,
    #[graphql(name = "ttl_gt")]
    pub ttl_gt: Option<String>,
    #[graphql(name = "ttl_lt")]
    pub ttl_lt: Option<String>,
    #[graphql(name = "ttl_gte")]
    pub ttl_gte: Option<String>,
    #[graphql(name = "ttl_lte")]
    pub ttl_lte: Option<String>,
    #[graphql(name = "ttl_in")]
    pub ttl_in: Option<Vec<String>>,
    #[graphql(name = "ttl_not_in")]
    pub ttl_not_in: Option<Vec<String>>,
}

impl ApplyEventFilter for TtlFieldFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.ttl = self.ttl;
        filter.ttl_not = self.ttl_not;
        filter.ttl_gt = self.ttl_gt;
        filter.ttl_lt = self.ttl_lt;
        filter.ttl_gte = self.ttl_gte;
        filter.ttl_lte = self.ttl_lte;
        filter.ttl_in = self.ttl_in;
        filter.ttl_not_in = self.ttl_not_in;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct ExpiryDateFieldFilter {
    #[graphql(name = "expiryDate")]
    pub expiry_date: Option<String>,
    #[graphql(name = "expiryDate_not")]
    pub expiry_date_not: Option<String>,
    #[graphql(name = "expiryDate_gt")]
    pub expiry_date_gt: Option<String>,
    #[graphql(name = "expiryDate_lt")]
    pub expiry_date_lt: Option<String>,
    #[graphql(name = "expiryDate_gte")]
    pub expiry_date_gte: Option<String>,
    #[graphql(name = "expiryDate_lte")]
    pub expiry_date_lte: Option<String>,
    #[graphql(name = "expiryDate_in")]
    pub expiry_date_in: Option<Vec<String>>,
    #[graphql(name = "expiryDate_not_in")]
    pub expiry_date_not_in: Option<Vec<String>>,
}

impl ApplyEventFilter for ExpiryDateFieldFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.expiry_date = self.expiry_date;
        filter.expiry_date_not = self.expiry_date_not;
        filter.expiry_date_gt = self.expiry_date_gt;
        filter.expiry_date_lt = self.expiry_date_lt;
        filter.expiry_date_gte = self.expiry_date_gte;
        filter.expiry_date_lte = self.expiry_date_lte;
        filter.expiry_date_in = self.expiry_date_in;
        filter.expiry_date_not_in = self.expiry_date_not_in;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct CoinTypeFieldFilter {
    #[graphql(name = "coinType")]
    pub coin_type: Option<String>,
    #[graphql(name = "coinType_not")]
    pub coin_type_not: Option<String>,
    #[graphql(name = "coinType_gt")]
    pub coin_type_gt: Option<String>,
    #[graphql(name = "coinType_lt")]
    pub coin_type_lt: Option<String>,
    #[graphql(name = "coinType_gte")]
    pub coin_type_gte: Option<String>,
    #[graphql(name = "coinType_lte")]
    pub coin_type_lte: Option<String>,
    #[graphql(name = "coinType_in")]
    pub coin_type_in: Option<Vec<String>>,
    #[graphql(name = "coinType_not_in")]
    pub coin_type_not_in: Option<Vec<String>>,
}

impl ApplyEventFilter for CoinTypeFieldFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.coin_type = self.coin_type;
        filter.coin_type_not = self.coin_type_not;
        filter.coin_type_gt = self.coin_type_gt;
        filter.coin_type_lt = self.coin_type_lt;
        filter.coin_type_gte = self.coin_type_gte;
        filter.coin_type_lte = self.coin_type_lte;
        filter.coin_type_in = self.coin_type_in;
        filter.coin_type_not_in = self.coin_type_not_in;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct ContentTypeFieldFilter {
    #[graphql(name = "contentType")]
    pub content_type: Option<String>,
    #[graphql(name = "contentType_not")]
    pub content_type_not: Option<String>,
    #[graphql(name = "contentType_gt")]
    pub content_type_gt: Option<String>,
    #[graphql(name = "contentType_lt")]
    pub content_type_lt: Option<String>,
    #[graphql(name = "contentType_gte")]
    pub content_type_gte: Option<String>,
    #[graphql(name = "contentType_lte")]
    pub content_type_lte: Option<String>,
    #[graphql(name = "contentType_in")]
    pub content_type_in: Option<Vec<String>>,
    #[graphql(name = "contentType_not_in")]
    pub content_type_not_in: Option<Vec<String>>,
}

impl ApplyEventFilter for ContentTypeFieldFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.content_type = self.content_type;
        filter.content_type_not = self.content_type_not;
        filter.content_type_gt = self.content_type_gt;
        filter.content_type_lt = self.content_type_lt;
        filter.content_type_gte = self.content_type_gte;
        filter.content_type_lte = self.content_type_lte;
        filter.content_type_in = self.content_type_in;
        filter.content_type_not_in = self.content_type_not_in;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct VersionFieldFilter {
    #[graphql(name = "version")]
    pub version: Option<String>,
    #[graphql(name = "version_not")]
    pub version_not: Option<String>,
    #[graphql(name = "version_gt")]
    pub version_gt: Option<String>,
    #[graphql(name = "version_lt")]
    pub version_lt: Option<String>,
    #[graphql(name = "version_gte")]
    pub version_gte: Option<String>,
    #[graphql(name = "version_lte")]
    pub version_lte: Option<String>,
    #[graphql(name = "version_in")]
    pub version_in: Option<Vec<String>>,
    #[graphql(name = "version_not_in")]
    pub version_not_in: Option<Vec<String>>,
}

impl ApplyEventFilter for VersionFieldFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.version = self.version;
        filter.version_not = self.version_not;
        filter.version_gt = self.version_gt;
        filter.version_lt = self.version_lt;
        filter.version_gte = self.version_gte;
        filter.version_lte = self.version_lte;
        filter.version_in = self.version_in;
        filter.version_not_in = self.version_not_in;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct IsAuthorizedFieldFilter {
    #[graphql(name = "isAuthorized")]
    pub is_authorized: Option<bool>,
    #[graphql(name = "isAuthorized_not")]
    pub is_authorized_not: Option<bool>,
    #[graphql(name = "isAuthorized_in")]
    pub is_authorized_in: Option<Vec<bool>>,
    #[graphql(name = "isAuthorized_not_in")]
    pub is_authorized_not_in: Option<Vec<bool>>,
}

impl ApplyEventFilter for IsAuthorizedFieldFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.is_authorized = self.is_authorized;
        filter.is_authorized_not = self.is_authorized_not;
        filter.is_authorized_in = self.is_authorized_in;
        filter.is_authorized_not_in = self.is_authorized_not_in;
    }
}
