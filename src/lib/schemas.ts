import { z } from 'zod';

export const PlayerSchema = z.enum(['player1', 'player2']);
export type Player = z.infer<typeof PlayerSchema>;

export const GameStatusSchema = z.enum(['waiting', 'playing', 'finished']);
export type GameStatus = z.infer<typeof GameStatusSchema>;

export const BoardSchema = z.array(z.array(PlayerSchema.nullable()).length(6)).length(7);
export type Board = z.infer<typeof BoardSchema>;

export const MoveRecordSchema = z.object({
  player: PlayerSchema,
  column: z.number(),
  row: z.number(),
});
export type MoveRecord = z.infer<typeof MoveRecordSchema>;

export const WinningLineSchema = z.object({
  positions: z.array(
    z.object({
      column: z.number(),
      row: z.number(),
    })
  ),
  direction: z.enum(['horizontal', 'vertical', 'diagonal']),
});
export type WinningLine = z.infer<typeof WinningLineSchema>;

export const GameStateSchema = z.object({
  board: BoardSchema,
  currentPlayer: PlayerSchema,
  gameStatus: GameStatusSchema,
  winner: PlayerSchema.nullable(),
  history: z.array(MoveRecordSchema),
  winningLine: WinningLineSchema.nullable(),
});
export type GameState = z.infer<typeof GameStateSchema>;

export const GameActionSchema = z.discriminatedUnion('type', [
  z.object({ type: z.literal('MAKE_MOVE'), column: z.number() }),
  z.object({ type: z.literal('RESET_GAME') }),
]);
export type GameAction = z.infer<typeof GameActionSchema>;
