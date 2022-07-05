use crate::version::{Version, VersionStore};
use crate::{Config, Credential, Error};
use rusqlite::params;

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
    cred.check_empty()?;

    let vs = table_config(conn, "version_store")?.parse::<u32>()?;
    let version = Version::from_store(vs).expect("Unable to parse version from store");
    Ok(Config {
        version,
        credential: cred,
    })
}
