# WASM AI Integration Plan

## Overview

The Connect Four game has a sophisticated Rust/WASM AI system built but not integrated. This document outlines the step-by-step plan to connect the WASM AI to the frontend game.

## Current Architecture

### What's Built (Not Used)

- **Rust AI Core**: `worker/rust_ai_core/src/`
  - Classic AI (minimax with alpha-beta pruning)
  - ML AI (neural networks)
  - Genetic parameter optimization
  - WASM bindings in `wasm_api.rs`

- **WASM Build System**:
  - Build scripts in `package.json`
  - WASM files output to `public/wasm/`
  - Type definitions in `env.d.ts`

### What's Active (JavaScript)

- **Game Logic**: `src/lib/game-logic.ts`
  - Simple heuristic AI in `makeAIMove()`
  - Basic win/block detection
  - Center column preference

## Integration Plan

### Phase 1: Basic WASM Integration

#### Step 1: Create WASM Service Layer

**File**: `src/lib/wasm-ai-service.ts`

```typescript
interface WASMAIResponse {
  move: number | null;
  evaluations: Array<{
    column: number;
    score: number;
    moveType: string;
  }>;
  nodesEvaluated: number;
  transpositionHits: number;
}

class WASMAIService {
  private ai: any = null;
  private isLoaded = false;

  async initialize(): Promise<void> {
    // Load WASM module
    // Initialize AI instance
  }

  async getBestMove(gameState: GameState, depth: number = 3): Promise<WASMAIResponse> {
    // Convert game state to WASM format
    // Call WASM AI
    // Return response
  }

  async getHeuristicMove(gameState: GameState): Promise<WASMAIResponse> {
    // Use heuristic AI
  }

  async getMLMove(gameState: GameState): Promise<WASMAIResponse> {
    // Use ML AI (if weights loaded)
  }
}
```

#### Step 2: Update Game Logic

**File**: `src/lib/game-logic.ts`

```typescript
import { WASMAIService } from './wasm-ai-service';

let wasmAI: WASMAIService | null = null;

export async function initializeWASMAI(): Promise<void> {
  try {
    wasmAI = new WASMAIService();
    await wasmAI.initialize();
  } catch (error) {
    console.warn('WASM AI failed to load, falling back to JavaScript AI:', error);
  }
}

export async function makeAIMove(gameState: GameState): Promise<number> {
  if (wasmAI?.isLoaded) {
    try {
      const response = await wasmAI.getBestMove(gameState);
      return response.move ?? -1;
    } catch (error) {
      console.warn('WASM AI failed, falling back to JavaScript AI:', error);
    }
  }

  // Fallback to current JavaScript implementation
  return makeAIMoveJavaScript(gameState);
}

function makeAIMoveJavaScript(gameState: GameState): number {
  // Current implementation
}
```

#### Step 3: Update Game Store

**File**: `src/lib/game-store.ts`

```typescript
import { initializeWASMAI } from './game-logic';

// Initialize WASM AI on store creation
initializeWASMAI().catch(console.error);

// Update makeAIMove to be async
makeAIMove: async () => {
  // Handle async AI move
};
```

### Phase 2: Advanced Features

#### Step 1: AI Type Selection

- Add AI type selection to game settings
- Support Classic, Heuristic, ML AI types
- Add difficulty levels (search depth)

#### Step 2: Move Analysis Display

- Show AI thinking process
- Display move evaluations
- Show search statistics (nodes evaluated, cache hits)

#### Step 3: Performance Monitoring

- Track AI response times
- Monitor WASM vs JavaScript performance
- Add performance metrics to UI

### Phase 3: ML AI Integration

#### Step 1: Weight Loading

- Load ML weights (currently using test weights from another game)
- Support multiple model variants
- Add model selection UI
- TODO: Train neural network specifically for Connect Four

#### Step 2: Training Integration

- In-browser training capabilities
- Model management interface
- Real-time AI improvement

## Implementation Details

### WASM Module Loading

```typescript
// In wasm-ai-service.ts
async initialize(): Promise<void> {
  try {
    const wasmModule = await import('/wasm/connect_four_ai_core.js');
    await wasmModule.default();
    this.ai = new wasmModule.ConnectFourAI();
    this.isLoaded = true;
  } catch (error) {
    throw new Error(`Failed to load WASM AI: ${error}`);
  }
}
```

### Game State Conversion

```typescript
function convertGameStateToWASM(gameState: GameState): any {
  // Convert TypeScript game state to WASM format
  return {
    board: gameState.board,
    current_player: gameState.currentPlayer === 'player1' ? 0 : 1,
    genetic_params: {
      // Default genetic parameters
    },
  };
}
```

### Error Handling

```typescript
async getBestMove(gameState: GameState, depth: number = 3): Promise<WASMAIResponse> {
  if (!this.isLoaded) {
    throw new Error('WASM AI not loaded');
  }

  try {
    const wasmState = convertGameStateToWASM(gameState);
    const result = this.ai.get_best_move(wasmState, depth);
    return JSON.parse(result);
  } catch (error) {
    throw new Error(`WASM AI calculation failed: ${error}`);
  }
}
```

## Testing Strategy

### Unit Tests

- Test WASM service initialization
- Test game state conversion
- Test error handling and fallbacks

### Integration Tests

- Test WASM AI vs JavaScript AI performance
- Test all AI types work correctly
- Test fallback mechanisms

### E2E Tests

- Test complete game flow with WASM AI
- Test AI type switching
- Test error scenarios

## Performance Considerations

### Loading Time

- WASM module is ~2MB
- Load asynchronously during app initialization
- Show loading indicator

### Runtime Performance

- WASM AI: ~17ms per move (depth 3)
- JavaScript AI: ~1ms per move
- Acceptable trade-off for much stronger play

### Memory Usage

- Transposition tables can grow large
- Implement table size limits
- Clear tables between games

## Rollback Plan

If WASM integration fails:

1. Keep JavaScript AI as fallback
2. Add feature flag to disable WASM
3. Graceful degradation to current implementation

## Success Criteria

- [ ] WASM AI loads successfully
- [ ] Game plays with WASM AI instead of JavaScript AI
- [ ] Performance is acceptable (< 100ms per move)
- [ ] Fallback to JavaScript AI works
- [ ] All existing functionality preserved
- [ ] Tests pass with WASM AI

## Timeline

- **Week 1**: Basic WASM integration
- **Week 2**: Testing and refinement
- **Week 3**: Advanced features (AI selection, analysis)
- **Week 4**: ML AI integration and optimization
