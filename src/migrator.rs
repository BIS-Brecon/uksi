use sqlx::{Acquire, Postgres, migrate::Migrator};

static MIGRATOR: Migrator = sqlx::migrate!();

/// Runs UKSI migrations.
///
/// These migrations must be applied before any user-defined migrations which
/// reference the uksi tables.
///
/// A transaction is acquired via the provided connection and migrations are run
/// via this transaction.
///
/// As there is no direct support for specifying the schema under which the
/// migrations table will live, we manually specify this via the search path.
/// This ensures that migrations are isolated to uksi._sqlx_migrations.
///
/// **Note**: Changes are managed within a dedicated schema, called "uksi".
///
/// # Example
///
///```rust,no_run
/// # use tokio::runtime::Runtime;
/// use std::env;
///
/// use sqlx::PgPool;
///
/// # fn main() {
/// # let rt = Runtime::new().unwrap();
/// # rt.block_on(async {
/// // Set up the database connection pool.
/// let database_url = &env::var("DATABASE_URL")?;
/// let pool = PgPool::connect(database_url).await?;
///
/// // Run migrations.
/// uksi::run_migrations(&pool).await?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// # });
/// # }
pub async fn run_migrations<'a, A>(conn: A) -> Result<(), sqlx::Error>
where
    A: Acquire<'a, Database = Postgres>,
{
    let mut tx = conn.begin().await?;

    // Ensure the 'uksi' schema exists
    sqlx::query!("create schema if not exists uksi;")
        .execute(&mut *tx)
        .await?;

    // Temporarily set search_path for this transaction
    sqlx::query!("set local search_path to uksi;")
        .execute(&mut *tx)
        .await?;

    // Run migrations within the 'uksi' schema
    MIGRATOR.run(&mut *tx).await?;

    tx.commit().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use super::run_migrations;

    #[sqlx::test(migrations = false)]
    async fn sanity_check_run_migrations(pool: PgPool) -> Result<(), sqlx::Error> {
        run_migrations(&pool).await?;

        let schema_exists: bool = sqlx::query_scalar!(
            r#"
            select exists (
              select 1 from pg_namespace where nspname = 'uksi'
            );
            "#,
        )
        .fetch_one(&pool)
        .await?
        .unwrap();
        assert!(
            schema_exists,
            "Schema 'uksi' should exist after migrations."
        );

        let migrations_table_exists: bool = sqlx::query_scalar!(
            r#"
            select exists (
                select 1 from information_schema.tables
                where table_schema = 'uksi' and
                      table_name = '_sqlx_migrations'
            );
            "#,
        )
        .fetch_one(&pool)
        .await?
        .unwrap();
        assert!(
            migrations_table_exists,
            "Migrations table should exist in 'uksi' schema."
        );

        let search_path: String = sqlx::query_scalar("show search_path;")
            .fetch_one(&pool)
            .await?;

        assert!(
            !search_path.contains("uksi"),
            "search_path should not include 'uksi' after the transaction."
        );

        assert!(
            search_path.contains("public"),
            "Default search_path should include 'public'."
        );

        Ok(())
    }
}
