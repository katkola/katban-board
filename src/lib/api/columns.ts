import { invoke } from '@tauri-apps/api/core';
import type { Column } from '../types';

/**
 * Column API functions
 */

export async function getColumns(boardId: string): Promise<Column[]> {
  return await invoke<Column[]>('get_columns', { boardId });
}

export async function reorderColumns(boardId: string, columnIds: string[]): Promise<void> {
  return await invoke<void>('reorder_columns', { boardId, columnIds });
}
