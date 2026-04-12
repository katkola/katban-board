<script lang="ts">
  import type { Card as CardType } from '$lib/types';

  export let card: CardType;

  const daysUntilDue = card.due_date
    ? Math.ceil((new Date(card.due_date).getTime() - Date.now()) / (1000 * 60 * 60 * 24))
    : null;

  const getDueDateClass = () => {
    if (!daysUntilDue) return '';
    if (daysUntilDue < 0) return 'overdue';
    if (daysUntilDue <= 3) return 'due-soon';
    return 'normal';
  };

  const formatDueDate = (date: string | null) => {
    if (!date) return null;
    return new Date(date).toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
  };
</script>

<div class="card">
  <h4 class="card-title">{card.title}</h4>

  {#if card.description}
    <p class="card-description">{card.description}</p>
  {/if}

  {#if card.due_date}
    <div class="card-meta {getDueDateClass()}">
      📅 {formatDueDate(card.due_date)}
      {#if daysUntilDue !== null}
        <span class="days-label">
          {daysUntilDue === 0
            ? 'Today'
            : daysUntilDue === 1
              ? 'Tomorrow'
              : daysUntilDue < 0
                ? `${Math.abs(daysUntilDue)} days overdue`
                : `in ${daysUntilDue} days`}
        </span>
      {/if}
    </div>
  {/if}
</div>

<style>
  .card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 6px;
    padding: 0.75rem;
    cursor: pointer;
    transition: all 0.2s;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
  }

  .card:hover {
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    transform: translateY(-2px);
  }

  .card-title {
    margin: 0 0 0.5rem 0;
    font-size: 0.95rem;
    font-weight: 600;
    color: #1a1a1a;
  }

  .card-description {
    margin: 0 0 0.75rem 0;
    font-size: 0.875rem;
    color: #666;
    line-height: 1.4;
  }

  .card-meta {
    font-size: 0.8rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem;
    border-radius: 4px;
    background: #f0f9ff;
  }

  .card-meta.normal {
    color: #0369a1;
    background: #f0f9ff;
  }

  .card-meta.due-soon {
    color: #b45309;
    background: #fffbeb;
  }

  .card-meta.overdue {
    color: #dc2626;
    background: #fee2e2;
  }

  .days-label {
    font-size: 0.75rem;
    font-weight: 500;
  }
</style>
