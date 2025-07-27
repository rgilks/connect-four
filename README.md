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

The game features a sophisticated AI system with:

- **Classic AI**: Minimax with alpha-beta pruning (Rust/WASM) at depth 1 for optimal performance
- **Heuristic AI**: Fast win/block detection
- **Evolved Parameters**: AI behavior optimized through genetic algorithms (76.0% win rate, 0.0ms/move)
- **Multiple Depths**: Configurable search depth for different difficulty levels

### Genetic Algorithm Evolution

The AI parameters are evolved using genetic algorithms:

```bash
# Run genetic parameter evolution
npm run evolve:genetic-params

# Plot evolution results
python scripts/plot_evolution.py evolution_params_20241201_143022.csv
```

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
