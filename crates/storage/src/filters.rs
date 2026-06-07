mod account;
mod domain;
mod event;
mod order;
mod registration;
mod resolver;
mod wrapped_domain;

#[derive(Debug, Clone, Default)]
pub struct TextOperatorFilter {
    pub not: Option<String>,
    pub gt: Option<String>,
    pub lt: Option<String>,
    pub gte: Option<String>,
    pub lte: Option<String>,
    pub in_values: Option<Vec<String>>,
    pub not_in: Option<Vec<String>>,
    pub contains: Option<String>,
    pub contains_nocase: Option<String>,
    pub not_contains: Option<String>,
    pub not_contains_nocase: Option<String>,
    pub starts_with: Option<String>,
    pub starts_with_nocase: Option<String>,
    pub not_starts_with: Option<String>,
    pub not_starts_with_nocase: Option<String>,
    pub ends_with: Option<String>,
    pub ends_with_nocase: Option<String>,
    pub not_ends_with: Option<String>,
    pub not_ends_with_nocase: Option<String>,
}

pub use account::*;
pub use domain::*;
pub use event::*;
pub use order::*;
pub use registration::*;
pub use resolver::*;
pub use wrapped_domain::*;
