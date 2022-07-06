pub fn structure(conn: &rusqlite::Connection) -> rusqlite::Result<usize> {
    conn.execute(
        "\
    CREATE TABLE config (\
        key TEXT NOT NULL PRIMARY KEY, \
        value TEXT NOT NULL\
    );",
        [],
    )?;
    conn.execute(
        "\
    CREATE TABLE tasks (\
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, \
        begin_at TEXT NOT NULL, \
        begin_done INTEGER NOT NULL, \
        tweet_id INTEGER,\
        end_at TEXT, \
        end_done INTEGER NOT NULL, \
        draft TEXT NOT NULL\
    );",
        [],
    )
}
