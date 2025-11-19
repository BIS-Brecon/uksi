use crate::key::NBNKey;
use sqlx::FromRow;

#[derive(Debug, PartialEq, FromRow)]
pub struct Owner {
    owner_key: NBNKey,
    owner_name: String,
    owner_shortname: String,
    home_url: Option<String>,
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
