pub mod items;
pub mod lists;
pub mod migrations;
pub mod settings;

use rusqlite::{Connection, Result};
use std::path::Path;

pub fn open(path: &Path) -> Result<Connection> {
    let connection = Connection::open(path)?;
    migrations::run(&connection)?;
    Ok(connection)
}
