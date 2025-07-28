# Connect Four

A modern, AI-powered implementation of the classic Connect Four game, built with Next.js, Rust/WASM, and advanced genetic algorithms.

## Features

- **Classic Gameplay**: Traditional Connect Four with modern UI
- **AI Opponents**: Multiple AI difficulty levels powered by Rust/WASM
- **Genetic Evolution**: AI parameters evolved through genetic algorithms
- **Parameter Tracking**: Comprehensive CSV logging and visualization of evolution
- **Offline-First**: Works completely offline once loaded
- **Mobile Optimized**: Responsive design for all devices
- **PWA Support**: Install as a native app

## Quick Start

```bash
# Install dependencies
npm install

# Setup database and build WASM
npm run db:setup
npm run build:wasm-assets

# Start development server
npm run dev
```

Visit [http://localhost:3000](http://localhost:3000) to play!

## AI System

The game features a sophisticated AI system with multiple opponents to choose from:

- **Classic AI**: Traditional minimax algorithm with alpha-beta pruning. Fast and reliable.
- **ML AI**: Neural network trained on genetic algorithm data. Balanced performance.
- **Self-Play AI**: Advanced neural network trained through self-play with MCTS exploration. Most sophisticated.

### AI Selection Interface

Players can select their preferred AI opponent from a beautiful card-based interface that shows:

- AI type and description
- Performance characteristics
- Visual indicators for selection

### Genetic Algorithm Evolution

The AI parameters are evolved using genetic algorithms:

```bash
# Run genetic parameter evolution
npm run evolve:genetic-params

# Plot evolution results
python scripts/plot_evolution.py evolution_params_20241201_143022.csv
```

### Self-Play Training

Advanced AI models are trained through self-play:

```bash
# Quick self-play training (100 games, 10 epochs)
npm run train:self-play:quick

# Standard self-play training (1000 games, 50 epochs)
npm run train:self-play

# Production self-play training (2000 games, 100 epochs)
npm run train:self-play:production

# Advanced training with attention and residual connections
npm run train:self-play:advanced

# Intensive 6-hour training session (5000 games, 200 epochs)
npm run train:intensive

# Monitor intensive training progress
npm run train:intensive:monitor
```

See [Self-Play Training Guide](./docs/SELF-PLAY-TRAINING.md) for detailed information.

This generates comprehensive CSV files tracking:

- Parameter evolution over generations
- Fitness and diversity metrics
- Convergence analysis
- Performance improvements

## Development

```bash
# Run all checks
npm run check

# Run tests
npm run test
npm run test:e2e

# Build for production
npm run build
```

See [DEVELOPMENT.md](./docs/DEVELOPMENT.md) for detailed development information.

## Architecture

- **Frontend**: Next.js 15 with React 19, TypeScript, Tailwind CSS
- **AI Engine**: Rust compiled to WebAssembly
- **Database**: SQLite (local) / Cloudflare D1 (production)
- **State Management**: Zustand with Immer
- **Testing**: Vitest (unit) + Playwright (e2e)

## Deployment

The game is deployed on Cloudflare Pages with automatic deployments from the main branch.

**Live Demo**: [https://connect-4.tre.systems](https://connect-4.tre.systems)

## Documentation

- [Development Guide](./docs/DEVELOPMENT.md) - Comprehensive development information
- [AI System](./docs/AI-SYSTEM.md) - Detailed AI system documentation
- [Architecture](./docs/ARCHITECTURE.md) - System architecture overview
- [Game Guide](./docs/GAME-GUIDE.md) - Game rules and strategy

## License

MIT License - see [LICENSE](./LICENSE) for details.
