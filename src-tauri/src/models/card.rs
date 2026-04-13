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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_creation() {
        let col_id = Uuid::new_v4();
        let card = Card::new(col_id, "Test Card".to_string(), None, None, 0);
        
        assert_eq!(card.title, "Test Card");
        assert_eq!(card.column_id, col_id);
        assert_eq!(card.position, 0);
        assert_eq!(card.description, None);
    }

    #[test]
    fn test_card_with_description() {
        let col_id = Uuid::new_v4();
        let card = Card::new(
            col_id,
            "Test Card".to_string(),
            Some("Test Description".to_string()),
            None,
            0,
        );
        
        assert_eq!(card.description, Some("Test Description".to_string()));
    }

    #[test]
    fn test_card_position() {
        let col_id = Uuid::new_v4();
        let card1 = Card::new(col_id, "First".to_string(), None, None, 0);
        let card2 = Card::new(col_id, "Second".to_string(), None, None, 1);
        
        assert!(card1.position < card2.position);
    }

    #[test]
    fn test_card_id_unique() {
        let col_id = Uuid::new_v4();
        let card1 = Card::new(col_id, "Card".to_string(), None, None, 0);
        let card2 = Card::new(col_id, "Card".to_string(), None, None, 0);
        
        assert_ne!(card1.id, card2.id);
    }

    #[test]
    fn test_card_with_due_date() {
        let col_id = Uuid::new_v4();
        let due_date = Some(Utc::now());
        let card = Card::new(col_id, "Test".to_string(), None, due_date, 0);
        
        assert!(card.due_date.is_some());
    }
}
