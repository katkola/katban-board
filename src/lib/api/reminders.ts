import { invoke } from '@tauri-apps/api/core';
import type { Task, Reminder } from '../types';

/**
 * Reminder API functions
 */

export async function getDueCards(
  boardId?: string,
  from?: string,
  to?: string
): Promise<Task[]> {
  return await invoke<Task[]>('get_due_cards', { boardId, from, to });
}

export async function scheduleReminder(cardId: string, scheduledAt: string): Promise<Reminder> {
  return await invoke<Reminder>('schedule_reminder', { cardId, scheduledAt });
}

export async function dismissReminder(id: string): Promise<void> {
  return await invoke<void>('dismiss_reminder', { id });
}
