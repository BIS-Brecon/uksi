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
