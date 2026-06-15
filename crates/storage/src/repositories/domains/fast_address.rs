use sqlx::{Postgres, QueryBuilder};

use super::DomainsRepo;
use crate::{
    error::*,
    filters::{DomainFilter, DomainOrderField, OrderDirection},
    models::DomainRow,
    query::{domain_filter_has_conditions, domain_order_column, domain_select_sql},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AddressLookupFilter {
    address: String,
    expiry_gt: Option<String>,
}

impl DomainsRepo<'_> {
    pub(crate) async fn list_for_address_fast(
        &self,
        first: i64,
        skip: i64,
        lookup: AddressLookupFilter,
        order_by: DomainOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<DomainRow>> {
        let mut query = QueryBuilder::<Postgres>::new("");
        query.push(domain_select_sql());
        query.push(" where (owner_id = ");
        query.push_bind(&lookup.address);
        query.push(" or registrant_id = ");
        query.push_bind(&lookup.address);
        query.push(" or wrapped_owner_id = ");
        query.push_bind(&lookup.address);
        query.push(" or resolved_address_id = ");
        query.push_bind(&lookup.address);
        query.push(")");

        if let Some(expiry_gt) = lookup.expiry_gt {
            query.push(" and (expiry_date > ");
            query.push_bind(expiry_gt);
            query.push("::numeric or expiry_date is null)");
        }

        query
            .push(" order by ")
            .push(domain_order_column(order_by))
            .push(" ")
            .push(direction.sql())
            .push(", id asc limit ")
            .push_bind(first)
            .push(" offset ")
            .push_bind(skip);

        Ok(query.build_query_as().fetch_all(self.pool).await?)
    }
}

pub(crate) fn detect_address_lookup_filter(filter: &DomainFilter) -> Option<AddressLookupFilter> {
    let mut root = filter.clone();
    let and_filters = root.and.take()?;
    root.or = None;
    if domain_filter_has_conditions(&root) {
        return None;
    }

    let mut address = None;
    let mut expiry_gt = None;
    for child in and_filters {
        if let Some(candidate) = detect_address_or_filter(&child) {
            merge_address(&mut address, candidate)?;
            continue;
        }
        if let Some(candidate) = detect_expiry_filter(&child) {
            if expiry_gt.replace(candidate).is_some() {
                return None;
            }
            continue;
        }
        return None;
    }

    address.map(|address| AddressLookupFilter { address, expiry_gt })
}

fn detect_address_or_filter(filter: &DomainFilter) -> Option<String> {
    let mut root = filter.clone();
    let or_filters = root.or.take()?;
    if domain_filter_has_conditions(&root) {
        return None;
    }

    let mut address = None;
    for child in or_filters {
        let Some(candidate) = detect_single_address_filter(&child) else {
            continue;
        };
        merge_address(&mut address, candidate)?;
    }

    address
}

fn detect_single_address_filter(filter: &DomainFilter) -> Option<String> {
    let mut remaining = filter.clone();
    let candidates = [
        remaining.owner_id.take(),
        remaining.registrant_id.take(),
        remaining.wrapped_owner_id.take(),
        remaining.resolved_address_id.take(),
    ];
    if domain_filter_has_conditions(&remaining) {
        return None;
    }

    let mut values = candidates.into_iter().flatten();
    let value = values.next()?;
    if values.next().is_some() {
        return None;
    }
    Some(value)
}

fn detect_expiry_filter(filter: &DomainFilter) -> Option<String> {
    if let Some(value) = detect_single_expiry_filter(filter) {
        return Some(value);
    }

    let mut root = filter.clone();
    let or_filters = root.or.take()?;
    if domain_filter_has_conditions(&root) {
        return None;
    }

    let mut expiry_gt = None;
    for child in or_filters {
        if let Some(candidate) = detect_single_expiry_filter(&child)
            && expiry_gt.replace(candidate).is_some()
        {
            return None;
        }
    }
    expiry_gt
}

fn detect_single_expiry_filter(filter: &DomainFilter) -> Option<String> {
    let mut remaining = filter.clone();
    let expiry_gt = remaining.expiry_date_gt.take()?;
    if domain_filter_has_conditions(&remaining) {
        return None;
    }
    Some(expiry_gt)
}

fn merge_address(address: &mut Option<String>, candidate: String) -> Option<()> {
    match address {
        Some(existing) if existing != &candidate => None,
        Some(_) => Some(()),
        None => {
            *address = Some(candidate);
            Some(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ADDRESS: &str = "0xd8da6bf26964af9d7eed9e03e53415d37aa96045";

    #[test]
    fn detects_ensjs_address_lookup_shape() {
        let filter = DomainFilter {
            and: Some(vec![
                DomainFilter {
                    or: Some(vec![
                        DomainFilter {
                            owner_id: Some(ADDRESS.to_owned()),
                            ..DomainFilter::default()
                        },
                        DomainFilter {
                            registrant_id: Some(ADDRESS.to_owned()),
                            ..DomainFilter::default()
                        },
                        DomainFilter {
                            wrapped_owner_id: Some(ADDRESS.to_owned()),
                            ..DomainFilter::default()
                        },
                        DomainFilter {
                            resolved_address_id: Some(ADDRESS.to_owned()),
                            ..DomainFilter::default()
                        },
                    ]),
                    ..DomainFilter::default()
                },
                DomainFilter {
                    or: Some(vec![
                        DomainFilter {
                            expiry_date_gt: Some("1780000000".to_owned()),
                            ..DomainFilter::default()
                        },
                        DomainFilter::default(),
                    ]),
                    ..DomainFilter::default()
                },
            ]),
            ..DomainFilter::default()
        };

        assert_eq!(
            detect_address_lookup_filter(&filter),
            Some(AddressLookupFilter {
                address: ADDRESS.to_owned(),
                expiry_gt: Some("1780000000".to_owned())
            })
        );
    }

    #[test]
    fn rejects_address_lookup_shape_with_extra_conditions() {
        let filter = DomainFilter {
            and: Some(vec![DomainFilter {
                or: Some(vec![DomainFilter {
                    owner_id: Some(ADDRESS.to_owned()),
                    name_contains: Some("eth".to_owned()),
                    ..DomainFilter::default()
                }]),
                ..DomainFilter::default()
            }]),
            ..DomainFilter::default()
        };

        assert_eq!(detect_address_lookup_filter(&filter), None);
    }
}
