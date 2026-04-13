import { invoke } from '@tauri-apps/api/core';
import type { Task } from '../types';

/**
 * Card API functions
 */

export async function createCard(
  columnId: string,
  title: string,
  description?: string,
  dueDate?: string
): Promise<Task> {
  if (!title || title.trim().length === 0) {
    throw new Error('Card title cannot be empty');
  }
  return await invoke<Task>('create_card', { columnId, title, description, dueDate });
}

export async function getCards(columnId?: string, boardId?: string): Promise<Task[]> {
  return await invoke<Task[]>('get_cards', { columnId, boardId });
}

export async function getCard(id: string): Promise<Task> {
  return await invoke<Task>('get_card', { id });
}

export async function updateCard(
  id: string,
  updates?: { title?: string; description?: string; due_date?: string }
): Promise<Task> {
  if (updates?.title !== undefined && updates.title.trim().length === 0) {
    throw new Error('Card title cannot be empty');
  }
  return await invoke<Task>('update_card', { id, ...updates });
}

export async function deleteCard(id: string): Promise<void> {
  return await invoke<void>('delete_card', { id });
}

export async function moveCard(id: string, columnId: string, position: number): Promise<Task> {
  return await invoke<Task>('move_card', { id, columnId, position });
}
