import { invoke } from '@tauri-apps/api/core';
import type { Board, Column, Task, Tag, Reminder } from '../types';

/**
 * Board API functions
 */

export async function createBoard(name: string, color?: string): Promise<Board> {
  if (!name || name.trim().length === 0) {
    throw new Error('Board name cannot be empty');
  }
  return await invoke<Board>('create_board', { name, color });
}

export async function getBoards(): Promise<Board[]> {
  return await invoke<Board[]>('get_boards');
}

export async function getBoard(id: string): Promise<Board> {
  return await invoke<Board>('get_board', { id });
}

export async function updateBoard(id: string, updates: { name?: string; color?: string }): Promise<Board> {
  if (updates.name !== undefined && updates.name.trim().length === 0) {
    throw new Error('Board name cannot be empty');
  }
  return await invoke<Board>('update_board', { id, ...updates });
}

export async function deleteBoard(id: string): Promise<void> {
  return await invoke<void>('delete_board', { id });
}
