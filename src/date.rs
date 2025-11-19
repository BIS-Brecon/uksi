use sqlx::{Row, Type, postgres::PgRow, prelude::FromRow, types::chrono::NaiveDate};

/// A 'vague' date spanning a time range.
#[derive(Debug, FromRow, PartialEq)]
pub struct VagueDate {
    pub start: Option<NaiveDate>,
    pub end: Option<NaiveDate>,
    pub date_type: VagueDateType,
}

/// The type of vague date stored.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Type)]
#[sqlx(type_name = "varchar")]
pub enum VagueDateType {
    /// Normal Date.
    /// E.g. 23 Mar 1987.
    #[sqlx(rename = "D")]
    Day,
    /// Range between two Dates.
    /// E.g. 23 Mar 1987 - 30 Mar 1987.
    #[sqlx(rename = "DD")]
    DayRange,
    /// Any time in given Month.
    /// E.g. Mar 1987.
    #[sqlx(rename = "O")]
    Month,
    /// Range between two Months.
    /// E.g. Mar 1987 - Jun 1987
    #[sqlx(rename = "OO")]
    MonthRange,
    /// Any time in given Season.
    /// E.g. Summer 1987.
    #[sqlx(rename = "P")]
    Season,
    /// Any time in given Year.
    /// E.g. 1987.
    #[sqlx(rename = "Y")]
    Year,
    /// Range between two Years.
    /// E.g. 1981 - 1987.
    #[sqlx(rename = "YY")]
    YearRange,
    /// Known start Year.
    /// E.g. 1987 - ????.
    #[sqlx(rename = "Y-")]
    FromYear,
    /// Known end Year.
    /// E.g. ???? - 1987.
    #[sqlx(rename = "-Y")]
    ToYear,
    /// Month only.
    /// E.g. July.
    #[sqlx(rename = "M")]
    MonthOnly,
    /// Season only.
    /// E.g. Summer.
    #[sqlx(rename = "S")]
    SeasonOnly,
    /// Completely Unknown.
    #[sqlx(rename = "U")]
    Unknown,
}

impl VagueDate {
    pub fn from_row(row: &PgRow, prefix: &str) -> Result<Option<Self>, sqlx::Error> {
        // Try and fetch the date type and ...
        let date_type: Option<VagueDateType> =
            row.try_get(&*format!("{prefix}_vague_date_type"))?;

        // ... if it exists, return the other columns also.
        let date = match date_type {
            Some(date_type) => Some(Self {
                start: row.try_get(&*format!("{prefix}_vague_date_start"))?,
                end: row.try_get(&*format!("{prefix}_vague_date_end"))?,
                date_type,
            }),
            None => None,
        };

        Ok(date)
    }
}
