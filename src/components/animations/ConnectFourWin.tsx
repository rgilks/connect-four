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
      {/* Winning pieces highlight - positioned exactly over game pieces */}
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
              left: `${(pos.column / 7) * 100 + 2.5}%`,
              top: `${(pos.row / 6) * 100 + 2.5}%`,
            }}
            initial={{ scale: 0.9, opacity: 0 }}
            animate={{
              scale: [0.9, 1.1, 1.05],
              opacity: [0, 1, 0.95],
            }}
            transition={{
              delay: index * 0.15,
              duration: 0.5,
              type: 'tween',
              ease: 'easeInOut',
            }}
          >
            {/* Simple, clear border highlight */}
            <motion.div
              className={`w-full h-full rounded-full border-4 ${
                isPlayer1
                  ? 'border-red-400 shadow-lg shadow-red-400/60'
                  : 'border-yellow-400 shadow-lg shadow-yellow-400/60'
              }`}
              animate={{
                boxShadow: [
                  isPlayer1
                    ? '0 0 15px rgba(239, 68, 68, 0.7), 0 0 30px rgba(239, 68, 68, 0.4)'
                    : '0 0 15px rgba(234, 179, 8, 0.7), 0 0 30px rgba(234, 179, 8, 0.4)',
                  isPlayer1
                    ? '0 0 25px rgba(239, 68, 68, 0.9), 0 0 50px rgba(239, 68, 68, 0.5)'
                    : '0 0 25px rgba(234, 179, 8, 0.9), 0 0 50px rgba(234, 179, 8, 0.5)',
                  isPlayer1
                    ? '0 0 15px rgba(239, 68, 68, 0.7), 0 0 30px rgba(239, 68, 68, 0.4)'
                    : '0 0 15px rgba(234, 179, 8, 0.7), 0 0 30px rgba(234, 179, 8, 0.4)',
                ],
              }}
              transition={{
                duration: 1.5,
                repeat: Infinity,
                delay: index * 0.1,
                type: 'tween',
                ease: 'easeInOut',
              }}
            />
          </motion.div>
        ))}
      </motion.div>

      {/* Victory particles emanating from winning pieces */}
      <motion.div
        className="absolute inset-0 z-40 pointer-events-none"
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ delay: 0.8, duration: 0.5 }}
      >
        {winningLine.map((pos, posIndex) =>
          Array.from({ length: 6 }).map((_, i) => (
            <motion.div
              key={`particle-${posIndex}-${i}`}
              className={`absolute w-2 h-2 rounded-full ${
                isPlayer1 ? 'bg-red-400' : 'bg-yellow-400'
              }`}
              style={{
                left: `${(pos.column / 7) * 100 + 12.5}%`,
                top: `${(pos.row / 6) * 100 + 12.5}%`,
              }}
              initial={{ scale: 0, opacity: 0, rotate: 0 }}
              animate={{
                scale: [0, 1.2, 0],
                opacity: [0, 1, 0],
                x: [0, (Math.random() - 0.5) * 100],
                y: [0, (Math.random() - 0.5) * 100],
                rotate: [0, 360],
              }}
              transition={{
                delay: 0.8 + posIndex * 0.1 + i * 0.08,
                duration: 1.2,
                ease: 'easeOut',
                type: 'tween',
              }}
            />
          ))
        )}
      </motion.div>

      {/* Victory text with enhanced effects */}
      <motion.div
        className="absolute inset-0 z-50 flex items-center justify-center pointer-events-none"
        initial={{ opacity: 0, scale: 0.8 }}
        animate={{ opacity: 1, scale: 1 }}
        transition={{ delay: 1.2, duration: 0.8, type: 'spring', stiffness: 200 }}
      >
        <div className="text-center">
          <motion.h2
            className={`text-5xl font-bold mb-4 ${isPlayer1 ? 'text-red-400' : 'text-yellow-400'}`}
            initial={{ y: 50, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            transition={{ delay: 1.4, duration: 0.6, type: 'spring' }}
          >
            <motion.span
              animate={{
                textShadow: [
                  isPlayer1
                    ? '0 0 10px rgba(239, 68, 68, 0.8), 0 0 20px rgba(239, 68, 68, 0.6)'
                    : '0 0 10px rgba(234, 179, 8, 0.8), 0 0 20px rgba(234, 179, 8, 0.6)',
                  isPlayer1
                    ? '0 0 20px rgba(239, 68, 68, 1), 0 0 40px rgba(239, 68, 68, 0.8), 0 0 60px rgba(239, 68, 68, 0.4)'
                    : '0 0 20px rgba(234, 179, 8, 1), 0 0 40px rgba(234, 179, 8, 0.8), 0 0 60px rgba(234, 179, 8, 0.4)',
                  isPlayer1
                    ? '0 0 10px rgba(239, 68, 68, 0.8), 0 0 20px rgba(239, 68, 68, 0.6)'
                    : '0 0 10px rgba(234, 179, 8, 0.8), 0 0 20px rgba(234, 179, 8, 0.6)',
                ],
                scale: [1, 1.05, 1],
              }}
              transition={{ duration: 2, repeat: Infinity, ease: 'easeInOut', type: 'tween' }}
            >
              {isPlayer1 ? 'RED WINS!' : 'YELLOW WINS!'}
            </motion.span>
          </motion.h2>

          <motion.div
            className="text-white/90 text-xl font-semibold"
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 1.8, duration: 0.6 }}
          >
            <motion.span
              animate={{
                textShadow: [
                  '0 0 10px rgba(255, 255, 255, 0.5)',
                  '0 0 20px rgba(255, 255, 255, 0.8)',
                  '0 0 10px rgba(255, 255, 255, 0.5)',
                ],
              }}
              transition={{ duration: 1.5, repeat: Infinity, type: 'tween', ease: 'easeInOut' }}
            >
              4 in a row! 🎉
            </motion.span>
          </motion.div>
        </div>
      </motion.div>

      {/* Background flash effect */}
      <motion.div
        className="absolute inset-0 z-20 pointer-events-none"
        initial={{ opacity: 0 }}
        animate={{ opacity: [0, 0.15, 0] }}
        transition={{ delay: 0.5, duration: 1, type: 'tween' }}
        style={{
          background: isPlayer1
            ? 'radial-gradient(circle, rgba(239, 68, 68, 0.15) 0%, transparent 70%)'
            : 'radial-gradient(circle, rgba(234, 179, 8, 0.15) 0%, transparent 70%)',
        }}
      />

      {/* Completion trigger */}
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 0 }}
        transition={{ delay: 3.5, duration: 0.5 }}
        onAnimationComplete={onComplete}
      />
    </>
  );
}
