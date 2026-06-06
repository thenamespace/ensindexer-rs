use async_graphql::InputObject;

use crate::filters::event::common::{ApplyEventFilter, EventFilter};

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct InterfaceIdFieldFilter {
    #[graphql(name = "interfaceID")]
    pub interface_id: Option<String>,
    #[graphql(name = "interfaceID_not")]
    pub interface_id_not: Option<String>,
    #[graphql(name = "interfaceID_gt")]
    pub interface_id_gt: Option<String>,
    #[graphql(name = "interfaceID_lt")]
    pub interface_id_lt: Option<String>,
    #[graphql(name = "interfaceID_gte")]
    pub interface_id_gte: Option<String>,
    #[graphql(name = "interfaceID_lte")]
    pub interface_id_lte: Option<String>,
    #[graphql(name = "interfaceID_in")]
    pub interface_id_in: Option<Vec<String>>,
    #[graphql(name = "interfaceID_not_in")]
    pub interface_id_not_in: Option<Vec<String>>,
    #[graphql(name = "interfaceID_contains")]
    pub interface_id_contains: Option<String>,
    #[graphql(name = "interfaceID_not_contains")]
    pub interface_id_not_contains: Option<String>,
}

impl ApplyEventFilter for InterfaceIdFieldFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.interface_id = self.interface_id;
        filter.interface_id_not = self.interface_id_not;
        filter.interface_id_gt = self.interface_id_gt;
        filter.interface_id_lt = self.interface_id_lt;
        filter.interface_id_gte = self.interface_id_gte;
        filter.interface_id_lte = self.interface_id_lte;
        filter.interface_id_in = self.interface_id_in;
        filter.interface_id_not_in = self.interface_id_not_in;
        filter.interface_id_contains = self.interface_id_contains;
        filter.interface_id_not_contains = self.interface_id_not_contains;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct ImplementerFieldFilter {
    #[graphql(name = "implementer")]
    pub implementer: Option<String>,
    #[graphql(name = "implementer_not")]
    pub implementer_not: Option<String>,
    #[graphql(name = "implementer_gt")]
    pub implementer_gt: Option<String>,
    #[graphql(name = "implementer_lt")]
    pub implementer_lt: Option<String>,
    #[graphql(name = "implementer_gte")]
    pub implementer_gte: Option<String>,
    #[graphql(name = "implementer_lte")]
    pub implementer_lte: Option<String>,
    #[graphql(name = "implementer_in")]
    pub implementer_in: Option<Vec<String>>,
    #[graphql(name = "implementer_not_in")]
    pub implementer_not_in: Option<Vec<String>>,
    #[graphql(name = "implementer_contains")]
    pub implementer_contains: Option<String>,
    #[graphql(name = "implementer_not_contains")]
    pub implementer_not_contains: Option<String>,
}

impl ApplyEventFilter for ImplementerFieldFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.implementer = self.implementer;
        filter.implementer_not = self.implementer_not;
        filter.implementer_gt = self.implementer_gt;
        filter.implementer_lt = self.implementer_lt;
        filter.implementer_gte = self.implementer_gte;
        filter.implementer_lte = self.implementer_lte;
        filter.implementer_in = self.implementer_in;
        filter.implementer_not_in = self.implementer_not_in;
        filter.implementer_contains = self.implementer_contains;
        filter.implementer_not_contains = self.implementer_not_contains;
    }
}

#[derive(Debug, Clone, InputObject, Default)]
pub(crate) struct TargetFieldFilter {
    #[graphql(name = "target")]
    pub target: Option<String>,
    #[graphql(name = "target_not")]
    pub target_not: Option<String>,
    #[graphql(name = "target_gt")]
    pub target_gt: Option<String>,
    #[graphql(name = "target_lt")]
    pub target_lt: Option<String>,
    #[graphql(name = "target_gte")]
    pub target_gte: Option<String>,
    #[graphql(name = "target_lte")]
    pub target_lte: Option<String>,
    #[graphql(name = "target_in")]
    pub target_in: Option<Vec<String>>,
    #[graphql(name = "target_not_in")]
    pub target_not_in: Option<Vec<String>>,
    #[graphql(name = "target_contains")]
    pub target_contains: Option<String>,
    #[graphql(name = "target_not_contains")]
    pub target_not_contains: Option<String>,
}

impl ApplyEventFilter for TargetFieldFilter {
    fn apply(self, filter: &mut EventFilter) {
        filter.target = self.target;
        filter.target_not = self.target_not;
        filter.target_gt = self.target_gt;
        filter.target_lt = self.target_lt;
        filter.target_gte = self.target_gte;
        filter.target_lte = self.target_lte;
        filter.target_in = self.target_in;
        filter.target_not_in = self.target_not_in;
        filter.target_contains = self.target_contains;
        filter.target_not_contains = self.target_not_contains;
    }
}
