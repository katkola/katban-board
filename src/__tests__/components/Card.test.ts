import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import Card from '$components/Card.svelte';
import type { Card as CardType } from '$lib/types';

describe('Card Component', () => {
  let mockCard: CardType;

  beforeEach(() => {
    mockCard = {
      id: '1',
      title: 'Test Card',
      description: 'Test Description',
      column_id: 'col1',
      board_id: 'board1',
      position: 0,
      due_date: null,
      is_completed: false,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    };
  });

  it('should render card with title and description', () => {
    render(Card, { props: { card: mockCard } });

    expect(screen.getByText('Test Card')).toBeInTheDocument();
    expect(screen.getByText('Test Description')).toBeInTheDocument();
  });

  it('should not render description if not provided', () => {
    const cardWithoutDesc = { ...mockCard, description: '' };
    render(Card, { props: { card: cardWithoutDesc } });

    expect(screen.getByText('Test Card')).toBeInTheDocument();
    expect(screen.queryByText('Test Description')).not.toBeInTheDocument();
  });

  it('should display due date when provided', () => {
    const tomorrow = new Date();
    tomorrow.setDate(tomorrow.getDate() + 1);
    const cardWithDueDate = { ...mockCard, due_date: tomorrow.toISOString() };

    render(Card, { props: { card: cardWithDueDate } });

    const dateElements = screen.getAllByText(/\d{1,2}/);
    expect(dateElements.length).toBeGreaterThan(0);
  });

  it('should apply overdue class when due date is in the past', () => {
    const yesterday = new Date();
    yesterday.setDate(yesterday.getDate() - 1);
    const overdueCard = { ...mockCard, due_date: yesterday.toISOString() };

    const { container } = render(Card, { props: { card: overdueCard } });
    const cardElement = container.querySelector('.card-meta');

    expect(cardElement).toHaveClass('overdue');
  });

  it('should apply due-soon class for cards due in 3 days or less', () => {
    const soonDue = new Date();
    soonDue.setDate(soonDue.getDate() + 2);
    const soonCard = { ...mockCard, due_date: soonDue.toISOString() };

    const { container } = render(Card, { props: { card: soonCard } });
    const cardElement = container.querySelector('.card-meta');

    expect(cardElement).toHaveClass('due-soon');
  });

  it('should be keyboard accessible', async () => {
    const user = userEvent.setup();
    const { container } = render(Card, { props: { card: mockCard } });
    const cardElement = container.querySelector('[role="button"]');

    expect(cardElement).toHaveAttribute('tabindex', '0');
  });
});
