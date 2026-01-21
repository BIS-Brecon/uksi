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

#[cfg(feature = "update")]
mod update {
    use crate::{
        taxa::TaxonVersion,
        update::{Table, access},
    };

    impl Table for TaxonVersion {
        const NUM_COLUMNS: usize = 18;

        const INSERT_QUERY: &str = r#"
            INSERT INTO uksi.taxon_version (
                taxon_version_key,
                taxon_key,
                attribute,
                date_from,
                date_to,
                comment,
                validation_level,
                uk_native,
                quality,
                output_group_key,
                taxon_rank_key,
                gender,
                plural,
                entered_by,
                entry_date,
                changed_by,
                changed_date,
                system_supplied_data
            )
        "#;

        const READ_QUERY: &str = r#"
            SELECT
                TAXON_VERSION_KEY,
                TAXON_KEY,
                ATTRIBUTE,
                DATE_FROM,
                DATE_TO,
                COMMENT,
                VALIDATION_LEVEL,
                UK_NATIVE,
                QUALITY,
                OUTPUT_GROUP_KEY,
                TAXON_RANK_KEY,
                GENDER,
                PLURAL,
                ENTERED_BY,
                ENTRY_DATE,
                CHANGED_BY,
                CHANGED_DATE,
                SYSTEM_SUPPLIED_DATA
            FROM
                TAXON_VERSION
        "#;

        fn bind_values<'a>(
            &self,
            mut builder: sqlx::query_builder::Separated<'_, 'a, sqlx::Postgres, &'static str>,
        ) {
            builder
                .push_bind(self.taxon_version_key.to_string())
                .push_bind(self.taxon_key.to_string())
                .push_bind(self.attribute.to_owned())
                .push_bind(self.date_from.to_owned())
                .push_bind(self.date_to.to_owned())
                .push_bind(self.comment.to_owned())
                .push_bind(self.validation_level.to_owned())
                .push_bind(self.uk_native.to_owned())
                .push_bind(self.quality.to_owned())
                .push_bind(self.output_group_key.as_ref().map(|key| key.to_string()))
                .push_bind(self.taxon_rank_key.to_string())
                .push_bind(self.gender.to_owned())
                .push_bind(self.plural.to_owned())
                .push_bind(self.entered_by.to_string())
                .push_bind(self.entry_date.to_owned())
                .push_bind(self.changed_by.as_ref().map(|key| key.to_string()))
                .push_bind(self.changed_date.to_owned())
                .push_bind(self.system_supplied_data);
        }

        fn from_row(row: mdbsql::mdbsql::Row) -> Result<Self, mdbsql::Error> {
            Ok(Self {
                taxon_version_key: row.get(0)?,
                taxon_key: row.get(1)?,
                attribute: row.get(2)?,
                date_from: access::datetime_from_row(&row, 3)?,
                date_to: access::datetime_from_row(&row, 4)?,
                comment: row.get(5)?,
                validation_level: row.get(6)?,
                uk_native: access::bool_from_row(&row, 7)?,
                quality: row.get(8)?,
                output_group_key: row.get(9)?,
                taxon_rank_key: row.get(10)?,
                gender: row.get(11)?,
                plural: access::bool_from_row(&row, 12)?,
                entered_by: row.get(13)?,
                entry_date: access::datetime_from_row(&row, 14)?.ok_or(
                    mdbsql::Error::FromSqlError(serde_plain::Error::Parse(
                        "Empty date",
                        format!("Expected date was empty."),
                    )),
                )?,
                changed_by: row.get(15)?,
                changed_date: access::datetime_from_row(&row, 16)?,
                system_supplied_data: access::bool_from_row(&row, 17)?,
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
