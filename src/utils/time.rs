use crate::err;
use chrono::{TimeZone, Utc};
use rusqlite::types::ToSqlOutput;
use rusqlite::ToSql;
use std::cmp::min;

const HOUR_AS_SEC: i32 = 60 * 60;

pub fn parse_date_time_ts(s: &String) -> Result<chrono::DateTime<Utc>, err::Error> {
    let ts = s.parse::<i64>();
    if ts.is_err() {
        return Err(err::Error::new(
            Some(11),
            "Unable to parse timestamp(string) to int".to_string(),
        ));
    }
    Ok(Utc.timestamp(ts.unwrap(), 0))
}

pub fn parse_date_time(s: &String) -> Result<chrono::DateTime<Utc>, err::Error> {
    let s = s.trim().to_string();

    // try to parse as unix timestamp
    let ts_result = parse_date_time_ts(&s);
    if ts_result.is_ok() {
        return ts_result;
    }

    // split date, time, timezone
    let dtt_spl = s.split(" ");
    let dtt_vec: Vec<&str> = dtt_spl.collect();
    if dtt_vec.len() != 3 {
        return Err(err::Error::new(
            Some(11),
            "Unable to parse datetime: invalid length of date, time, tz split".to_string(),
        ));
    }

    // parse date
    let date_spl = dtt_vec[0].split("-");
    let date_vec: Vec<&str> = date_spl.collect();
    if date_vec.len() != 3 {
        return Err(err::Error::new(
            Some(11),
            "Unable to parse date: invalid length of date split".to_string(),
        ));
    }

    let year = date_vec[0].parse::<i32>();
    if year.is_err() {
        return Err(err::Error::new(
            Some(11),
            "Unable to parse year: cannot convert year(string) to int".to_string(),
        ));
    }

    let month = date_vec[1].parse::<u32>();
    if month.is_err() {
        return Err(err::Error::new(
            Some(11),
            "Unable to parse month: cannot convert month(string) to uint".to_string(),
        ));
    }

    let day = date_vec[2].parse::<u32>();
    if day.is_err() {
        return Err(err::Error::new(
            Some(11),
            "Unable to parse day: cannot convert day(string) to uint".to_string(),
        ));
    }

    // parse time
    let time_spl = dtt_vec[1].split(":");
    let time_vec: Vec<&str> = time_spl.collect();
    if time_vec.len() != 3 {
        return Err(err::Error::new(
            Some(11),
            "Unable to parse time: invalid length of time split".to_string(),
        ));
    }

    let hour = time_vec[0].parse::<u32>();
    if hour.is_err() {
        return Err(err::Error::new(
            Some(11),
            "Unable to parse hour: cannot convert hour(string) to uint".to_string(),
        ));
    }

    let minute = time_vec[1].parse::<u32>();
    if minute.is_err() {
        return Err(err::Error::new(
            Some(11),
            "Unable to parse minute: cannot convert minute(string) to uint".to_string(),
        ));
    }

    let second = time_vec[2].parse::<u32>();
    if second.is_err() {
        return Err(err::Error::new(
            Some(11),
            "Unable to parse second: cannot convert second(string) to uint".to_string(),
        ));
    }

    // parse tz
    let tz = dtt_vec[2].parse::<i32>();
    if tz.is_err() {
        return Err(err::Error::new(
            Some(11),
            "Unable to parse timezone: cannot convert timezone(string) to int".to_string(),
        ));
    }

    Ok(chrono::DateTime::from(
        chrono::FixedOffset::east(tz.unwrap() * HOUR_AS_SEC)
            .ymd(year.unwrap(), month.unwrap(), day.unwrap())
            .and_hms(hour.unwrap(), minute.unwrap(), second.unwrap()),
    ))
}
