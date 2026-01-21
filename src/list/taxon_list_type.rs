use crate::key::NBNKey;
use sqlx::{FromRow, types::chrono::NaiveDateTime};

#[derive(Debug, PartialEq, FromRow)]
pub struct TaxonListType {
    taxon_list_type_key: NBNKey,
    short_name: String,
    long_name: String,
    description: Option<String>,
    schedule: bool,
    entered_by: NBNKey,
    entry_date: NaiveDateTime,
    system_supplied_data: bool,
}

#[cfg(feature = "update")]
mod update {
    use crate::{
        list::TaxonListType,
        update::{Table, access},
    };

    impl Table for TaxonListType {
        const NUM_COLUMNS: usize = 8;

        const INSERT_QUERY: &str = r#"
            INSERT INTO uksi.taxon_list_type (
                taxon_list_type_key,
                short_name,
                long_name,
                description,
                schedule,
                entered_by,
                entry_date,
                system_supplied_data
            )
        "#;

        const READ_QUERY: &str = r#"
            SELECT
                TAXON_LIST_TYPE_KEY,
                SHORT_NAME,
                LONG_NAME,
                DESCRIPTION,
                SCHEDULE,
                ENTERED_BY,
                ENTRY_DATE,
                SYSTEM_SUPPLIED_DATA
            FROM
                TAXON_LIST_TYPE
        "#;

        fn bind_values<'a>(
            &self,
            mut builder: sqlx::query_builder::Separated<'_, 'a, sqlx::Postgres, &'static str>,
        ) {
            builder
                .push_bind(self.taxon_list_type_key.to_string())
                .push_bind(self.short_name.to_owned())
                .push_bind(self.long_name.to_owned())
                .push_bind(self.description.to_owned())
                .push_bind(self.schedule.to_owned())
                .push_bind(self.entered_by.to_string())
                .push_bind(self.entry_date.to_owned())
                .push_bind(self.system_supplied_data);
        }

        fn from_row(row: mdbsql::mdbsql::Row) -> Result<Self, mdbsql::Error> {
            Ok(Self {
                taxon_list_type_key: row.get(0)?,
                short_name: row.get(1)?,
                long_name: row.get(2)?,
                description: row.get(3)?,
                schedule: access::bool_from_row(&row, 4)?,
                entered_by: row.get(5)?,
                entry_date: access::datetime_from_row(&row, 6)?.ok_or(
                    mdbsql::Error::FromSqlError(serde_plain::Error::Parse(
                        "Empty date",
                        format!("Expected date was empty."),
                    )),
                )?,
                system_supplied_data: access::bool_from_row(&row, 7)?,
            })
        }
    }
}
