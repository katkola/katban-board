# Katban Board — Workspace Instructions

Fullstack Kanban board app: **Rust backend** (Tauri 2) + **Svelte frontend** (SvelteKit). This guide helps agents understand the architecture, conventions, and Rust best practices for new contributors unfamiliar with the language.

## Architecture Overview

**Monolith pattern**: Single Tauri app with IPC between frontend and backend.

- **Frontend** (`src/`): SvelteKit + TypeScript. Routes mapped to Tauri commands.
- **Backend** (`src-tauri/`): Rust. Stateless API commands (Tauri IPC protocol), SQLite persistence.
- **Database**: SQLite at `~/.config/katban-board/katban.db` (auto-initialized on first run).

**Dataflow**: Frontend → Tauri IPC (JSON-RPC) → Rust handlers → SQLite → back to frontend.

### Key Files

| Path | Purpose |
|------|---------|
| `src-tauri/src/lib.rs` | App entry point; command registration |
| `src-tauri/src/api/*.rs` | Tauri IPC handlers (the "API") |
| `src-tauri/src/models/*.rs` | Data structures (Board, Card, Tag, etc.) |
| `src-tauri/src/db/*.rs` | SQLite schema + connection mgmt |
| `src/lib/api/*.ts` | Frontend API client (wrappers around IPC) |
| `src/routes/` | SvelteKit pages |

## Rust Convention Guide

### Tauri Commands (API Handlers)

**Location**: `src-tauri/src/api/*.rs`

**Pattern**:
```rust
#[tauri::command]
pub fn create_board(name: String, color: Option<String>) -> Result<Board, String> {
    let conn = create_connection().map_err(|e| e.to_string())?;
    // Validate inputs
    if name.is_empty() {
        return Err("Board name cannot be empty".to_string());
    }
    // Execute business logic
    let board = Board::new(name, color);
    boards_db::insert_board(&conn, &board)?;
    Ok(board)
}
```

**Rules**:
1. **Naming**: Verb-based (`create_board`, `get_cards`, `update_card`). Match IPC call names exactly.
2. **Registration**: Add to `tauri::generate_handler![]` in `lib.rs`.
3. **Error handling**: Always return `Result<T, String>`. Convert errors with `.map_err(|e| e.to_string())?`.
4. **Connection**: Fresh DB connection per command (stateless).
5. **Validation**: Reject invalid inputs early with descriptive messages.

**Common mistake**: Forgetting `.map_err(|e| e.to_string())?` when propagating errors. All errors must become `String` for frontend consumption.

### Database Layer

**Pattern**: Raw SQL with `rusqlite`, parametrized queries for safety.

**Schema** (`src-tauri/src/db/schema.rs`):
- Auto-runs on connection initialization
- Foreign keys enforced: `PRAGMA foreign_keys = ON;`

**Queries** (e.g., `src-tauri/src/db/connection.rs`):
```rust
const QUERY: &str = "INSERT INTO cards (id, title, column_id, board_id) VALUES (?, ?, ?, ?)";
conn.execute(QUERY, params![&card.id, &card.title, &card.column_id, &card.board_id])?;
```

**Row parsing helpers**:
```rust
fn parse_card_row(row: &Row) -> rusqlite::Result<Card> {
    Ok(Card {
        id: row.get("id")?,
        title: row.get("title")?,
        // ... more fields
    })
}
```

**Rules**:
1. Always use `params![]` macro—never string interpolation.
2. Fetch with `query_row()` for single result, `prepare()` + iteration for multiple.
3. Use row helper functions to reduce boilerplate.

### Models (Data Structures)

**Location**: `src-tauri/src/models/*.rs`

