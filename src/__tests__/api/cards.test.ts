import { describe, it, expect, vi, beforeEach } from 'vitest';
import { invoke } from '@tauri-apps/api/core';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

import type { Card } from '$lib/types';

describe('Card API', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('getCards', () => {
    it('should fetch cards for a board', async () => {
      const mockCards: Card[] = [
        {
          id: '1',
          title: 'Task 1',
          description: 'Do something',
          column_id: 'col1',
          board_id: 'board1',
          position: 0,
          due_date: null,
          is_completed: false,
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
        },
        {
          id: '2',
          title: 'Task 2',
          description: 'Do something else',
          column_id: 'col1',
          board_id: 'board1',
          position: 1,
          due_date: new Date().toISOString(),
          is_completed: false,
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
        },
      ];

      vi.mocked(invoke).mockResolvedValue(mockCards);

      const { getCards } = await import('$lib/api/cards');
      const cards = await getCards(undefined, 'board1');

      expect(invoke).toHaveBeenCalledWith('get_cards', { column_id: undefined, board_id: 'board1' });
      expect(cards).toEqual(mockCards);
      expect(cards).toHaveLength(2);
    });

    it('should filter cards by column', async () => {
      const mockCards: Card[] = [
        {
          id: '1',
          title: 'Task 1',
          description: 'Do something',
          column_id: 'col1',
          board_id: 'board1',
          position: 0,
          due_date: null,
          is_completed: false,
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
        },
      ];

      vi.mocked(invoke).mockResolvedValue(mockCards);

      const { getCards } = await import('$lib/api/cards');
      const cards = await getCards('col1', 'board1');

      expect(invoke).toHaveBeenCalledWith('get_cards', { column_id: 'col1', board_id: 'board1' });
      expect(cards).toHaveLength(1);
    });
  });

  describe('createCard', () => {
    it('should create a new card', async () => {
      const mockCard: Card = {
        id: '3',
        title: 'New Task',
        description: '',
        column_id: 'col1',
        board_id: 'board1',
        position: 2,
        due_date: null,
        is_completed: false,
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
      };

      vi.mocked(invoke).mockResolvedValue(mockCard);

      const { createCard } = await import('$lib/api/cards');
      const card = await createCard('col1', 'New Task');

      expect(invoke).toHaveBeenCalledWith('create_card', {
        column_id: 'col1',
        title: 'New Task',
      });
      expect(card.title).toBe('New Task');
    });

    it('should reject empty card titles', async () => {
      const { createCard } = await import('$lib/api/cards');

      await expect(createCard('col1', '')).rejects.toThrow();
    });
  });

  describe('deleteCard', () => {
    it('should delete a card by ID', async () => {
      vi.mocked(invoke).mockResolvedValue(null);

      const { deleteCard } = await import('$lib/api/cards');
      await deleteCard('1');

      expect(invoke).toHaveBeenCalledWith('delete_card', { id: '1' });
    });
  });

  describe('updateCard', () => {
    it('should update card properties', async () => {
      const updatedCard: Card = {
        id: '1',
        title: 'Updated Task',
        description: 'Updated description',
        column_id: 'col1',
        board_id: 'board1',
        position: 0,
        due_date: null,
        is_completed: true,
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
      };

      vi.mocked(invoke).mockResolvedValue(updatedCard);

      const { updateCard } = await import('$lib/api/cards');
      const card = await updateCard('1', { title: 'Updated Task', is_completed: true });

      expect(invoke).toHaveBeenCalledWith('update_card', {
        id: '1',
        title: 'Updated Task',
        is_completed: true,
      });
      expect(card.is_completed).toBe(true);
    });
  });
});
