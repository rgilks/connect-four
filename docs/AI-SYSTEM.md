# AI System Documentation

**✅ CURRENT STATE: WASM AI INTEGRATED**

The sophisticated Rust/WASM AI system has been successfully integrated and is now being used in the game. The game now uses the advanced WASM AI with fallback to JavaScript AI.

## Recent Improvements (27/07/2025)

### EMM AI Evaluation Function Fixed

- **Issue**: Critical bug in evaluation function causing inconsistent player perspective
- **Fix**: Removed incorrect score flipping based on current player
- **Result**: EMM AI now performs consistently across all depths
- **Performance**: EMM-Depth3 now balanced at 50% win rate (was 87% - too strong)
- **Scores**: Now reasonable (thousands instead of millions)

### Heuristic vs EMM Analysis

- **Key Finding**: Heuristic AI (57.1% win rate) outperforms EMM-Depth6 (50.0% win rate)
- **Reason**: Evaluation function quality is more important than search depth
- **Implication**: Strategic understanding can beat tactical calculation
- **Documentation**: See [EMM vs Heuristic Analysis](EMM-VS-HEURISTIC-ANALYSIS.md) for detailed explanation

### Technical Changes

- Evaluation function now consistently from Player1's perspective
- Added center control bonus to evaluation
- Improved minimax algorithm consistency
- Better depth progression (EMM-Depth1/2: 31.2%, EMM-Depth3: 50%)

## Current Implementation (WASM AI with JavaScript Fallback)

The game now uses the advanced WASM AI system with fallback to JavaScript AI:

- **Primary AI**: Rust/WASM Classic AI (minimax with alpha-beta pruning)
- **Fallback AI**: JavaScript heuristic (win/block detection)
- **Strategy**:
  - WASM AI: Advanced search with transposition tables, genetic parameters
  - JavaScript AI: Simple heuristic (win/block/center preference)
- **Performance**:
  - WASM AI: ~17ms per move, competitive play
  - JavaScript AI: ~1ms per move, basic play

**Note**: ML AI models are not yet trained and are excluded from comparison tests until training is completed.

## Available WASM AI Infrastructure (Now Integrated)

The codebase contains a complete Rust/WASM AI system that's now integrated:

- **Classic AI**: Minimax with alpha-beta pruning, transposition tables ✅
- **ML AI**: Neural networks with value/policy networks (available)
- **Genetic Parameters**: Evolved evaluation functions ✅
- **Training System**: Self-play training with GPU acceleration
- **Performance**: 60+ games/second, competitive with strong play ✅

## Integration Status

### ✅ Phase 1: Basic WASM Integration (COMPLETED)

1. **Import WASM module** in frontend ✅
2. **Replace JavaScript AI** with WASM Classic AI ✅
3. **Test integration** and performance ✅
4. **Update UI** to show AI thinking/analysis ✅

### 🔄 Phase 2: Advanced Features (IN PROGRESS)

1. **Add ML AI options** (PyTorch V5, ML-V2, etc.)
2. **Implement AI selection** in game settings
3. **Add move analysis** display
4. **Performance monitoring**

### 📋 Phase 3: Training Integration (PLANNED)

1. **In-browser training** capabilities
2. **Model management** UI
3. **Real-time AI improvement**

## Next Steps

1. **✅ Immediate**: Create WASM integration service (COMPLETED)
2. **✅ Short-term**: Replace JavaScript AI with WASM Classic AI (COMPLETED)
3. **🔄 Medium-term**: Add ML AI options and analysis
4. **📋 Long-term**: Full training system integration

---

This document describes the comprehensive AI system for Connect Four, including both Classic AI (minimax) and ML AI (neural network) implementations, performance analysis, testing strategies, and development history.

## Overview

The game features two distinct AI opponents, each with unique playstyles and architectures:

- **Classic AI**: Strategic opponent using minimax algorithm with alpha-beta pruning
- **ML AI**: Modern neural network AI trained through self-play with multiple model variants

Both AIs run locally in the browser via WebAssembly, providing instant responses without network latency.

## Classic AI

The Classic AI is the default and most robust opponent, using the minimax algorithm with alpha-beta pruning - designed for deterministic games with perfect information like Connect 4.

### Core Algorithm

- **Minimax**: For deterministic game states where players make choices
- **Expectation Nodes**: For chance-based events, calculating expected value based on probabilities
- **Alpha-Beta Pruning**: Powerful optimization that prunes search tree portions that cannot influence decisions

### Game Probabilities

| Roll | Probability |
| ---- | ----------- |
| 0    | 1/16        |
| 1    | 4/16        |
| 2    | 6/16        |
| 3    | 4/16        |
| 4    | 1/16        |

### Position Evaluation

The evaluation function now uses evolved genetic parameters that are loaded from `ml/data/genetic_params/evolved.json` and applied in real-time during move evaluation. These parameters were optimized through a genetic algorithm process (50 generations, 50 individuals per generation, 100 games per evaluation).

**Current Evolved Parameters (July 2025)**:

