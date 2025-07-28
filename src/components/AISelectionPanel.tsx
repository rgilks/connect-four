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
    description: 'Traditional minimax algorithm with alpha-beta pruning. Depth 5 for strong play.',
    subtitle: 'Minimax + Alpha-Beta (Depth 5)',
    colorClass: 'text-green-400',
    borderColorClass: 'border-green-500/50',
    icon: '🧠',
  },
  {
    aiType: 'ml' as AIType,
    title: 'ML AI',
    description: 'Monte Carlo Tree Search with neural network evaluation. Advanced strategic play.',
    subtitle: 'MCTS + Neural Network',
    colorClass: 'text-blue-400',
    borderColorClass: 'border-blue-500/50',
    icon: '🤖',
  },
];

interface AISelectionPanelProps {
  onStartGame?: () => void;
}

export default function AISelectionPanel({ onStartGame }: AISelectionPanelProps) {
  const { actions } = useGameStore();

  const handleAISelection = (aiType: AIType) => {
    actions.setAI(aiType);
    actions.setGameMode('human-vs-ai');
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
        <p className="text-gray-300 text-lg">Click on an AI to start playing immediately</p>
      </div>

      <div className="grid gap-6 md:grid-cols-1 lg:grid-cols-2">
        {AI_OPTIONS.map(option => (
          <AISelectionCard
            key={option.aiType}
            aiType={option.aiType}
            title={option.title}
            description={option.description}
            subtitle={option.subtitle}
            colorClass={option.colorClass}
            borderColorClass={option.borderColorClass}
            isSelected={false}
            onClick={() => handleAISelection(option.aiType)}
            data-testid={`ai-selection-${option.aiType}`}
          />
        ))}
      </div>

      <div className="mt-8 text-center">
        <p className="text-gray-400 text-sm">Or watch them play against each other</p>
        <motion.button
          onClick={() => {
            actions.setAI('classic');
            actions.setGameMode('ai-vs-ai');
            actions.reset();
            onStartGame?.();
          }}
          className="mt-4 px-8 py-3 bg-purple-600 hover:bg-purple-700 text-white font-semibold rounded-lg transition-colors duration-200"
          whileHover={{ scale: 1.05 }}
          whileTap={{ scale: 0.95 }}
          data-testid="ai-vs-ai-button"
        >
          Watch AI vs AI
        </motion.button>
      </div>
    </motion.div>
  );
}
