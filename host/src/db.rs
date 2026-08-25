use rusqlite::{Connection, Result};
use std::sync::Mutex;
use once_cell::sync::Lazy;

pub static DB_CONN: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let conn = Connection::open("metadata.db").expect("Failed to open DB");
    Mutex::new(conn)
});

pub fn init_db() -> Result<()> {
    let conn = DB_CONN.lock().unwrap();
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS secrets_metadata (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            created_at TEXT NOT NULL
        )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS audit_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            action TEXT NOT NULL,
            timestamp TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tokens (
            token_id TEXT PRIMARY KEY,
            revoked INTEGER DEFAULT 0
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS dkg_nodes (
            node_id TEXT PRIMARY KEY,
            status TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS entropy_audits (
            audit_id TEXT PRIMARY KEY,
            status TEXT NOT NULL,
            timestamp TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}