- `win_score`: 8354 (reduced from 10000)
- `position_weight`: 30 (increased from 15)
- `safety_bonus`: -13 (reduced from 25)
- `advancement_bonus`: 11 (increased from 5)
- `center_column_bonus`: 4 (increased from 2)
- `center_control_weight`: 1.0
- `piece_count_weight`: 0.5
- `threat_weight`: 2.0
- `mobility_weight`: 0.8
- `vertical_control_weight`: 1.2
- `horizontal_control_weight`: 1.0

**Implementation**: The evaluation function now dynamically loads these parameters from the evolved.json file and applies them to all evaluation components including center control, threat detection, piece count, mobility, and positional control.

**Performance**: Evolved parameters significantly outperform default parameters with 100% win rate in validation tests.

### Search Depth Optimization

Optimized for depth 5 search, providing the best performance/speed ratio:

- **Production**: Depth 5 search for optimal balance (93.8% win rate)
- **Maximum Strength**: Depth 7 search for strongest play
- **Fast Alternative**: Depth 3 search for instant speed
- **Testing**: Various depths for performance analysis

## ML AI

The ML AI offers a different challenge with playstyle developed from observing thousands of games.

### Architecture

- **Input**: 150-dimensional feature vector representing game state
- **Model**: Two neural networks sharing input:
  - Value network: predicts expected outcome
  - Policy network: predicts best move (probability distribution)
- **Output**: Move with highest combined score (value + policy + bonuses)

### Model Structure

- Input: 150 features
- Hidden: 256 → 128 → 64 → 32 (ReLU activation)
- Output: Value (1 neuron, tanh), Policy (7 neurons, softmax)

### Training System

**Pure Rust Architecture** with optimized CPU parallel processing:

1. **🦀 Rust Data Generation**: Fast parallel game simulation using all CPU cores
2. **⚡ CPU Training**: Efficient neural network training with custom implementation
3. **🍎 Apple Silicon Optimization**: Uses 8 performance cores on M1/M2/M3
4. **📊 Comprehensive Logging**: Detailed progress tracking and performance metrics

### Model Variants

| Model          | Training Games | Epochs | Status                    |
| -------------- | -------------- | ------ | ------------------------- |
| **PyTorch V5** | 2000           | 100    | ✅ **Latest Model**       |
| **ML-V2**      | 1000           | 50     | ✅ **Strong Performance** |
| **ML-Fast**    | 1000           | 50     | ✅ **Good Performance**   |
| **ML-V4**      | 5000           | 100    | ✅ **Good Performance**   |
| **ML-Hybrid**  | 1000           | 50     | ✅ **Hybrid Approach**    |

## Performance Analysis

> **For the latest, detailed AI performance results, see [AI-MATRIX-RESULTS.md](./AI-MATRIX-RESULTS.md).**

The AI matrix test provides comprehensive performance analysis including:

- Win rates across all AI combinations
- Speed analysis and recommendations
- Production recommendations
- Detailed performance metrics

## AI Testing Strategy

### Test Infrastructure

**Core Components**:

1. **AI Matrix Test** (`worker/rust_ai_core/tests/ai_matrix_test.rs`)
   - Comprehensive testing infrastructure
   - Unified player interface
   - Complete matrix of all AI vs all AI comparisons
   - Automated performance analysis and recommendations

2. **Test Runner Script** (`scripts/test-ai-comparison.sh`)
   - Unified test execution
   - Configurable test parameters
   - Automated result generation

### Test Categories

**Matrix Tests (Primary)**:

- Comprehensive AI Matrix: Every AI vs every other AI
- Performance Rankings: Win rates, speed analysis, recommendations
- Configurable Games: 5-100 games per match via NUM_GAMES environment variable

**Default Tests (50 games, EMM depths 1-6)**:

- Standard AI comparison with 50 games per match
- Includes Random, Heuristic, and EMM depths 1-6
- Balanced performance and speed testing

**Fast Tests (10 games)**:

- Quick validation with 10 games per match
- Basic functionality validation
- AI player trait verification

**Slow Tests (Optional)**:

- Depth 7+ minimax testing
- Comprehensive ML model evaluation
- Extended game simulations (100+ games)

### Running Tests

```bash
# Default test suite (50 games, EMM depths 1-6)
npm run test:ai-comparison

# Fast test suite (10 games)
npm run test:ai-comparison:fast

# Comprehensive test suite (100 games, includes depth 7+)
npm run test:ai-comparison:comprehensive

# Matrix test only
cd worker/rust_ai_core
cargo test test_ai_matrix -- --nocapture
```

### Test Results Format

The AI matrix test generates comprehensive results including:

- Win rate matrix for all AI combinations
- Performance rankings and recommendations
- Speed analysis and categorization
- Detailed configuration and timing information

### **1. ai_matrix_test.rs** (Most Important)

**Purpose**: Comprehensive AI comparison and performance evaluation

**What it tests**:

- Full matrix comparison of all AI types
- Performance rankings and win rate analysis
- Speed analysis with move timing
- Enhanced recommendations based on performance data

**Key Features**:

