use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use super::card::CardId;

/// Unique identifier for a Reminder
pub type ReminderId = Uuid;

/// Represents a reminder for a card's due date
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reminder {
    pub id: ReminderId,
    pub card_id: CardId,
    pub scheduled_at: DateTime<Utc>,
    pub notified: bool,
}

impl Reminder {
    /// Create a new reminder
    pub fn new(card_id: CardId, scheduled_at: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4(),
            card_id,
            scheduled_at,
            notified: false,
        }
    }
}
