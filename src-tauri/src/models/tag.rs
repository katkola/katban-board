use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::board::BoardId;
use super::card::CardId;

/// Unique identifier for a Tag
pub type TagId = Uuid;

/// Represents a tag/label for organizing cards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: TagId,
    pub board_id: BoardId,
    pub name: String,
    pub color: Option<String>,
}

impl Tag {
    /// Create a new tag
    pub fn new(board_id: BoardId, name: String, color: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            board_id,
            name,
            color,
        }
    }
}

/// Join table for many-to-many relationship between cards and tags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardTag {
    pub card_id: CardId,
    pub tag_id: TagId,
}

// impl CardTag {
//     /// Create a new card-tag association
//     pub fn new(card_id: CardId, tag_id: TagId) -> Self {
//         Self { card_id, tag_id }
//     }
// }
