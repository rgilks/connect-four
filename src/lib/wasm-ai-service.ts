import { GameState } from './schemas';

export interface WASMAIResponse {
  move: number | null;
  evaluations: Array<{
    column: number;
    score: number;
    moveType: string;
  }>;
  nodesEvaluated: number;
  transpositionHits: number;
}

export interface WASMHeuristicResponse {
  move: number | null;
  evaluations: Array<{
    column: number;
    score: number;
    moveType: string;
  }>;
  nodesEvaluated: number;
}

export interface WASMMLResponse {
  move: number | null;
  evaluation: number;
  thinking: string;
  diagnostics: {
    validMoves: number[];
    moveEvaluations: Array<{
      column: number;
      score: number;
      moveType: string;
    }>;
    valueNetworkOutput: number;
    policyNetworkOutputs: number[];
  };
}

interface WASMAIInstance {
  get_best_move: (state: unknown, depth: number) => string;
  get_heuristic_move: (state: unknown) => string;
  get_ml_move: (state: unknown) => string;
  evaluate_position: (state: unknown) => number;
  load_ml_weights: (value_weights: unknown, policy_weights: unknown) => void;
  clear_transposition_table: () => void;
  get_transposition_table_size: () => number;
}

interface WASMModule {
  default: () => Promise<unknown>;
  ConnectFourAI: new () => WASMAIInstance;
}

class WASMAIService {
  private ai: WASMAIInstance | null = null;
  private isLoaded = false;
  private loadPromise: Promise<void> | null = null;

  async initialize(): Promise<void> {
    if (this.loadPromise) {
      return this.loadPromise;
    }

    this.loadPromise = this._initialize();
    return this.loadPromise;
  }

  private async _initialize(): Promise<void> {
    // Only load WASM in browser environment
    if (typeof window === 'undefined') {
      console.log('🔄 Skipping WASM AI initialization in non-browser environment');
      return;
    }

    try {
      // Use dynamic import to load the WASM module
      console.log('🔄 Loading WASM module...');
      const wasmModule = (await import('/wasm/connect_four_ai_core.js')) as WASMModule;
      console.log('🔄 WASM module imported, initializing...');
      await wasmModule.default();
      console.log('🔄 WASM module initialized, creating AI instance...');
      this.ai = new wasmModule.ConnectFourAI();
      this.isLoaded = true;
      console.log('✅ WASM AI loaded successfully');
    } catch (error) {
      console.error('❌ Failed to load WASM AI:', error);
      console.error('❌ Error details:', error instanceof Error ? error.stack : error);
      throw new Error(`Failed to load WASM AI: ${error}`);
    }
  }

  private async convertGameStateToWASM(gameState: GameState): Promise<unknown> {
    const board = gameState.board.map(col =>
      col.map(cell => {
        if (cell === null) return 'empty';
        return cell;
      })
    );

    // Load genetic parameters from evolved.json
    const geneticParams = await this.loadGeneticParams();

    return {
      board,
      current_player: gameState.currentPlayer,
      genetic_params: geneticParams,
    };
  }

  private async loadGeneticParams(): Promise<Record<string, number>> {
    try {
      // Try to load from the evolved.json file
      const response = await fetch('/ml/data/genetic_params/evolved.json');
      if (response.ok) {
        return await response.json();
      }
    } catch (error) {
      console.warn('Failed to load evolved genetic parameters, using defaults:', error);
    }

    // Fallback to default parameters
    return {
      center_control_weight: 1.0,
      piece_count_weight: 0.5,
      threat_weight: 2.0,
      mobility_weight: 0.8,
      vertical_control_weight: 1.2,
      horizontal_control_weight: 1.0,
    };
  }

