import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';
import { immer } from 'zustand/middleware/immer';
import { initializeGame, makeMove as makeMoveLogic, makeAIMove } from './game-logic';
import { initializeWASMAI } from './wasm-ai-service';
import type { GameState, AIType, GameMode } from './types';
import { useUIStore } from './ui-store';

const LATEST_VERSION = 1;

type GameStore = {
  gameState: GameState;
  aiThinking: boolean;
  pendingMove: { column: number; player: 'player1' | 'player2' } | null;
  showWinnerModal: boolean;
  selectedAI: AIType;
  player1AI: AIType;
  player2AI: AIType;
  gameMode: GameMode;
  actions: {
    initialize: (fromStorage?: boolean) => void;
    startGame: () => void;
    makeMove: (column: number) => void;
    completeMove: () => void;
    makeAIMove: () => void;
    reset: () => void;
    showWinnerModal: () => void;
    setAI: (aiType: AIType) => void;
    setPlayer1AI: (aiType: AIType) => void;
    setPlayer2AI: (aiType: AIType) => void;
    setGameMode: (mode: GameMode) => void;
  };
};

export const useGameStore = create<GameStore>()(
  persist(
    immer((set, get) => ({
      gameState: {
        board: Array.from({ length: 7 }, () => Array.from({ length: 6 }, () => null)),
        currentPlayer: 'player1',
        gameStatus: 'not_started' as const,
        winner: null,
        history: [],
        winningLine: null,
      },
      aiThinking: false,
      pendingMove: null,
      showWinnerModal: false,
      selectedAI: 'classic' as AIType,
      player1AI: 'classic' as AIType,
      player2AI: 'classic' as AIType,
      gameMode: 'human-vs-ai' as GameMode,
      actions: {
        initialize: () => {
          // Initialize WASM AI in the background
          initializeWASMAI().catch(error => {
            console.warn('Failed to initialize WASM AI:', error);
          });
        },
        startGame: () => {
          // Create a fresh game with new random starting player
          set(state => {
            state.gameState = { ...initializeGame() };
            state.aiThinking = false;
            state.showWinnerModal = false;
            state.pendingMove = null;
          });
        },
        makeMove: (column: number) => {
          const { gameState } = get();
          if (gameState.gameStatus !== 'playing') return;

          const playerName = gameState.currentPlayer === 'player1' ? 'Red' : 'Yellow';
          console.log(`🎯 ${playerName} selecting column ${column}...`);

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

            // If game is finished with a winner, delay showing the modal
            if (newState.gameStatus === 'finished' && newState.winner) {
              // Don't show modal immediately - let the win animation play first
              state.showWinnerModal = false;
            }
          });
        },
        makeAIMove: async () => {
          const { gameState, selectedAI, player1AI, player2AI, gameMode } = get();

          // In AI vs AI mode, both players are AI
          // In human vs AI mode, only player2 is AI
          const isAITurn =
            gameMode === 'ai-vs-ai' ||
            (gameMode === 'human-vs-ai' && gameState.currentPlayer === 'player2');

          if (gameState.gameStatus !== 'playing' || !isAITurn) return;

          set(state => {
            state.aiThinking = true;
          });

          // Add a small delay to make AI thinking visible
          setTimeout(async () => {
            const currentState = get().gameState;
            const currentGameMode = get().gameMode;
            const isStillAITurn =
              currentGameMode === 'ai-vs-ai' ||
              (currentGameMode === 'human-vs-ai' && currentState.currentPlayer === 'player2');

            if (currentState.gameStatus === 'playing' && isStillAITurn) {
              try {
                // Determine which AI to use based on current player
                let aiTypeToUse: AIType;
                if (currentGameMode === 'ai-vs-ai') {
                  aiTypeToUse = currentState.currentPlayer === 'player1' ? player1AI : player2AI;
                } else {
                  aiTypeToUse = selectedAI; // For human vs AI, use the selected AI
                }

                const aiColumn = await makeAIMove(currentState, aiTypeToUse);
                // Set pending move for AI animation
                set(state => {
                  state.pendingMove = { column: aiColumn, player: currentState.currentPlayer };
                  state.aiThinking = false;
                });

                // Complete the AI move after animation delay
                setTimeout(() => {
                  const { gameState: updatedState, pendingMove } = get();
                  if (pendingMove) {
                    const newState = makeMoveLogic(updatedState, pendingMove.column);
                    set(state => {
                      state.gameState = newState;
                      state.pendingMove = null;

                      // If game is finished with a winner, delay showing the modal
                      if (newState.gameStatus === 'finished' && newState.winner) {
                        // Don't show modal immediately - let the win animation play first
                        state.showWinnerModal = false;
                      }
                    });
                  }
                }, 800);
              } catch (error) {
                console.error('AI move calculation failed:', error);
                // Show error to user via UI store
                const errorMessage = `AI calculation failed: ${error instanceof Error ? error.message : 'Unknown error'}. Please refresh the page.`;
                useUIStore.getState().actions.showError(errorMessage);
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
            state.gameState = {
              board: Array.from({ length: 7 }, () => Array.from({ length: 6 }, () => null)),
              currentPlayer: 'player1',
              gameStatus: 'not_started' as const,
              winner: null,
              history: [],
              winningLine: null,
            };
            state.aiThinking = false;
            state.pendingMove = null;
            state.showWinnerModal = false;
          });
        },
        showWinnerModal: () => {
          set(state => {
            state.showWinnerModal = true;
          });
        },
        setAI: (aiType: AIType) => {
          set(state => {
            state.selectedAI = aiType;
            // For human vs AI mode, set player2AI to the selected AI
            state.player2AI = aiType;
          });
        },
        setPlayer1AI: (aiType: AIType) => {
          set(state => {
            state.player1AI = aiType;
          });
        },
        setPlayer2AI: (aiType: AIType) => {
          set(state => {
            state.player2AI = aiType;
          });
        },
        setGameMode: (mode: GameMode) => {
          set(state => {
            state.gameMode = mode;
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
