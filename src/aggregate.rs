use crate::{date::VagueDate, key::NBNKey};
use sqlx::{FromRow, Row, postgres::PgRow, types::chrono::NaiveDateTime};

#[derive(Debug, PartialEq)]
pub struct Aggregate {
    aggregate_id_key: NBNKey,
    aggregate_tvk: NBNKey,
    component_tvk: NBNKey,
    source: String,
    notes: Option<String>,
    vague_date: Option<VagueDate>,
    entered_by: NBNKey,
    entry_date: NaiveDateTime,
    changed_by: Option<NBNKey>,
    changed_date: Option<NaiveDateTime>,
}

impl FromRow<'_, PgRow> for Aggregate {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            aggregate_id_key: row.try_get("aggregate_id_key")?,
            aggregate_tvk: row.try_get("aggregate_tvk")?,
            component_tvk: row.try_get("component_tvk")?,
            source: row.try_get("source")?,
            notes: row.try_get("notes")?,
            vague_date: VagueDate::from_row(row, Some("vague_date"))?,
            entered_by: row.try_get("entered_by")?,
            entry_date: row.try_get("entry_date")?,
            changed_by: row.try_get("changed_by")?,
            changed_date: row.try_get("changed_date")?,
        })
    }
}

#[cfg(feature = "update")]
mod update {
    use crate::{
        Aggregate,
        update::{Table, access},
    };

    impl Table for Aggregate {
        const NUM_COLUMNS: usize = 12;

        const INSERT_QUERY: &str = r#"
            INSERT INTO uksi.aggregate (
                aggregate_id_key,
                aggregate_tvk,
                component_tvk,
                source,
                notes,
                vague_date_start,
                vague_date_end,
                vague_date_type,
                entered_by,
                entry_date,
                changed_by,
                changed_date
            )
        "#;

        const READ_QUERY: &str = r#"
            SELECT
                AGGREGATE_ID_KEY,
                AGGREGATE_TVK,
                COMPONENT_TVK,
                SOURCE,
                NOTES,
                VAGUE_DATE_START,
                VAGUE_DATE_END,
                VAGUE_DATE_TYPE,
                ENTERED_BY,
                ENTRY_DATE,
                CHANGED_BY,
                CHANGED_DATE
            FROM
                AGGREGATE
        "#;

        fn bind_values<'a>(
            &self,
            mut builder: sqlx::query_builder::Separated<'_, 'a, sqlx::Postgres, &'static str>,
        ) {
            builder
                .push_bind(self.aggregate_id_key.to_owned())
                .push_bind(self.aggregate_tvk.to_owned())
                .push_bind(self.component_tvk.to_owned())
                .push_bind(self.source.to_owned())
                .push_bind(self.notes.to_owned())
                .push_bind(self.vague_date.as_ref().map(|date| date.start))
                .push_bind(self.vague_date.as_ref().map(|date| date.end))
                .push_bind(self.vague_date.as_ref().map(|date| date.date_type))
                .push_bind(self.entered_by.to_string())
                .push_bind(self.entry_date.to_owned())
                .push_bind(self.changed_by.as_ref().map(|key| key.to_string()))
                .push_bind(self.changed_date.to_owned());
        }

        fn from_row(row: mdbsql::mdbsql::Row) -> Result<Self, mdbsql::Error> {
            let vague_date = access::vague_date_from_row(&row, 5)?;

            Ok(Self {
                aggregate_id_key: row.get(0)?,
                aggregate_tvk: row.get(1)?,
                component_tvk: row.get(2)?,
                source: row.get(3)?,
                notes: row.get(4)?,
                vague_date,
                entered_by: row.get(8)?,
                entry_date: access::datetime_from_row(&row, 9)?.ok_or(
                    mdbsql::Error::FromSqlError(serde_plain::Error::Parse(
                        "Empty date",
                        format!("Expected date was empty."),
                    )),
                )?,
                changed_by: row.get(10)?,
                changed_date: access::datetime_from_row(&row, 11)?,
            })
        }
    }
}
