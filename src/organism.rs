use crate::key::NBNKey;
use sqlx::{FromRow, types::chrono::NaiveDateTime};

#[derive(Debug, PartialEq, FromRow)]
pub struct Organism {
    organism_key: NBNKey,
    parent_key: Option<NBNKey>,
    taxon_version_key: NBNKey,
    uk_status_key: Option<String>,
    marine_flag: Option<String>,
    terrestrial_freshwater_flag: Option<String>,
    freshwater: Option<String>,
    redundant_flag: Option<String>,
    non_native_flag: Option<String>,
    only_in_not_fit_for_web: Option<String>,
    orphan: Option<String>,
    vernacular: Option<String>,
    organism_rank_key: Option<NBNKey>,
    lineage: Option<String>,
    sort_level: Option<i32>,
    weight: Option<i32>,
    sort_order: Option<String>,
    sort_code: Option<String>,
    has_children: bool,
    entered_by: NBNKey,
    entry_date: NaiveDateTime,
    changed_by: Option<NBNKey>,
    changed_date: Option<NaiveDateTime>,
    system_supplied_data: bool,
}

#[cfg(feature = "update")]
mod update {
    use crate::{
        Organism,
        update::{Table, access},
    };

    impl Table for Organism {
        const NUM_COLUMNS: usize = 24;

        const INSERT_QUERY: &str = r#"
            INSERT INTO uksi.organism (
                organism_key,
                parent_key,
                taxon_version_key,
                uk_status_key,
                marine_flag,
                terrestrial_freshwater_flag,
                freshwater,
                redundant_flag,
                non_native_flag,
                only_in_not_fit_for_web,
                orphan,
                vernacular,
                organism_rank_key,
                lineage,
                sort_level,
                weight,
                sort_order,
                sort_code,
                has_children,
                entered_by,
                entry_date,
                changed_by,
                changed_date,
                system_supplied_data
            )
        "#;

        const READ_QUERY: &str = r#"
            SELECT
                ORGANISM_KEY,
                PARENT_KEY,
                TAXON_VERSION_KEY,
                UK_STATUS_KEY,
                MARINE_FLAG,
                TERRESTRIAL_FRESHWATER_FLAG,
                FRESHWATER,
                REDUNDANT_FLAG,
                NON_NATIVE_FLAG,
                ONLY_IN_NOT_FIT_FOR_WEB,
                ORPHAN,
                VERNACULAR,
                ORGANISM_RANK_KEY,
                LINEAGE,
                SORT_LEVEL,
                WEIGHT,
                SORT_ORDER,
                SORT_CODE,
                HAS_CHILDREN,
                ENTERED_BY,
                ENTRY_DATE,
                CHANGED_BY,
                CHANGED_DATE,
                SYSTEM_SUPPLIED_DATA
            FROM
                ORGANISM_MASTER
            WHERE
                DELETED_DATE = ''
        "#;

        fn bind_values<'a>(
            &self,
            mut builder: sqlx::query_builder::Separated<'_, 'a, sqlx::Postgres, &'static str>,
        ) {
            builder
                .push_bind(self.organism_key.to_string())
                .push_bind(self.parent_key.as_ref().map(|key| key.to_string()))
                .push_bind(self.taxon_version_key.to_string())
                .push_bind(self.uk_status_key.to_owned())
                .push_bind(self.marine_flag.to_owned())
                .push_bind(self.terrestrial_freshwater_flag.to_owned())
                .push_bind(self.freshwater.to_owned())
                .push_bind(self.redundant_flag.to_owned())
                .push_bind(self.non_native_flag.to_owned())
                .push_bind(self.only_in_not_fit_for_web.to_owned())
                .push_bind(self.orphan.to_owned())
                .push_bind(self.vernacular.to_owned())
                .push_bind(self.organism_rank_key.as_ref().map(|key| key.to_string()))
                .push_bind(self.lineage.to_owned())
                .push_bind(self.sort_level.to_owned())
                .push_bind(self.weight.to_owned())
                .push_bind(self.sort_order.to_owned())
                .push_bind(self.sort_code.to_owned())
                .push_bind(self.has_children)
                .push_bind(self.entered_by.to_string())
                .push_bind(self.entry_date.to_owned())
                .push_bind(self.changed_by.as_ref().map(|key| key.to_string()))
                .push_bind(self.changed_date.to_owned())
                .push_bind(self.system_supplied_data);
        }

        fn from_row(row: mdbsql::mdbsql::Row) -> Result<Self, mdbsql::Error> {
            Ok(Self {
                organism_key: row.get(0)?,
                parent_key: row.get(1)?,
                taxon_version_key: row.get(2)?,
                uk_status_key: row.get(3)?,
                marine_flag: row.get(4)?,
                terrestrial_freshwater_flag: row.get(5)?,
                freshwater: row.get(6)?,
                redundant_flag: row.get(7)?,
                non_native_flag: row.get(8)?,
                only_in_not_fit_for_web: row.get(9)?,
                orphan: row.get(10)?,
                vernacular: row.get(11)?,
                organism_rank_key: row.get(12)?,
                lineage: row.get(13)?,
                sort_level: row.get(14)?,
                weight: row.get(15)?,
                sort_order: row.get(16)?,
                sort_code: row.get(17)?,
                has_children: access::bool_from_row(&row, 18)?,
                entered_by: row.get(19)?,
                entry_date: access::datetime_from_row(&row, 20)?.ok_or(
                    mdbsql::Error::FromSqlError(serde_plain::Error::Parse(
                        "Empty date",
                        format!("Expected date was empty."),
                    )),
                )?,
                changed_by: row.get(21)?,
                changed_date: access::datetime_from_row(&row, 22)?,
                system_supplied_data: access::bool_from_row(&row, 23)?,
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

    #[sqlx::test(fixtures("../tests/fixtures.sql"))]
    async fn organism_can_be_fetched_from_db(pool: PgPool) {
        let organism: Organism = query_as(
            r#"
            SELECT *
            FROM uksi.organism
            WHERE organism_key = 'NBNORG0000008217';
        "#,
        )
        .fetch_one(&pool)
        .await
        .expect("Error fetch rows from db.");

        assert_eq!(
            organism,
            Organism {
                organism_key: NBNKey(FStr::try_from(b"NBNORG0000008217").unwrap()),
                parent_key: Some(NBNKey(FStr::try_from(b"NBNORG0000054419").unwrap())),
                taxon_version_key: NBNKey(FStr::try_from(b"NBNSYS0000003949").unwrap()),
                uk_status_key: None,
                marine_flag: None,
                terrestrial_freshwater_flag: None,
                freshwater: None,
                redundant_flag: None,
                non_native_flag: None,
                only_in_not_fit_for_web: None,
                orphan: None,
                vernacular: None,
                organism_rank_key: Some(NBNKey(FStr::try_from(b"NBNSYS0000000028").unwrap())),
                lineage: None,
                sort_level: Some(9),
                weight: None,
                sort_order: Some("000204040M05150A0203000000000000000000000000000000".to_string()),
                sort_code: Some("125047".to_string()),
                has_children: true,
                entered_by: NBNKey(FStr::try_from(b"TESTDATA00000001").unwrap()),
                entry_date: NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2012, 03, 05).unwrap(),
                    NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap()
                ),
                changed_by: None,
                changed_date: None,
                system_supplied_data: true,
            }
        );
    }
}
