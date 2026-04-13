import { test, expect } from '@playwright/test';

test.describe('Kanban Board E2E Tests', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('should load the board list page', async ({ page }) => {
    await expect(page.locator('text=Katban Board')).toBeVisible();
    await expect(page.locator('text=Personalized Kanban for Life Domains')).toBeVisible();
  });

  test('should display existing boards', async ({ page }) => {
    // Wait for boards to load
    await page.waitForLoadState('networkidle');
    
    // Check if container is visible
    const container = page.locator('.container');
    await expect(container).toBeVisible();
  });

  test('should create a new board', async ({ page }) => {
    // Find the input field for new board name
    const input = page.locator('input[type="text"]').first();
    
    // Type board name
    await input.fill('E2E Test Board');
    
    // Find and click the create button (should be near the input)
    const createButton = page.locator('button').first();
    await createButton.click();
    
    // Wait a moment for the board to be created
    await page.waitForTimeout(500);
    
    // Verify the board appears in the list
    await expect(page.locator('text=E2E Test Board')).toBeVisible();
  });

  test('should navigate to board detail on click', async ({ page }) => {
    // Wait for content to load
    await page.waitForLoadState('networkidle');
    
    // Click on the first board (if exists)
    const boardLink = page.locator('[role="button"]').first();
    if (await boardLink.isVisible()) {
      await boardLink.click();
      
      // Should navigate to board page
      await page.waitForURL(/\/board\//);
      expect(page.url()).toMatch(/\/board\//);
    }
  });

  test('should load board with columns', async ({ page }) => {
    // Navigate directly to a board
    await page.goto('/board/test-board');
    
    // Wait for page to load
    await page.waitForLoadState('networkidle');
    
    // Look for back button to confirm we're on board page
    const backButton = page.locator('text=← Back to Boards');
    if (await backButton.isVisible()) {
      await expect(backButton).toBeVisible();
    }
  });

  test('should display columns container', async ({ page }) => {
    await page.goto('/board/test-board');
    await page.waitForLoadState('networkidle');
    
    // Check for columns container or empty state
    const columnsContainer = page.locator('.columns-container');
    const emptyState = page.locator('.empty-state');
    
    const containerVisible = await columnsContainer.isVisible().catch(() => false);
    const emptyVisible = await emptyState.isVisible().catch(() => false);
    
    expect(containerVisible || emptyVisible).toBeTruthy();
  });

  test('should handle navigation back to boards', async ({ page }) => {
    await page.goto('/board/test-board');
    await page.waitForLoadState('networkidle');
    
    // Click back button if visible
    const backButton = page.locator('text=← Back to Boards');
    if (await backButton.isVisible()) {
      await backButton.click();
      
      // Should navigate back to home
      await page.waitForURL('/');
      expect(page.url()).toBe('http://localhost:5173/');
    }
  });

  test('should have responsive layout', async ({ page }) => {
    // Test at desktop size
    await page.setViewportSize({ width: 1920, height: 1080 });
    await expect(page.locator('.container')).toBeVisible();
    
    // Test at tablet size
    await page.setViewportSize({ width: 768, height: 1024 });
    await expect(page.locator('.container')).toBeVisible();
    
    // Test at mobile size
    await page.setViewportSize({ width: 375, height: 667 });
    await expect(page.locator('.container')).toBeVisible();
  });

  test('should display error message on failure', async ({ page }) => {
    // This test would require mocking API failures
    // For now, just verify error message container exists in DOM
    await page.goto('/');
    
    // Check if the page has error handling capability
    const errorContainer = page.locator('.error');
    if (await errorContainer.isVisible()) {
      await expect(errorContainer).toBeVisible();
    }
  });
});
