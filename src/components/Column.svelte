<script lang="ts">
  import Card from './Card.svelte';
  import { createCard } from '$lib/api/cards';
  import type { Column as ColumnType, Task as CardType } from '$lib/types';

  export let column: ColumnType;
  export let cards: CardType[] = [];
  export let onCardAdded: (() => void) | null = null;

  let newCardTitle = '';
  let isAdding = false;
  let error: string | null = null;

  const columnCards = cards.filter((card) => card.column_id === column.id);

  async function handleAddCard() {
    if (!newCardTitle.trim()) return;

    isAdding = true;
    error = null;
    try {
      await createCard(column.id, newCardTitle.trim());
      newCardTitle = '';
      if (onCardAdded) {
        onCardAdded();
      }
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to create card';
    } finally {
      isAdding = false;
    }
  }
</script>

<div class="column">
  <div class="column-header">
    <h3>{column.name}</h3>
    <span class="card-count">{columnCards.length}</span>
  </div>

  <div class="column-content">
    {#if error}
      <div class="error-message">{error}</div>
    {/if}

    <div class="add-card-form">
      <input
        type="text"
        bind:value={newCardTitle}
        on:keydown={(e) => e.key === 'Enter' && handleAddCard()}
        placeholder="Add a new task..."
        disabled={isAdding}
      />
      <button on:click={handleAddCard} disabled={isAdding || !newCardTitle.trim()}>
        {isAdding ? 'Adding...' : '+'}
      </button>
    </div>

    {#if columnCards.length === 0}
      <p class="empty">No cards yet</p>
    {:else}
      <div class="cards-list">
        {#each columnCards as card (card.id)}
          <Card {card} />
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  .column {
    background: #f9fafb;
    border-radius: 8px;
    display: flex;
    flex-direction: column;
    min-height: 400px;
    overflow: hidden;
  }

  .column-header {
    background: white;
    border-bottom: 2px solid #e5e7eb;
    padding: 1rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .column-header h3 {
    margin: 0;
    font-size: 1.1rem;
    color: #1a1a1a;
  }

  .card-count {
    background: #3b82f6;
    color: white;
    border-radius: 12px;
    padding: 0.25rem 0.75rem;
    font-size: 0.875rem;
    font-weight: 500;
  }

  .column-content {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .error-message {
    background: #fee2e2;
    color: #991b1b;
    padding: 0.75rem;
    border-radius: 4px;
    font-size: 0.875rem;
  }

  .add-card-form {
    display: flex;
    gap: 0.5rem;
  }

  .add-card-form input {
    flex: 1;
    padding: 0.5rem;
    border: 1px solid #e5e7eb;
    border-radius: 4px;
    font-size: 0.875rem;
    transition: border-color 0.2s;
  }

  .add-card-form input:focus {
    outline: none;
    border-color: #3b82f6;
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.1);
  }

  .add-card-form input:disabled {
    background: #f3f4f6;
    cursor: not-allowed;
  }

  .add-card-form button {
    padding: 0.5rem 0.75rem;
    background: #3b82f6;
    color: white;
    border: none;
    border-radius: 4px;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.2s;
  }

  .add-card-form button:hover:not(:disabled) {
    background: #2563eb;
  }

  .add-card-form button:disabled {
    background: #9ca3af;
    cursor: not-allowed;
  }

  .empty {
    color: #999;
    text-align: center;
    padding: 2rem 1rem;
    margin: 0;
  }

  .cards-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
</style>
