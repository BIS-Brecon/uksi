use crate::key::NBNKey;
use chrono::NaiveDate;
use sqlx::{FromRow, types::chrono::NaiveDateTime};

#[derive(Debug, PartialEq, FromRow)]
pub struct TaxonDesignation {
    taxon_designation_key: NBNKey,
    date_from: Option<NaiveDate>,
    date_to: Option<NaiveDate>,
    status_geographic_area: Option<String>,
    status_constraint: Option<String>,
    detail: Option<String>,
    taxon_designation_type_key: NBNKey,
    taxon_list_item_key: NBNKey,
    status_exclusion: bool,
    entered_by: NBNKey,
    entry_date: NaiveDateTime,
    changed_by: Option<NBNKey>,
    changed_date: Option<NaiveDateTime>,
    system_supplied_data: bool,
}

#[cfg(feature = "update")]
mod update {
    use crate::{
        designation::TaxonDesignation,
        update::{Table, access},
    };

    impl Table for TaxonDesignation {
        const NUM_COLUMNS: usize = 14;

        const INSERT_QUERY: &str = r#"
            INSERT INTO uksi.taxon_designation (
                taxon_designation_key,
                date_from,
                date_to,
                status_geographic_area,
                status_constraint,
                detail,
                taxon_designation_type_key,
                taxon_list_item_key,
                status_exclusion,
                entered_by,
                entry_date,
                changed_by,
                changed_date,
                system_supplied_data
            )
        "#;

        const READ_QUERY: &str = r#"
            SELECT
                TAXON_DESIGNATION_KEY,
                DATE_FROM,
                DATE_TO,
                STATUS_GEOGRAPHIC_AREA,
                STATUS_CONSTRAINT,
                DETAIL,
                TAXON_DESIGNATION_TYPE_KEY,
                TAXON_LIST_ITEM_KEY,
                STATUS_EXCLUSION,
                ENTERED_BY,
                ENTRY_DATE,
                CHANGED_BY,
                CHANGED_DATE,
                SYSTEM_SUPPLIED_DATA
            FROM
                TAXON_DESIGNATION
        "#;

        fn bind_values<'a>(
            &self,
            mut builder: sqlx::query_builder::Separated<'_, 'a, sqlx::Postgres, &'static str>,
        ) {
            builder
                .push_bind(self.taxon_designation_key.to_string())
                .push_bind(self.date_from.to_owned())
                .push_bind(self.date_to.to_owned())
                .push_bind(self.status_geographic_area.to_owned())
                .push_bind(self.status_constraint.to_owned())
                .push_bind(self.detail.to_owned())
                .push_bind(self.taxon_designation_type_key.to_string())
                .push_bind(self.taxon_list_item_key.to_string())
                .push_bind(self.status_exclusion.to_owned())
                .push_bind(self.entered_by.to_string())
                .push_bind(self.entry_date.to_owned())
                .push_bind(self.changed_by.as_ref().map(|key| key.to_string()))
                .push_bind(self.changed_date.to_owned())
                .push_bind(self.system_supplied_data);
        }

        fn from_row(row: mdbsql::mdbsql::Row) -> Result<Self, mdbsql::Error> {
            Ok(Self {
                taxon_designation_key: row.get(0)?,
                date_from: access::date_from_row(&row, 1)?,
                date_to: access::date_from_row(&row, 2)?,
                status_geographic_area: row.get(3)?,
                status_constraint: row.get(4)?,
                detail: row.get(5)?,
                taxon_designation_type_key: row.get(6)?,
                taxon_list_item_key: row.get(7)?,
                status_exclusion: access::bool_from_row(&row, 8)?,
                entered_by: row.get(9)?,
                entry_date: access::datetime_from_row(&row, 10)?.ok_or(
                    mdbsql::Error::FromSqlError(serde_plain::Error::Parse(
                        "Empty date",
                        format!("Expected date was empty."),
                    )),
                )?,
                changed_by: row.get(11)?,
                changed_date: access::datetime_from_row(&row, 12)?,
                system_supplied_data: access::bool_from_row(&row, 13)?,
            })
        }
    }
}
