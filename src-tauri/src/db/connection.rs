use rusqlite::{Connection, Result};
use std::path::PathBuf;
use dirs::config_dir;

/// Get the database file path
pub fn get_db_path() -> PathBuf {
    let config_dir = config_dir().expect("Failed to find config directory");
    let app_dir = config_dir.join("katban-board");
    
    // Create directory if it doesn't exist
    std::fs::create_dir_all(&app_dir).expect("Failed to create app config directory");
    
    app_dir.join("katban.db")
}

/// Create a database connection
pub fn create_connection() -> Result<Connection> {
    let db_path = get_db_path();
    let mut conn = Connection::open(db_path)?;
    
    // Initialize schema
    crate::db::schema::init_db(&conn)?;
    
    // Seed default data
    crate::db::schema::seed_default_data(&mut conn)?;
    
    Ok(conn)
}
