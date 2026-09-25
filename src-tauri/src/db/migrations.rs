use rusqlite::{Connection, Result};

pub fn run(connection: &Connection) -> Result<()> {
    connection.execute_batch(
        "
        PRAGMA foreign_keys = ON;
        PRAGMA journal_mode = WAL;

        CREATE TABLE IF NOT EXISTS lists (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            created_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS items (
            list_id INTEGER NOT NULL,
            code TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            expires_at INTEGER NOT NULL,
            PRIMARY KEY (list_id, code),
            FOREIGN KEY (list_id) REFERENCES lists(id) ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_items_expiry ON items(expires_at);
        CREATE INDEX IF NOT EXISTS idx_items_list_created ON items(list_id, created_at);

        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );

        INSERT OR IGNORE INTO settings(key, value) VALUES
            ('first_run_completed', 'false'),
            ('run_mode', 'local');
        ",
    )
}
