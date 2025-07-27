import { GameState, Player, Board, MoveRecord } from './schemas';
import { getWASMAIService } from './wasm-ai-service';

const ROWS = 6;
const COLS = 7;

export interface WinningLine {
  positions: Array<{ column: number; row: number }>;
  direction: 'horizontal' | 'vertical' | 'diagonal';
}

function createEmptyBoard(): Board {
  return Array.from({ length: COLS }, () => Array(ROWS).fill(null));
}

export function initializeGame(): GameState {
  const startingPlayer: Player = 'player1'; // Fixed to avoid hydration mismatch
  return {
    board: createEmptyBoard(),
    currentPlayer: startingPlayer,
    gameStatus: 'playing',
    winner: null,
    history: [],
    winningLine: null,
  };
}

export function makeMove(gameState: GameState, column: number): GameState {
  if (gameState.gameStatus !== 'playing') return gameState;
  if (column < 0 || column >= COLS) return gameState;

  // Find the lowest empty row in the column (bottom is index 5, top is index 0)
  const col = gameState.board[column];
  const row = col.lastIndexOf(null);
  if (row === -1) return gameState; // Column full

  // Place the piece in the lowest empty row
  const newBoard: Board = gameState.board.map((c, i) =>
    i === column ? [...c.slice(0, row), gameState.currentPlayer, ...c.slice(row + 1)] : [...c]
  );

  const newHistory: MoveRecord[] = [
    ...gameState.history,
    { player: gameState.currentPlayer, column, row },
  ];

  // Check for win
  const winResult = checkWin(newBoard, column, row, gameState.currentPlayer);
  const winner = winResult ? gameState.currentPlayer : null;
  const isDrawn = !winner && isDraw(newBoard);

  return {
    board: newBoard,
    currentPlayer:
      winner || isDrawn ? gameState.currentPlayer : otherPlayer(gameState.currentPlayer),
    gameStatus: winner ? 'finished' : isDrawn ? 'finished' : 'playing',
    winner: winner,
    history: newHistory,
    winningLine: winResult,
  };
}

export function getValidMoves(board: Board): number[] {
  return board
    .map((col, index) => (col.some(cell => cell === null) ? index : -1))
    .filter(index => index !== -1);
}

export async function makeAIMove(gameState: GameState): Promise<number> {
  const wasmAI = getWASMAIService();

  if (!wasmAI.isReady) {
    throw new Error('WASM AI not loaded. Please refresh the page and try again.');
  }

  try {
    // Clear transposition table to ensure fresh calculations
    wasmAI.clearTranspositionTable();
    const response = await wasmAI.getBestMove(gameState, 3);
    if (response.move !== null && response.move !== undefined) {
      console.log(
        `🤖 WASM AI chose column ${response.move} (evaluated ${response.nodesEvaluated} nodes)`
      );
      return response.move;
    }
  } catch (error) {
    console.error('WASM AI failed:', error);
    throw new Error(`AI calculation failed: ${error}`);
  }

  throw new Error('No valid move found');
}



function otherPlayer(player: Player): Player {
  return player === 'player1' ? 'player2' : 'player1';
}

export function isDraw(board: Board): boolean {
  return board.every(col => col.every(cell => cell !== null));
}

export function checkWin(
  board: Board,
  col: number,
  row: number,
  player: Player
): WinningLine | null {
  // Check horizontal
  const horizontalLine = checkDirection(board, col, row, 1, 0, player);
  if (horizontalLine) return { positions: horizontalLine, direction: 'horizontal' };

  // Check vertical
  const verticalLine = checkDirection(board, col, row, 0, 1, player);
  if (verticalLine) return { positions: verticalLine, direction: 'vertical' };

  // Check diagonal /
  const diagonal1Line = checkDirection(board, col, row, 1, 1, player);
  if (diagonal1Line) return { positions: diagonal1Line, direction: 'diagonal' };

  // Check diagonal \
  const diagonal2Line = checkDirection(board, col, row, 1, -1, player);
  if (diagonal2Line) return { positions: diagonal2Line, direction: 'diagonal' };

  return null;
}

function checkDirection(
  board: Board,
  col: number,
  row: number,
  dCol: number,
  dRow: number,
  player: Player
): Array<{ column: number; row: number }> | null {
  const positions: Array<{ column: number; row: number }> = [];

  // Count in positive direction
  let count = 1;
  positions.push({ column: col, row });
  let c = col + dCol;
  let r = row + dRow;
  while (c >= 0 && c < COLS && r >= 0 && r < ROWS && board[c][r] === player) {
    count++;
    positions.push({ column: c, row: r });
    c += dCol;
    r += dRow;
  }

  // Count in negative direction
  c = col - dCol;
  r = row - dRow;
  while (c >= 0 && c < COLS && r >= 0 && r < ROWS && board[c][r] === player) {
    count++;
    positions.unshift({ column: c, row: r });
    c -= dCol;
    r -= dRow;
  }

  return count >= 4 ? positions : null;
}
