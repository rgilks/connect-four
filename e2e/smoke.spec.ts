import { test, expect, Page } from '@playwright/test';
import Database from 'better-sqlite3';
import { existsSync } from 'fs';
import { execSync } from 'child_process';

// Ensure database is set up before running tests
test.beforeAll(async () => {
  const dbPath = 'local.db';
  if (!existsSync(dbPath)) {
    console.log('Database not found, setting up...');
    execSync('npm run db:local:reset', { stdio: 'inherit' });
  }
});

async function startGame(page: Page) {
  await page.goto('/');
  // The game starts directly without mode selection
  await expect(page.getByTestId('game-board')).toBeVisible();
}

async function waitForGameCompletion(page: Page) {
  await expect(page.getByTestId('game-completion-overlay')).toBeVisible({ timeout: 10000 });
  await expect(page.getByTestId('game-completion-title')).toBeVisible();
  await expect(page.getByTestId('game-completion-message')).toBeVisible();
}

async function verifyDatabaseSave(expectedGameType: string, expectedWinner: string = 'player1') {
  const dbPath = 'local.db';
  if (!existsSync(dbPath)) {
    console.error(`Database file not found: ${dbPath}`);
    console.log('Attempting to set up database...');
    execSync('npm run db:local:reset', { stdio: 'inherit' });

    if (!existsSync(dbPath)) {
      throw new Error(`Database file still not found after setup: ${dbPath}`);
    }
  }

  const db = new Database(dbPath);
  try {
    // Verify the games table exists
    const tableExists = db
      .prepare(
        `
      SELECT name FROM sqlite_master 
      WHERE type='table' AND name='games'
    `
      )
      .get();

    if (!tableExists) {
      console.error('Games table does not exist in database');
      console.log('Available tables:');
      const tables = db.prepare("SELECT name FROM sqlite_master WHERE type='table'").all();
      console.log(tables);
      throw new Error(
        'Games table does not exist in database. Run "npm run db:local:reset" to set up the database.'
      );
    }

    // Get the most recent game
    const row = db
      .prepare(
        `
      SELECT * FROM games 
      WHERE winner = ? AND gameType = ? 
      ORDER BY completedAt DESC 
      LIMIT 1
    `
      )
      .get(expectedWinner, expectedGameType) as any;

    if (!row) {
      throw new Error(
        `No game found with winner=${expectedWinner} and gameType=${expectedGameType}`
      );
    }

    // Verify required fields
    expect(row.winner).toBe(expectedWinner);
    expect(row.gameType).toBe(expectedGameType);
    expect(row.playerId).toBeTruthy();
    expect(row.completedAt).toBeTruthy();
    expect(row.moveCount).toBeGreaterThan(0);
    expect(row.history).toBeTruthy();

    // Verify history is valid JSON
    const history = JSON.parse(row.history);
    expect(Array.isArray(history)).toBe(true);
    expect(history.length).toBeGreaterThan(0);

    return row;
  } finally {
    db.close();
  }
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
    await page.getByTestId('square-3-5').click();

    // Wait a moment for the move to complete
    await page.waitForTimeout(500);

    // The board should still be visible after the move
    await expect(gameBoard).toBeVisible();
  });

  test('can make a move by clicking on a column', async ({ page }) => {
    // Click on a column to drop a piece
    await page.getByTestId('square-3-5').click();
    await page.waitForTimeout(500);

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
  async function simulateGameWin(page: Page) {
    // Make moves to create a winning scenario
    // Player 1: column 0
    await page.getByTestId('square-0-5').click();
    await page.waitForTimeout(200);

    // Player 2: column 1
    await page.getByTestId('square-1-5').click();
    await page.waitForTimeout(200);

    // Player 1: column 0
    await page.getByTestId('square-0-4').click();
    await page.waitForTimeout(200);

    // Player 2: column 1
    await page.getByTestId('square-1-4').click();
    await page.waitForTimeout(200);

    // Player 1: column 0
    await page.getByTestId('square-0-3').click();
    await page.waitForTimeout(200);

    // Player 2: column 1
    await page.getByTestId('square-1-3').click();
    await page.waitForTimeout(200);

    // Player 1: column 0 (winning move)
    await page.getByTestId('square-0-2').click();
    await page.waitForTimeout(200);

    // Wait for game completion
    await waitForGameCompletion(page);
  }

  test('completes a game and shows completion overlay', async ({ page }) => {
    await startGame(page);
    await simulateGameWin(page);

    // Verify completion overlay
    await expect(page.getByTestId('game-completion-overlay')).toBeVisible();
    await expect(page.getByTestId('game-completion-title')).toBeVisible();
    await expect(page.getByTestId('game-completion-message')).toBeVisible();

    // Verify stats panel shows the win
    await expect(page.getByTestId('stats-panel')).toBeVisible();
    await expect(page.getByTestId('wins-count')).toContainText('1');
  });

  test('saves completed game to database', async ({ page }) => {
    await startGame(page);
    await simulateGameWin(page);

    // Verify the game was saved to database
    const savedGame = await verifyDatabaseSave('classic');
    expect(savedGame).toBeTruthy();
    expect(savedGame.winner).toBe('player1');
    expect(savedGame.gameType).toBe('classic');
  });

  test('can reset game after completion', async ({ page }) => {
    await startGame(page);
    await simulateGameWin(page);

    // Click reset button
    await page.getByTestId('reset-game-button').click();

    // Should return to game board
    await expect(page.getByTestId('game-board')).toBeVisible();
  });
});

test.describe('Error Handling and Edge Cases', () => {
  test('handles rapid column clicks gracefully', async ({ page }) => {
    await startGame(page);

    // Rapidly click on different columns
    for (let i = 0; i < 3; i++) {
      await page.getByTestId(`square-${i}-5`).click();
      await page.waitForTimeout(50);
    }

    // Should still be functional
    await expect(page.getByTestId('game-board')).toBeVisible();
  });

  test('handles rapid column selections gracefully', async ({ page }) => {
    await startGame(page);

    // Rapidly click on different columns
    for (let i = 0; i < 3; i++) {
      await page.getByTestId(`square-${i}-5`).click();
      await page.waitForTimeout(50);
    }

    // Should still be functional
    await expect(page.getByTestId('game-board')).toBeVisible();
  });

  test('maintains game state during navigation', async ({ page }) => {
    await startGame(page);

    // Make some game progress
    await page.getByTestId('square-3-5').click();
    await page.waitForTimeout(500);

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
    await page.waitForTimeout(500);
    await expect(page.getByTestId('game-board')).toBeVisible();
  });
});
