use rusqlite::Connection;

pub mod init {
    use super::*;

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
} 