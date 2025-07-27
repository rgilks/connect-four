import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';
import { immer } from 'zustand/middleware/immer';
import { initializeGame, makeMove as makeMoveLogic, makeAIMove } from './game-logic';
import { initializeWASMAI } from './wasm-ai-service';
import type { GameState } from './types';

const LATEST_VERSION = 1;

type GameStore = {
  gameState: GameState;
  aiThinking: boolean;
  pendingMove: { column: number; player: 'player1' | 'player2' } | null;
  actions: {
    initialize: (fromStorage?: boolean) => void;
    makeMove: (column: number) => void;
    completeMove: () => void;
    makeAIMove: () => void;
    reset: () => void;
  };
};

export const useGameStore = create<GameStore>()(
  persist(
    immer((set, get) => ({
      gameState: { ...initializeGame() },
      aiThinking: false,
      pendingMove: null,
      actions: {
        initialize: (fromStorage = false) => {
          if (!fromStorage) {
            set(state => {
              state.gameState = { ...initializeGame() };
              state.aiThinking = false;
            });
          }
          
          // Initialize WASM AI in the background
          initializeWASMAI().catch(error => {
            console.warn('Failed to initialize WASM AI:', error);
          });
        },
        makeMove: (column: number) => {
          const { gameState } = get();
          if (gameState.gameStatus !== 'playing') return;

          // Set pending move for animation
          set(state => {
            state.pendingMove = { column, player: gameState.currentPlayer };
          });
        },
        completeMove: () => {
          const { gameState, pendingMove } = get();
          if (!pendingMove) return;

          const newState = makeMoveLogic(gameState, pendingMove.column);
          set(state => {
            state.gameState = newState;
            state.pendingMove = null;
          });
        },
        makeAIMove: async () => {
          const { gameState } = get();
          if (gameState.gameStatus !== 'playing' || gameState.currentPlayer !== 'player2') return;

          set(state => {
            state.aiThinking = true;
          });

          // Add a small delay to make AI thinking visible
          setTimeout(async () => {
            const currentState = get().gameState;
            if (currentState.gameStatus === 'playing' && currentState.currentPlayer === 'player2') {
              try {
                const aiColumn = await makeAIMove(currentState);
                // Set pending move for AI animation
                set(state => {
                  state.pendingMove = { column: aiColumn, player: 'player2' };
                  state.aiThinking = false;
                });

                // Complete the AI move after animation delay
                setTimeout(() => {
                  const { gameState: updatedState, pendingMove } = get();
                  if (pendingMove && pendingMove.player === 'player2') {
                    const newState = makeMoveLogic(updatedState, pendingMove.column);
                    set(state => {
                      state.gameState = newState;
                      state.pendingMove = null;
                    });
                  }
                }, 800);
              } catch (error) {
                console.error('AI move calculation failed:', error);
                // Show error to user
                alert(`AI Error: ${error instanceof Error ? error.message : 'Unknown error'}. Please refresh the page.`);
                set(state => {
                  state.aiThinking = false;
                });
              }
            } else {
              // Game state changed, reset thinking state
              set(state => {
                state.aiThinking = false;
              });
            }
          }, 500);
        },
        reset: () => {
          set(state => {
            state.gameState = { ...initializeGame() };
            state.aiThinking = false;
            state.pendingMove = null;
          });
        },
      },
    })),
    {
      name: 'connect-4-game-storage',
      storage: createJSONStorage(() => localStorage),
      onRehydrateStorage: () => (state, error) => {
        if (error) {
          console.error('Failed to rehydrate game store:', error);
        }
        if (state) {
          state.actions.initialize(true);
        }
      },
      version: LATEST_VERSION,
      migrate: (persistedState, version) => {
        const state = persistedState as Partial<GameStore>;
        if (version < LATEST_VERSION || !state || !state.gameState) {
          return { gameState: initializeGame() };
        }
        return { gameState: state.gameState };
      },
      partialize: state => ({
        gameState: state.gameState,
      }),
    }
  )
);

export const useGameStoreActions = () => useGameStore(state => state.actions);
export const useGameState = () => useGameStore(state => state.gameState);
export const useGameActions = () => useGameStore(state => state.actions);
