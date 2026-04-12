use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use super::column::ColumnId;

/// Unique identifier for a Card
pub type CardId = Uuid;

/// Represents a task card in a column
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: CardId,
    pub column_id: ColumnId,
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub position: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Card {
    /// Create a new card
    pub fn new(
        column_id: ColumnId,
        title: String,
        description: Option<String>,
        due_date: Option<DateTime<Utc>>,
        position: i32,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            column_id,
            title,
            description,
            due_date,
            position,
            created_at: now,
            updated_at: now,
        }
    }
}
