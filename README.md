# Connect Four

[![CI/CD](https://github.com/rgilks/connect-four/actions/workflows/deploy.yml/badge.svg)](https://github.com/rgilks/connect-four/actions/workflows/deploy.yml)

<div align="center">
 <img src="/docs/screenshot.png" alt="Connect Four Screenshot" width="415" />
  <br />
  <a href='https://ko-fi.com/N4N31DPNUS' target='_blank'><img height='36' style='border:0px;height:36px;' src='https://storage.ko-fi.com/cdn/kofi2.png?v=6' border='0' alt='Buy Me a Coffee at ko-fi.com' /></a>
  <hr />
</div>

## ✅ **CONVERSION COMPLETE - FULLY FUNCTIONAL** ✅

**A modern web implementation of the classic Connect Four game with AI opponents, offline support, and beautiful animations.**

### ✅ **Conversion Status - COMPLETE**

- ✅ **Frontend UI**: Updated to Connect Four terminology and interface
- ✅ **User-facing text**: All references updated for Connect Four
- ✅ **Documentation**: Updated to reflect Connect Four game
- ✅ **Core game logic**: Implemented Connect Four (6x7 board, column-based dropping)
- ✅ **AI system**: Converted to minimax algorithm (perfect for deterministic Connect Four)
- ✅ **Game mechanics**: Column-based dropping with proper win detection
- ✅ **Rust AI core**: Fully converted with genetic algorithm training
- ✅ **Tests**: All backend tests passing

### 🎯 **Key Features Implemented**

#### **Connect Four Game Logic:**

- 6x7 board with column-based piece dropping
- Win detection for 4-in-a-row (horizontal, vertical, diagonal)
- Draw detection when board is full
- Proper turn management

#### **AI System:**

- Minimax algorithm with alpha-beta pruning
- Genetic algorithm for parameter optimization
- Multiple AI difficulty levels
- Transposition table for performance
- Heuristic evaluation functions

#### **Training & Evolution:**

- Genetic algorithm training for AI parameters
- Performance evaluation and comparison
- Automated testing and benchmarking
- Parameter optimization for different strategies

---

A modern web implementation of the classic Connect Four game with AI opponents, offline support, and beautiful animations. Built with Next.js, TypeScript, Rust, and WebAssembly.

## 🎮 Play Now

**[Play Online](https://connect-4.tre.systems/)** - Works in any modern browser, no installation required.

## ✨ Features

- **AI Opponent**: Play against a depth-5 minimax AI with 83.1% win rate (evolved genetic parameters)
- **Browser-Native**: All AI runs locally via WebAssembly
- **Offline Support**: PWA with full offline gameplay
- **Modern UI**: Responsive design with animations and sound effects
- **Advanced AI**: Genetic algorithm optimized evaluation functions for superior performance
- **User-Friendly Error Handling**: Graceful error modals instead of browser alerts for better UX

## 📚 Documentation

- **[docs/README.md](./docs/README.md)** – Documentation index and quick reference
- **[DEVELOPMENT.md](./docs/DEVELOPMENT.md)** – Complete development guide, testing, troubleshooting
- **[AI-SYSTEM.md](./docs/AI-SYSTEM.md)** – AI architecture, training, performance analysis
- **[DEPLOYMENT.md](./docs/DEPLOYMENT.md)** – Cloudflare deployment, monitoring, troubleshooting
- **[GAME-GUIDE.md](./docs/GAME-GUIDE.md)** – Game rules, strategy, AI opponents
- **[ARCHITECTURE.md](./docs/ARCHITECTURE.md)** – System design, data flow, infrastructure
- **[TODO.md](./docs/TODO.md)** – Current tasks and future plans

## 🚀 Quick Start

### Prerequisites

- **Node.js 20+** ([Download](https://nodejs.org/)) - Required for Next.js 15
- **Rust & Cargo** ([Install](https://www.rust-lang.org/tools/install)) - For WebAssembly compilation
- **wasm-pack**: `cargo install wasm-pack --version 0.13.1 --locked` - For WASM builds

**Note**: This project was developed on an M1 Mac. While it should work on other platforms, some optimizations (especially for AI training) are specifically tuned for Apple Silicon.

### Development Setup

```bash
git clone https://github.com/rgilks/connect-four.git
cd connect-four
npm install
npm run db:setup
npm run build:wasm-assets
npm run dev
```

The game will open at http://localhost:3000

### First Run Notes

- The first run may take longer as it builds WebAssembly assets
- If you encounter WASM build issues, run: `npm run build:wasm-assets`

## 🚀 Deployment

### Cloudflare Deployment

Your Connect Four application is configured for deployment on Cloudflare Workers with D1 Database.

#### Quick Deploy

```bash
# One-command deployment
npm run deploy

# Or use the script directly
./scripts/deploy.sh
```

#### Manual Deployment

```bash
# Build for Cloudflare
npm run build:cf

# Run database migrations
npm run db:migrate

# Deploy to Cloudflare
wrangler deploy
```

#### Useful Commands

```bash
# View deployment logs
npm run logs

# Database shell
npm run db:shell

# Check deployment status
wrangler status
```

For complete deployment documentation, see **[DEPLOYMENT.md](./docs/DEPLOYMENT.md)**.

### Common Setup Issues

- **WASM Build Failures**: Ensure wasm-pack version 0.12.1 is installed
- **Database Issues**: Run `npm run db:setup` to set up local SQLite
- **Dependency Issues**: Try `npm run nuke` to reset the environment

See [DEVELOPMENT.md](./docs/DEVELOPMENT.md) for detailed solutions.

## 🤖 AI System

**✅ Current State**: The game now uses the sophisticated Rust/WASM AI system with fallback to JavaScript AI.

### Primary AI (WASM)

- **Algorithm**: Minimax with alpha-beta pruning, transposition tables
- **Strategy**: Advanced search with genetic parameters, evolved evaluation
- **Performance**: ~17ms per move, competitive play

### Fallback AI (JavaScript)

- **Algorithm**: Basic heuristic with win/block detection
- **Strategy**: Look for wins, block opponent wins, prefer center columns
- **Performance**: ~1ms per move, basic play

### Integration Status

The WASM AI system has been successfully integrated. See [AI-SYSTEM.md](./docs/AI-SYSTEM.md) for detailed documentation.

See [AI-SYSTEM.md](./docs/AI-SYSTEM.md) for detailed system documentation.

## 🧠 AI Development

Train and improve the Connect Four AI with genetic algorithms:

```bash
# Run genetic algorithm training
cd worker/rust_ai_core
cargo run --bin train train 50 100 0.1 0.2 0.7 10

# Evaluate AI performance
cargo run --bin train evaluate 100

# Run AI comparison tests
cargo test test_ai_matrix -- --nocapture
```

**Note**: The genetic algorithm training optimizes AI parameters for Connect Four strategy.

## 🧪 Testing

The project includes comprehensive testing:

```bash
npm run check
npm run test
npm run test:e2e
```

See [DEVELOPMENT.md](./docs/DEVELOPMENT.md) for detailed testing information.

## 📋 Available Scripts

The project includes a comprehensive set of npm scripts for development, testing, and deployment. For a complete reference with detailed explanations, see **[DEVELOPMENT.md](./docs/DEVELOPMENT.md)**.

### 🚀 Quick Start Commands

```bash
npm run dev
npm run build
npm run check
npm run nuke
```

### 🧠 AI Development Commands

```bash
npm run test:ai-quick
npm run test:ai
```

### 🏗️ Build Commands

```bash
npm run build:wasm-assets
npm run build:cf
npm run generate:sw
```

### 🗄️ Database Commands

```bash
npm run db:setup
npm run migrate:local
npm run migrate:d1
```

### 🧪 Testing Commands

```bash
npm run test
npm run test:e2e
npm run test:coverage
```

See **[DEVELOPMENT.md](./docs/DEVELOPMENT.md)** for the complete list with detailed explanations and usage examples.

## 🏗️ Architecture

The project is a pure client-side implementation for optimal performance and offline capability. See [ARCHITECTURE.md](./docs/ARCHITECTURE.md) for detailed system design.

### Key Components

- **Frontend**: Next.js with React, TypeScript, Tailwind CSS
- **AI Engine**: Rust compiled to WebAssembly
- **Database**: Cloudflare D1 (production), SQLite (development)
- **Deployment**: Cloudflare Pages with GitHub Actions

## 📚 Documentation

### Core Documentation

- **[DEVELOPMENT.md](./docs/DEVELOPMENT.md)** – Complete development guide
- **[AI-SYSTEM.md](./docs/AI-SYSTEM.md)** – AI architecture and training
- **[DEPLOYMENT.md](./docs/DEPLOYMENT.md)** – Cloudflare deployment guide
- **[GAME-GUIDE.md](./docs/GAME-GUIDE.md)** – Game rules and strategy

### Reference Documentation

- **[ARCHITECTURE.md](./docs/ARCHITECTURE.md)** – System design and infrastructure
- **[TODO.md](./docs/TODO.md)** – Current tasks and roadmap

## 🔧 Troubleshooting

### Common Issues

- **WASM Build Failures**: Run `npm run build:wasm-assets`
- **Database Issues**: Run `npm run db:setup`
- **Dependency Issues**: Try `npm run nuke`
- **Deployment**: Pin exact dependency versions for Cloudflare compatibility

See [DEVELOPMENT.md](./docs/DEVELOPMENT.md) for detailed solutions.

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Contributions are welcome! Please read the documentation and ensure all tests pass before submitting a pull request.

## 🙏 Acknowledgments

- **Connect Four Community** - For strategy and analysis resources
- **Rust Community** - Excellent WebAssembly tooling and ecosystem

### Current Results (July 2025)

**Evolved Parameters Performance:**

- **Significant improvement** over default parameters
- **Evolution time:** ~42 minutes
- **Validation confirmed:** 1000-game test confirms improvement
- **Implementation:** Parameters are now actively used in the evaluation function

For detailed performance metrics, see [AI-MATRIX-RESULTS.md](./AI-MATRIX-RESULTS.md).

**Key Parameter Changes:**

- `win_score`: 10000 → 8354 (-1646)
- `position_weight`: 15 → 30 (+15)
- `safety_bonus`: 25 → -13 (-38)
- `advancement_bonus`: 5 → 11 (+6)
- `center_column_bonus`: 2 → 4 (+2)

The evolved parameters significantly outperform the defaults and are now actively used in production AI evaluation.
