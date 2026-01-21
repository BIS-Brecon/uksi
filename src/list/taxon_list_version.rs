use crate::{date::VagueDate, key::NBNKey};
use chrono::NaiveDate;
use sqlx::{FromRow, Row, postgres::PgRow, types::chrono::NaiveDateTime};

#[derive(Debug, PartialEq)]
pub struct TaxonListVersion {
    taxon_list_version_key: NBNKey,
    taxon_list_key: NBNKey,
    version: i32,
    authority: String,
    owner_key: Option<NBNKey>,
    vague_date: Option<VagueDate>,
    description: Option<String>,
    version_is_amendment: bool,
    quality: Option<String>,
    recommended_list: bool,
    last_updated: Option<NaiveDate>,
    entered_by: NBNKey,
    entry_date: NaiveDateTime,
    changed_by: Option<NBNKey>,
    changed_date: Option<NaiveDateTime>,
    system_supplied_data: bool,
}

impl FromRow<'_, PgRow> for TaxonListVersion {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            taxon_list_version_key: row.try_get("taxon_list_version_key")?,
            taxon_list_key: row.try_get("taxon_list_key")?,
            version: row.try_get("version")?,
            authority: row.try_get("authority")?,
            owner_key: row.try_get("owner_key")?,
            description: row.try_get("description")?,
            version_is_amendment: row.try_get("version_is_amendment")?,
            quality: row.try_get("quality")?,
            recommended_list: row.try_get("recommended_list")?,
            last_updated: row.try_get("last_updated")?,
            vague_date: VagueDate::from_row(row, None)?,
            entered_by: row.try_get("entered_by")?,
            entry_date: row.try_get("entry_date")?,
            changed_by: row.try_get("changed_by")?,
            changed_date: row.try_get("changed_date")?,
            system_supplied_data: row.try_get("system_supplied_data")?,
        })
    }
}

#[cfg(feature = "update")]
mod update {
    use crate::{
        list::TaxonListVersion,
        update::{Table, access},
    };

    impl Table for TaxonListVersion {
        const NUM_COLUMNS: usize = 18;

        const INSERT_QUERY: &str = r#"
            INSERT INTO uksi.taxon_list_version (
                taxon_list_version_key,
                taxon_list_key,
                version,
                authority,
                owner_key,
                vague_date_start,
                vague_date_end,
                vague_date_type,
                description,
                version_is_amendment,
                quality,
                recommended_list,
                last_updated,
                entered_by,
                entry_date,
                changed_by,
                changed_date,
                system_supplied_data
            )
        "#;

        // IIf function present to single row with incorrect ENTERED_BY.
        // Last checked as present in: UKSI-20251104a
        const READ_QUERY: &str = r#"
            SELECT
                TAXON_LIST_VERSION_KEY,
                TAXON_LIST_KEY,
                VERSION,
                AUTHORITY,
                OWNER_KEY,
                VAGUE_DATE_START,
                VAGUE_DATE_END,
                VAGUE_DATE_TYPE,
                DESCRIPTION,
                VERSION_IS_AMENDMENT,
                QUALITY,
                RECOMMENDED_LIST,
                LAST_UPDATED,
                ENTERED_BY,
                ENTRY_DATE,
                CHANGED_BY,
                CHANGED_DATE,
                SYSTEM_SUPPLIED_DATA
            FROM
                TAXON_LIST_VERSION
        "#;

        fn bind_values<'a>(
            &self,
            mut builder: sqlx::query_builder::Separated<'_, 'a, sqlx::Postgres, &'static str>,
        ) {
            builder
                .push_bind(self.taxon_list_version_key.to_string())
                .push_bind(self.taxon_list_key.to_string())
                .push_bind(self.version.to_owned())
                .push_bind(self.authority.to_owned())
                .push_bind(self.owner_key.as_ref().map(|key| key.to_string()))
                .push_bind(self.vague_date.as_ref().map(|vague_date| vague_date.start))
                .push_bind(self.vague_date.as_ref().map(|vague_date| vague_date.end))
                .push_bind(
                    self.vague_date
                        .as_ref()
                        .map(|vague_date| vague_date.date_type),
                )
                .push_bind(self.description.to_owned())
                .push_bind(self.version_is_amendment.to_owned())
                .push_bind(self.quality.to_owned())
                .push_bind(self.recommended_list.to_owned())
                .push_bind(self.last_updated.to_owned())
                .push_bind(self.entered_by.to_string())
                .push_bind(self.entry_date.to_owned())
                .push_bind(self.changed_by.as_ref().map(|key| key.to_string()))
                .push_bind(self.changed_date.to_owned())
                .push_bind(self.system_supplied_data);
        }

        fn from_row(row: mdbsql::mdbsql::Row) -> Result<Self, mdbsql::Error> {
            // Correct row NBNSYS0000000186 with incorrect ENTERED_BY.
            // Last checked as present in: UKSI-20251104a
            let mut entered_by: String = row.get(13)?;
            if entered_by == "NBNSYS000000003" {
                entered_by = "NBNSYS0000000003".to_string();
            }

            let vague_date = access::vague_date_from_row(&row, 5)?;

            Ok(Self {
                taxon_list_version_key: row.get(0)?,
                taxon_list_key: row.get(1)?,
                version: row.get(2)?,
                authority: row.get(3)?,
                owner_key: row.get(4)?,
                vague_date: vague_date,
                description: row.get(8)?,
                version_is_amendment: access::bool_from_row(&row, 9)?,
                quality: row.get(10)?,
                recommended_list: access::bool_from_row(&row, 11)?,
                last_updated: None,
                entered_by: serde_plain::from_str(&entered_by)?,
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
