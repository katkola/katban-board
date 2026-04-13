// Tests for Card model and operations
#[cfg(test)]
mod tests {
    use crate::models::card::Card;

    #[test]
    fn test_card_creation() {
        let card = Card {
            id: "card-1".to_string(),
            title: "Test Card".to_string(),
            description: Some("Test Description".to_string()),
            column_id: "col-1".to_string(),
            board_id: "board-1".to_string(),
            position: 0,
            due_date: None,
            is_completed: false,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };

        assert_eq!(card.title, "Test Card");
        assert_eq!(card.column_id, "col-1");
        assert_eq!(card.position, 0);
        assert!(!card.is_completed);
    }

    #[test]
    fn test_card_without_description() {
        let card = Card {
            id: "card-1".to_string(),
            title: "Test Card".to_string(),
            description: None,
            column_id: "col-1".to_string(),
            board_id: "board-1".to_string(),
            position: 0,
            due_date: None,
            is_completed: false,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };

        assert!(card.description.is_none());
    }

    #[test]
    fn test_card_completion_status() {
        let mut card = Card {
            id: "card-1".to_string(),
            title: "Test Card".to_string(),
            description: None,
            column_id: "col-1".to_string(),
            board_id: "board-1".to_string(),
            position: 0,
            due_date: None,
            is_completed: false,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };

        assert!(!card.is_completed);
        card.is_completed = true;
        assert!(card.is_completed);
    }

    #[test]
    fn test_card_position_ordering() {
        let card1 = Card {
            id: "card-1".to_string(),
            title: "First".to_string(),
            description: None,
            column_id: "col-1".to_string(),
            board_id: "board-1".to_string(),
            position: 0,
            due_date: None,
            is_completed: false,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };

        let card2 = Card {
            id: "card-2".to_string(),
            title: "Second".to_string(),
            description: None,
            column_id: "col-1".to_string(),
            board_id: "board-1".to_string(),
            position: 1,
            due_date: None,
            is_completed: false,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };

        assert!(card1.position < card2.position);
    }
}
