use serde::{Deserialize, Serialize};
use struct_field_names::StructFieldNames;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, StructFieldNames)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Template {
    pub unid: Uuid,
    pub name: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[cfg(feature = "ssr")]
mod db {
    use sqlx::{PgPool, query_as};

    use super::*;

    impl Template {
        pub async fn get_all(pool: &PgPool) -> Result<Vec<Template>, sqlx::Error> {
            query_as!(
                Template,
                r#"
                    SELECT
                        unid,
                        name,
                        created_at,
                        updated_at
                    FROM templates
                "#
            )
            .fetch_all(pool)
            .await
        }

        pub async fn add(pool: &PgPool, name: String) -> Result<Template, sqlx::Error> {
            query_as!(
                Template,
                r#"
                    INSERT INTO templates (name)
                    VALUES ($1)
                    RETURNING unid, name, created_at, updated_at
                "#,
                name
            )
            .fetch_one(pool)
            .await
        }

        pub async fn delete(pool: &PgPool, unid: Uuid) -> Result<Uuid, sqlx::Error> {
            let result = sqlx::query!(
                r#"
                    DELETE FROM templates
                    WHERE unid = $1
                    RETURNING unid
                "#,
                unid
            )
            .fetch_one(pool)
            .await?;

            Ok(result.unid)
        }
    }
}
