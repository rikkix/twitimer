use rusqlite::params;

pub fn start_done(conn: &rusqlite::Connection, id: u32) -> rusqlite::Result<()> {
    conn.execute(
        "UPDATE tasks SET start_done = 1 WHERE id == ?1 ",
        params![id],
    )?;
    Ok(())
}

pub fn end_done(conn: &rusqlite::Connection, id: u32) -> rusqlite::Result<()> {
    conn.execute("UPDATE tasks SET end_done = 1 WHERE id == ?1 ", params![id])?;
    Ok(())
}

pub fn tweet_id(conn: &rusqlite::Connection, id: u32, tweet_id: u64) -> rusqlite::Result<()> {
    conn.execute(
        "UPDATE tasks SET tweet_id = ?2 WHERE id == ?1 ",
        params![id, tweet_id],
    )?;
    Ok(())
}
