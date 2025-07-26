// Simple debug test
const { initializeGame, makeMove, checkWin } = require('./src/lib/game-logic.ts');

console.log('Starting debug test...');

const game = initializeGame();
console.log('Initial game:', game);

let currentGame = game;

// Make moves one by one
currentGame = makeMove(currentGame, 0);
console.log(
  'After move 0:',
  currentGame.board.map(col => col[0])
);

currentGame = makeMove(currentGame, 1);
console.log(
  'After move 1:',
  currentGame.board.map(col => col[0])
);

currentGame = makeMove(currentGame, 2);
console.log(
  'After move 2:',
  currentGame.board.map(col => col[0])
);

currentGame = makeMove(currentGame, 3);
console.log(
  'After move 3:',
  currentGame.board.map(col => col[0])
);

currentGame = makeMove(currentGame, 4);
console.log(
  'After move 4:',
  currentGame.board.map(col => col[0])
);
console.log('Full board:', currentGame.board);
console.log('Game status:', currentGame.gameStatus);
console.log('Winner:', currentGame.winner);
console.log('Winning line:', currentGame.winningLine);

// Test win detection manually
const winResult = checkWin(currentGame.board, 4, 0, 'player1');
console.log('Manual win check result:', winResult);
