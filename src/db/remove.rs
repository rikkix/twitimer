use itertools::Itertools;
use rusqlite::params;

pub fn task(conn: &rusqlite::Connection, id: u32) -> rusqlite::Result<usize> {
    conn.execute("DELETE FROM tasks WHERE id == ?1;", params![id])
}

pub fn tasks(conn: &rusqlite::Connection, ids: &Vec<u32>) -> rusqlite::Result<usize> {
    conn.execute(
        format!(
            "DELETE FROM tasks WHERE id IN ({});",
            Itertools::join(&mut ids.iter(), ",")
        )
        .as_str(),
        [],
    )
}
