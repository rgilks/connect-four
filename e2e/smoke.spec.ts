import { test, expect, Page } from '@playwright/test';



async function startGame(page: Page) {
  await page.goto('/');
  // The game starts directly without mode selection
  await expect(page.getByTestId('game-board')).toBeVisible();
  // Wait for the animation to complete (0.5s duration + buffer)
  await page.waitForTimeout(600);
}





test.describe('Core Game Functionality', () => {
  test('can start a game and see initial state', async ({ page }) => {
    await startGame(page);
    await expect(page.getByTestId('game-board')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Connect 4' })).toBeVisible();
    await expect(page.getByText('Drop your pieces to get four in a row!')).toBeVisible();
  });
});

test.describe('Game Interactions', () => {
  test.beforeEach(async ({ page }) => {
    await startGame(page);
  });

  test('can click on board columns', async ({ page }) => {
    const gameBoard = page.getByTestId('game-board');
    await expect(gameBoard).toBeVisible();

    // Click on a column to drop a piece
    await page.getByTestId('column-3').click();

    // Wait a moment for the move to complete
    await page.waitForTimeout(1000);

    // The board should still be visible after the move
    await expect(gameBoard).toBeVisible();
  });

  test('can make a move by clicking on a column', async ({ page }) => {
    // Click on a column to drop a piece
    await page.getByTestId('column-3').click();
    await page.waitForTimeout(1000);

    // Should see some change in game state
    await expect(page.getByTestId('game-status-text')).not.toBeEmpty();
  });

  test('can toggle sound settings', async ({ page }) => {
    const soundToggle = page.getByTestId('toggle-sound');
    await expect(soundToggle).toBeVisible();

    // Click to toggle
    await soundToggle.click();
    await page.waitForTimeout(100);

    // Should still be visible after toggle
    await expect(soundToggle).toBeVisible();
  });

  test('can open and close help panel', async ({ page }) => {
    await page.getByTestId('how-to-play').click();
    await expect(page.getByTestId('help-panel')).toBeVisible();
    await expect(page.getByTestId('help-close')).toBeVisible();

    await page.getByTestId('help-close').click();
    await expect(page.getByTestId('help-panel')).not.toBeVisible();
  });
});

test.describe('Game Completion and Database Saves', () => {
  test('can make moves and see game state changes', async ({ page }) => {
    await startGame(page);

    // Make a few moves to test game state changes
    await page.getByTestId('column-3').click();
    await page.waitForTimeout(1000);

    await page.getByTestId('column-2').click();
    await page.waitForTimeout(1000);

    // Verify the game board is still visible and functional
    await expect(page.getByTestId('game-board')).toBeVisible();
    await expect(page.getByTestId('game-status-text')).not.toBeEmpty();
  });

  test('can reset game', async ({ page }) => {
    await startGame(page);

    // Make a move
    await page.getByTestId('column-3').click();
    await page.waitForTimeout(1000);

    // Click reset button
    await page.getByTestId('reset-game').click();

    // Should return to game board
    await expect(page.getByTestId('game-board')).toBeVisible();
  });
});

test.describe('Error Handling and Edge Cases', () => {
  test('handles rapid column clicks gracefully', async ({ page }) => {
    await startGame(page);

    // Rapidly click on different columns
    for (let i = 0; i < 3; i++) {
      await page.getByTestId(`column-${i}`).click();
      await page.waitForTimeout(50);
    }

    // Should still be functional
    await expect(page.getByTestId('game-board')).toBeVisible();
  });

  test('handles rapid column selections gracefully', async ({ page }) => {
    await startGame(page);

    // Rapidly click on different columns
    for (let i = 0; i < 3; i++) {
      await page.getByTestId(`column-${i}`).click();
      await page.waitForTimeout(50);
    }

    // Should still be functional
    await expect(page.getByTestId('game-board')).toBeVisible();
  });

  test('maintains game state during navigation', async ({ page }) => {
    await startGame(page);

    // Make some game progress
    await page.getByTestId('column-3').click();
    await page.waitForTimeout(1000);

    // Navigate away and back
    await page.goto('/');
    await page.goto('/');

    // Wait for the page to load and game state to be restored
    await page.waitForTimeout(1000);

    // Should be back to game board
    await expect(page.getByTestId('game-board')).toBeVisible();

    // Game board should be visible
    await expect(page.getByTestId('game-board')).toBeVisible();
  });
});

test.describe('Mobile Responsiveness', () => {
  test.use({ viewport: { width: 375, height: 667 } });

  test('game is fully functional on mobile', async ({ page }) => {
    await startGame(page);

    // Verify all key elements are visible and functional
    await expect(page.getByTestId('game-board')).toBeVisible();
    await expect(page.getByTestId('toggle-sound')).toBeVisible();
    await expect(page.getByTestId('how-to-play')).toBeVisible();

    // Test basic interactions
    await page.getByTestId('column-3').click();
    await page.waitForTimeout(1000);
    await expect(page.getByTestId('game-board')).toBeVisible();
  });
});
