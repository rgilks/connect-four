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
```

### Simple ML Training

Lightweight neural networks for fast, efficient AI:

```bash
# Train basic simple model (20 epochs, 500 games)
python3 ml/scripts/simple_train.py --epochs 20 --num-games 500

# Train enhanced simple model (50 epochs, 1000 games)
python3 ml/scripts/simple_train.py --epochs 50 --num-games 1000 --batch-size 64 --learning-rate 0.0005 --output simple_model_enhanced.json
```

## Current Status

### ✅ Successfully Trained Models (July 2025)

- **Simple Model**: 50 epochs, 1000 games, 64 batch size
- **Enhanced Model**: 50 epochs, 1000 games, 64 batch size, 0.0005 learning rate
- **Training Time**: ~2.5 seconds for enhanced model
- **Model Size**: 297KB (67x smaller than complex models)
- **Integration**: Successfully integrated with WASM AI system

### 🏆 AI Performance Rankings

Based on comprehensive testing:

1. **EMM-Depth1**: 81.8% average win rate (Best performance)
2. **EMM-Depth2**: 81.8% average win rate (Very fast)
3. **EMM-Depth3**: 65.9% average win rate (Fast)
4. **EMM-Depth5**: 63.6% average win rate (Moderate)
5. **EMM-Depth4**: 63.6% average win rate (Moderate)
6. **EMM-Depth6**: 56.8% average win rate (Strong but slow)
7. **Heuristic**: 56.8% average win rate (Educational)
8. **ML-Default**: 38.6% average win rate (Fast, lightweight)
9. **ML-PolicyFix**: 34.1% average win rate (Fast, lightweight)
10. **ML-Intensive**: 27.3% average win rate (Fast, lightweight)
11. **ML-SelfPlay**: 25.0% average win rate (Fast, lightweight)
12. **Random**: 4.5% average win rate (Baseline)

## Development

### Testing

```bash
# Run all tests
npm run test

# Run tests with coverage
npm run test:coverage

# Run E2E tests
npm run test:e2e

# Run AI comparison tests
npm run test:ai-comparison:fast
```

### Building

```bash
# Build for development
npm run build

# Build for production
npm run build:cf

# Build WASM assets
npm run build:wasm-assets
```

### Database

```bash
# Setup local database
npm run db:setup

# Run migrations
npm run db:migrate

# Database shell
npm run db:shell
```

## Deployment

The application is deployed to Cloudflare Pages with automatic deployments from the main branch.

```bash
# Deploy manually
npm run deploy

# Quick deploy
npm run deploy:quick
```

## Architecture

- **Frontend**: Next.js 15 with React, TypeScript, Tailwind CSS
- **AI Engine**: Rust compiled to WebAssembly for client-side execution
- **Database**: Cloudflare D1 (production), SQLite (development)
- **Deployment**: Cloudflare Workers with GitHub Actions CI/CD

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `npm run check`
5. Submit a pull request

## License

MIT License - see LICENSE file for details.
