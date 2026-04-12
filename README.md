# Katban Board

A personalized Kanban board for managing different domains of your life, built with Tauri + Rust + Svelte.

## Features

- **Multiple Life Domains**: Separate boards for Health & Fitness, Learning & Education, Relationships, Hobbies, and Household
- **Default Columns**: Each board comes with "To Do → In Progress → Done" workflow
- **Tag System**: Organize cards with custom tags and filter by multiple tags
- **Due Dates & Reminders**: Set due dates and get system notifications
- **SQLite Storage**: Fast, efficient local database at `~/.config/katban-board/katban.db`
- **API-First Design**: Clean Rust backend with TypeScript frontend
- **Linux Native**: Built as AppImage, .deb, and .rpm packages

## Tech Stack

- **Frontend**: Svelte + TypeScript
- **Backend**: Rust with Tauri 2
- **Database**: SQLite (via rusqlite)
- **Build**: Vite + Tauri CLI

## Project Structure

```
katban-board/
├── src/                          # Frontend (Svelte)
│   ├── components/               # Svelte components
│   │   └── BoardList.svelte     # Board list component
│   ├── lib/
│   │   ├── api/                 # API client functions
│   │   │   ├── boards.ts
│   │   │   ├── columns.ts
│   │   │   ├── cards.ts
│   │   │   ├── tags.ts
│   │   │   └── reminders.ts
│   │   └── types.ts             # TypeScript types
│   ├── stores/                   # Svelte stores
│   └── routes/
│       └── +page.svelte         # Main page
├── src-tauri/                    # Backend (Rust)
│   ├── src/
│   │   ├── api/                 # Tauri commands (API endpoints)
│   │   │   ├── boards.rs
│   │   │   ├── columns.rs
│   │   │   ├── cards.rs
│   │   │   ├── tags.rs
│   │   │   └── reminders.rs
│   │   ├── db/                  # Database layer
│   │   │   ├── schema.rs        # SQLite schema
│   │   │   └── connection.rs    # DB connection
│   │   ├── models/              # Data models
│   │   │   ├── board.rs
│   │   │   ├── column.rs
│   │   │   ├── card.rs
│   │   │   ├── tag.rs
│   │   │   └── reminder.rs
│   │   ├── lib.rs               # Tauri app entry point
│   │   └── main.rs
│   └── Cargo.toml               # Rust dependencies
└── TODO.md                       # Implementation todo list
```

## Prerequisites

### Linux (Ubuntu/Debian-based)

```bash
# Install system dependencies
sudo apt install -y \
  libwebkit2gtk-4.1-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libjavascriptcoregtk-4.1-dev \
  libsoup-3.0-dev

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Development

1. **Install dependencies**:
   ```bash
   npm install
   ```

2. **Run in development mode**:
   ```bash
   npm run tauri dev
   ```

   This will:
   - Compile the Rust backend
   - Start the Svelte frontend with hot-reload
   - Open the app in a window

## Building for Production

```bash
# Build for your current platform
npm run tauri build

# Output will be in:
# - src-tauri/target/release/bundle/deb/
# - src-tauri/target/release/bundle/rpm/
# - src-tauri/target/release/bundle/appimage/
```

## API Reference

### Boards
- `create_board(name: string, color?: string)`: Create a new board
- `get_boards()`: Get all boards
- `get_board(id: string)`: Get a specific board
- `update_board(id: string, name: string, color?: string)`: Update a board
- `delete_board(id: string)`: Delete a board

### Cards
- `create_card(column_id: string, title: string, description?: string, due_date?: string)`: Create a card
- `get_cards(column_id?: string, board_id?: string)`: Get cards with optional filters
- `update_card(id: string, title?: string, description?: string, due_date?: string)`: Update a card
- `move_card(id: string, column_id: string, position: number)`: Move card to different column/position

### Tags
- `get_tags(board_id: string)`: Get all tags for a board
- `create_tag(board_id: string, name: string, color?: string)`: Create a tag
- `add_tag_to_card(card_id: string, tag_id: string)`: Add tag to card
- `filter_cards_by_tag(board_id: string, tag_ids: string[])`: Filter cards by tags

### Reminders
- `get_due_cards(board_id?: string, from: string, to: string)`: Get cards due in date range
- `schedule_reminder(card_id: string, scheduled_at: string)`: Schedule a reminder
- `dismiss_reminder(id: string)`: Dismiss a reminder

## Database Location

The SQLite database is stored at:
```
~/.config/katban-board/katban.db
```

## Default Life Domains

On first run, the app creates these boards:
1. Health & Fitness (Green)
2. Learning & Education (Blue)
3. Relationships (Pink)
4. Hobbies & Creative Projects (Amber)
5. Household & Chores (Gray)

## License

MIT

## Status

🚧 **Under Active Development**

Currently implemented:
- ✅ Project initialization
- ✅ Rust backend with SQLite
- ✅ API commands for boards, columns, cards, tags, reminders
- ✅ TypeScript API client layer
- ✅ Basic UI component

Next steps:
- [ ] Full Kanban board UI with drag-and-drop
- [ ] Tag filtering system
- [ ] Due date picker and reminders
- [ ] Dark/Light theme
- [ ] Production builds
