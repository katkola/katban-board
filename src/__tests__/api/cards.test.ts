import { describe, it, expect, vi, beforeEach } from 'vitest';

// Mock invoke before importing the API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

import { invoke } from '@tauri-apps/api/core';
import { getCards, createCard, deleteCard, updateCard } from '$lib/api/cards';
import { Status, type Task } from '$lib/types';

describe('Card API', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('getCards', () => {
    it('should fetch cards for a board', async () => {
      const mockCards: Task[] = [
        {
          id: '1',
          title: 'Task 1',
          description: 'Do something',
          column_id: 'col1',
          board_id: 'board1',
          position: 0,
          status: Status.ToDo,
          due_date: null,
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
          status: Status.InProgress,
          due_date: new Date().toISOString(),
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
        },
      ];

      vi.mocked(invoke).mockResolvedValue(mockCards);

      const cards = await getCards(undefined, 'board1');

      expect(invoke).toHaveBeenCalledWith('get_cards', { columnId: undefined, boardId: 'board1' });
      expect(cards).toEqual(mockCards);
      expect(cards).toHaveLength(2);
    });

    it('should filter cards by column', async () => {
      const mockCards: Task[] = [
        {
          id: '1',
          title: 'Task 1',
          description: 'Do something',
          column_id: 'col1',
          board_id: 'board1',
          position: 0,
          status: Status.ToDo,
          due_date: null,
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
        },
      ];

      vi.mocked(invoke).mockResolvedValue(mockCards);

      const cards = await getCards('col1', 'board1');

      expect(invoke).toHaveBeenCalledWith('get_cards', { columnId: 'col1', boardId: 'board1' });
      expect(cards).toHaveLength(1);
    });
  });

  describe('createCard', () => {
    it('should create a new card', async () => {
      const mockCard: Task = {
        id: '3',
        title: 'New Task',
        description: null,
        column_id: 'col1',
        board_id: 'board1',
        position: 2,
        status: Status.ToDo,
        due_date: null,
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
      };

      vi.mocked(invoke).mockResolvedValue(mockCard);

      const card = await createCard('col1', 'New Task');

      expect(invoke).toHaveBeenCalledWith('create_card', {
        columnId: 'col1',
        title: 'New Task',
        description: undefined,
        dueDate: undefined,
      });
      expect(card.title).toBe('New Task');
    });

    it('should reject empty card titles', async () => {
      await expect(createCard('col1', '')).rejects.toThrow('Card title cannot be empty');
      expect(invoke).not.toHaveBeenCalled();
    });

    it('should reject whitespace-only titles', async () => {
      await expect(createCard('col1', '   ')).rejects.toThrow('Card title cannot be empty');
      expect(invoke).not.toHaveBeenCalled();
    });
  });

  describe('deleteCard', () => {
    it('should delete a card by ID', async () => {
      vi.mocked(invoke).mockResolvedValue(undefined);

      await deleteCard('1');

      expect(invoke).toHaveBeenCalledWith('delete_card', { id: '1' });
    });
  });

  describe('updateCard', () => {
    it('should update card title', async () => {
      const updatedCard: Task = {
        id: '1',
        title: 'Updated Task',
        description: 'Updated description',
        column_id: 'col1',
        board_id: 'board1',
        position: 0,
        status: Status.ToDo,
        due_date: null,
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
      };

      vi.mocked(invoke).mockResolvedValue(updatedCard);

      const card = await updateCard('1', { title: 'Updated Task' });

      expect(invoke).toHaveBeenCalledWith('update_card', {
        id: '1',
        title: 'Updated Task',
      });
      expect(card.title).toBe('Updated Task');
    });

    it('should update card description', async () => {
      const updatedCard: Task = {
        id: '1',
        title: 'Task',
        description: 'Updated description',
        column_id: 'col1',
        board_id: 'board1',
        position: 0,
        status: Status.Done,
        due_date: null,
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
      };

      vi.mocked(invoke).mockResolvedValue(updatedCard);

      const card = await updateCard('1', { description: 'Updated description' });

      expect(invoke).toHaveBeenCalledWith('update_card', {
        id: '1',
        description: 'Updated description',
      });
      expect(card.description).toBe('Updated description');
    });

    it('should reject empty card titles', async () => {
      await expect(updateCard('1', { title: '' })).rejects.toThrow('Card title cannot be empty');
      expect(invoke).not.toHaveBeenCalled();
    });
  });
});
