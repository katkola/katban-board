// Tests for Board model and operations
#[cfg(test)]
mod tests {
    use crate::models::board::Board;

    #[test]
    fn test_board_creation() {
        let board = Board {
            id: "test-id".to_string(),
            name: "Test Board".to_string(),
            color: "#FF6B6B".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };

        assert_eq!(board.name, "Test Board");
        assert_eq!(board.color, "#FF6B6B");
        assert_eq!(board.id, "test-id");
    }

    #[test]
    fn test_board_color_validation() {
        let valid_colors = vec!["#FF6B6B", "#000000", "#FFFFFF", "#4ECDC4"];

        for color in valid_colors {
            let board = Board {
                id: "test".to_string(),
                name: "Test".to_string(),
                color: color.to_string(),
                created_at: "2024-01-01T00:00:00Z".to_string(),
                updated_at: "2024-01-01T00:00:00Z".to_string(),
            };
            assert!(board.color.starts_with('#'));
            assert_eq!(board.color.len(), 7); // #RRGGBB format
        }
    }

    #[test]
    fn test_board_equality() {
        let board1 = Board {
            id: "1".to_string(),
            name: "Board A".to_string(),
            color: "#FF6B6B".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };

        let board2 = Board {
            id: "1".to_string(),
            name: "Board A".to_string(),
            color: "#FF6B6B".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };

        assert_eq!(board1.id, board2.id);
        assert_eq!(board1.name, board2.name);
    }
}
