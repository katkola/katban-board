use rusqlite::Connection;
use crate::models::{Column, BoardId};
use crate::db::connection::create_connection;

/// Get all columns for a board
#[tauri::command]
pub fn get_columns(board_id: String) -> Result<Vec<Column>, String> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    let bid: BoardId = board_id.parse().map_err(|_| "Invalid board ID".to_string())?;
    
    let mut stmt = conn
        .prepare(
            "SELECT id, board_id, name, position, created_at 
             FROM columns 
             WHERE board_id = ?1 
             ORDER BY position",
        )
        .map_err(|e| e.to_string())?;
    
    let columns = stmt
        .query_map(rusqlite::params![bid.to_string()], |row| {
            Ok(Column {
                id: row.get::<_, String>(0)?.parse().map_err(|_| rusqlite::Error::InvalidQuery)?,
                board_id: row.get::<_, String>(1)?.parse().map_err(|_| rusqlite::Error::InvalidQuery)?,
                name: row.get(2)?,
                position: row.get(3)?,
                created_at: row.get::<_, String>(4)?.parse().map_err(|_| rusqlite::Error::InvalidQuery)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|result| result.ok())
        .collect();
    
    Ok(columns)
}

/// Reorder columns in a board
#[tauri::command]
pub fn reorder_columns(_board_id: String, column_ids: Vec<String>) -> Result<(), String> {
    let mut conn = create_connection().map_err(|e| e.to_string())?;
    
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    
    for (position, id) in column_ids.iter().enumerate() {
        tx.execute(
            "UPDATE columns SET position = ?1 WHERE id = ?2",
            rusqlite::params![position as i32, id.clone()],
        )
        .map_err(|e| e.to_string())?;
    }
    
    tx.commit().map_err(|e| e.to_string())?;
    Ok(())
}
