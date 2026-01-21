use crate::{date::VagueDate, key::NBNKey};
use sqlx::{FromRow, Row, postgres::PgRow, types::chrono::NaiveDateTime};

#[derive(Debug, PartialEq)]
pub struct Individual {
    name_key: NBNKey,
    title: Option<String>,
    forename: Option<String>,
    initials: Option<String>,
    honorifics: Option<String>,
    surname: Option<String>,
    comment: Option<String>,
    born: Option<VagueDate>,
    died: Option<VagueDate>,
    person_floreat: Option<String>,
    entered_by: Option<NBNKey>,
    entry_date: Option<NaiveDateTime>,
    changed_by: Option<NBNKey>,
    changed_date: Option<NaiveDateTime>,
    system_supplied_data: bool,
}

impl FromRow<'_, PgRow> for Individual {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            name_key: row.try_get("name_key")?,
            title: row.try_get("title")?,
            forename: row.try_get("forename")?,
            initials: row.try_get("initials")?,
            honorifics: row.try_get("honorifics")?,
            surname: row.try_get("surname")?,
            comment: row.try_get("comment")?,
            born: VagueDate::from_row(row, Some("born"))?,
            died: VagueDate::from_row(row, Some("died"))?,
            person_floreat: row.try_get("person_floreat")?,
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
        Individual,
        update::{Table, access},
    };

    impl Table for Individual {
        const NUM_COLUMNS: usize = 19;

        const INSERT_QUERY: &str = r#"
            INSERT INTO uksi.individual(
                name_key,
                title,
                forename,
                initials,
                honorifics,
                surname,
                comment,
                born_vague_date_start,
                born_vague_date_end,
                born_vague_date_type,
                died_vague_date_start,
                died_vague_date_end,
                died_vague_date_type,
                person_floreat,
                entered_by,
                entry_date,
                changed_by,
                changed_date,
                system_supplied_data
            )
        "#;

        const READ_QUERY: &str = r#"
            SELECT
                NAME_KEY,
                TITLE,
                FORENAME,
                INITIALS,
                HONORIFICS,
                SURNAME,
                COMMENT,
                BORN_VAGUE_DATE_START,
                BORN_VAGUE_DATE_END,
                BORN_VAGUE_DATE_TYPE,
                DIED_VAGUE_DATE_START,
                DIED_VAGUE_DATE_END,
                DIED_VAGUE_DATE_TYPE,
                PERSON_FLOREAT,
                ENTERED_BY,
                ENTRY_DATE,
                CHANGED_BY,
                CHANGED_DATE,
                SYSTEM_SUPPLIED_DATA
            FROM
                INDIVIDUAL
        "#;

        fn bind_values<'a>(
            &self,
            mut builder: sqlx::query_builder::Separated<'_, 'a, sqlx::Postgres, &'static str>,
        ) {
            builder
                .push_bind(self.name_key.to_string())
                .push_bind(self.title.to_owned())
                .push_bind(self.forename.to_owned())
                .push_bind(self.initials.to_owned())
                .push_bind(self.honorifics.to_owned())
                .push_bind(self.surname.to_owned())
                .push_bind(self.comment.to_owned())
                .push_bind(self.born.as_ref().map(|born| born.start))
                .push_bind(self.born.as_ref().map(|born| born.end))
                .push_bind(self.born.as_ref().map(|born| born.date_type))
                .push_bind(self.died.as_ref().map(|died| died.start))
                .push_bind(self.died.as_ref().map(|died| died.end))
                .push_bind(self.died.as_ref().map(|died| died.date_type))
                .push_bind(self.person_floreat.to_owned())
                .push_bind(self.entered_by.as_ref().map(|key| key.to_string()))
                .push_bind(self.entry_date.to_owned())
                .push_bind(self.changed_by.as_ref().map(|key| key.to_string()))
                .push_bind(self.changed_date.to_owned())
                .push_bind(self.system_supplied_data);
        }

        fn from_row(row: mdbsql::mdbsql::Row) -> Result<Self, mdbsql::Error> {
            let born = access::vague_date_from_row(&row, 7)?;
            let died = access::vague_date_from_row(&row, 10)?;

            Ok(Self {
                name_key: row.get(0)?,
                title: row.get(1)?,
                forename: row.get(2)?,
                initials: row.get(3)?,
                honorifics: row.get(4)?,
                surname: row.get(5)?,
                comment: row.get(6)?,
                born,
                died,
                person_floreat: row.get(13)?,
                entered_by: row.get(14)?,
                entry_date: access::datetime_from_row(&row, 15)?,
                changed_by: row.get(16)?,
                changed_date: access::datetime_from_row(&row, 17)?,
                system_supplied_data: access::bool_from_row(&row, 18)?,
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
    use crate::date::VagueDateType;

    #[sqlx::test(fixtures("../tests/fixtures.sql"))]
    async fn individual_can_be_fetched_from_db(pool: PgPool) {
        let individual: Individual = query_as(
            r#"
            SELECT *
            FROM uksi.individual
            LIMIT 1;
        "#,
        )
        .fetch_one(&pool)
        .await
        .expect("Error fetch rows from db.");

        assert_eq!(
            individual,
            Individual {
                name_key: NBNKey(FStr::try_from(b"TESTDATA00000001").unwrap()),
                title: None,
                forename: Some("Default".to_string()),
                initials: None,
                honorifics: None,
                surname: Some("User".to_string()),
                comment: None,
                born: Some(VagueDate {
                    start: Some(NaiveDate::from_ymd_opt(2025, 10, 13).unwrap()),
                    end: Some(NaiveDate::from_ymd_opt(2025, 10, 13).unwrap()),
                    date_type: VagueDateType::Day
                }),
                died: None,
                person_floreat: None,
                entered_by: Some(NBNKey(FStr::try_from(b"TESTDATA00000001").unwrap())),
                entry_date: Some(NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(1999, 07, 16).unwrap(),
                    NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap()
                )),
                changed_by: None,
                changed_date: None,
                system_supplied_data: true
            }
        );
    }
}
