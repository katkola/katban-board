import { invoke } from '@tauri-apps/api/core';
import type { Card } from '../types';

/**
 * Card API functions
 */

export async function createCard(
  columnId: string,
  title: string,
  description?: string,
  dueDate?: string
): Promise<Card> {
  return await invoke<Card>('create_card', { columnId, title, description, dueDate });
}

export async function getCards(columnId?: string, boardId?: string): Promise<Card[]> {
  return await invoke<Card[]>('get_cards', { columnId, boardId });
}

export async function getCard(id: string): Promise<Card> {
  return await invoke<Card>('get_card', { id });
}

export async function updateCard(
  id: string,
  title?: string,
  description?: string,
  dueDate?: string
): Promise<Card> {
  return await invoke<Card>('update_card', { id, title, description, dueDate });
}

export async function deleteCard(id: string): Promise<void> {
  return await invoke<void>('delete_card', { id });
}

export async function moveCard(id: string, columnId: string, position: number): Promise<Card> {
  return await invoke<Card>('move_card', { id, columnId, position });
}
