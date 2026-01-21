use crate::key::NBNKey;
use sqlx::{FromRow, types::chrono::NaiveDateTime};

#[derive(Debug, PartialEq, FromRow)]
pub struct TaxonDesignationType {
    taxon_designation_type_key: NBNKey,
    short_name: String,
    long_name: String,
    description: Option<String>,
    kind: String,
    entered_by: NBNKey,
    entry_date: NaiveDateTime,
    changed_by: Option<NBNKey>,
    changed_date: Option<NaiveDateTime>,
    system_supplied_data: bool,
}

#[cfg(feature = "update")]
mod update {
    use crate::{
        designation::TaxonDesignationType,
        update::{Table, access},
    };

    impl Table for TaxonDesignationType {
        const NUM_COLUMNS: usize = 10;

        const INSERT_QUERY: &str = r#"
            INSERT INTO uksi.taxon_designation_type (
                taxon_designation_type_key,
                short_name,
                long_name,
                description,
                kind,
                entered_by,
                entry_date,
                changed_by,
                changed_date,
                system_supplied_data
            )
        "#;

        const READ_QUERY: &str = r#"
            SELECT
                TAXON_DESIGNATION_TYPE_KEY,
                SHORT_NAME,
                LONG_NAME,
                DESCRIPTION,
                KIND,
                ENTERED_BY,
                ENTRY_DATE,
                CHANGED_BY,
                CHANGED_DATE,
                SYSTEM_SUPPLIED_DATA
            FROM
                TAXON_DESIGNATION_TYPE
        "#;

        fn bind_values<'a>(
            &self,
            mut builder: sqlx::query_builder::Separated<'_, 'a, sqlx::Postgres, &'static str>,
        ) {
            builder
                .push_bind(self.taxon_designation_type_key.to_string())
                .push_bind(self.short_name.to_owned())
                .push_bind(self.long_name.to_owned())
                .push_bind(self.description.as_ref().map(|s| s.to_owned()))
                .push_bind(self.kind.to_owned())
                .push_bind(self.entered_by.to_string())
                .push_bind(self.entry_date.to_owned())
                .push_bind(self.changed_by.as_ref().map(|key| key.to_string()))
                .push_bind(self.changed_date.to_owned())
                .push_bind(self.system_supplied_data);
        }

        fn from_row(row: mdbsql::mdbsql::Row) -> Result<Self, mdbsql::Error> {
            Ok(Self {
                taxon_designation_type_key: row.get(0)?,
                short_name: row.get(1)?,
                long_name: row.get(2)?,
                description: row.get(3)?,
                kind: row.get(4)?,
                entered_by: row.get(5)?,
                entry_date: access::datetime_from_row(&row, 6)?.ok_or(
                    mdbsql::Error::FromSqlError(serde_plain::Error::Parse(
                        "Empty date",
                        format!("Expected date was empty."),
                    )),
                )?,
                changed_by: row.get(7)?,
                changed_date: access::datetime_from_row(&row, 8)?,
                system_supplied_data: access::bool_from_row(&row, 9)?,
            })
        }
    }
}
