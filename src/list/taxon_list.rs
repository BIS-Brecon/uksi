use crate::key::NBNKey;
use sqlx::{FromRow, types::chrono::NaiveDateTime};

#[derive(Debug, PartialEq, FromRow)]
pub struct TaxonList {
    taxon_list_key: NBNKey,
    item_name: String,
    description_long: Option<String>,
    description_short: Option<String>,
    authority: String,
    owner_key: Option<NBNKey>,
    taxon_list_type_key: NBNKey,
    update_mechanism: Option<String>,
    entered_by: NBNKey,
    entry_date: NaiveDateTime,
    changed_by: Option<NBNKey>,
    changed_date: Option<NaiveDateTime>,
    system_supplied_data: bool,
}

#[cfg(feature = "update")]
mod update {
    use crate::{
        list::TaxonList,
        update::{Table, access},
    };

    impl Table for TaxonList {
        const NUM_COLUMNS: usize = 13;

        const INSERT_QUERY: &str = r#"
            INSERT INTO uksi.taxon_list (
                taxon_list_key,
                item_name,
                description_long,
                description_short,
                authority,
                owner_key,
                taxon_list_type_key,
                update_mechanism,
                entered_by,
                entry_date,
                changed_by,
                changed_date,
                system_supplied_data
            )
        "#;

        const READ_QUERY: &str = r#"
            SELECT
                TAXON_LIST_KEY,
                ITEM_NAME,
                DESCRIPTION_LONG,
                DESCRIPTION_SHORT,
                AUTHORITY,
                OWNER_KEY,
                TAXON_LIST_TYPE_KEY,
                UPDATE_MECHANISM,
                ENTERED_BY,
                ENTRY_DATE,
                CHANGED_BY,
                CHANGED_DATE,
                SYSTEM_SUPPLIED_DATA
            FROM
                TAXON_LIST
        "#;

        fn bind_values<'a>(
            &self,
            mut builder: sqlx::query_builder::Separated<'_, 'a, sqlx::Postgres, &'static str>,
        ) {
            builder
                .push_bind(self.taxon_list_key.to_string())
                .push_bind(self.item_name.to_owned())
                .push_bind(self.description_long.to_owned())
                .push_bind(self.description_short.to_owned())
                .push_bind(self.authority.to_owned())
                .push_bind(self.owner_key.as_ref().map(|key| key.to_string()))
                .push_bind(self.taxon_list_type_key.to_string())
                .push_bind(self.update_mechanism.to_owned())
                .push_bind(self.entered_by.to_string())
                .push_bind(self.entry_date.to_owned())
                .push_bind(self.changed_by.as_ref().map(|key| key.to_string()))
                .push_bind(self.changed_date.to_owned())
                .push_bind(self.system_supplied_data);
        }

        fn from_row(row: mdbsql::mdbsql::Row) -> Result<Self, mdbsql::Error> {
            Ok(Self {
                taxon_list_key: row.get(0)?,
                item_name: row.get(1)?,
                description_long: row.get(2)?,
                description_short: row.get(3)?,
                authority: row.get(4)?,
                owner_key: row.get(5)?,
                taxon_list_type_key: row.get(6)?,
                update_mechanism: row.get(7)?,
                entered_by: row.get(8)?,
                entry_date: access::datetime_from_row(&row, 9)?.ok_or(
                    mdbsql::Error::FromSqlError(serde_plain::Error::Parse(
                        "Empty date",
                        format!("Expected date was empty."),
                    )),
                )?,
                changed_by: row.get(10)?,
                changed_date: access::datetime_from_row(&row, 11)?,
                system_supplied_data: access::bool_from_row(&row, 12)?,
            })
        }
    }
}
