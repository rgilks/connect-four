/// <reference types="@cloudflare/workers-types" />

interface CloudflareEnv {
  DB: D1Database;
  ASSETS: Fetcher;
}

declare global {
  var DB: D1Database;
}

declare module '/wasm/connect_four_ai_core.js' {
  export class ConnectFourAI {
    free(): void;
    constructor();
    get_best_move(board_state: any, depth: number): any;
    get_heuristic_move(board_state: any): any;
    get_ml_move(board_state: any): any;
    evaluate_position(board_state: any): number;
    evaluate_position_ml(board_state: any): number;
    get_valid_moves(board_state: any): any;
    make_move(board_state: any, column: number): any;
    is_game_over(board_state: any): boolean;
    get_winner(board_state: any): any;
    create_new_game(): any;
    create_game_with_params(params: any): any;
    clear_transposition_table(): void;
    get_transposition_table_size(): number;
    load_ml_weights(value_weights: any, policy_weights: any): void;
  }

  export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

  export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_connectfourai_free: (a: number, b: number) => void;
    readonly connectfourai_new: () => number;
    readonly connectfourai_get_best_move: (a: number, b: number, c: number, d: number) => void;
    readonly connectfourai_get_heuristic_move: (a: number, b: number, c: number) => void;
    readonly connectfourai_get_ml_move: (a: number, b: number, c: number) => void;
    readonly connectfourai_evaluate_position: (a: number, b: number, c: number) => void;
    readonly connectfourai_evaluate_position_ml: (a: number, b: number, c: number) => void;
    readonly connectfourai_get_valid_moves: (a: number, b: number, c: number) => void;
    readonly connectfourai_make_move: (a: number, b: number, c: number, d: number) => void;
    readonly connectfourai_is_game_over: (a: number, b: number, c: number) => void;
    readonly connectfourai_get_winner: (a: number, b: number, c: number) => void;
    readonly connectfourai_create_new_game: (a: number, b: number) => void;
    readonly connectfourai_create_game_with_params: (a: number, b: number, c: number) => void;
    readonly connectfourai_clear_transposition_table: (a: number) => void;
    readonly connectfourai_get_transposition_table_size: (a: number) => number;
    readonly connectfourai_load_ml_weights: (a: number, b: number, c: number, d: number) => void;
    readonly __wbindgen_export_0: (a: number, b: number) => number;
    readonly __wbindgen_export_1: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_export_2: (a: number) => void;
    readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  }

  export type SyncInitInput = BufferSource | WebAssembly.Module;

  export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

  export default function __wbg_init(
    module_or_path?:
      | { module_or_path: InitInput | Promise<InitInput> }
      | InitInput
      | Promise<InitInput>
  ): Promise<InitOutput>;
}
