<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import { getCard, updateCard, moveCard, deleteCard } from '$lib/api/cards';
  import { getBoard } from '$lib/api/boards';
  import { getColumns } from '$lib/api/columns';
  import type { Card, Board, Column } from '$lib/types';

  let card: Card | null = null;
  let board: Board | null = null;
  let columns: Column[] = [];
  let loading = true;
  let error: string | null = null;
  let isEditing = false;
  let isSaving = false;

  let editTitle = '';
  let editDescription = '';
  let editDueDate = '';
  let editColumnId = '';

  async function loadData() {
    try {
      const cardId = $page.params.cardId;
      const boardId = $page.params.id;

      if (!cardId || !boardId) throw new Error('Missing card or board ID');

      card = await getCard(cardId);
      board = await getBoard(boardId);
      columns = await getColumns(boardId);

      if (card) {
        editTitle = card.title;
        editDescription = card.description || '';
        editDueDate = card.due_date || '';
        editColumnId = card.column_id;
      }
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to load card';
    } finally {
      loading = false;
    }
  }

  async function handleSave() {
    if (!card) return;

    isSaving = true;
    error = null;
    try {
      // Check if we need to move the card
      if (editColumnId !== card.column_id) {
        await moveCard(card.id, editColumnId, 0);
      }

      // Update card details
      await updateCard(card.id, editTitle, editDescription || undefined, editDueDate || undefined);

      // Reload card data
      await loadData();
      isEditing = false;
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to save card';
    } finally {
      isSaving = false;
    }
  }

  async function handleDelete() {
    if (!card || !confirm('Are you sure you want to delete this task?')) return;

    try {
      await deleteCard(card.id);
      goto(`/board/${$page.params.boardId}`);
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to delete card';
    }
  }

  function getCurrentColumnName() {
    return columns.find((col) => col.id === card?.column_id)?.name || 'Unknown';
  }

  function formatDateForInput(dateStr: string | null) {
    if (!dateStr) return '';
    const date = new Date(dateStr);
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    return `${year}-${month}-${day}`;
  }

  onMount(() => {
    loadData();
  });
</script>

