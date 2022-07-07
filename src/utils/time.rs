use crate::err;
use chrono::{TimeZone, Utc};
use rusqlite::types::ToSqlOutput;
use rusqlite::ToSql;
use std::ops::Add;

const HOUR_AS_SEC: i32 = 60 * 60;

const HOUR_AS_MIN: i64 = 60;

pub struct SqlTimestamp(pub chrono::DateTime<Utc>);

impl ToSql for SqlTimestamp {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.0.timestamp()))
    }
}

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

pub fn parse_date_time_now_plus(s: &String) -> Result<chrono::DateTime<Utc>, err::Error> {
    let s = s.trim_start_matches("now");

    let dur_hour: i64;
    let dur_minute: i64;

    let r1 = regex::Regex::new(r"(\d+)h").expect("Error when creating regex pattern");
    let c1 = r1.captures(s);
    if c1.is_none() {
        dur_hour = 0;
    } else {
        let c1 = c1.unwrap().get(1);
        if c1.is_none() {
            dur_hour = 0;
        } else {
            let h = c1.unwrap().as_str().parse::<i64>();
            if h.is_err() {
                return Err(err::Error::new(
                    Some(11),
                    "Unable to parse dur_hour(string) to int".to_string(),
                ));
            }
            dur_hour = h.unwrap();
        }
    }

    let r2 = regex::Regex::new(r"(\d+)m").expect("Error when creating regex pattern");
    let c2 = r2.captures(s);
    if c2.is_none() {
        dur_minute = 0;
    } else {
        let c2 = c2.unwrap().get(1);
        if c2.is_none() {
            dur_minute = 0;
        } else {
            let m = c2.unwrap().as_str().parse::<i64>();
            if m.is_err() {
                return Err(err::Error::new(
                    Some(11),
                    "Unable to parse dur_minute(string) to int".to_string(),
                ));
            }
            dur_minute = m.unwrap();
        }
    }

    Ok(Utc::now().add(chrono::Duration::minutes(
        dur_hour * HOUR_AS_MIN + dur_minute,
    )))
}

// "now+3h", "now+3h20m", "now+20m"
// TIME_STAMP
// 2022-02-12 13:23:45 +9
pub fn parse_date_time(s: &String) -> Result<chrono::DateTime<Utc>, err::Error> {
    let s = s.trim().to_string();

    if s.starts_with("now") {
        return parse_date_time_now_plus(&s);
    }

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
