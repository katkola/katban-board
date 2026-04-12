// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod models;
mod db;
mod api;

use api::boards::{create_board, get_boards, get_board, update_board, delete_board};
use api::columns::{get_columns, reorder_columns};
use api::cards::{create_card, get_cards, get_card, update_card, delete_card, move_card};
use api::tags::{get_tags, create_tag, delete_tag, add_tag_to_card, remove_tag_from_card, filter_cards_by_tag};
use api::reminders::{get_due_cards, schedule_reminder, dismiss_reminder};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            // Board commands
            create_board,
            get_boards,
            get_board,
            update_board,
            delete_board,
            // Column commands
            get_columns,
            reorder_columns,
            // Card commands
            create_card,
            get_cards,
            get_card,
            update_card,
            delete_card,
            move_card,
            // Tag commands
            get_tags,
            create_tag,
            delete_tag,
            add_tag_to_card,
            remove_tag_from_card,
            filter_cards_by_tag,
            // Reminder commands
            get_due_cards,
            schedule_reminder,
            dismiss_reminder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
