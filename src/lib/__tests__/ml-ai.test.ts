import { describe, it, expect, beforeAll } from 'vitest';
import { initializeWASMAI, getWASMAIService } from '../wasm-ai-service';
import { initializeGame } from '../game-logic';

describe('ML AI Integration', () => {
  beforeAll(async () => {
    // Initialize WASM AI before running tests
    await initializeWASMAI();
  }, 30000); // 30 second timeout for WASM loading

  it('should load WASM AI successfully', async () => {
    const service = getWASMAIService();
    expect(service.isReady).toBe(true);
  });

  it('should make ML moves', async () => {
    const service = getWASMAIService();
    expect(service.isReady).toBe(true);

    const gameState = initializeGame();

    // Test that we can get an ML move
    const result = await service.getMLMove(gameState);

    expect(result).toBeDefined();
    expect(result.move).toBeDefined();
    expect(typeof result.move).toBe('number');
    expect(result.move).toBeGreaterThanOrEqual(0);
    expect(result.move).toBeLessThan(7);
    expect(result.evaluation).toBeDefined();
    expect(typeof result.evaluation).toBe('number');
    expect(result.diagnostics).toBeDefined();
    expect(result.diagnostics.validMoves).toBeDefined();
    expect(Array.isArray(result.diagnostics.validMoves)).toBe(true);
  });

  it('should make classic AI moves', async () => {
    const service = getWASMAIService();
    expect(service.isReady).toBe(true);

    const gameState = initializeGame();

    // Test that we can get a classic AI move
    const result = await service.getBestMove(gameState, 1);

    expect(result).toBeDefined();
    expect(result.move).toBeDefined();
    expect(typeof result.move).toBe('number');
    expect(result.move).toBeGreaterThanOrEqual(0);
    expect(result.move).toBeLessThan(7);
    expect(result.evaluations).toBeDefined();
    expect(Array.isArray(result.evaluations)).toBe(true);
  });
});
