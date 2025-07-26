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

- **AI Opponent**: Play against a minimax-based AI with adjustable difficulty
- **Browser-Native**: All AI runs locally via WebAssembly
- **Offline Support**: PWA with full offline gameplay
- **Modern UI**: Responsive design with animations and sound effects

## 📚 Documentation

- **[docs/README.md](./docs/README.md)** – Documentation index and quick reference
- **[ARCHITECTURE.md](./docs/ARCHITECTURE.md)** – System design, components, deployment, and infrastructure
- **[DEVELOPMENT.md](./docs/DEVELOPMENT.md)** – Development workflow, testing, troubleshooting, best practices
- **[GAME-GUIDE.md](./docs/GAME-GUIDE.md)** – Game rules, strategy, and user info
- **[CLOUDFLARE-DEPLOYMENT.md](./docs/CLOUDFLARE-DEPLOYMENT.md)** – Complete Cloudflare deployment guide
- **[TODO.md](./docs/TODO.md)** – Project TODOs and improvements

## 🚀 Quick Start

### Prerequisites

- **Node.js 20+** ([Download](https://nodejs.org/)) - Required for Next.js 15
- **Rust & Cargo** ([Install](https://www.rust-lang.org/tools/install)) - For WebAssembly compilation
- **wasm-pack**: `cargo install wasm-pack --version 0.12.1 --locked` - For WASM builds

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

For complete deployment documentation, see **[CLOUDFLARE-DEPLOYMENT.md](./docs/CLOUDFLARE-DEPLOYMENT.md)**.

### Common Setup Issues

- **WASM Build Failures**: Ensure wasm-pack version 0.12.1 is installed
- **Database Issues**: Run `npm run db:setup` to set up local SQLite
- **Dependency Issues**: Try `npm run nuke` to reset the environment

See [DEVELOPMENT.md](./docs/DEVELOPMENT.md) for detailed solutions.

## 🤖 AI System

The project features a minimax-based AI opponent for Connect Four, running entirely in the browser via WebAssembly. The AI system includes:

- **Minimax Algorithm**: Perfect for deterministic Connect Four
- **Alpha-Beta Pruning**: Optimized search performance
- **Genetic Algorithm Training**: Automated parameter optimization
- **Multiple Difficulty Levels**: From random to expert play
- **Transposition Tables**: Memory-efficient caching

See [ARCHITECTURE.md](./docs/ARCHITECTURE.md) for detailed system design.

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

- **[ARCHITECTURE.md](./docs/ARCHITECTURE.md)**
- **[DEVELOPMENT.md](./docs/DEVELOPMENT.md)**
- **[GAME-GUIDE.md](./docs/GAME-GUIDE.md)**

### Additional Files

- **[TODO.md](./docs/TODO.md)**

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
