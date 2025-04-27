use rusqlite::{Connection, Result};

pub fn init_blacklist_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS blacklist (
            mac_address TEXT PRIMARY KEY,
            reason TEXT,
            added_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    Ok(())
}

pub fn is_blacklisted(conn: &Connection, mac_address: &str) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT 1 FROM blacklist WHERE mac_address = ?")?;
    Ok(stmt.exists([mac_address])?)
}

pub fn add_to_blacklist(conn: &Connection, mac_address: &str, reason: &str) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO blacklist (mac_address, reason) VALUES (?1, ?2)",
        [mac_address, reason],
    )?;
    Ok(())
}
