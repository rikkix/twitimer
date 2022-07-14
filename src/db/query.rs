use crate::twitter::Credential;
use crate::utils::time::SqlTimestamp;
use crate::version::Version;
use crate::{Config, Error, Twitimer};
use chrono::{TimeZone, Utc};
use itertools::Itertools;
use rusqlite::{params, Rows};

pub fn table_config(conn: &rusqlite::Connection, key: &str) -> rusqlite::Result<String> {
    let mut stmt = conn.prepare("SELECT value FROM config WHERE key = ?1;")?;
    stmt.query_row(params![key], |row| Ok(row.get(0)))?
}

pub fn config(conn: &rusqlite::Connection) -> Result<Config, Error> {
    let cred = Credential {
        consumer_key: table_config(conn, "consumer_key")?,
        consumer_secret: table_config(conn, "consumer_secret")?,
        access_key: table_config(conn, "access_key")?,
        access_secret: table_config(conn, "access_secret")?,
    };

    let vs = table_config(conn, "version_store")?.parse::<u32>()?;
    let version = Version::from_store(vs).expect("Unable to parse version from store");
    Ok(Config {
        version,
        credential: cred,
    })
}

pub fn all_tasks(conn: &rusqlite::Connection) -> rusqlite::Result<Vec<Twitimer>> {
    let mut stmt = conn.prepare(
        "SELECT id, begin_at, begin_done, tweet_id, end_at, end_done, draft FROM tasks;",
    )?;

    let mut rows = stmt.query([])?;
    let mut result: Vec<Twitimer> = Vec::new();
    loop {
        let row = rows.next()?;
        if row.is_none() {
            break;
        }
        let row = row.unwrap();

        let end_at: Option<chrono::DateTime<Utc>>;
        let end: Option<SqlTimestamp> = row.get(4)?;
        if end.is_none() {
            end_at = None
        } else {
            end_at = Some(end.unwrap().0);
        }

        result.push(Twitimer {
            id: row.get(0)?,
            begin_at: Utc.timestamp(row.get(1)?, 0),
            begin_done: row.get(2)?,
            tweet_id: row.get(3)?,
            end_at,
            end_done: row.get(5)?,
            draft: row.get(6)?,
        })
    }
    Ok(result)
}

pub fn tasks(conn: &rusqlite::Connection, ids: &Vec<u32>) -> rusqlite::Result<Vec<Twitimer>> {
    // hack
    let mut stmt = conn.prepare(
        format!(
            "SELECT id, begin_at, begin_done, tweet_id, end_at, end_done, draft FROM tasks WHERE id IN ({});", 
                Itertools::join(&mut ids.iter(), ",")
        ).as_str()
    )?;

    let mut rows = stmt.query([])?;
    parse_rows_to_vec_twitimer(&mut rows)
}

pub fn tasks_to_start(conn: &rusqlite::Connection) -> rusqlite::Result<Vec<Twitimer>> {
    let mut stmt = conn.prepare(
        "SELECT id, begin_at, begin_done, tweet_id, end_at, end_done, draft FROM tasks \
        WHERE begin_done == 0 AND begin_at <= ?1 AND tweet_id IS NULL;",
    )?;

    let mut rows = stmt.query(params![Utc::now().timestamp()])?;
    parse_rows_to_vec_twitimer(&mut rows)
}

pub fn tasks_to_end(conn: &rusqlite::Connection) -> rusqlite::Result<Vec<Twitimer>> {
    let mut stmt = conn.prepare(
        "SELECT id, begin_at, begin_done, tweet_id, end_at, end_done, draft FROM tasks \
        WHERE begin_done == 1 AND end_done == 0 AND end_at IS NOT NULL AND end_at <= ?1 AND tweet_id IS NOT NULL;",
    )?;

    let mut rows = stmt.query(params![Utc::now().timestamp()])?;
    parse_rows_to_vec_twitimer(&mut rows)
}

fn parse_rows_to_vec_twitimer(rows: &mut Rows) -> rusqlite::Result<Vec<Twitimer>> {
    let mut result: Vec<Twitimer> = Vec::new();
    loop {
        let row = rows.next()?;
        if row.is_none() {
            break;
        }
        let row = row.unwrap();

        let end_at: Option<chrono::DateTime<Utc>>;
        let end: Option<SqlTimestamp> = row.get(4)?;
        if end.is_none() {
            end_at = None
        } else {
            end_at = Some(end.unwrap().0);
        }

        result.push(Twitimer {
            id: row.get(0)?,
            begin_at: Utc.timestamp(row.get(1)?, 0),
            begin_done: row.get(2)?,
            tweet_id: row.get(3)?,
            end_at,
            end_done: row.get(5)?,
            draft: row.get(6)?,
        })
    }
    Ok(result)
}
