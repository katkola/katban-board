use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Unique identifier for a Board
pub type BoardId = Uuid;

/// Represents a Kanban board (domain)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    pub id: BoardId,
    pub name: String,
    pub color: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Board {
    /// Create a new board with the given name and optional color
    pub fn new(name: String, color: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            color,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Default boards for the user's life domains
pub fn default_boards() -> Vec<Board> {
    vec![
        Board::new("Health & Fitness".to_string(), Some("#10B981".to_string())),
        Board::new("Learning & Education".to_string(), Some("#3B82F6".to_string())),
        Board::new("Relationships".to_string(), Some("#EC4899".to_string())),
        Board::new("Hobbies & Creative Projects".to_string(), Some("#F59E0B".to_string())),
        Board::new("Household & Chores".to_string(), Some("#6B7280".to_string())),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_creation() {
        let board = Board::new("Test Board".to_string(), Some("#FF6B6B".to_string()));
        assert_eq!(board.name, "Test Board");
        assert_eq!(board.color, Some("#FF6B6B".to_string()));
    }

    #[test]
    fn test_board_without_color() {
        let board = Board::new("Test Board".to_string(), None);
        assert_eq!(board.name, "Test Board");
        assert_eq!(board.color, None);
    }

    #[test]
    fn test_default_boards_count() {
        let boards = default_boards();
        assert_eq!(boards.len(), 5);
    }

    #[test]
    fn test_default_boards_have_names() {
        let boards = default_boards();
        assert!(boards.iter().all(|b| !b.name.is_empty()));
    }

    #[test]
    fn test_default_boards_have_colors() {
        let boards = default_boards();
        assert!(boards.iter().all(|b| b.color.is_some()));
    }

    #[test]
    fn test_board_id_is_unique() {
        let board1 = Board::new("Test".to_string(), None);
        let board2 = Board::new("Test".to_string(), None);
        assert_ne!(board1.id, board2.id);
    }
}
