use mdbsql::mdbsql::Row;
use sqlx::{Postgres, query_builder::Separated};

pub(crate) trait Table
where
    Self: Sized,
{
    const NUM_COLUMNS: usize;
    const INSERT_QUERY: &str;
    const READ_QUERY: &str;

    fn bind_values<'a>(&self, builder: Separated<'_, 'a, Postgres, &'static str>);

    fn from_row(row: Row) -> Result<Self, mdbsql::Error>;
}
