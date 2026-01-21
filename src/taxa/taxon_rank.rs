use crate::key::NBNKey;
use sqlx::{FromRow, types::chrono::NaiveDateTime};

#[derive(Debug, PartialEq, FromRow)]
pub struct TaxonRank {
    taxon_rank_key: NBNKey,
    sequence: i16,
    short_name: Option<String>,
    long_name: String,
    abbreviation: Option<String>,
    description: Option<String>,
    list_font_italic: bool,
    display_in_details: bool,
    entered_by: NBNKey,
    entry_date: NaiveDateTime,
    changed_by: Option<NBNKey>,
    changed_date: Option<NaiveDateTime>,
    system_supplied_data: bool,
}

#[cfg(feature = "update")]
mod update {
    use crate::{
        taxa::TaxonRank,
        update::{Table, access},
    };

    impl Table for TaxonRank {
        const NUM_COLUMNS: usize = 13;

        const INSERT_QUERY: &str = r#"
            INSERT INTO uksi.taxon_rank (
                taxon_rank_key,
                sequence,
                short_name,
                long_name,
                abbreviation,
                description,
                list_font_italic,
                display_in_details,
                entered_by,
                entry_date,
                changed_by,
                changed_date,
                system_supplied_data
            )
        "#;

        const READ_QUERY: &str = r#"
            SELECT
                TAXON_RANK_KEY,
                SEQUENCE,
                SHORT_NAME,
                LONG_NAME,
                ABBREVIATION,
                DESCRIPTION,
                LIST_FONT_ITALIC,
                DISPLAY_IN_DETAILS,
                ENTERED_BY,
                ENTRY_DATE,
                CHANGED_BY,
                CHANGED_DATE,
                SYSTEM_SUPPLIED_DATA
            FROM
                TAXON_RANK
        "#;

        fn bind_values<'a>(
            &self,
            mut builder: sqlx::query_builder::Separated<'_, 'a, sqlx::Postgres, &'static str>,
        ) {
            builder
                .push_bind(self.taxon_rank_key.to_string())
                .push_bind(self.sequence.to_owned())
                .push_bind(self.short_name.to_owned())
                .push_bind(self.long_name.to_owned())
                .push_bind(self.abbreviation.to_owned())
                .push_bind(self.description.to_owned())
                .push_bind(self.list_font_italic.to_owned())
                .push_bind(self.display_in_details.to_owned())
                .push_bind(self.entered_by.to_string())
                .push_bind(self.entry_date.to_owned())
                .push_bind(self.changed_by.as_ref().map(|key| key.to_string()))
                .push_bind(self.changed_date.to_owned())
                .push_bind(self.system_supplied_data);
        }

        fn from_row(row: mdbsql::mdbsql::Row) -> Result<Self, mdbsql::Error> {
            Ok(Self {
                taxon_rank_key: row.get(0)?,
                sequence: row.get(1)?,
                short_name: row.get(2)?,
                long_name: row.get(3)?,
                abbreviation: row.get(4)?,
                description: row.get(5)?,
                list_font_italic: access::bool_from_row(&row, 6)?,
                display_in_details: access::bool_from_row(&row, 7)?,
                entered_by: row.get(8)?,
                entry_date: access::datetime_from_row(&row, 9)?.ok_or(
                    mdbsql::Error::FromSqlError(serde_plain::Error::Parse(
                        "Empty date",
                        format!("Expected date was empty."),
                    )),
                )?,
                changed_by: row.get(10)?,
                changed_date: access::datetime_from_row(&row, 11)?,
                system_supplied_data: access::bool_from_row(&row, 12)?,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use fstr::FStr;
    use sqlx::{PgPool, query_as};

    use super::*;

    #[sqlx::test(fixtures("../../tests/fixtures.sql"))]
    async fn taxon_rank_can_be_fetched_from_db(pool: PgPool) {
        let taxon_rank: TaxonRank = query_as(
            r#"
            SELECT *
            FROM uksi.taxon_rank
            WHERE taxon_rank_key = 'NBNSYS0000000002';
        "#,
        )
        .fetch_one(&pool)
        .await
        .expect("Error fetch rows from db.");

        assert_eq!(
            taxon_rank,
            TaxonRank {
                taxon_rank_key: NBNKey(FStr::try_from(b"NBNSYS0000000002").unwrap()),
                sequence: 10,
                short_name: Some("Kng".to_string()),
                long_name: "Kingdom".to_string(),
                list_font_italic: false,
                display_in_details: true,
                abbreviation: None,
                description: None,
                entered_by: NBNKey(FStr::try_from(b"TESTDATA00000001").unwrap()),
                entry_date: NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(1999, 01, 01).unwrap(),
                    NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap()
                ),
                changed_by: None,
                changed_date: None,
                system_supplied_data: true
            }
        );
    }
}
