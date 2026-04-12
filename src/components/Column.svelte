<script lang="ts">
  import Card from './Card.svelte';
  import type { Column as ColumnType, Card as CardType } from '$lib/types';

  export let column: ColumnType;
  export let cards: CardType[] = [];

  const columnCards = cards.filter((card) => card.column_id === column.id);
</script>

<div class="column">
  <div class="column-header">
    <h3>{column.name}</h3>
    <span class="card-count">{columnCards.length}</span>
  </div>

  <div class="column-content">
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
  }

  .empty {
    color: #999;
    text-align: center;
    padding: 2rem 1rem;
  }

  .cards-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
</style>
