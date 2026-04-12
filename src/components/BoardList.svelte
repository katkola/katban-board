<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { getBoards, createBoard } from '$lib/api/boards';
  import type { Board } from '$lib/types';

  let boards: Board[] = [];
  let newBoardName = '';
  let loading = false;
  let error: string | null = null;

  function handleBoardClick(boardId: string) {
    goto(`/board/${boardId}`);
  }

  async function loadBoards() {
    loading = true;
    error = null;
    try {
      boards = await getBoards();
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to load boards';
    } finally {
      loading = false;
    }
  }

  async function handleCreateBoard() {
    if (!newBoardName.trim()) return;
    
    try {
      await createBoard(newBoardName.trim());
      newBoardName = '';
      await loadBoards();
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to create board';
    }
  }

  onMount(() => {
    loadBoards();
  });
</script>

<div class="container">
  <h1>Katban Board</h1>
  <p class="subtitle">Personalized Kanban for Life Domains</p>

  {#if error}
    <div class="error">{error}</div>
  {/if}

  <div class="create-board">
    <input
      type="text"
      bind:value={newBoardName}
      placeholder="Enter board name..."
      on:keydown={(e) => e.key === 'Enter' && handleCreateBoard()}
    />
    <button on:click={handleCreateBoard} disabled={loading}>
      Create Board
    </button>
  </div>

  {#if loading}
    <div class="loading">Loading boards...</div>
  {:else}
    <div class="boards-list">
      {#if boards.length === 0}
        <p class="empty-state">No boards yet. Create your first board!</p>
      {:else}
        <h2>Your Boards ({boards.length})</h2>
        <div class="boards-grid">
          {#each boards as board}
            <div
              class="board-card"
              style="border-left-color: {board.color || '#6B7280'}"
              on:click={() => handleBoardClick(board.id)}
              on:keydown={(e) => (e.key === 'Enter' || e.key === ' ') && handleBoardClick(board.id)}
              role="button"
              tabindex="0"
            >
              <h3>{board.name}</h3>
              <p class="board-id">ID: {board.id}</p>
              <p class="board-date">
                Created: {new Date(board.created_at).toLocaleDateString()}
              </p>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
  }

  h1 {
    font-size: 2.5rem;
    color: #1a1a1a;
    margin-bottom: 0.5rem;
  }

  .subtitle {
    color: #666;
    font-size: 1.1rem;
    margin-bottom: 2rem;
  }

  .error {
    background: #fee;
    color: #c00;
    padding: 1rem;
    border-radius: 4px;
    margin-bottom: 1rem;
  }

  .create-board {
    display: flex;
    gap: 1rem;
    margin-bottom: 2rem;
  }

  input[type="text"] {
    flex: 1;
    padding: 0.75rem;
    font-size: 1rem;
    border: 2px solid #ddd;
    border-radius: 4px;
    transition: border-color 0.2s;
  }

  input[type="text"]:focus {
    outline: none;
    border-color: #3B82F6;
  }

  button {
    padding: 0.75rem 1.5rem;
    font-size: 1rem;
    background: #3B82F6;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: background 0.2s;
  }

  button:hover:not(:disabled) {
    background: #2563EB;
  }

  button:disabled {
    background: #9CA3AF;
    cursor: not-allowed;
  }

  .loading {
    text-align: center;
    color: #666;
    padding: 2rem;
  }

  .empty-state {
    text-align: center;
    color: #999;
    padding: 3rem;
    font-size: 1.1rem;
  }

  .boards-list h2 {
    font-size: 1.5rem;
    margin-bottom: 1rem;
    color: #333;
  }

  .boards-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1.5rem;
  }

  .board-card {
    background: white;
    border: 1px solid #e5e7eb;
    border-left: 4px solid #6B7280;
    border-radius: 8px;
    padding: 1.5rem;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    transition: transform 0.2s, box-shadow 0.2s;
    cursor: pointer;
  }

  .board-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  }

  .board-card:focus {
    outline: 2px solid #3b82f6;
    outline-offset: 2px;
  }

  .board-card h3 {
    margin: 0 0 0.5rem 0;
    font-size: 1.25rem;
    color: #1a1a1a;
  }

  .board-id,
  .board-date {
    color: #666;
    font-size: 0.875rem;
    margin: 0.25rem 0;
  }
</style>
