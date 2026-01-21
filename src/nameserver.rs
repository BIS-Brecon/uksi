use crate::key::NBNKey;
use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Debug, PartialEq, FromRow)]
pub(crate) struct Nameserver {
    input_taxon_version_key: NBNKey,
    taxon_version_form: Option<String>,
    taxon_version_status: Option<String>,
    taxon_type: Option<String>,
    recommended_taxon_version_key: NBNKey,
    entered_by: NBNKey,
    entry_date: NaiveDateTime,
    changed_by: Option<NBNKey>,
    changed_date: Option<NaiveDateTime>,
}

#[cfg(feature = "update")]
mod update {
    use crate::{
        nameserver::Nameserver,
        update::{Table, access},
    };

    impl Table for Nameserver {
        const NUM_COLUMNS: usize = 9;

        const INSERT_QUERY: &str = r#"
            INSERT INTO
            uksi.nameserver (
                input_taxon_version_key,
                taxon_version_form,
                taxon_version_status,
                taxon_type,
                recommended_taxon_version_key,
                entered_by,
                entry_date,
                changed_by,
                changed_date
            )
        "#;

        const READ_QUERY: &str = r#"
            SELECT
                input_taxon_version_key,
                taxon_version_form,
                taxon_version_status,
                taxon_type,
                recommended_taxon_version_key,
                entered_by,
                entry_date,
                changed_by,
                changed_date
            FROM
                nameserver
        "#;

        fn bind_values<'a>(
            &self,
            mut builder: sqlx::query_builder::Separated<'_, 'a, sqlx::Postgres, &'static str>,
        ) {
            builder
                .push_bind(self.input_taxon_version_key.to_string())
                .push_bind(self.taxon_version_form.to_owned())
                .push_bind(self.taxon_version_status.to_owned())
                .push_bind(self.taxon_type.to_owned())
                .push_bind(self.recommended_taxon_version_key.to_string())
                .push_bind(self.entered_by.to_string())
                .push_bind(self.entry_date.to_owned())
                .push_bind(self.changed_by.as_ref().map(|key| key.to_string()))
                .push_bind(self.changed_date.to_owned());
        }

        fn from_row(row: mdbsql::mdbsql::Row) -> Result<Self, mdbsql::Error> {
            Ok(Self {
                input_taxon_version_key: row.get(0)?,
                taxon_version_form: row.get(1)?,
                taxon_version_status: row.get(2)?,
                taxon_type: row.get(3)?,
                recommended_taxon_version_key: row.get(4)?,
                entered_by: row.get(5)?,
                entry_date: access::datetime_from_row(&row, 6)?.ok_or(
                    mdbsql::Error::FromSqlError(serde_plain::Error::Parse(
                        "Empty date",
                        format!("Expected date was empty."),
                    )),
                )?,
                changed_by: row.get(7)?,
                changed_date: access::datetime_from_row(&row, 8)?,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveTime};
    use fstr::FStr;
    use sqlx::{PgPool, query_as};

    use super::*;

    #[sqlx::test(fixtures("../tests/fixtures.sql"))]
    async fn nameserver_can_be_fetched_from_db(pool: PgPool) {
        let nameserver: Nameserver = query_as(
            r#"
            SELECT *
            FROM uksi.nameserver
            LIMIT 1;
        "#,
        )
        .fetch_one(&pool)
        .await
        .expect("Error fetch rows from db.");

        assert_eq!(
            nameserver,
            Nameserver {
                input_taxon_version_key: NBNKey(FStr::try_from(b"BMSSYS0000000808").unwrap()),
                taxon_version_form: Some("U".to_string()),
                taxon_version_status: Some("U".to_string()),
                taxon_type: Some("S".to_string()),
                recommended_taxon_version_key: NBNKey(FStr::try_from(b"BMSSYS0000042432").unwrap()),
                entered_by: NBNKey(FStr::try_from(b"TESTDATA00000001").unwrap()),
                entry_date: NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2006, 06, 23).unwrap(),
                    NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap()
                ),
                changed_by: None,
                changed_date: None
            }
        );
    }
}
