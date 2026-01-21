use crate::key::NBNKey;
use sqlx::{FromRow, types::chrono::NaiveDateTime};

#[derive(Debug, PartialEq, FromRow)]
pub struct TaxonDesignationTypeKind {
    taxon_designation_type_kind_key: NBNKey,
    kind: Option<String>,
    item_name: Option<String>,
    entered_by: NBNKey,
    entry_date: NaiveDateTime,
    changed_by: Option<NBNKey>,
    changed_date: Option<NaiveDateTime>,
}

#[cfg(feature = "update")]
mod update {
    use crate::{
        designation::TaxonDesignationTypeKind,
        update::{Table, access},
    };

    impl Table for TaxonDesignationTypeKind {
        const NUM_COLUMNS: usize = 7;

        const INSERT_QUERY: &str = r#"
            INSERT INTO uksi.taxon_designation_type_kind (
                taxon_designation_type_kind_key,
                kind,
                item_name,
                entered_by,
                entry_date,
                changed_by,
                changed_date
            )
        "#;

        const READ_QUERY: &str = r#"
            SELECT
                TAXON_DESIGNATION_TYPE_KIND_KEY,
                KIND,
                ITEM_NAME,
                ENTERED_BY,
                ENTRY_DATE,
                CHANGED_BY,
                CHANGED_DATE
            FROM
                TAXON_DESIGNATION_TYPE_KIND
        "#;

        fn bind_values<'a>(
            &self,
            mut builder: sqlx::query_builder::Separated<'_, 'a, sqlx::Postgres, &'static str>,
        ) {
            builder
                .push_bind(self.taxon_designation_type_kind_key.to_string())
                .push_bind(self.kind.to_owned())
                .push_bind(self.item_name.to_owned())
                .push_bind(self.entered_by.to_string())
                .push_bind(self.entry_date.to_owned())
                .push_bind(self.changed_by.as_ref().map(|key| key.to_string()))
                .push_bind(self.changed_date.to_owned());
        }

        fn from_row(row: mdbsql::mdbsql::Row) -> Result<Self, mdbsql::Error> {
            Ok(Self {
                taxon_designation_type_kind_key: row.get(0)?,
                kind: row.get(1)?,
                item_name: row.get(2)?,
                entered_by: row.get(3)?,
                entry_date: access::datetime_from_row(&row, 4)?.ok_or(
                    mdbsql::Error::FromSqlError(serde_plain::Error::Parse(
                        "Empty date",
                        format!("Expected date was empty."),
                    )),
                )?,
                changed_by: row.get(5)?,
                changed_date: access::datetime_from_row(&row, 6)?,
            })
        }
    }
}
