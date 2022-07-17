use crate::db::query::config;
use rusqlite;
use std::{env, path};

pub mod init;
pub mod insert;
pub mod query;
pub mod remove;
pub mod update;

pub fn db_path() -> String {
    let s = env::var("TWITIMER_DB");
    match s {
        Ok(str) => str,
        Err(_) => dirs::home_dir()
            .expect("Error getting home dir")
            .join(path::Path::new(".twitimer.db"))
            .into_os_string()
            .into_string()
            .expect("Error when unwrapping OsString"),
    }
}

pub fn new_conn() -> Result<rusqlite::Connection, rusqlite::Error> {
    rusqlite::Connection::open(db_path())
}

pub fn exist() -> bool {
    path::Path::new(db_path().as_str()).is_file()
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
