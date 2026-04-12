use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use super::board::BoardId;

/// Unique identifier for a Column
pub type ColumnId = Uuid;

/// Represents a column in a Kanban board
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub id: ColumnId,
    pub board_id: BoardId,
    pub name: String,
    pub position: i32,
    pub created_at: DateTime<Utc>,
}

impl Column {
    /// Create a new column
    pub fn new(board_id: BoardId, name: String, position: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            board_id,
            name,
            position,
            created_at: Utc::now(),
        }
    }
}

/// Default columns for a new board
pub fn default_columns(board_id: BoardId) -> Vec<Column> {
    vec![
        Column::new(board_id, "To Do".to_string(), 0),
        Column::new(board_id, "In Progress".to_string(), 1),
        Column::new(board_id, "Done".to_string(), 2),
    ]
}
