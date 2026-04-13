import { describe, it, expect, vi, beforeEach } from 'vitest';
import { invoke } from '@tauri-apps/api/core';

// Mock the Tauri invoke function
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

import type { Board } from '$lib/types';

describe('Board API', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('getBoards', () => {
    it('should fetch all boards', async () => {
      const mockBoards: Board[] = [
        {
          id: '1',
          name: 'Personal',
          color: '#FF6B6B',
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
        },
        {
          id: '2',
          name: 'Work',
          color: '#4ECDC4',
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
        },
      ];

      vi.mocked(invoke).mockResolvedValue(mockBoards);

      const { getBoards } = await import('$lib/api/boards');
      const boards = await getBoards();

      expect(invoke).toHaveBeenCalledWith('get_boards');
      expect(boards).toEqual(mockBoards);
      expect(boards).toHaveLength(2);
    });

    it('should handle errors when fetching boards', async () => {
      const error = new Error('Database error');
      vi.mocked(invoke).mockRejectedValue(error);

      const { getBoards } = await import('$lib/api/boards');

      await expect(getBoards()).rejects.toThrow('Database error');
    });
  });

  describe('createBoard', () => {
    it('should create a new board', async () => {
      const mockBoard: Board = {
        id: '3',
        name: 'New Board',
        color: '#95E1D3',
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
      };

      vi.mocked(invoke).mockResolvedValue(mockBoard);

      const { createBoard } = await import('$lib/api/boards');
      const board = await createBoard('New Board');

      expect(invoke).toHaveBeenCalledWith('create_board', { name: 'New Board' });
      expect(board).toEqual(mockBoard);
      expect(board.name).toBe('New Board');
    });

    it('should reject empty board names', async () => {
      const { createBoard } = await import('$lib/api/boards');

      await expect(createBoard('')).rejects.toThrow();
    });
  });

  describe('deleteBoard', () => {
    it('should delete a board by ID', async () => {
      vi.mocked(invoke).mockResolvedValue(null);

      const { deleteBoard } = await import('$lib/api/boards');
      await deleteBoard('1');

      expect(invoke).toHaveBeenCalledWith('delete_board', { id: '1' });
    });
  });

  describe('updateBoard', () => {
    it('should update board properties', async () => {
      const updatedBoard: Board = {
        id: '1',
        name: 'Updated Name',
        color: '#FF6B6B',
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
      };

      vi.mocked(invoke).mockResolvedValue(updatedBoard);

      const { updateBoard } = await import('$lib/api/boards');
      const board = await updateBoard('1', { name: 'Updated Name' });

      expect(invoke).toHaveBeenCalledWith('update_board', {
        id: '1',
        name: 'Updated Name',
      });
      expect(board.name).toBe('Updated Name');
    });
  });
});
