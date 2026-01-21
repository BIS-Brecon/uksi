use chrono::{NaiveDate, NaiveDateTime};
use mdbsql::{Error, mdbsql::Row};

use crate::date::{VagueDate, VagueDateType};

/// Try and parse a bool from an Access DB
pub(crate) fn bool_from_row(row: &Row, index: usize) -> Result<bool, Error> {
    let int: isize = row.get(index)?;

    Ok(match int {
        0 => false,
        _ => true,
    })
}

/// Try and parse a date from an Access DB
pub(crate) fn date_from_row(row: &Row, index: usize) -> Result<Option<NaiveDate>, Error> {
    let dat = datetime_from_row(row, index)?.map(|d| NaiveDate::from(d));

    Ok(dat)
}

/// Try and parse a datetime from an Access DB
pub(crate) fn datetime_from_row(row: &Row, index: usize) -> Result<Option<NaiveDateTime>, Error> {
    let s: Option<String> = row.get(index)?;
    if let Some(s) = s
        && !s.is_empty()
    {
        let date = NaiveDateTime::parse_from_str(&s.trim(), "%m/%d/%Y %H:%M:%S").map_err(|e| {
            Error::FromSqlError(serde_plain::Error::Parse(
                "Error parsing datetime",
                format!("{s}: {}", e.to_string()),
            ))
        })?;

        return Ok(Some(date));
    }

    Ok(None)
}

/// Try and parse a vague date type from an Access DB
pub(crate) fn vague_date_type_from_row(
    row: &Row,
    index: usize,
) -> Result<Option<VagueDateType>, Error> {
    let s: Option<String> = row.get(index)?;
    if let Some(s) = s
        && !s.is_empty()
    {
        let t = match s.trim() {
            "D" => VagueDateType::Day,
            "DD" => VagueDateType::DayRange,
            "D-" => VagueDateType::FromDay,
            "O" => VagueDateType::Month,
            "OO" => VagueDateType::MonthRange,
            "P" => VagueDateType::Season,
            "Y" => VagueDateType::Year,
            "YY" => VagueDateType::YearRange,
            "Y-" => VagueDateType::FromYear,
            "-Y" => VagueDateType::ToYear,
            "M" => VagueDateType::MonthOnly,
            "S" => VagueDateType::SeasonOnly,
            "U" => VagueDateType::Unknown,
            _ => {
                return Err(Error::FromSqlError(serde_plain::Error::Parse(
                    "Error parsing date type",
                    format!("{s} is not a valid VagueDateType"),
                )));
            }
        };

        return Ok(Some(t));
    }

    Ok(None)
}

/// Try and parse a vague date from an Access DB
pub(crate) fn vague_date_from_row(row: &Row, index: usize) -> Result<Option<VagueDate>, Error> {
    if let Some(date_type) = vague_date_type_from_row(row, index + 2)? {
        return Ok(Some(VagueDate {
            start: datetime_from_row(row, index)?.map(|t| NaiveDate::from(t)),
            end: datetime_from_row(row, index + 1)?.map(|t| NaiveDate::from(t)),
            date_type,
        }));
    }

    Ok(None)
}
