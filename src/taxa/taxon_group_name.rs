use crate::key::NBNKey;
use sqlx::{FromRow, types::chrono::NaiveDateTime};

#[derive(Debug, PartialEq, FromRow)]
pub struct TaxonGroupName {
    taxon_group_key: NBNKey,
    taxon_group_name: String,
    kingdom: Option<String>,
    input_level_0_flag: bool,
    input_level1_flag: bool,
    input_level1_sort_code: Option<i16>,
    input_level2_flag: bool,
    input_level2_sort_code: Option<i16>,
    output_flag: bool,
    input_level1_descriptor: Option<String>,
    input_level2_descriptor: Option<String>,
    parent: Option<NBNKey>,
    entered_by: NBNKey,
    entry_date: NaiveDateTime,
    changed_by: Option<NBNKey>,
    changed_date: Option<NaiveDateTime>,
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use fstr::FStr;
    use sqlx::{PgPool, query_as};

    use super::*;

    #[sqlx::test(fixtures("../../tests/fixtures.sql"))]
    async fn taxon_group_name_can_be_fetched_from_db(pool: PgPool) {
        let taxon_group_name: TaxonGroupName = query_as(
            r#"
            SELECT *
            FROM uksi.taxon_group_name
            WHERE taxon_group_key = 'NHMSYS0000080054';
        "#,
        )
        .fetch_one(&pool)
        .await
        .expect("Error fetch rows from db.");

        assert_eq!(
            taxon_group_name,
            TaxonGroupName {
                taxon_group_key: NBNKey(FStr::try_from(b"NHMSYS0000080054").unwrap()),
                taxon_group_name: "flowering plant".to_string(),
                kingdom: Some("PL".to_string()),
                input_level_0_flag: false,
                input_level1_flag: false,
                input_level1_sort_code: None,
                input_level2_flag: false,
                input_level2_sort_code: None,
                output_flag: true,
                input_level1_descriptor: None,
                input_level2_descriptor: None,
                parent: Some(NBNKey(FStr::try_from(b"NHMSYS0000375171").unwrap())),
                entered_by: NBNKey(FStr::try_from(b"TESTDATA00000001").unwrap()),
                entry_date: NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2003, 06, 14).unwrap(),
                    NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap()
                ),
                changed_by: None,
                changed_date: None
            }
        );
    }
}
