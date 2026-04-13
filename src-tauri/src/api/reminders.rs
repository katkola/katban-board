// use rusqlite::Connection;
use chrono::{DateTime, Utc};
use crate::models::{Reminder, ReminderId, CardId};
use crate::db::connection::create_connection;

/// Helper to parse a card row
fn parse_card_row(row: &rusqlite::Row) -> rusqlite::Result<crate::models::Card> {
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
}

/// Get cards due within a date range
#[tauri::command]
pub fn get_due_cards(
    board_id: Option<String>,
    from: String,
    to: String,
) -> Result<Vec<crate::models::Card>, String> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    
    let from_dt = from.parse::<DateTime<Utc>>().map_err(|e| e.to_string())?;
    let to_dt = to.parse::<DateTime<Utc>>().map_err(|e| e.to_string())?;
    
    let query = if board_id.is_some() {
        "SELECT c.id, c.column_id, c.title, c.description, c.due_date, c.position, c.created_at, c.updated_at \
         FROM cards c JOIN columns col ON c.column_id = col.id \
         WHERE col.board_id = ?1 AND c.due_date IS NOT NULL AND c.due_date BETWEEN ?2 AND ?3 \
         ORDER BY c.due_date"
    } else {
        "SELECT id, column_id, title, description, due_date, position, created_at, updated_at \
         FROM cards WHERE due_date IS NOT NULL AND due_date BETWEEN ?1 AND ?2 ORDER BY due_date"
    };
    
    let mut stmt = conn.prepare(query).map_err(|e| e.to_string())?;
    
    let cards = if let Some(bid) = board_id {
        stmt.query_map(
            rusqlite::params![bid, from_dt.to_rfc3339(), to_dt.to_rfc3339()],
            parse_card_row,
        )
    } else {
        stmt.query_map(
            rusqlite::params![from_dt.to_rfc3339(), to_dt.to_rfc3339()],
            parse_card_row,
        )
    }
    .map_err(|e| e.to_string())?
    .filter_map(|result| result.ok())
    .collect();
    
    Ok(cards)
}

/// Schedule a reminder for a card
#[tauri::command]
pub fn schedule_reminder(card_id: String, scheduled_at: String) -> Result<Reminder, String> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    let cid: CardId = card_id.parse().map_err(|_| "Invalid card ID".to_string())?;
    let scheduled_dt = scheduled_at.parse::<DateTime<Utc>>().map_err(|e| e.to_string())?;
    
    let reminder = Reminder::new(cid, scheduled_dt);
    
    conn.execute(
        "INSERT INTO reminders (id, card_id, scheduled_at, notified) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![
            reminder.id.to_string(),
            reminder.card_id.to_string(),
            reminder.scheduled_at.to_rfc3339(),
            0,
        ],
    )
    .map_err(|e| e.to_string())?;
    
    Ok(reminder)
}

/// Dismiss a reminder
#[tauri::command]
pub fn dismiss_reminder(id: String) -> Result<(), String> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    let reminder_id: ReminderId = id.parse().map_err(|_| "Invalid reminder ID".to_string())?;
    
    conn.execute(
        "UPDATE reminders SET notified = 1 WHERE id = ?1",
        rusqlite::params![reminder_id.to_string()],
    )
    .map_err(|e| e.to_string())?;
    
    Ok(())
}
