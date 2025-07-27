# Connect Four Game

A modern, responsive Connect Four game built with Next.js, TypeScript, and Rust WASM AI.

## Features

- 🎮 **Classic Connect Four gameplay** with smooth animations
- 🤖 **Intelligent AI opponents** using Rust WASM with multiple algorithms:
  - ExpectiMinimax with configurable depth (1-6)
  - Heuristic-based AI
  - Random AI for testing
- 🧠 **Machine Learning AI** with neural networks (PyTorch backend)
- 🎯 **Genetic Algorithm optimization** for AI parameters
- 📊 **Comprehensive AI testing** with performance matrix
- 🎨 **Beautiful UI** with animations and sound effects
- 📱 **Progressive Web App** with offline support
- 🗄️ **Database integration** for game history
- 🧪 **Full test coverage** with unit, integration, and e2e tests

## AI Performance

The AI system includes multiple algorithms with different performance characteristics:

| AI Type    | Win Rate | Speed | Use Case                 |
| ---------- | -------- | ----- | ------------------------ |
| EMM-Depth1 | 92.9%    | 0.0ms | Production (recommended) |
| EMM-Depth2 | 71.4%    | 0.0ms | Fast gameplay            |
| EMM-Depth3 | 32.1%    | 3.7ms | Balanced performance     |
| Heuristic  | 46.4%    | 0.0ms | Educational              |
| Random     | 7.1%     | 0.0ms | Testing baseline         |

## Quick Start

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Run all tests
npm run check

# Build for production
npm run build
```

## AI Development

The AI system is built in Rust and compiled to WebAssembly for optimal performance:

```bash
# Build Rust AI
npm run build:wasm

# Test AI performance
npm run test:ai-comparison

# Train genetic parameters
npm run evolve:genetic-params

# Train ML models
npm run train:rust
```

## Testing

- **Unit Tests**: `npm test`
- **E2E Tests**: `npm run test:e2e`
- **AI Matrix**: `npm run test:ai-comparison`
- **Full Check**: `npm run check`

## Deployment

```bash
# Deploy to Cloudflare Pages
npm run deploy

# Quick deploy
npm run deploy:quick
```

## Architecture

- **Frontend**: Next.js 15 with TypeScript
- **AI Engine**: Rust compiled to WebAssembly
- **Database**: Drizzle ORM with SQLite/Cloudflare D1
- **Testing**: Vitest + Playwright
- **Deployment**: Cloudflare Pages

## Recent Updates

- ✅ **Fixed AI win detection** - AI now correctly identifies and makes winning moves
- ✅ **Improved serialization** - Fixed Player enum serialization for proper game state conversion
- ✅ **Enhanced performance** - Optimized AI algorithms and reduced move evaluation time
- ✅ **Better testing** - Comprehensive AI matrix testing with performance analysis

## License

MIT License - see LICENSE file for details.
