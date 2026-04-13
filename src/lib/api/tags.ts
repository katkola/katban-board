import { invoke } from '@tauri-apps/api/core';
import type { Tag, Task } from '../types';

/**
 * Tag API functions
 */

export async function getTags(boardId: string): Promise<Tag[]> {
  return await invoke<Tag[]>('get_tags', { boardId });
}

export async function createTag(boardId: string, name: string, color?: string): Promise<Tag> {
  return await invoke<Tag>('create_tag', { boardId, name, color });
}

export async function deleteTag(id: string): Promise<void> {
  return await invoke<void>('delete_tag', { id });
}

export async function addTagToCard(cardId: string, tagId: string): Promise<void> {
  return await invoke<void>('add_tag_to_card', { cardId, tagId });
}

export async function removeTagFromCard(cardId: string, tagId: string): Promise<void> {
  return await invoke<void>('remove_tag_from_card', { cardId, tagId });
}

export async function filterCardsByTag(boardId: string, tagIds: string[]): Promise<Task[]> {
  return await invoke<Task[]>('filter_cards_by_tag', { boardId, tagIds });
}
