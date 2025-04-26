use rusqlite::{params, Connection, Result};
use crate::utils::device_parser::DeviceInfo;

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("devices.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS devices (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            mac_address TEXT NOT NULL,
            ip_address TEXT NOT NULL,
            device_type TEXT,
            brand TEXT,
            connection_type TEXT,
            data_transferred_bytes INTEGER
        )",
        [],
    )?;
    Ok(conn)
}

pub fn insert_device(conn: &Connection, device: &DeviceInfo) -> Result<()> {
    conn.execute(
        "INSERT INTO devices (mac_address, ip_address, device_type, brand, connection_type, data_transferred_bytes) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            device.mac_address,
            device.ip_address.to_string(),
            device.device_type,
            device.brand,
            device.connection_type,
            device.data_transferred_bytes
        ],
    )?;
    Ok(())
}