**Pattern**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    pub id: String,           // UUID as String
    pub name: String,
    pub color: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Board {
    pub fn new(name: String, color: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            color,
            created_at: now,
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_creation() {
        let board = Board::new("Test".to_string(), None);
        assert_eq!(board.name, "Test");
        assert!(!board.id.is_empty());
    }
}
```

**Rules**:
1. **Derive `Serialize, Deserialize`** for IPC.
2. **Always include timestamps**: `created_at`, `updated_at` as `DateTime<Utc>`.
3. **Use type-safe IDs**: Store UUIDs as `String` in models; use `uuid::Uuid::new_v4()` in constructors.
4. **Optional fields**: Use `Option<T>`. Serialize as `null` in JSON.
5. **Tests inline** at bottom of file in `#[cfg(test)]` block.

**Rust-beginner notes**:
- `#[derive(...)]` auto-implements traits (like Java annotations).
- `Option<T>` = "value or nothing" (no null references).
- `DateTime<Utc>` from `chrono` crate; serializes as RFC3339 ISO-8601 string.

### Error Handling

**Current approach**: Simple string errors (pragmatic, not type-safe).

```rust
Err("Invalid board ID".to_string())
```

**Rust-beginner note**: No custom `Result` types or error enums yet. Everything propagates as `String`. This avoids complexity but sacrifices type safety. If errors become hard to track, consider upgrading to `anyhow::anyhow!()` or a custom error type.

### Testing

**Unit tests** in model files (`src-tauri/src/models/*_tests.rs`):
```bash
cargo test
```

Tests cover:
- Object creation validity
- ID uniqueness
- Field initialization

**Rust-beginner note**: No mocking framework. Test behavior of actual structs only. Database integration tests would require fixtures; see `tests/` folder for patterns.

## Build and Test Commands

### Frontend (TypeScript/Svelte)

```bash
# Install dependencies
npm install

# Development (file watcher + hot reload)
npm run dev

# Build for production
npm run build

# Type check
npm run check
npm run check:watch

# Unit tests (Vitest)
npm test
npm run test:ui
npm run test:run

# E2E tests (Playwright)
npm run test:e2e
```

### Backend (Rust)

```bash
# Compile Rust (debug mode)
cargo build

# Full app in dev mode (compiles frontend + Rust, launches window)
npm run tauri dev

# Unit tests
cargo test

# Specific test
cargo test test_name -- --nocapture

# Check code without building (faster)
cargo check
```

### Combined (Recommended)

```bash
# Install everything
npm install

# Dev mode with hot reload on both sides
npm run tauri dev

# Run all tests (frontend + backend)
npm test && cargo test
```

## Code Style & Conventions

### Rust Style

- **Formatting**: `rustfmt` (automatic). Run with `cargo fmt`.
- **Linting**: `clippy`. Run with `cargo clippy`.
- **Naming**:
  - Functions: `snake_case` (`create_board`, `insert_card`)
  - Types: `PascalCase` (`Board`, `Card`)
  - Constants: `UPPER_SNAKE` (`MAX_TITLE_LENGTH`)
- **Strings**: Use `.to_string()` for owned strings, prefer `String` over `&str` in public APIs.

### TypeScript / Svelte

- **Strict mode**: `<script lang="ts">` required.
- **Style**: ESM imports, no CommonJS.
- **Naming**: `camelCase` for functions and variables, `PascalCase` for components.
- **Reactive**: Use `let` for component state. Svelte auto-subscribes to stores.

## Database Schema & Types

**Domains**:
- `boards` → contains columns
- `columns` (status: To Do, In Progress, Done)
- `cards` (tasks, belong to a column + board)
- `tags` (linked to cards via junction table)
- `reminders` (linked to cards)

**Type system**: TypeScript types in `src/lib/types.ts` match Rust models. Keep them synchronized.

## Debugging Tips

### When Rust Compilation Fails

1. Check compiler message for line + crate info.
2. Common issues for newcomers:
   - Missing `.to_string()` or `.clone()` (ownership)
   - Type mismatches (SQLite returns different types than expected)
   - Forgetting `?` to propagate errors
3. Run `cargo check` first (faster) before full `cargo build`.

### When IPC Calls Fail

1. **Frontend**: Check browser console for error from Tauri.
2. **Backend**: Use `println!()` macros in handlers to debug (visible in terminal).
3. Verify command names match exactly between `lib.rs` registration and frontend call.

### Database Inspection

```bash
sqlite3 ~/.config/katban-board/katban.db
```

Common queries:
```sql
.tables
SELECT * FROM boards;
SELECT * FROM cards WHERE board_id = '<id>';
.schema cards  -- View schema
```

## Monorepo Structure Notes

This is a **single-app monorepo** (not multi-package):
- Frontend and backend share semver version (`package.json` and `Cargo.toml`).
- Tauri CLI manages both; no separate build steps needed.
- Frontend types must match backend models—keep `src/lib/types.ts` in sync with `models/*.rs`.

## When to Ask for Help

- **Rust ownership issues**: Compiler error about moves/borrows? Look for `.clone()` or `&` fixes.
- **Type mismatches in DB**: SQLite returns `String`, but you need `i32`? Use `.parse::<i32>()`.
- **IPC serialization**: Frontend can't deserialize response? Check model derives and field names.
- **Command not found**: Forgot to register in `tauri::generate_handler![]`? Add it to `lib.rs`.

---

**Links**: See [README.md](../../README.md) for feature overview, prerequisites, and deployment steps.
