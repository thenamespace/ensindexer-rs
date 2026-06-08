use std::{collections::HashMap, future::Future};

use async_graphql::dataloader::Loader;
use storage::{AccountRow, RegistrationRow, ResolverRow, Storage, WrappedDomainRow};

#[derive(Clone)]
pub struct EntityLoader {
    storage: Storage,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct AccountKey(pub String);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ResolverKey(pub String);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RegistrationByDomainKey(pub String);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct WrappedDomainByDomainKey(pub String);

impl EntityLoader {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}

impl Loader<AccountKey> for EntityLoader {
    type Value = AccountRow;
    type Error = String;

    fn load(
        &self,
        keys: &[AccountKey],
    ) -> impl Future<Output = Result<HashMap<AccountKey, Self::Value>, Self::Error>> + Send {
        let storage = self.storage.clone();
        let ids: Vec<_> = keys.iter().map(|key| key.0.clone()).collect();
        async move {
            storage
                .accounts()
                .find_by_ids(&ids)
                .await
                .map(|rows| {
                    rows.into_iter()
                        .map(|(id, row)| (AccountKey(id), row))
                        .collect()
                })
                .map_err(|error| error.to_string())
        }
    }
}

impl Loader<ResolverKey> for EntityLoader {
    type Value = ResolverRow;
    type Error = String;

    fn load(
        &self,
        keys: &[ResolverKey],
    ) -> impl Future<Output = Result<HashMap<ResolverKey, Self::Value>, Self::Error>> + Send {
        let storage = self.storage.clone();
        let ids: Vec<_> = keys.iter().map(|key| key.0.clone()).collect();
        async move {
            storage
                .resolvers()
                .find_by_ids(&ids)
                .await
                .map(|rows| {
                    rows.into_iter()
                        .map(|(id, row)| (ResolverKey(id), row))
                        .collect()
                })
                .map_err(|error| error.to_string())
        }
    }
}

impl Loader<RegistrationByDomainKey> for EntityLoader {
    type Value = RegistrationRow;
    type Error = String;

    fn load(
        &self,
        keys: &[RegistrationByDomainKey],
    ) -> impl Future<Output = Result<HashMap<RegistrationByDomainKey, Self::Value>, Self::Error>> + Send
    {
        let storage = self.storage.clone();
        let ids: Vec<_> = keys.iter().map(|key| key.0.clone()).collect();
        async move {
            storage
                .registrations()
                .find_by_domain_ids(&ids)
                .await
                .map(|rows| {
                    rows.into_iter()
                        .map(|(id, row)| (RegistrationByDomainKey(id), row))
                        .collect()
                })
                .map_err(|error| error.to_string())
        }
    }
}

impl Loader<WrappedDomainByDomainKey> for EntityLoader {
    type Value = WrappedDomainRow;
    type Error = String;

    fn load(
        &self,
        keys: &[WrappedDomainByDomainKey],
    ) -> impl Future<Output = Result<HashMap<WrappedDomainByDomainKey, Self::Value>, Self::Error>> + Send
    {
        let storage = self.storage.clone();
        let ids: Vec<_> = keys.iter().map(|key| key.0.clone()).collect();
        async move {
            storage
                .wrapped_domains()
                .find_by_domain_ids(&ids)
                .await
                .map(|rows| {
                    rows.into_iter()
                        .map(|(id, row)| (WrappedDomainByDomainKey(id), row))
                        .collect()
                })
                .map_err(|error| error.to_string())
        }
    }
}
