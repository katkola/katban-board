use rusqlite::Connection;
use crate::models::{Tag, TagId, BoardId, CardId, Card, CardTag};
use crate::db::connection::create_connection;

/// Get all tags for a board
#[tauri::command]
pub fn get_tags(board_id: String) -> Result<Vec<Tag>, String> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    let bid: BoardId = board_id.parse::<BoardId>().map_err(|_| "Invalid board ID".to_string())?;
    
    let mut stmt = conn
        .prepare("SELECT id, board_id, name, color FROM tags WHERE board_id = ?1")
        .map_err(|e| e.to_string())?;
    
    let tags: Vec<Tag> = stmt
        .query_map(rusqlite::params![bid.to_string()], |row| {
            Ok(Tag {
                id: row.get::<_, String>(0)?.parse().map_err(|_| rusqlite::Error::InvalidQuery)?,
                board_id: row.get::<_, String>(1)?.parse().map_err(|_| rusqlite::Error::InvalidQuery)?,
                name: row.get(2)?,
                color: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|result| result.ok())
        .collect();
    
    Ok(tags)
}

/// Create a new tag
#[tauri::command]
pub fn create_tag(board_id: String, name: String, color: Option<String>) -> Result<Tag, String> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    let bid: BoardId = board_id.parse::<BoardId>().map_err(|_| "Invalid board ID".to_string())?;
    
    let tag = Tag::new(bid, name.clone(), color.clone());
    
    conn.execute(
        "INSERT INTO tags (id, board_id, name, color) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![
            tag.id.to_string(),
            tag.board_id.to_string(),
            tag.name.clone(),
            tag.color.clone().unwrap_or_default(),
        ],
    )
    .map_err(|e| e.to_string())?;
    
    Ok(tag)
}

/// Delete a tag
#[tauri::command]
pub fn delete_tag(id: String) -> Result<(), String> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    let tag_id: TagId = id.parse().map_err(|_| "Invalid tag ID".to_string())?;
    
    conn.execute("DELETE FROM tags WHERE id = ?1", rusqlite::params![tag_id.to_string()])
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

/// Add a tag to a card
#[tauri::command]
pub fn add_tag_to_card(card_id: String, tag_id: String) -> Result<(), String> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    let cid: CardId = card_id.parse::<CardId>().map_err(|_| "Invalid card ID".to_string())?;
    let tid: TagId = tag_id.parse::<TagId>().map_err(|_| "Invalid tag ID".to_string())?;
    
    conn.execute(
        "INSERT OR IGNORE INTO card_tags (card_id, tag_id) VALUES (?1, ?2)",
        rusqlite::params![cid.to_string(), tid.to_string()],
    )
    .map_err(|e| e.to_string())?;
    
    Ok(())
}

/// Remove a tag from a card
#[tauri::command]
pub fn remove_tag_from_card(card_id: String, tag_id: String) -> Result<(), String> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    let cid: CardId = card_id.parse::<CardId>().map_err(|_| "Invalid card ID".to_string())?;
    let tid: TagId = tag_id.parse::<TagId>().map_err(|_| "Invalid tag ID".to_string())?;
    
    conn.execute(
        "DELETE FROM card_tags WHERE card_id = ?1 AND tag_id = ?2",
        rusqlite::params![cid.to_string(), tid.to_string()],
    )
    .map_err(|e| e.to_string())?;
    
    Ok(())
}

/// Filter cards by tags
#[tauri::command]
pub fn filter_cards_by_tag(board_id: String, tag_ids: Vec<String>) -> Result<Vec<Card>, String> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    
    if tag_ids.is_empty() {
        return get_cards_for_board(&conn, board_id);
    }
    
    let placeholders = tag_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT DISTINCT c.id, c.column_id, c.title, c.description, c.due_date, c.position, c.created_at, c.updated_at
         FROM cards c
         JOIN columns col ON c.column_id = col.id
         JOIN card_tags ct ON c.id = ct.card_id
         WHERE col.board_id = ?1 AND ct.tag_id IN ({})",
        placeholders
    );
    
    let mut params = vec![board_id.clone()];
    params.extend(tag_ids);
    
    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    
    let cards = stmt
        .query_map(rusqlite::params_from_iter(params.iter()), |row| {
            Ok(crate::models::Card {
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
        })
        .map_err(|e| e.to_string())?
        .filter_map(|result| result.ok())
        .collect();
    
    Ok(cards)
}

fn get_cards_for_board(conn: &Connection, board_id: String) -> Result<Vec<crate::models::Card>, String> {
    let query = "SELECT c.id, c.column_id, c.title, c.description, c.due_date, c.position, c.created_at, c.updated_at
                 FROM cards c
                 JOIN columns col ON c.column_id = col.id
                 WHERE col.board_id = ?1
                 ORDER BY c.position";
    
    let mut stmt = conn.prepare(query).map_err(|e| e.to_string())?;
    
    let cards = stmt
        .query_map([board_id], |row| {
            Ok(crate::models::Card {
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
        })
        .map_err(|e| e.to_string())?
        .filter_map(|result| result.ok())
        .collect();
    
    Ok(cards)
}
