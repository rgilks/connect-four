'use client';

import React, { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { cn } from '@/lib/utils';
import { GameState } from '@/lib/types';
import { Crown, Zap, Trophy, XCircle } from 'lucide-react';

interface GameStatusProps {
  gameState: GameState;
  aiThinking: boolean;
  gameMode?: 'human-vs-human' | 'human-vs-ai' | 'ai-vs-ai';
}

export default function GameStatus({
  gameState,
  aiThinking,
  gameMode = 'human-vs-ai',
}: GameStatusProps) {
  const [isMounted, setIsMounted] = useState(false);

  useEffect(() => {
    setIsMounted(true);
  }, []);

  const getStatusMessage = () => {
    if (gameState.gameStatus === 'not_started') {
      return {
        text: 'Select AI and start game',
        icon: Crown,
        color: 'text-gray-400',
      };
    }

    if (gameState.gameStatus === 'finished') {
      if (gameState.winner) {
        return {
          text: gameState.winner === 'player1' ? 'Red Wins!' : 'Yellow Wins!',
          icon: gameState.winner === 'player1' ? Trophy : Zap,
          color: gameState.winner === 'player1' ? 'text-red-400' : 'text-yellow-400',
        };
      } else {
        return {
          text: 'Draw!',
          icon: XCircle,
          color: 'text-gray-400',
        };
      }
    }

    if (gameState.currentPlayer === 'player1') {
      if (gameMode === 'ai-vs-ai') {
        return {
          text: aiThinking ? 'Red AI thinking...' : "Red AI's turn",
          icon: Crown,
          color: 'text-red-400',
        };
      }
      return {
        text: "Red's turn",
        icon: Crown,
        color: 'text-red-400',
      };
    } else {
      if (aiThinking) {
        return {
          text: gameMode === 'ai-vs-ai' ? 'Yellow AI thinking...' : 'Yellow thinking...',
          icon: Zap,
          color: 'text-yellow-400',
        };
      }
      return {
        text: gameMode === 'ai-vs-ai' ? "Yellow AI's turn" : "Yellow's turn",
        icon: Zap,
        color: 'text-yellow-400',
      };
    }
  };

  const status = getStatusMessage();
  const StatusIcon = status.icon;
  const isValidIcon =
    typeof StatusIcon === 'function' || (typeof StatusIcon === 'object' && StatusIcon !== null);

  // Don't render anything until mounted to prevent hydration mismatch
  if (!isMounted) {
    return (
      <div className="text-center mb-3">
        <div className="mt-2 h-10 flex flex-col justify-start relative pt-1">
          <div className="flex items-center justify-center space-x-2 h-6">
            <span className="font-bold text-lg text-gray-400">Loading...</span>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="text-center mb-3">
      <div className="mt-2 h-10 flex flex-col justify-start relative pt-1">
        <motion.div
          className="flex items-center justify-center space-x-2 h-6"
          animate={{ scale: aiThinking ? [1, 1.05, 1] : 1 }}
          transition={{ repeat: aiThinking ? Infinity : 0, duration: 1 }}
        >
          {isValidIcon ? (
            <StatusIcon className={cn('w-4 h-4', status.color)} data-testid="game-status-icon" />
          ) : null}
          <span
            className={cn('font-bold text-lg', status.color, 'neon-text')}
            data-testid="game-status-text"
          >
            {status.text}
          </span>
        </motion.div>

        <AnimatePresence>
          {aiThinking && (
            <motion.div
              className="absolute bottom-1 left-0 right-0 flex justify-center space-x-1"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
            >
              {[...Array(3)].map((_, i) => (
                <motion.div
                  key={i}
                  className="w-1.5 h-1.5 bg-yellow-400 rounded-full"
                  animate={{
                    y: [0, -6, 0],
                    opacity: [0.3, 1, 0.3],
                  }}
                  transition={{
                    repeat: Infinity,
                    duration: 0.8,
                    delay: i * 0.2,
                    ease: 'easeInOut',
                  }}
                />
              ))}
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </div>
  );
}