  async getBestMove(gameState: GameState, depth: number = 5): Promise<WASMAIResponse> {
    if (!this.isLoaded || !this.ai) {
      throw new Error('WASM AI not loaded');
    }

    try {
      const wasmState = await this.convertGameStateToWASM(gameState);
      console.log('WASM AI: Converted state:', JSON.stringify(wasmState, null, 2));
      const result = this.ai.get_best_move(wasmState, depth);

      console.log('WASM AI: Raw result:', result);
      const parsedResult = typeof result === 'string' ? JSON.parse(result) : result;

      return {
        move: parsedResult.move,
        evaluations: parsedResult.evaluations || [],
        nodesEvaluated: parsedResult.nodes_evaluated || 0,
        transpositionHits: parsedResult.transposition_hits || 0,
      };
    } catch (error) {
      console.error('WASM AI error:', error);
      throw new Error(`WASM AI failed: ${error}`);
    }
  }

  async getHeuristicMove(gameState: GameState): Promise<WASMHeuristicResponse> {
    if (!this.isLoaded || !this.ai) {
      throw new Error('WASM AI not loaded');
    }

    try {
      const wasmState = await this.convertGameStateToWASM(gameState);
      const result = this.ai.get_heuristic_move(wasmState);
      return typeof result === 'string' ? JSON.parse(result) : result;
    } catch (error) {
      throw new Error(`WASM heuristic AI failed: ${error}`);
    }
  }

  async getMLMove(gameState: GameState): Promise<WASMMLResponse> {
    if (!this.isLoaded || !this.ai) {
      throw new Error('WASM AI not loaded');
    }

    try {
      const wasmState = await this.convertGameStateToWASM(gameState);
      const result = this.ai.get_ml_move(wasmState);
      return typeof result === 'string' ? JSON.parse(result) : result;
    } catch (error) {
      throw new Error(`WASM ML AI failed: ${error}`);
    }
  }

  async evaluatePosition(gameState: GameState): Promise<number> {
    if (!this.isLoaded || !this.ai) {
      throw new Error('WASM AI not loaded');
    }

    try {
      const wasmState = await this.convertGameStateToWASM(gameState);
      return this.ai.evaluate_position(wasmState);
    } catch (error) {
      throw new Error(`WASM position evaluation failed: ${error}`);
    }
  }

  async loadMLWeights(valueWeights: number[], policyWeights: number[]): Promise<void> {
    if (!this.isLoaded || !this.ai) {
      throw new Error('WASM AI not loaded');
    }

    try {
      this.ai.load_ml_weights(valueWeights, policyWeights);
      console.log('✅ ML weights loaded successfully');
    } catch (error) {
      throw new Error(`Failed to load ML weights: ${error}`);
    }
  }

  get isReady(): boolean {
    return this.isLoaded;
  }

  clearTranspositionTable(): void {
    if (this.isLoaded && this.ai) {
      this.ai.clear_transposition_table();
    }
  }

  getTranspositionTableSize(): number {
    if (this.isLoaded && this.ai) {
      return this.ai.get_transposition_table_size();
    }
    return 0;
  }
}

// Singleton instance
let wasmAIInstance: WASMAIService | null = null;

export function getWASMAIService(): WASMAIService {
  if (!wasmAIInstance) {
    wasmAIInstance = new WASMAIService();
  }
  return wasmAIInstance;
}

// For testing purposes
export function resetWASMAIService(): void {
  wasmAIInstance = null;
}

export async function initializeWASMAI(): Promise<void> {
  const service = getWASMAIService();
  await service.initialize();

  // Try to load ML weights (currently using test weights from another game)
  try {
    const weightsResponse = await fetch('/ml/data/weights/test_weights.json');
    if (weightsResponse.ok) {
      const weights = (await weightsResponse.json()) as {
        value_weights?: number[];
        policy_weights?: number[];
      };
      if (weights.value_weights && weights.policy_weights) {
        await service.loadMLWeights(weights.value_weights, weights.policy_weights);
        console.log('✅ ML weights loaded successfully (test weights from another game)');
      }
    }
  } catch (error) {
    console.warn('Could not load ML weights:', error);
  }
}

export default WASMAIService;
