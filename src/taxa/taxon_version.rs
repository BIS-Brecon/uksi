use crate::key::NBNKey;
use sqlx::{FromRow, types::chrono::NaiveDateTime};

#[derive(Debug, PartialEq, FromRow)]
pub struct TaxonVersion {
    taxon_version_key: NBNKey,
    taxon_key: NBNKey,
    attribute: Option<String>,
    date_from: Option<NaiveDateTime>,
    date_to: Option<NaiveDateTime>,
    comment: Option<String>,
    validation_level: Option<i16>,
    uk_native: bool,
    quality: Option<String>,
    output_group_key: Option<NBNKey>,
    taxon_rank_key: NBNKey,
    gender: Option<String>,
    plural: bool,
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
    async fn taxon_version_can_be_fetched_from_db(pool: PgPool) {
        let taxon_version: TaxonVersion = query_as(
            r#"
            SELECT *
            FROM uksi.taxon_version
            WHERE taxon_version_key = 'BMSSYS0000000808';
        "#,
        )
        .fetch_one(&pool)
        .await
        .expect("Error fetch rows from db.");

        assert_eq!(
            taxon_version,
            TaxonVersion {
                taxon_version_key: NBNKey(FStr::try_from(b"BMSSYS0000000808").unwrap()),
                taxon_key: NBNKey(FStr::try_from(b"BMSSYS0000000808").unwrap()),
                attribute: None,
                date_from: None,
                date_to: Some(NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2020, 03, 20).unwrap(),
                    NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap()
                )),
                comment: None,
                validation_level: Some(0),
                uk_native: false,
                quality: None,
                output_group_key: Some(NBNKey(FStr::try_from(b"NHMSYS0000629144").unwrap())),
                taxon_rank_key: NBNKey(FStr::try_from(b"NBNSYS0000000028").unwrap()),
                gender: None,
                plural: false,
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
