# Connect Four

[![CI/CD](https://github.com/rgilks/rgou-cloudflare/actions/workflows/deploy.yml/badge.svg)](https://github.com/rgilks/rgou-cloudflare/actions/workflows/deploy.yml)

<div align="center">
 <img src="/docs/screenshot.png" alt="Connect Four Screenshot" width="408" />
  <br />
  <a href='https://ko-fi.com/N4N31DPNUS' target='_blank'><img height='36' style='border:0px;height:36px;' src='https://storage.ko-fi.com/cdn/kofi2.png?v=6' border='0' alt='Buy Me a Coffee at ko-fi.com' /></a>
  <hr />
</div>

## 🚧 **WORK IN PROGRESS - NOT YET FUNCTIONAL** 🚧

**This repository is currently being converted from the [Royal Game of Ur](https://github.com/rgilks/rgou-cloudflare) to Connect Four. The conversion is incomplete and the game is not yet playable.**

### Current Status

- ✅ **Frontend UI**: Updated to Connect Four terminology and interface
- ✅ **User-facing text**: All references changed from "Royal Game of Ur" to "Connect Four"
- ✅ **Documentation**: Updated to reflect Connect Four game
- ❌ **Core game logic**: Still implements Royal Game of Ur (21-square board, dice, piece movement)
- ❌ **AI system**: Still uses expectiminimax algorithm (designed for dice-based games)
- ❌ **Game mechanics**: Still uses piece-based movement instead of column-based dropping

### 🎯 **TODO List**

#### **Critical Backend Changes Needed:**

1. **Replace 21-square board with 6x7 Connect 4 board**
2. **Remove dice system entirely** (Connect 4 is deterministic)
3. **Replace piece movement with column-based dropping**
4. **Replace expectiminimax with minimax** (Connect 4 has no chance elements)
5. **Implement Connect 4 win detection** (4 in a row, not race to finish)
6. **Remove all track-based logic** (Royal Game of Ur specific)

#### **Frontend Updates Needed:**

1. **Replace `pieceMove()` sound with `pieceDrop()`**
2. **Update game logic to handle column selection instead of piece movement**
3. **Remove remaining Royal Game of Ur specific UI elements**

#### **Testing & Validation:**

1. **Update all tests** to reflect Connect 4 mechanics
2. **Rewrite AI tests** for minimax instead of expectiminimax
3. **Update e2e tests** for column-based gameplay

### **⚠️ Important Notes:**

- The frontend appears ready but will not work until the backend is converted
- The AI system needs complete rewriting for Connect 4
- This is a significant architectural change requiring major refactoring

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
- **[TODO.md](./docs/TODO.md)** – Project TODOs and improvements

## 🚀 Quick Start

### Prerequisites

- **Node.js 20+** ([Download](https://nodejs.org/)) - Required for Next.js 15
- **Rust & Cargo** ([Install](https://www.rust-lang.org/tools/install)) - For WebAssembly compilation
- **wasm-pack**: `cargo install wasm-pack --version 0.12.1 --locked` - For WASM builds

**Note**: This project was developed on an M1 Mac. While it should work on other platforms, some optimizations (especially for AI training) are specifically tuned for Apple Silicon.

### Development Setup

```bash
git clone https://github.com/rgilks/rgou-cloudflare.git
cd rgou-cloudflare
npm install
npm run db:setup
npm run build:wasm-assets
npm run dev
```

The game will open at http://localhost:3000

### First Run Notes

- The first run may take longer as it builds WebAssembly assets
- If you encounter WASM build issues, run: `npm run build:wasm-assets`

### Common Setup Issues

- **WASM Build Failures**: Ensure wasm-pack version 0.12.1 is installed
- **Database Issues**: Run `npm run db:setup` to set up local SQLite
- **Dependency Issues**: Try `npm run nuke` to reset the environment

See [DEVELOPMENT.md](./docs/DEVELOPMENT.md) for detailed solutions.

## 🤖 AI System

The project features a minimax-based AI opponent for Connect Four, running entirely in the browser via WebAssembly. See [ARCHITECTURE.md](./docs/ARCHITECTURE.md) for details.

## 🧠 AI Development

Train and improve the Connect Four AI with minimax and optional ML approaches:

```bash
# Run quick AI test
yarn run test:ai-quick

# Run standard AI test
yarn run test:ai
```

**Note**: ML training requires GPU acceleration (CUDA or Apple Metal). Rust training works on any system but is slower.

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
