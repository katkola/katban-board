<script lang="ts">
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import Column from '../../../components/Column.svelte';
  import { getBoard } from '$lib/api/boards';
  import { getColumns } from '$lib/api/columns';
  import { getCards } from '$lib/api/cards';
  import type { Board, Column as ColumnType, Task as CardType } from '$lib/types';

  let board: Board | null = null;
  let columns: ColumnType[] = [];
  let cards: CardType[] = [];
  let loading = true;
  let error: string | null = null;

  async function loadBoardData() {
    try {
      const boardId = $page.params.id;
      if (!boardId) throw new Error('No board ID provided');

      board = await getBoard(boardId);
      columns = await getColumns(boardId);
      cards = await getCards(undefined, boardId);
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to load board';
    } finally {
      loading = false;
    }
  }

  function handleCardAdded() {
    // Reload cards to show the newly added card
    loadBoardData();
  }

  onMount(async () => {
    await loadBoardData();
  });

</script>

<div class="board-view">
  <div class="header">
    <a href="/" class="back-button">← Back to Boards</a>
    {#if board}
      <div class="board-header-info">
        <div
          class="board-indicator"
          style="background-color: {board.color || '#6B7280'}"
        ></div>
        <h1>{board.name}</h1>
      </div>
    {/if}
  </div>

  {#if error}
    <div class="error-message">{error}</div>
  {/if}

  {#if loading}
    <div class="loading">Loading board...</div>
  {:else if board && columns.length > 0}
    <div class="columns-container">
      {#each columns as column (column.id)}
        <Column {column} {cards} onCardAdded={handleCardAdded} />
      {/each}
    </div>
  {:else if board}
    <div class="empty-state">No columns found for this board</div>
  {/if}
</div>

<style>
  .board-view {
    min-height: 100vh;
    background: #f5f5f5;
  }

  .header {
    background: white;
    border-bottom: 1px solid #e5e7eb;
    padding: 1rem 2rem;
    position: sticky;
    top: 0;
    z-index: 10;
  }

  .back-button {
    display: inline-block;
    margin-bottom: 1rem;
    color: #3b82f6;
    text-decoration: none;
    font-weight: 500;
    transition: color 0.2s;
  }

  .back-button:hover {
    color: #2563eb;
    text-decoration: underline;
  }

  .board-header-info {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .board-indicator {
    width: 12px;
    height: 40px;
    border-radius: 4px;
  }

  .board-header-info h1 {
    margin: 0;
    font-size: 2rem;
    color: #1a1a1a;
  }

  .loading,
  .empty-state {
    padding: 3rem 2rem;
    text-align: center;
    color: #666;
    font-size: 1.1rem;
  }

  .error-message {
    margin: 2rem;
    padding: 1rem;
    background: #fee;
    color: #c00;
    border-radius: 4px;
  }

  .columns-container {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
    gap: 1.5rem;
    padding: 2rem;
    max-width: 1400px;
    margin: 0 auto;
  }
</style>
