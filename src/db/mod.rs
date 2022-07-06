use crate::db::query::config;
use chrono::Utc;
use rusqlite;
use rusqlite::types::ToSqlOutput;
use rusqlite::ToSql;
use std::path;

pub mod init;
pub mod insert;
pub mod query;
pub mod remove;

pub const DB_PATH: &str = {
    let s = option_env!("TWITIMER_DB");
    match s {
        Some(str) => str,
        None => "twitimer.db",
    }
};

pub fn new_conn() -> Result<rusqlite::Connection, rusqlite::Error> {
    rusqlite::Connection::open(DB_PATH)
}

pub fn exist() -> bool {
    path::Path::new(DB_PATH).is_file()
}

pub fn check_available() -> bool {
    if !exist() {
        return false;
    }
    let conn = new_conn();
    if let Err(_) = conn {
        return false;
    }

    let conn = conn.unwrap();
    if config(&conn).is_err() {
        return false;
    }

    let stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='tasks';");
    if stmt.is_err() {
        return false;
    }
    let mut stmt = stmt.unwrap();
    let result: rusqlite::Result<String> = stmt.query_row([], |row| row.get(0));
    if result.is_err() || result.unwrap().is_empty() {
        return false;
    }

    true
}
