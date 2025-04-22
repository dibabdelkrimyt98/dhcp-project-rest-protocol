use rusqlite::Connection;

pub fn init_db(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS leases (
            ip TEXT PRIMARY KEY,
            mac TEXT NOT NULL,
            expiry INTEGER NOT NULL
        )",
        [],
    )?;
    Ok(())
}
