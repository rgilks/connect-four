'use client';

import React from 'react';
import { motion } from 'framer-motion';
import { Player } from '@/lib/types';

interface GamePieceProps {
  player: Player;
  isClickable: boolean;
}

const GamePiece = React.memo(function GamePiece({
  player,
  isClickable,
}: GamePieceProps) {
  const isPlayer1 = player === 'player1';
  const colors = isPlayer1
    ? {
        bg: 'bg-red-500',
        border: 'border-red-400',
        shadow: 'shadow-red-500/50',
        glow: 'shadow-red-400',
        inner: 'bg-red-300',
      }
    : {
        bg: 'bg-yellow-500',
        border: 'border-yellow-400',
        shadow: 'shadow-yellow-500/50',
        glow: 'shadow-yellow-400',
        inner: 'bg-yellow-300',
      };

  return (
    <motion.div
      className={`w-full h-full rounded-full border-2 relative overflow-hidden ${
        isClickable ? 'cursor-pointer' : 'cursor-default'
      } ${colors.bg} ${colors.border} ${colors.shadow}`}
      whileHover={isClickable ? { scale: 1.1, boxShadow: `0 0 20px ${colors.glow}` } : {}}
      whileTap={isClickable ? { scale: 0.95 } : {}}
      animate={{}}
      transition={{ type: 'spring', stiffness: 400, damping: 25 }}
      data-testid={`game-piece-${player}-${isClickable ? 'clickable' : 'static'}`}
    >
      <div className="absolute inset-0 bg-gradient-to-br from-white/30 to-transparent" />
      <div className="absolute inset-0 bg-gradient-to-tl from-black/20 to-transparent" />

      {isClickable && (
        <motion.div
          className="absolute inset-0 rounded-full border-2 border-white/50 pointer-events-none"
          animate={{
            boxShadow: [`0 0 0 0 rgba(255, 255, 255, 0.7)`, `0 0 0 8px rgba(255, 255, 255, 0)`],
          }}
          transition={{ duration: 1.5, repeat: Infinity }}
        />
      )}



      <div className="absolute inset-0 flex items-center justify-center">
        <div className={`w-1/3 h-1/3 rounded-full ${colors.inner} shadow-inner`} />
      </div>
    </motion.div>
  );
});

export default GamePiece;
