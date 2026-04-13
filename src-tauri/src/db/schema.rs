use rusqlite::Connection;

/// Initialize the database schema
pub fn init_db(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Enable foreign keys
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;

    // Create boards table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS boards (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            color TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;

    // Create columns table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS columns (
            id TEXT PRIMARY KEY,
            board_id TEXT NOT NULL,
            name TEXT NOT NULL,
            position INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (board_id) REFERENCES boards(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // Create cards table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS cards (
            id TEXT PRIMARY KEY,
            column_id TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            due_date TEXT,
            position INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (column_id) REFERENCES columns(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // Create tags table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id TEXT PRIMARY KEY,
            board_id TEXT NOT NULL,
            name TEXT NOT NULL,
            color TEXT,
            FOREIGN KEY (board_id) REFERENCES boards(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // Create card_tags join table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS card_tags (
            card_id TEXT NOT NULL,
            tag_id TEXT NOT NULL,
            PRIMARY KEY (card_id, tag_id),
            FOREIGN KEY (card_id) REFERENCES cards(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // Create reminders table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS reminders (
            id TEXT PRIMARY KEY,
            card_id TEXT NOT NULL,
            scheduled_at TEXT NOT NULL,
            notified INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY (card_id) REFERENCES cards(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // Create indexes for efficient filtering
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_columns_board_id ON columns(board_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_cards_column_id ON cards(column_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_cards_due_date ON cards(due_date)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_tags_board_id ON tags(board_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_card_tags_card_id ON card_tags(card_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_card_tags_tag_id ON card_tags(tag_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_reminders_card_id ON reminders(card_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_reminders_scheduled_at ON reminders(scheduled_at)",
        [],
    )?;

    Ok(())
}

/// Insert default boards with their default columns
pub fn seed_default_data(conn: &mut Connection) -> Result<(), rusqlite::Error> {
    use crate::models::{board::default_boards, column::default_columns};

    // Check if we already have boards
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM boards", [], |row| row.get(0))?;
    
    if count > 0 {
        return Ok(()); // Already seeded
    }

    let tx = conn.transaction()?;

    for board in default_boards() {
        // Insert board
        tx.execute(
            "INSERT INTO boards (id, name, color, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
            [
                board.id.to_string(),
                board.name.clone(),
                board.color.clone().unwrap_or_default(),
                board.created_at.to_rfc3339(),
                board.updated_at.to_rfc3339(),
            ],
        )?;

        // Insert default columns for this board
        for column in default_columns(board.id) {
            tx.execute(
                "INSERT INTO columns (id, board_id, name, position, created_at) 
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                [
                    column.id.to_string(),
                    column.board_id.to_string(),
                    column.name,
                    column.position.to_string(),
                    column.created_at.to_rfc3339(),
                ],
            )?;
        }
    }

    tx.commit()?;
    Ok(())
}
