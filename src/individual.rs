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
            born: VagueDate::from_row(row, "born")?,
            died: VagueDate::from_row(row, "died")?,
            person_floreat: row.try_get("person_floreat")?,
            entered_by: row.try_get("entered_by")?,
            entry_date: row.try_get("entry_date")?,
            changed_by: row.try_get("changed_by")?,
            changed_date: row.try_get("changed_date")?,
            system_supplied_data: row.try_get("system_supplied_data")?,
        })
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
