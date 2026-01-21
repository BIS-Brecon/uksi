use crate::key::NBNKey;
use sqlx::{FromRow, types::chrono::NaiveDateTime};

#[derive(Debug, PartialEq, FromRow)]
pub struct TaxonListItem {
    taxon_list_item_key: NBNKey,
    taxon_version_key: NBNKey,
    taxon_list_version_key: NBNKey,
    taxon_list_version_to: Option<NBNKey>,
    preferred_name: NBNKey,
    sort_code: Option<i32>,
    lst_itm_code: Option<String>,
    parent: Option<NBNKey>,
    taxon_rank_key: NBNKey,
    code_source: Option<String>,
    note: Option<String>,
    entered_by: NBNKey,
    entry_date: NaiveDateTime,
    changed_by: Option<NBNKey>,
    changed_date: Option<NaiveDateTime>,
    system_supplied_data: bool,
}

#[cfg(feature = "update")]
mod update {
    use fstr::FStr;

    use crate::{
        list::TaxonListItem,
        update::{Table, access},
    };

    impl Table for TaxonListItem {
        const NUM_COLUMNS: usize = 16;

        const INSERT_QUERY: &str = r#"
            INSERT INTO uksi.taxon_list_item (
                taxon_list_item_key,
                taxon_version_key,
                taxon_list_version_key,
                taxon_list_version_to,
                preferred_name,
                sort_code,
                lst_itm_code,
                parent,
                taxon_rank_key,
                code_source,
                note,
                entered_by,
                entry_date,
                changed_by,
                changed_date,
                system_supplied_data
            )
        "#;

        const READ_QUERY: &str = r#"
            SELECT
                TAXON_LIST_ITEM_KEY,
                TAXON_VERSION_KEY,
                TAXON_LIST_VERSION_KEY,
                TAXON_LIST_VERSION_TO,
                PREFERRED_NAME,
                SORT_CODE,
                LST_ITM_CODE,
                PARENT,
                TAXON_RANK_KEY,
                CODE_SOURCE,
                NOTE,
                ENTERED_BY,
                ENTRY_DATE,
                CHANGED_BY,
                CHANGED_DATE,
                SYSTEM_SUPPLIED_DATA
            FROM
                TAXON_LIST_ITEM
        "#;

        fn bind_values<'a>(
            &self,
            mut builder: sqlx::query_builder::Separated<'_, 'a, sqlx::Postgres, &'static str>,
        ) {
            builder
                .push_bind(self.taxon_list_item_key.to_string())
                .push_bind(self.taxon_version_key.to_string())
                .push_bind(self.taxon_list_version_key.to_string())
                .push_bind(
                    self.taxon_list_version_to
                        .as_ref()
                        .map(|key| key.to_string()),
                )
                .push_bind(self.preferred_name.to_owned())
                .push_bind(self.sort_code.to_owned())
                .push_bind(self.lst_itm_code.to_owned())
                .push_bind(self.parent.as_ref().map(|key| key.to_string()))
                .push_bind(self.taxon_rank_key.to_owned())
                .push_bind(self.code_source.to_owned())
                .push_bind(self.note.to_owned())
                .push_bind(self.entered_by.to_string())
                .push_bind(self.entry_date.to_owned())
                .push_bind(self.changed_by.as_ref().map(|key| key.to_string()))
                .push_bind(self.changed_date.to_owned())
                .push_bind(self.system_supplied_data);
        }

        fn from_row(row: mdbsql::mdbsql::Row) -> Result<Self, mdbsql::Error> {
            // Correct lowercase ENTERED_BY and CHANGED_BY values.
            // Last checked as present in: UKSI-20251104a
            let entered_by = row.get::<String>(11)?.to_uppercase();
            let entered_by = crate::key::NBNKey(serde_plain::from_str(&entered_by)?);

            let changed_by: Option<String> = row.get(13)?;
            let changed_by = match changed_by {
                Some(s) => {
                    let s = serde_plain::from_str::<FStr<16>>(&s.to_uppercase())?;

                    Some(crate::key::NBNKey(s))
                }
                None => None,
            };

            Ok(Self {
                taxon_list_item_key: row.get(0)?,
                taxon_version_key: row.get(1)?,
                taxon_list_version_key: row.get(2)?,
                taxon_list_version_to: row.get(3)?,
                preferred_name: row.get(4)?,
                sort_code: row.get(5)?,
                lst_itm_code: row.get(6)?,
                parent: row.get(7)?,
                taxon_rank_key: row.get(8)?,
                code_source: row.get(9)?,
                note: row.get(10)?,
                entered_by,
                entry_date: access::datetime_from_row(&row, 12)?.ok_or(
                    mdbsql::Error::FromSqlError(serde_plain::Error::Parse(
                        "Empty date",
                        format!("Expected date was empty."),
                    )),
                )?,
                changed_by,
                changed_date: access::datetime_from_row(&row, 14)?,
                system_supplied_data: access::bool_from_row(&row, 15)?,
            })
        }
    }
}
