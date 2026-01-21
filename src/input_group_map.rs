use crate::key::NBNKey;
use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Debug, PartialEq, FromRow)]
pub(crate) struct InputGroupMap {
    input_group_map_key: NBNKey,
    taxon_group_key: NBNKey,
    taxon_version_key: NBNKey,
    entered_by: NBNKey,
    entry_date: NaiveDateTime,
    changed_by: Option<NBNKey>,
    changed_date: Option<NaiveDateTime>,
}

#[cfg(feature = "update")]
mod update {
    use crate::{
        input_group_map::InputGroupMap,
        update::{Table, access},
    };

    impl Table for InputGroupMap {
        const NUM_COLUMNS: usize = 7;

        const INSERT_QUERY: &str = r#"
            INSERT INTO
            uksi.input_group_map (
                input_group_map_key,
                taxon_group_key,
                taxon_version_key,
                entered_by,
                entry_date,
                changed_by,
                changed_date
            )
        "#;

        const READ_QUERY: &str = r#"
            SELECT
                INPUT_GROUP_MAP_KEY,
                TAXON_GROUP_KEY,
                TAXON_VERSION_KEY,
                ENTERED_BY,
                ENTRY_DATE,
                CHANGED_BY,
                CHANGED_DATE
            FROM
                INPUT_GROUP_MAP
        "#;

        fn bind_values<'a>(
            &self,
            mut builder: sqlx::query_builder::Separated<'_, 'a, sqlx::Postgres, &'static str>,
        ) {
            builder
                .push_bind(self.input_group_map_key.to_string())
                .push_bind(self.taxon_group_key.to_string())
                .push_bind(self.taxon_version_key.to_string())
                .push_bind(self.entered_by.to_string())
                .push_bind(self.entry_date.to_owned())
                .push_bind(self.changed_by.as_ref().map(|key| key.to_string()))
                .push_bind(self.changed_date.to_owned());
        }

        fn from_row(row: mdbsql::mdbsql::Row) -> Result<Self, mdbsql::Error> {
            Ok(Self {
                input_group_map_key: row.get(0)?,
                taxon_group_key: row.get(1)?,
                taxon_version_key: row.get(2)?,
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
