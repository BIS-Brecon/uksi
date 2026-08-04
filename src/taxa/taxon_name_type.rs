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

#[cfg(feature = "update")]
mod update {
    use crate::{
        taxa::TaxonNameType,
        update::{Table, access},
    };

    impl Table for TaxonNameType {
        const NUM_COLUMNS: usize = 8;

        const INSERT_QUERY: &str = r#"
            INSERT INTO uksi.taxon_name_type (
                taxon_name_type_key,
                short_name,
                long_name,
                description,
                authority,
                entered_by,
                entry_date,
                system_supplied_data
            )
        "#;

        const READ_QUERY: &str = r#"
            SELECT
                TAXON_NAME_TYPE_KEY,
                SHORT_NAME,
                LONG_NAME,
                DESCRIPTION,
                AUTHORITY,
                ENTERED_BY,
                ENTRY_DATE,
                SYSTEM_SUPPLIED_DATA
            FROM
                TAXON_NAME_TYPE
        "#;

        fn bind_values<'a>(
            &self,
            builder: &mut sqlx::query_builder::Separated<'_, 'a, sqlx::Postgres, &'static str>,
        ) {
            builder
                .push_bind(self.taxon_name_type_key.to_string())
                .push_bind(self.short_name.to_owned())
                .push_bind(self.long_name.to_owned())
                .push_bind(self.description.to_owned())
                .push_bind(self.authority.to_owned())
                .push_bind(self.entered_by.to_string())
                .push_bind(self.entry_date.to_owned())
                .push_bind(self.system_supplied_data);
        }

        fn from_row(row: mdbsql::mdbsql::Row) -> Result<Self, mdbsql::Error> {
            Ok(Self {
                taxon_name_type_key: row.get(0)?,
                short_name: row.get(1)?,
                long_name: row.get(2)?,
                description: row.get(3)?,
                authority: row.get(4)?,
                entered_by: row.get(5)?,
                entry_date: access::datetime_from_row(&row, 6)?.ok_or(
                    mdbsql::Error::FromSqlError(serde_plain::Error::Parse(
                        "Empty date",
                        format!("Expected date was empty."),
                    )),
                )?,
                system_supplied_data: access::bool_from_row(&row, 7)?,
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
