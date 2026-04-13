use chrono::{DateTime, Utc};
use crate::models::{Card, CardId, ColumnId};
use crate::db::connection::create_connection;

/// Helper function to parse a card from a database row
fn parse_card_row(row: &rusqlite::Row) -> rusqlite::Result<Card> {
    Ok(Card {
        id: row.get::<_, String>(0)?.parse().map_err(|_| rusqlite::Error::InvalidQuery)?,
        column_id: row.get::<_, String>(1)?.parse().map_err(|_| rusqlite::Error::InvalidQuery)?,
        title: row.get(2)?,
        description: row.get(3)?,
        due_date: row.get::<_, String>(4).ok().and_then(|s| {
            if s.is_empty() {
                None
            } else {
                s.parse().ok()
            }
        }),
        position: row.get(5)?,
        created_at: row.get::<_, String>(6)?.parse().map_err(|_| rusqlite::Error::InvalidQuery)?,
        updated_at: row.get::<_, String>(7)?.parse().map_err(|_| rusqlite::Error::InvalidQuery)?,
    })
}

/// Create a new card
#[tauri::command]
pub fn create_card(
    column_id: String,
    title: String,
    description: Option<String>,
    due_date: Option<String>,
) -> Result<Card, String> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    let cid: ColumnId = column_id.parse::<ColumnId>().map_err(|_| "Invalid column ID".to_string())?;
    
    // Get the next position
    let position: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(position), -1) + 1 FROM cards WHERE column_id = ?1",
            rusqlite::params![column_id.clone()],
            |row| row.get::<_, i32>(0),
        )
        .map_err(|e| e.to_string())?;
    
    let due_date_parsed = if let Some(date_str) = due_date {
        Some(date_str.parse::<DateTime<Utc>>().map_err(|e| e.to_string())?)
    } else {
        None
    };
    
    let card = Card::new(cid, title.clone(), description.clone(), due_date_parsed, position);
    
    conn.execute(
        "INSERT INTO cards (id, column_id, title, description, due_date, position, created_at, updated_at) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        rusqlite::params![
            card.id.to_string(),
            card.column_id.to_string(),
            card.title.clone(),
            card.description.clone().unwrap_or_default(),
            card.due_date.map(|d| d.to_rfc3339()).unwrap_or_default(),
            card.position,
            card.created_at.to_rfc3339(),
            card.updated_at.to_rfc3339(),
        ],
    )
    .map_err(|e| e.to_string())?;
    
    Ok(card)
}

/// Get cards, optionally filtered by column or board
#[tauri::command]
pub fn get_cards(column_id: Option<String>, board_id: Option<String>) -> Result<Vec<Card>, String> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    
    let cards: Vec<Card> = if let Some(cid) = column_id {
        let mut stmt = conn.prepare(
            "SELECT id, column_id, title, description, due_date, position, created_at, updated_at 
             FROM cards WHERE column_id = ?1 ORDER BY position"
        ).map_err(|e| e.to_string())?;
        
        let result: Vec<Card> = stmt.query_map(rusqlite::params![cid], |row| parse_card_row(row))
            .map_err(|e| e.to_string())?
            .filter_map(|result| result.ok())
            .collect();
        result
    } else if let Some(bid) = board_id {
        let mut stmt = conn.prepare(
            "SELECT c.id, c.column_id, c.title, c.description, c.due_date, c.position, c.created_at, c.updated_at
             FROM cards c
             JOIN columns col ON c.column_id = col.id
             WHERE col.board_id = ?1
             ORDER BY c.position"
        ).map_err(|e| e.to_string())?;
        
        let result: Vec<Card> = stmt.query_map(rusqlite::params![bid], |row| parse_card_row(row))
            .map_err(|e| e.to_string())?
            .filter_map(|result| result.ok())
            .collect();
        result
    } else {
        let mut stmt = conn.prepare(
            "SELECT id, column_id, title, description, due_date, position, created_at, updated_at 
             FROM cards ORDER BY position"
        ).map_err(|e| e.to_string())?;
        
        let result: Vec<Card> = stmt.query_map([], |row| parse_card_row(row))
            .map_err(|e| e.to_string())?
            .filter_map(|result| result.ok())
            .collect();
        result
    };
    
    Ok(cards)
}

/// Get a single card by ID
#[tauri::command]
pub fn get_card(id: String) -> Result<Card, String> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    let card_id: CardId = id.parse().map_err(|_| "Invalid card ID".to_string())?;
    
    let card = conn
        .query_row(
            "SELECT id, column_id, title, description, due_date, position, created_at, updated_at 
             FROM cards WHERE id = ?1",
            rusqlite::params![card_id.to_string()],
            |row| parse_card_row(row),
        )
        .map_err(|e| e.to_string())?;
    
    Ok(card)
}

/// Update a card
#[tauri::command]
pub fn update_card(
    id: String,
    title: Option<String>,
    description: Option<String>,
    due_date: Option<String>,
) -> Result<Card, String> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    let card_id: CardId = id.parse().map_err(|_| "Invalid card ID".to_string())?;
    let updated_at = Utc::now();
    
    let due_date_str = due_date.map(|d| {
        d.parse::<DateTime<Utc>>()
            .map(|dt| dt.to_rfc3339())
            .unwrap_or_default()
    });
    
    conn.execute(
        "UPDATE cards SET title = COALESCE(?1, title), 
                        description = COALESCE(?2, description), 
                        due_date = COALESCE(?3, due_date), 
                        updated_at = ?4 
         WHERE id = ?5",
        rusqlite::params![
            title.clone().unwrap_or_default(),
            description.clone().unwrap_or_default(),
            due_date_str.unwrap_or_default(),
            updated_at.to_rfc3339(),
            card_id.to_string(),
        ],
    )
    .map_err(|e| e.to_string())?;
    
    // Fetch updated card
    get_card(id)
}

/// Delete a card
#[tauri::command]
pub fn delete_card(id: String) -> Result<(), String> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    let card_id: CardId = id.parse().map_err(|_| "Invalid card ID".to_string())?;
    
    conn.execute("DELETE FROM cards WHERE id = ?1", rusqlite::params![card_id.to_string()])
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

/// Move a card to a different column or position
#[tauri::command]
pub fn move_card(id: String, column_id: String, position: i32) -> Result<Card, String> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    let card_id: CardId = id.parse().map_err(|_| "Invalid card ID".to_string())?;
    let cid: ColumnId = column_id.parse().map_err(|_| "Invalid column ID".to_string())?;
    let updated_at = Utc::now();
    
    conn.execute(
        "UPDATE cards SET column_id = ?1, position = ?2, updated_at = ?3 WHERE id = ?4",
        rusqlite::params![cid.to_string(), position, updated_at.to_rfc3339(), card_id.to_string()],
    )
    .map_err(|e| e.to_string())?;
    
    // Fetch updated card
    get_card(id)
}
