use crate::{Config, Twitimer};
use rusqlite::params;

pub fn table_config(conn: &rusqlite::Connection, k: &str, v: &String) -> rusqlite::Result<usize> {
    conn.execute("INSERT INTO config VALUES (?1, ?2);", params![k, v])
}

pub fn config(conn: &rusqlite::Connection, conf: &Config) -> rusqlite::Result<()> {
    table_config(
        conn,
        "version_store",
        &conf.version.clone().to_store().to_string(),
    )?;
    table_config(conn, "consumer_key", &conf.credential.consumer_key)?;
    table_config(conn, "consumer_secret", &conf.credential.consumer_secret)?;
    table_config(conn, "access_key", &conf.credential.access_key)?;
    table_config(conn, "access_secret", &conf.credential.access_secret)?;
    Ok(())
}

pub fn task(conn: &rusqlite::Connection, task: &Twitimer) -> rusqlite::Result<usize> {
    conn.execute(
        "INSERT INTO tasks (begin_at, begin_done, end_at, end_done, draft) VALUES (?1, ?2, ?3, ?4, ?5);",
        params![
            &task.begin_at,
            &task.begin_done,
            &task.end_at,
            &task.end_done,
            &task.draft
        ],
    )
}