<div class="card-detail">
  <div class="header">
    <a href="/board/{$page.params.id}" class="back-button">← Back to Board</a>
  </div>

  {#if error}
    <div class="error-message">{error}</div>
  {/if}

  {#if loading}
    <div class="loading">Loading task...</div>
  {:else if card}
    <div class="content">
      <div class="card-main">
        {#if isEditing}
          <div class="edit-form">
            <div class="form-group">
              <label for="title">Title</label>
              <input
                id="title"
                type="text"
                bind:value={editTitle}
                placeholder="Task title"
              />
            </div>

            <div class="form-group">
              <label for="description">Description</label>
              <textarea
                id="description"
                bind:value={editDescription}
                placeholder="Task details..."
                rows="6"
              ></textarea>
            </div>

            <div class="form-group">
              <label for="due-date">Due Date</label>
              <input
                id="due-date"
                type="date"
                bind:value={editDueDate}
              />
            </div>

            <div class="form-group">
              <label for="column">Status/Column</label>
              <select id="column" bind:value={editColumnId}>
                {#each columns as col}
                  <option value={col.id}>{col.name}</option>
                {/each}
              </select>
            </div>

            <div class="form-actions">
              <button class="save-btn" on:click={handleSave} disabled={isSaving}>
                {isSaving ? 'Saving...' : 'Save Changes'}
              </button>
              <button class="cancel-btn" on:click={() => (isEditing = false)} disabled={isSaving}>
                Cancel
              </button>
            </div>
          </div>
        {:else}
          <div class="view-form">
            <h1>{card.title}</h1>

            {#if card.description}
              <div class="description-section">
                <h3>Description</h3>
                <p>{card.description}</p>
              </div>
            {/if}

            <div class="details-grid">
              <div class="detail-item">
                <label>Status</label>
                <div class="value">{getCurrentColumnName()}</div>
              </div>

              {#if card.due_date}
                <div class="detail-item">
                  <label>Due Date</label>
                  <div class="value">
                    {new Date(card.due_date).toLocaleDateString('en-US', {
                      weekday: 'short',
                      month: 'short',
                      day: 'numeric',
                      year: 'numeric',
                    })}
                  </div>
                </div>
              {/if}

              <div class="detail-item">
                <label>Created</label>
                <div class="value">
                  {new Date(card.created_at).toLocaleDateString('en-US', {
                    month: 'short',
                    day: 'numeric',
                    year: 'numeric',
                  })}
                </div>
              </div>

              <div class="detail-item">
                <label>ID</label>
                <div class="value mono">{card.id}</div>
              </div>
            </div>

            <div class="actions">
              <button class="edit-btn" on:click={() => {
                editDueDate = formatDateForInput(card?.due_date || null);
                isEditing = true;
              }}>
                Edit Task
              </button>
              <button class="delete-btn" on:click={handleDelete}>
                Delete Task
              </button>
            </div>
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>

<style>
  .card-detail {
    min-height: 100vh;
    background: #f5f5f5;
  }

  .header {
    background: white;
    border-bottom: 1px solid #e5e7eb;
    padding: 1rem 2rem;
    sticky: 0;
    top: 0;
    z-index: 10;
  }

  .back-button {
    display: inline-block;
    color: #3b82f6;
    text-decoration: none;
    font-weight: 500;
    transition: color 0.2s;
  }

  .back-button:hover {
    color: #2563eb;
    text-decoration: underline;
  }

  .error-message {
    margin: 2rem;
    padding: 1rem;
    background: #fee2e2;
    color: #991b1b;
    border-radius: 4px;
  }

  .loading {
    padding: 3rem 2rem;
    text-align: center;
    color: #666;
    font-size: 1.1rem;
  }

  .content {
    max-width: 800px;
    margin: 2rem auto;
    padding: 0 2rem;
  }

  .card-main {
    background: white;
    border-radius: 8px;
    padding: 2rem;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }

  .view-form h1 {
    margin: 0 0 2rem 0;
    font-size: 2rem;
    color: #1a1a1a;
  }

  .description-section {
    margin-bottom: 2rem;
  }

  .description-section h3 {
    margin: 0 0 0.5rem 0;
    font-size: 1rem;
    color: #666;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .description-section p {
    margin: 0;
    color: #333;
    line-height: 1.6;
    white-space: pre-wrap;
  }

  .details-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 2rem;
    margin-bottom: 2rem;
    padding: 2rem 0;
    border-top: 1px solid #e5e7eb;
    border-bottom: 1px solid #e5e7eb;
  }

  .detail-item label {
    display: block;
    font-size: 0.875rem;
    color: #666;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: 0.5rem;
  }

  .detail-item .value {
    font-size: 1.125rem;
    color: #1a1a1a;
    font-weight: 500;
  }

  .detail-item .value.mono {
    font-family: 'Courier New', monospace;
    font-size: 0.875rem;
    word-break: break-all;
  }

  .actions {
    display: flex;
    gap: 1rem;
  }

  .edit-btn,
  .delete-btn,
  .save-btn,
  .cancel-btn {
    padding: 0.75rem 1.5rem;
    font-size: 1rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;
    font-weight: 500;
  }

  .edit-btn {
    background: #3b82f6;
    color: white;
  }

  .edit-btn:hover {
    background: #2563eb;
  }

  .delete-btn {
    background: #ef4444;
    color: white;
  }

  .delete-btn:hover {
    background: #dc2626;
  }

  .save-btn {
    background: #10b981;
    color: white;
  }

  .save-btn:hover:not(:disabled) {
    background: #059669;
  }

  .save-btn:disabled {
    background: #9ca3af;
  }

  .cancel-btn {
    background: #6b7280;
    color: white;
  }

  .cancel-btn:hover:not(:disabled) {
    background: #4b5563;
  }

  .cancel-btn:disabled {
    background: #9ca3af;
  }

  .edit-form {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
  }

  .form-group label {
    margin-bottom: 0.5rem;
    font-weight: 500;
    color: #1a1a1a;
  }

  .form-group input,
  .form-group textarea,
  .form-group select {
    padding: 0.75rem;
    border: 1px solid #e5e7eb;
    border-radius: 4px;
    font-size: 1rem;
    font-family: inherit;
    transition: border-color 0.2s;
  }

  .form-group input:focus,
  .form-group textarea:focus,
  .form-group select:focus {
    outline: none;
    border-color: #3b82f6;
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.1);
  }

  .form-actions {
    display: flex;
    gap: 1rem;
    margin-top: 1rem;
  }
</style>
