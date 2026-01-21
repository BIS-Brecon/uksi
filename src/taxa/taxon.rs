use crate::{date::VagueDate, key::NBNKey};
use sqlx::{FromRow, Row, postgres::PgRow, types::chrono::NaiveDateTime};

#[derive(Debug, PartialEq)]
pub struct Taxon {
    taxon_key: NBNKey,
    item_name: String,
    authority: Option<String>,
    introduced: Option<VagueDate>,
    language: String,
    taxon_name_type_key: NBNKey,
    abbreviation: Option<String>,
    entered_by: NBNKey,
    entry_date: NaiveDateTime,
    changed_by: Option<NBNKey>,
    changed_date: Option<NaiveDateTime>,
    system_supplied_data: bool,
}

impl FromRow<'_, PgRow> for Taxon {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            taxon_key: row.try_get("taxon_key")?,
            item_name: row.try_get("item_name")?,
            authority: row.try_get("authority")?,
            introduced: VagueDate::from_row(row, Some("introduced"))?,
            language: row.try_get("language")?,
            taxon_name_type_key: row.try_get("taxon_name_type_key")?,
            abbreviation: row.try_get("abbreviation")?,
            entered_by: row.try_get("entered_by")?,
            entry_date: row.try_get("entry_date")?,
            changed_by: row.try_get("changed_by")?,
            changed_date: row.try_get("changed_date")?,
            system_supplied_data: row.try_get("system_supplied_data")?,
        })
    }
}

#[cfg(feature = "update")]
mod update {
    use crate::{
        taxa::Taxon,
        update::{Table, access},
    };

    impl Table for Taxon {
        const NUM_COLUMNS: usize = 14;

        const INSERT_QUERY: &str = r#"
            INSERT INTO uksi.taxon (
                taxon_key,
                item_name,
                authority,
                introduced_vague_date_start,
                introduced_vague_date_end,
                introduced_vague_date_type,
                language,
                taxon_name_type_key,
                abbreviation,
                entered_by,
                entry_date,
                changed_by,
                changed_date,
                system_supplied_data
            )
        "#;

        const READ_QUERY: &str = r#"
            SELECT
                TAXON_KEY,
                ITEM_NAME,
                AUTHORITY,
                INTRODUCED_VAGUE_DATE_START,
                INTRODUCED_VAGUE_DATE_END,
                INTRODUCED_VAGUE_DATE_TYPE,
                LANGUAGE,
                TAXON_NAME_TYPE_KEY,
                ABBREVIATION,
                ENTERED_BY,
                ENTRY_DATE,
                CHANGED_BY,
                CHANGED_DATE,
                SYSTEM_SUPPLIED_DATA
            FROM
                TAXON
        "#;

        fn bind_values<'a>(
            &self,
            mut builder: sqlx::query_builder::Separated<'_, 'a, sqlx::Postgres, &'static str>,
        ) {
            builder
                .push_bind(self.taxon_key.to_string())
                .push_bind(self.item_name.to_owned())
                .push_bind(self.authority.to_owned())
                .push_bind(self.introduced.as_ref().map(|date| date.start))
                .push_bind(self.introduced.as_ref().map(|date| date.end))
                .push_bind(self.introduced.as_ref().map(|date| date.date_type))
                .push_bind(self.language.to_owned())
                .push_bind(self.taxon_name_type_key.to_string())
                .push_bind(self.abbreviation.to_owned())
                .push_bind(self.entered_by.to_string())
                .push_bind(self.entry_date.to_owned())
                .push_bind(self.changed_by.as_ref().map(|key| key.to_string()))
                .push_bind(self.changed_date.to_owned())
                .push_bind(self.system_supplied_data);
        }

        fn from_row(row: mdbsql::mdbsql::Row) -> Result<Self, mdbsql::Error> {
            let introduced = access::vague_date_from_row(&row, 3)?;

            Ok(Self {
                taxon_key: row.get(0)?,
                item_name: row.get(1)?,
                authority: row.get(2)?,
                introduced,
                language: row.get(6)?,
                taxon_name_type_key: row.get(7)?,
                abbreviation: row.get(8)?,
                entered_by: row.get(9)?,
                entry_date: access::datetime_from_row(&row, 10)?.ok_or(
                    mdbsql::Error::FromSqlError(serde_plain::Error::Parse(
                        "Empty date",
                        format!("Expected date was empty."),
                    )),
                )?,
                changed_by: row.get(11)?,
                changed_date: access::datetime_from_row(&row, 12)?,
                system_supplied_data: access::bool_from_row(&row, 13)?,
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
    async fn taxon_can_be_fetched_from_db(pool: PgPool) {
        let taxon: Taxon = query_as(
            r#"
            SELECT *
            FROM uksi.taxon
            WHERE taxon_key = 'BMSSYS0000000808';
        "#,
        )
        .fetch_one(&pool)
        .await
        .expect("Error fetch rows from db.");

        assert_eq!(
            taxon,
            Taxon {
                taxon_key: NBNKey(FStr::try_from(b"BMSSYS0000000808").unwrap()),
                item_name: "Agaricus nobilis".to_string(),
                authority: Some("Bolton".to_string()),
                introduced: None,
                language: "la".to_string(),
                taxon_name_type_key: NBNKey(FStr::try_from(b"NBNSYS0000000001").unwrap()),
                abbreviation: Some("agnob".to_string()),
                entered_by: NBNKey(FStr::try_from(b"TESTDATA00000001").unwrap()),
                entry_date: NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2002, 05, 16).unwrap(),
                    NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap()
                ),
                changed_by: None,
                changed_date: None,
                system_supplied_data: true
            }
        );
    }
}
