'use client';

import React from 'react';
import { motion } from 'framer-motion';
import { Player } from '@/lib/types';

interface ConnectFourWinProps {
  winningLine: Array<{ column: number; row: number }>;
  player: Player;
  onComplete?: () => void;
}

export default function ConnectFourWin({ winningLine, player, onComplete }: ConnectFourWinProps) {
  const isPlayer1 = player === 'player1';

  return (
    <>
      {/* Winning line highlight */}
      <motion.div
        className="absolute inset-0 z-30 pointer-events-none"
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ duration: 0.5 }}
      >
        {winningLine.map((pos, index) => (
          <motion.div
            key={`win-${pos.column}-${pos.row}`}
            className="absolute w-4/5 h-4/5"
            style={{
              left: `${(pos.column / 7) * 100}%`,
              top: `${(pos.row / 6) * 100}%`,
            }}
            initial={{ scale: 0.8, opacity: 0 }}
            animate={{ scale: 1.2, opacity: 1 }}
            transition={{
              delay: index * 0.1,
              duration: 0.3,
              type: 'spring',
              stiffness: 300,
            }}
          >
            <div
              className={`w-full h-full rounded-full border-4 ${
                isPlayer1
                  ? 'border-red-400 shadow-lg shadow-red-400/50'
                  : 'border-yellow-400 shadow-lg shadow-yellow-400/50'
              }`}
            />
          </motion.div>
        ))}
      </motion.div>

      {/* Victory particles */}
      <motion.div
        className="absolute inset-0 z-40 pointer-events-none"
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ delay: 0.5, duration: 0.5 }}
        onAnimationComplete={onComplete}
      >
        {Array.from({ length: 20 }).map((_, i) => (
          <motion.div
            key={`particle-${i}`}
            className={`absolute w-2 h-2 rounded-full ${
              isPlayer1 ? 'bg-red-400' : 'bg-yellow-400'
            }`}
            style={{
              left: `${Math.random() * 100}%`,
              top: `${Math.random() * 100}%`,
            }}
            initial={{ scale: 0, opacity: 0 }}
            animate={{
              scale: [0, 1, 0],
              opacity: [0, 1, 0],
              x: [0, (Math.random() - 0.5) * 100],
              y: [0, (Math.random() - 0.5) * 100],
            }}
            transition={{
              delay: 0.5 + i * 0.05,
              duration: 1.5,
              ease: 'easeOut',
            }}
          />
        ))}
      </motion.div>

      {/* Victory text */}
      <motion.div
        className="absolute inset-0 z-50 flex items-center justify-center pointer-events-none"
        initial={{ opacity: 0, scale: 0.8 }}
        animate={{ opacity: 1, scale: 1 }}
        transition={{ delay: 0.8, duration: 0.5 }}
      >
        <div className="text-center">
          <motion.h2
            className={`text-4xl font-bold mb-2 ${isPlayer1 ? 'text-red-400' : 'text-yellow-400'}`}
            animate={{
              textShadow: [
                '0 0 10px currentColor',
                '0 0 20px currentColor',
                '0 0 10px currentColor',
              ],
            }}
            transition={{ duration: 1, repeat: Infinity }}
          >
            {isPlayer1 ? 'RED WINS!' : 'YELLOW WINS!'}
          </motion.h2>
          <motion.p
            className="text-white/80 text-lg"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ delay: 1.2, duration: 0.5 }}
          >
            4 in a row!
          </motion.p>
        </div>
      </motion.div>
    </>
  );
}
