use crate::key::NBNKey;
use sqlx::FromRow;

#[derive(Debug, PartialEq, FromRow)]
pub struct Owner {
    owner_key: NBNKey,
    owner_name: String,
    owner_shortname: String,
    home_url: Option<String>,
}

#[cfg(feature = "update")]
mod update {
    use crate::{Owner, update::Table};

    impl Table for Owner {
        const NUM_COLUMNS: usize = 4;

        const INSERT_QUERY: &str = r#"
            INSERT INTO
            uksi.owner (
                owner_key,
                owner_name,
                owner_shortname,
                home_url
            )
        "#;

        const READ_QUERY: &str = r#"
            SELECT
                OWNER_KEY,
                OWNER_NAME,
                OWNER_SHORTNAME,
                HOME_URL
            FROM
                OWNER
        "#;

        fn bind_values<'a>(
            &self,
            mut builder: sqlx::query_builder::Separated<'_, 'a, sqlx::Postgres, &'static str>,
        ) {
            builder
                .push_bind(self.owner_key.to_string())
                .push_bind(self.owner_name.to_owned())
                .push_bind(self.owner_shortname.to_owned())
                .push_bind(self.home_url.to_owned());
        }

        fn from_row(row: mdbsql::mdbsql::Row) -> Result<Self, mdbsql::Error> {
            Ok(Self {
                owner_key: row.get(0)?,
                owner_name: row.get(1)?,
                owner_shortname: row.get(2)?,
                home_url: row.get(3)?,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use fstr::FStr;
    use sqlx::{PgPool, query_as};

    use super::*;

    #[sqlx::test(fixtures("../tests/fixtures.sql"))]
    async fn owner_can_be_fetched_from_db(pool: PgPool) {
        let owner: Owner = query_as(
            r#"
            SELECT *
            FROM uksi.owner
            LIMIT 1;
        "#,
        )
        .fetch_one(&pool)
        .await
        .expect("Error fetch rows from db.");

        assert_eq!(
            owner,
            Owner {
                owner_key: NBNKey(FStr::try_from(b"TESTDATA00000001").unwrap()),
                owner_name: "Test Owner".to_string(),
                owner_shortname: "Test".to_string(),
                home_url: None
            }
        );
    }
}
