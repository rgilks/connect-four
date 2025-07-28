'use client';

import React from 'react';
import { motion } from 'framer-motion';
import { useGameStore } from '../lib/game-store';
import AISelectionCard from './AISelectionCard';
import type { AIType } from '../lib/types';

const AI_OPTIONS = [
  {
    aiType: 'classic' as AIType,
    title: 'Classic AI',
    description: 'Traditional minimax algorithm with alpha-beta pruning. Fast and reliable.',
    subtitle: 'Minimax + Alpha-Beta',
    colorClass: 'text-green-400',
    borderColorClass: 'border-green-500/50',
    icon: '🧠',
  },
  {
    aiType: 'ml' as AIType,
    title: 'ML AI',
    description: 'Neural network trained on genetic algorithm data. Balanced performance.',
    subtitle: 'Neural Network',
    colorClass: 'text-blue-400',
    borderColorClass: 'border-blue-500/50',
    icon: '🤖',
  },
  {
    aiType: 'self-play' as AIType,
    title: 'Self-Play AI',
    description: 'Advanced neural network trained through self-play with MCTS. Most sophisticated.',
    subtitle: 'Self-Play + MCTS',
    colorClass: 'text-purple-400',
    borderColorClass: 'border-purple-500/50',
    icon: '🎯',
  },
];

interface AISelectionPanelProps {
  onStartGame?: () => void;
}

export default function AISelectionPanel({ onStartGame }: AISelectionPanelProps) {
  const { selectedAI, actions } = useGameStore();

  const handleAISelection = (aiType: AIType) => {
    actions.setAI(aiType);
  };

  const handleStartGame = () => {
    actions.reset();
    onStartGame?.();
  };

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.5 }}
      className="w-full max-w-4xl mx-auto p-6"
    >
      <div className="text-center mb-8">
        <h2 className="text-3xl font-bold text-white mb-4">Choose Your AI Opponent</h2>
        <p className="text-gray-300 text-lg">
          Select from our advanced AI systems, each with different strengths and strategies
        </p>
      </div>

      <div className="grid gap-6 md:grid-cols-1 lg:grid-cols-3">
        {AI_OPTIONS.map(option => (
          <AISelectionCard
            key={option.aiType}
            aiType={option.aiType}
            title={option.title}
            description={option.description}
            subtitle={option.subtitle}
            colorClass={option.colorClass}
            borderColorClass={option.borderColorClass}
            isSelected={selectedAI === option.aiType}
            onClick={() => handleAISelection(option.aiType)}
            data-testid={`ai-selection-${option.aiType}`}
          />
        ))}
      </div>

      <div className="mt-8 text-center">
        <motion.button
          onClick={handleStartGame}
          className="px-8 py-3 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded-lg transition-colors duration-200"
          whileHover={{ scale: 1.05 }}
          whileTap={{ scale: 0.95 }}
          data-testid="start-game-button"
        >
          Start Game with {AI_OPTIONS.find(opt => opt.aiType === selectedAI)?.title}
        </motion.button>
      </div>

      <div className="mt-6 text-center text-sm text-gray-400">
        <p>
          Current selection:{' '}
          <span className="text-white font-semibold">
            {AI_OPTIONS.find(opt => opt.aiType === selectedAI)?.title}
          </span>
        </p>
      </div>
    </motion.div>
  );
}
