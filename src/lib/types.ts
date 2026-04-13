/**
 * TypeScript types matching Rust structs
 * These mirror the API-first design from the Rust backend
 */

/** Unique identifier type */
export type UUID = string;

/** Represents a Kanban board (domain) */
export interface Board {
  id: UUID;
  name: string;
  color: string | null;
  created_at: string; // ISO 8601 datetime
  updated_at: string; // ISO 8601 datetime
}

/** Represents a column in a Kanban board */
export interface Column {
  id: UUID;
  board_id: UUID;
  name: string;
  position: number;
  created_at: string; // ISO 8601 datetime
}

export enum Status {
  ToDo = 'To Do',
  InProgress = 'In Progress',
  Done = 'Done',
}

/** Represents a task card in a column */
export interface Task {
  id: UUID;
  column_id: UUID;
  board_id: UUID;
  title: string;
  status: Status;
  description: string | null;
  due_date: string | null; // ISO 8601 datetime or null
  position: number;
  created_at: string; // ISO 8601 datetime
  updated_at: string; // ISO 8601 datetime
}

/** Represents a tag/label for organizing cards */
export interface Tag {
  id: UUID;
  board_id: UUID;
  name: string;
  color: string | null;
}

/** Represents a reminder for a card's due date */
export interface Reminder {
  id: UUID;
  card_id: UUID;
  scheduled_at: string; // ISO 8601 datetime
  notified: boolean;
}

/** API Response types */
export interface ApiResponse<T> {
  data?: T;
  error?: string;
}

/** Filter options for cards */
export interface CardFilter {
  board_id?: UUID;
  column_id?: UUID;
  tag_ids?: UUID[];
  due_date_from?: string;
  due_date_to?: string;
}

/** Default life domains */
export const DEFAULT_DOMAINS = [
  { name: 'Health & Fitness', color: '#10B981' },
  { name: 'Learning & Education', color: '#3B82F6' },
  { name: 'Relationships', color: '#EC4899' },
  { name: 'Hobbies & Creative Projects', color: '#F59E0B' },
  { name: 'Household & Chores', color: '#6B7280' },
];

/** Default column names */
export const DEFAULT_COLUMNS = ['To Do', 'In Progress', 'Done'];