- **🍎 Apple Silicon Optimization**: Automatically detects M1/M2/M3 Macs and uses all 8 performance cores
- **🚀 Parallel Execution**: All AI match combinations run simultaneously using rayon
- **⚡ Performance**: 60+ games per second with full CPU utilization
- **📊 Real-time Progress**: Shows parallel match execution with core utilization

**Quick run**:

```bash
# Default (50 games per match, EMM depths 1-6) - Now runs in parallel!
cargo test test_ai_matrix -- --nocapture

# Fast test (10 games per match)
NUM_GAMES=10 cargo test test_ai_matrix -- --nocapture

# Comprehensive test (100 games per match, includes depth 7+)
NUM_GAMES=100 RUN_SLOW_TESTS=1 cargo test test_ai_matrix -- --nocapture
```

**Performance Improvements**:

- **Before**: Sequential execution using 1 core
- **After**: Parallel execution using all 8 performance cores
- **Speed**: 60+ games/second vs. ~10 games/second previously
- **CPU Utilization**: 100% of performance cores vs. ~12% previously

## Training System

### Data Generation

- **Method**: Self-play games with parallel processing
- **Features**: 150+ game state features
- **Targets**: Value function (win/loss prediction) and policy (move probabilities)

### Training Presets

**Quick Preset**:

- Games: 100
- Epochs: 10
- Batch Size: 32
- Use Case: Testing and development

**Default Preset**:

- Games: 1000
- Epochs: 50
- Batch Size: 32
- Use Case: Standard training runs

**Production Preset**:

- Games: 2000
- Epochs: 100
- Batch Size: 64
- Use Case: Final model training

### Backend Selection

**Auto (Default)**:

- Automatically selects best available backend
- PyTorch if GPU acceleration is available
- Rust if no GPU acceleration

**Rust**:

- CPU-based training
- Always available
- Slower but more reliable

**PyTorch**:

- GPU-accelerated training
- Requires CUDA or Apple Metal (MPS)
- Faster training when available

### Training Commands

```bash
# Quick development training
npm run train:quick

# Production PyTorch training
npm run train:pytorch:production

# Custom Rust training
npm run train:rust -- --num-games 500 --epochs 25
```

## Development History

### Key Milestones

**July 2025 - PyTorch V5 Breakthrough**:

- First ML model competitive with strongest classic AI (EMM-4)
- Strong performance vs EMM-4 with significant speed advantage
- Represents breakthrough in ML AI development

**July 2025 - Pure Rust Training Migration**:

- Complete migration from Python to Rust with custom neural network
- 10-20x faster training with Apple Silicon optimization
- Eliminated all Python dependencies

**2024 - Genetic Parameter Evolution**:

- Evolved genetic parameters through 50 generations
- Significant improvement over default parameters
- All classic AI now uses evolved parameters by default

### Lessons Learned

1. **Training Data Quality > Quantity**: v2 model (1,000 games) outperforms newer models (5,000+ games)
2. **Validation Loss ≠ Competitive Performance**: Models with excellent validation can perform poorly in competition
3. **Simpler Architectures Can Be Better**: v2 model's success with simple architecture
4. **Pure Rust Provides Significant Benefits**: 10-20x performance improvements
5. **Apple Silicon Optimization is Critical**: Native Metal backend provides massive benefits

## Implementation Details

### Core Files

- **Classic AI Core**: `worker/rust_ai_core/src/lib.rs`
- **WASM Interface**: `worker/rust_ai_core/src/wasm_api.rs`
- **Frontend Integration**: `src/lib/wasm-ai-service.ts`
- **ML AI Service**: `src/lib/ml-ai-service.ts`
- **Training System**: `ml/scripts/train.sh`

### Genetic Parameter Evolution

```bash
# Run evolution
cd worker/rust_ai_core
cargo run --release --bin evolve_params

# Validate results
cargo test test_genetic_params_comparison -- --nocapture
```

## Future Directions

### Short Term (Next 3 Months)

- Investigate ONNX and 'trace' for ML AI
- Optimize neural network architecture
- Implement GPU training acceleration with Rust

### Medium Term (Next 6 Months)

- Add self-play reinforcement learning
- Implement Monte Carlo Tree Search on top of neural network
- Optimize feature engineering (review 150 features)

### Long Term (Next Year)

- Add multiplayer support
- Create mobile app version
- Implement continuous AI improvement

## Summary

The AI system provides a comprehensive suite of opponents ranging from educational baselines to competitive neural networks. The Classic AI offers reliable, strong play while the ML AI demonstrates the potential of modern machine learning approaches. Both systems run efficiently in the browser, providing instant responses and enabling true offline play.

**Current Recommendations**:

- **Production**: EMM-3 (Depth 3) for best overall performance (see [AI-MATRIX-RESULTS.md](./AI-MATRIX-RESULTS.md))
- **ML Research**: PyTorch V5 and ML-Hybrid for advanced AI development
- **Educational**: Heuristic AI for understanding game strategy
- **Baseline**: Random AI for performance comparisons

> **Note:** All AI performance stats, win rates, and timing data in this document are generated by the automated AI matrix test and saved to [AI-MATRIX-RESULTS.md](./AI-MATRIX-RESULTS.md). For the latest results, see that file.
