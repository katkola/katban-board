import { invoke } from '@tauri-apps/api/core';
import type { Board, Column, Card, Tag, Reminder } from '../types';

/**
 * Board API functions
 */

export async function createBoard(name: string, color?: string): Promise<Board> {
  return await invoke<Board>('create_board', { name, color });
}

export async function getBoards(): Promise<Board[]> {
  return await invoke<Board[]>('get_boards');
}

export async function getBoard(id: string): Promise<Board> {
  return await invoke<Board>('get_board', { id });
}

export async function updateBoard(id: string, name: string, color?: string): Promise<Board> {
  return await invoke<Board>('update_board', { id, name, color });
}

export async function deleteBoard(id: string): Promise<void> {
  return await invoke<void>('delete_board', { id });
}
