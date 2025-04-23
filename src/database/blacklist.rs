use rusqlite::{params, Connection};
use std::error::Error;

pub fn init_blacklist_table(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS blacklist (
            mac TEXT PRIMARY KEY,
            reason TEXT
        )",
        [],
    )?;
    Ok(())
}

pub fn is_blacklisted(conn: &Connection, mac: &str) -> rusqlite::Result<bool> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM blacklist WHERE mac = ?1")?;
    let count: i64 = stmt.query_row(params![mac], |row| row.get(0))?;
    Ok(count > 0)
}

pub fn add_to_blacklist(conn: &Connection, mac: &str, reason: &str) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO blacklist (mac, reason) VALUES (?1, ?2)",
        params![mac, reason],
    )?;
    Ok(())
}
