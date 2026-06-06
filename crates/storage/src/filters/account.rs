#[derive(Debug, Clone, Default)]
pub struct AccountFilter {
    pub id: Option<String>,
    pub id_not: Option<String>,
    pub id_gt: Option<String>,
    pub id_lt: Option<String>,
    pub id_gte: Option<String>,
    pub id_lte: Option<String>,
    pub id_in: Option<Vec<String>>,
    pub id_not_in: Option<Vec<String>>,
    pub and: Option<Vec<AccountFilter>>,
    pub or: Option<Vec<AccountFilter>>,
}
