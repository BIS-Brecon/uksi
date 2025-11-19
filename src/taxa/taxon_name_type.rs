use crate::key::NBNKey;
use sqlx::{FromRow, types::chrono::NaiveDateTime};

#[derive(Debug, PartialEq, FromRow)]
pub struct TaxonNameType {
    taxon_name_type_key: NBNKey,
    short_name: String,
    long_name: Option<String>,
    description: Option<String>,
    authority: Option<String>,
    entered_by: NBNKey,
    entry_date: NaiveDateTime,
    system_supplied_data: bool,
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use fstr::FStr;
    use sqlx::{PgPool, query_as};

    use super::*;

    #[sqlx::test(fixtures("../../tests/fixtures.sql"))]
    async fn taxon_name_type_can_be_fetched_from_db(pool: PgPool) {
        let taxon_name_type: TaxonNameType = query_as(
            r#"
            SELECT *
            FROM uksi.taxon_name_type
            WHERE taxon_name_type_key = 'NBNSYS0000000001';
        "#,
        )
        .fetch_one(&pool)
        .await
        .expect("Error fetch rows from db.");

        assert_eq!(
            taxon_name_type,
            TaxonNameType {
                taxon_name_type_key: NBNKey(FStr::try_from(b"NBNSYS0000000001").unwrap()),
                short_name: "Formal".to_string(),
                long_name: Some("A Formal name of the taxon".to_string()),
                description: None,
                authority: None,
                entered_by: NBNKey(FStr::try_from(b"TESTDATA00000001").unwrap()),
                entry_date: NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(1999, 03, 25).unwrap(),
                    NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap()
                ),
                system_supplied_data: true
            }
        );
    }
}
