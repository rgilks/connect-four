'use client';

import React from 'react';
import { motion } from 'framer-motion';

interface GamePieceProps {
  player: 'player1' | 'player2';
  isClickable?: boolean;
}

const GamePiece = React.memo(function GamePiece({ player, isClickable }: GamePieceProps) {
  const isPlayer1 = player === 'player1';
  const colors = isPlayer1
    ? {
        bg: 'bg-gradient-to-br from-red-500 to-red-600',
        border: 'border-red-400',
        shadow: 'shadow-red-500/50',
        glow: 'shadow-red-400',
        inner: 'bg-gradient-to-br from-red-300 to-red-400',
        pulse: 'shadow-red-400/60',
        highlight: 'from-red-200/40 to-transparent',
      }
    : {
        bg: 'bg-gradient-to-br from-yellow-500 to-yellow-600',
        border: 'border-yellow-400',
        shadow: 'shadow-yellow-500/50',
        glow: 'shadow-yellow-400',
        inner: 'bg-gradient-to-br from-yellow-300 to-yellow-400',
        pulse: 'shadow-yellow-400/60',
        highlight: 'from-yellow-200/40 to-transparent',
      };

  return (
    <motion.div
      className={`w-full h-full rounded-full border-2 relative overflow-hidden ${
        isClickable ? 'cursor-pointer' : 'cursor-default'
      } ${colors.bg} ${colors.border} ${colors.shadow}`}
      whileHover={isClickable ? { scale: 1.1, boxShadow: `0 0 20px ${colors.glow}` } : {}}
      whileTap={isClickable ? { scale: 0.95 } : {}}
      animate={{
        boxShadow: [
          `0 0 8px ${colors.pulse}`,
          `0 0 16px ${colors.pulse}`,
          `0 0 8px ${colors.pulse}`,
        ],
      }}
      transition={{
        type: 'spring',
        stiffness: 400,
        damping: 25,
        boxShadow: {
          duration: 2,
          repeat: Infinity,
          ease: 'easeInOut',
        },
      }}
      data-testid={`game-piece-${player}-${isClickable ? 'clickable' : 'static'}`}
    >
      {/* Enhanced gradient overlays */}
      <div className={`absolute inset-0 bg-gradient-to-br ${colors.highlight}`} />
      <div className="absolute inset-0 bg-gradient-to-tl from-black/30 to-transparent" />
      <div className="absolute inset-0 bg-gradient-to-tr from-white/20 to-transparent" />

      {/* Animated inner glow */}
      <motion.div
        className="absolute inset-0 rounded-full"
        animate={{
          boxShadow: [
            'inset 0 0 8px rgba(255, 255, 255, 0.3)',
            'inset 0 0 16px rgba(255, 255, 255, 0.1)',
            'inset 0 0 8px rgba(255, 255, 255, 0.3)',
          ],
        }}
        transition={{
          duration: 3,
          repeat: Infinity,
          ease: 'easeInOut',
        }}
      />

      {isClickable && (
        <motion.div
          className="absolute inset-0 rounded-full border-2 border-white/60 pointer-events-none"
          animate={{
            boxShadow: ['0 0 0 0 rgba(255, 255, 255, 0.8)', '0 0 0 12px rgba(255, 255, 255, 0)'],
            scale: [1, 1.05, 1],
          }}
          transition={{
            duration: 1.5,
            repeat: Infinity,
            ease: 'easeInOut',
          }}
        />
      )}

      {/* Enhanced center highlight */}
      <div className="absolute inset-0 flex items-center justify-center">
        <motion.div
          className={`w-1/3 h-1/3 rounded-full ${colors.inner} shadow-inner`}
          animate={{
            boxShadow: [
              'inset 0 2px 4px rgba(0, 0, 0, 0.3)',
              'inset 0 1px 2px rgba(0, 0, 0, 0.2)',
              'inset 0 2px 4px rgba(0, 0, 0, 0.3)',
            ],
          }}
          transition={{
            duration: 2,
            repeat: Infinity,
            ease: 'easeInOut',
          }}
        />
      </div>

      {/* Floating sparkle effect */}
      <motion.div
        className="absolute top-1/4 left-1/4 w-1 h-1 bg-white rounded-full"
        animate={{
          opacity: [0, 1, 0],
          scale: [0, 1, 0],
          x: [0, 2, 0],
          y: [0, -2, 0],
        }}
        transition={{
          duration: 2,
          repeat: Infinity,
          delay: 0.5,
          ease: 'easeInOut',
        }}
      />
    </motion.div>
  );
});

export default GamePiece;
