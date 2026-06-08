use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use sqlx::PgPool;

use crate::{StorageError, StorageResult};

pub struct LabelPreimagesRepo<'a> {
    pub(crate) pool: &'a PgPool,
    pub(crate) cache: Arc<Mutex<HashMap<String, Option<String>>>>,
}

impl LabelPreimagesRepo<'_> {
    pub async fn upsert(&self, labelhash: &str, label_name: &str) -> StorageResult<()> {
        sqlx::query(
            r#"
            insert into label_preimages (labelhash, label_name)
            values ($1, $2)
            on conflict (labelhash) do update set label_name = excluded.label_name
            "#,
        )
        .bind(labelhash)
        .bind(label_name)
        .execute(self.pool)
        .await?;
        self.cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?
            .insert(labelhash.to_owned(), Some(label_name.to_owned()));
        Ok(())
    }

    pub async fn find(&self, labelhash: &str) -> StorageResult<Option<String>> {
        if let Some(cached) = self
            .cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?
            .get(labelhash)
            .cloned()
        {
            return Ok(cached);
        }

        let label_name = sqlx::query_scalar(
            r#"
            select label_name
            from label_preimages
            where labelhash = $1
            "#,
        )
        .bind(labelhash)
        .fetch_optional(self.pool)
        .await?;
        self.cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?
            .insert(labelhash.to_owned(), label_name.clone());
        Ok(label_name)
    }

    pub async fn upsert_many(&self, labels: &[(String, String)]) -> StorageResult<()> {
        if labels.is_empty() {
            return Ok(());
        }

        let mut query =
            sqlx::QueryBuilder::new("insert into label_preimages (labelhash, label_name) values ");
        query.push_values(labels, |mut row, (labelhash, label_name)| {
            row.push_bind(labelhash).push_bind(label_name);
        });
        query.push(" on conflict (labelhash) do update set label_name = excluded.label_name");
        query.build().execute(self.pool).await?;
        let mut cache = self
            .cache
            .lock()
            .map_err(|_| StorageError::EntityCachePoisoned)?;
        for (labelhash, label_name) in labels {
            cache.insert(labelhash.clone(), Some(label_name.clone()));
        }
        Ok(())
    }
}
