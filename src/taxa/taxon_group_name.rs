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

#[cfg(feature = "update")]
mod update {
    use crate::{
        taxa::TaxonGroupName,
        update::{Table, access},
    };

    impl Table for TaxonGroupName {
        const NUM_COLUMNS: usize = 16;

        const INSERT_QUERY: &str = r#"
            INSERT INTO uksi.taxon_group_name (
                taxon_group_key,
                taxon_group_name,
                kingdom,
                input_level_0_flag,
                input_level1_flag,
                input_level1_sort_code,
                input_level2_flag,
                input_level2_sort_code,
                output_flag,
                input_level1_descriptor,
                input_level2_descriptor,
                parent,
                entered_by,
                entry_date,
                changed_by,
                changed_date
            )
        "#;

        const READ_QUERY: &str = r#"
            SELECT
                TAXON_GROUP_KEY,
                TAXON_GROUP_NAME,
                KINGDOM,
                INPUT_LEVEL_0_FLAG,
                INPUT_LEVEL1_FLAG,
                INPUT_LEVEL1_SORT_CODE,
                INPUT_LEVEL2_FLAG,
                INPUT_LEVEL2_SORT_CODE,
                OUTPUT_FLAG,
                INPUT_LEVEL1_DESCRIPTOR,
                INPUT_LEVEL2_DESCRIPTOR,
                PARENT,
                ENTERED_BY,
                ENTRY_DATE,
                CHANGED_BY,
                CHANGED_DATE
            FROM
                TAXON_GROUP_NAME
        "#;

        fn bind_values<'a>(
            &self,
            mut builder: sqlx::query_builder::Separated<'_, 'a, sqlx::Postgres, &'static str>,
        ) {
            builder
                .push_bind(self.taxon_group_key.to_string())
                .push_bind(self.taxon_group_name.to_owned())
                .push_bind(self.kingdom.to_owned())
                .push_bind(self.input_level_0_flag.to_owned())
                .push_bind(self.input_level1_flag.to_owned())
                .push_bind(self.input_level1_sort_code.to_owned())
                .push_bind(self.input_level2_flag.to_owned())
                .push_bind(self.input_level2_sort_code.to_owned())
                .push_bind(self.output_flag.to_owned())
                .push_bind(self.input_level1_descriptor.to_owned())
                .push_bind(self.input_level2_descriptor.to_owned())
                .push_bind(self.parent.as_ref().map(|key| key.to_string()))
                .push_bind(self.entered_by.to_string())
                .push_bind(self.entry_date.to_owned())
                .push_bind(self.changed_by.as_ref().map(|key| key.to_string()))
                .push_bind(self.changed_date.to_owned());
        }

        fn from_row(row: mdbsql::mdbsql::Row) -> Result<Self, mdbsql::Error> {
            Ok(Self {
                taxon_group_key: row.get(0)?,
                taxon_group_name: row.get(1)?,
                kingdom: row.get(2)?,
                input_level_0_flag: access::bool_from_row(&row, 3)?,
                input_level1_flag: access::bool_from_row(&row, 4)?,
                input_level1_sort_code: row.get(5)?,
                input_level2_flag: access::bool_from_row(&row, 6)?,
                input_level2_sort_code: row.get(7)?,
                output_flag: access::bool_from_row(&row, 8)?,
                input_level1_descriptor: row.get(9)?,
                input_level2_descriptor: row.get(10)?,
                parent: row.get(11)?,
                entered_by: row.get(12)?,
                entry_date: access::datetime_from_row(&row, 13)?.ok_or(
                    mdbsql::Error::FromSqlError(serde_plain::Error::Parse(
                        "Empty date",
                        format!("Expected date was empty."),
                    )),
                )?,
                changed_by: row.get(14)?,
                changed_date: access::datetime_from_row(&row, 15)?,
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
