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

    pub async fn repairable_labelhashes(&self, limit: i64) -> StorageResult<Vec<String>> {
        Ok(sqlx::query_scalar(
            r#"
            select distinct domain.labelhash
            from domains as domain
            join label_preimages as preimage on preimage.labelhash = domain.labelhash
            where domain.labelhash is not null
              and domain.label_name like '[%'
              and domain.label_name is distinct from preimage.label_name
            order by domain.labelhash
            limit $1
            "#,
        )
        .bind(limit)
        .fetch_all(self.pool)
        .await?)
    }

    pub async fn upsert_many(&self, labels: &[(String, String)]) -> StorageResult<()> {
        if labels.is_empty() {
            return Ok(());
        }

        let mut query =
            sqlx::QueryBuilder::new("insert into label_preimages (labelhash, label_name) ");
        query.push_values(labels, |mut row, (labelhash, label_name)| {
            row.push_bind(labelhash).push_bind(label_name);
        });
        query.push(" on conflict (labelhash) do update set label_name = excluded.label_name");
        query.build().execute(self.pool).await?;
        {
            let mut cache = self
                .cache
                .lock()
                .map_err(|_| StorageError::EntityCachePoisoned)?;
            for (labelhash, label_name) in labels {
                cache.insert(labelhash.clone(), Some(label_name.clone()));
            }
        }
        Ok(())
    }

    pub async fn repair_domain_names_for_labelhashes(
        &self,
        labelhashes: &[String],
        max_passes: usize,
    ) -> StorageResult<u64> {
        if labelhashes.is_empty() {
            return Ok(0);
        }

        let mut changed_ids = sqlx::query_scalar::<_, String>(
            r#"
            update domains as domain
            set
              label_name = preimage.label_name,
              name = case
                when domain.parent_id is not null then preimage.label_name || '.' || (
                  select parent.name from domains as parent where parent.id = domain.parent_id
                )
                else preimage.label_name
              end
            from label_preimages as preimage
            where domain.labelhash = any($1)
              and domain.labelhash = preimage.labelhash
              and (
                domain.label_name is distinct from preimage.label_name
                or domain.name is distinct from case
                  when domain.parent_id is not null then preimage.label_name || '.' || (
                    select parent.name from domains as parent where parent.id = domain.parent_id
                  )
                  else preimage.label_name
                end
              )
              and (
                domain.parent_id is null
                or exists (
                  select 1 from domains as parent
                  where parent.id = domain.parent_id
                    and parent.name is not null
                )
              )
            returning domain.id
            "#,
        )
        .bind(labelhashes)
        .fetch_all(self.pool)
        .await?;

        let mut total = changed_ids.len() as u64;
        for _ in 0..max_passes.saturating_sub(1) {
            if changed_ids.is_empty() {
                break;
            }
            changed_ids = sqlx::query_scalar::<_, String>(
                r#"
                update domains as child
                set name = child.label_name || '.' || parent.name
                from domains as parent
                where child.parent_id = parent.id
                  and child.parent_id = any($1)
                  and child.label_name is not null
                  and parent.name is not null
                  and child.name is distinct from child.label_name || '.' || parent.name
                returning child.id
                "#,
            )
            .bind(&changed_ids)
            .fetch_all(self.pool)
            .await?;
            total += changed_ids.len() as u64;
        }
        Ok(total)
    }
}
