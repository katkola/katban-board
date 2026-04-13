use chrono::Utc;
// use rusqlite::Connection;

use crate::db::connection::create_connection;
use crate::models::{Board, BoardId};

/// Database state for Tauri
// pub struct DbState(pub Connection);

/// Create a new board
#[tauri::command]
pub fn create_board(name: String, color: Option<String>) -> Result<Board, String> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    let board = Board::new(name, color);

    conn.execute(
        "INSERT INTO boards (id, name, color, created_at, updated_at) 
         VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![
            board.id.to_string(),
            board.name.clone(),
            board.color.clone().unwrap_or_default(),
            board.created_at.to_rfc3339(),
            board.updated_at.to_rfc3339(),
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(board)
}

/// Get all boards
#[tauri::command]
pub fn get_boards() -> Result<Vec<Board>, String> {
    let conn = create_connection().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, name, color, created_at, updated_at FROM boards ORDER BY created_at")
        .map_err(|e| e.to_string())?;

    let boards = stmt
        .query_map([], |row| {
            Ok(Board {
                id: row
                    .get::<_, String>(0)?
                    .parse()
                    .map_err(|_| rusqlite::Error::InvalidQuery)?,
                name: row.get(1)?,
                color: row.get(2)?,
                created_at: row
                    .get::<_, String>(3)?
                    .parse()
                    .map_err(|_| rusqlite::Error::InvalidQuery)?,
                updated_at: row
                    .get::<_, String>(4)?
                    .parse()
                    .map_err(|_| rusqlite::Error::InvalidQuery)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|result| result.ok())
        .collect();

    Ok(boards)
}

/// Get a single board by ID
#[tauri::command]
pub fn get_board(id: String) -> Result<Board, String> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    let board_id: BoardId = id.parse().map_err(|_| "Invalid board ID".to_string())?;

    let board = conn
        .query_row(
            "SELECT id, name, color, created_at, updated_at FROM boards WHERE id = ?1",
            rusqlite::params![board_id.to_string()],
            |row| {
                Ok(Board {
                    id: row
                        .get::<_, String>(0)?
                        .parse()
                        .map_err(|_| rusqlite::Error::InvalidQuery)?,
                    name: row.get(1)?,
                    color: row.get(2)?,
                    created_at: row
                        .get::<_, String>(3)?
                        .parse()
                        .map_err(|_| rusqlite::Error::InvalidQuery)?,
                    updated_at: row
                        .get::<_, String>(4)?
                        .parse()
                        .map_err(|_| rusqlite::Error::InvalidQuery)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;

    Ok(board)
}

/// Update a board
#[tauri::command]
pub fn update_board(id: String, name: String, color: Option<String>) -> Result<Board, String> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    let board_id: BoardId = id.parse().map_err(|_| "Invalid board ID".to_string())?;
    let updated_at = Utc::now();

    conn.execute(
        "UPDATE boards SET name = ?1, color = ?2, updated_at = ?3 WHERE id = ?4",
        rusqlite::params![
            name.clone(),
            color.clone().unwrap_or_default(),
            updated_at.to_rfc3339(),
            board_id.to_string(),
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(Board {
        id: board_id,
        name,
        color,
        created_at: Utc::now(), // Will be fetched properly in get_board
        updated_at,
    })
}

/// Delete a board
#[tauri::command]
pub fn delete_board(id: String) -> Result<(), String> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    let board_id: BoardId = id.parse().map_err(|_| "Invalid board ID".to_string())?;

    conn.execute(
        "DELETE FROM boards WHERE id = ?1",
        rusqlite::params![board_id.to_string()],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}
