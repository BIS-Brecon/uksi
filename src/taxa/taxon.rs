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
            introduced: VagueDate::from_row(row, "introduced")?,
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
