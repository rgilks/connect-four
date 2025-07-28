# ML Directory

This directory contains all machine learning related components for the Connect Four AI system.

## 🚀 Quick Start

### For Newcomers

If you're new to the project and want to try ML training:

```bash
# 1. Install Python dependencies
pip install -r requirements.txt

# 2. Quick test training (5 minutes)
npm run train:quick

# 3. Standard training (30 minutes)
npm run train:genetic:pytorch

# 4. Check results
ls ml/data/weights/
```

### Prerequisites

- **Python 3.8+** with pip
- **Rust & Cargo** (for data generation)
- **GPU** (recommended for PyTorch training)
  - **Apple Silicon**: Apple Metal (MPS) support
  - **NVIDIA**: CUDA support
  - **CPU-only**: Use Rust backend instead

## 🍎 Performance Optimizations

### Intelligent CPU Optimization

The ML system automatically detects your system architecture and optimizes CPU utilization:

- **Apple Silicon (M1/M2/M3)**: Uses all 8 performance cores, leaves efficiency cores for system tasks
- **High-core systems (16+)**: Uses most cores but leaves 2 for system responsiveness
- **Standard systems**: Uses all available cores for maximum performance

### GPU Acceleration

- **PyTorch training**: **REQUIRES** GPU acceleration (CUDA or Apple Metal)
- **Rust training**: Uses optimized CPU parallelization
- **Auto-detection**: Automatically selects the best backend for your system

## 📁 Structure

```
ml/
├── README.md              # This file
├── config/                # Unified configuration
│   └── training.json      # Training parameters and network architecture
├── scripts/               # Training scripts
│   ├── train.py          # Unified training script (Python/Rust backends)
│   ├── train.sh          # Shell wrapper with caffeinate
│   ├── convert_weights.py # Unified weight conversion utility
│   ├── train_pytorch.py   # PyTorch backend (used by train.py)
│   └── load_pytorch_weights.py # Legacy weight loader (deprecated)
├── data/                  # Training data and configuration
│   ├── weights/           # Trained model weights
│   └── genetic_params/    # Genetic algorithm parameters
```

## 🧠 Training System

The ML training system has been consolidated into a single, unified interface that supports both Rust and PyTorch backends, plus advanced self-play training with MCTS:

### Quick Start Commands

```bash
# Auto-detect best backend and use default settings
npm run train

# Quick test training (100 games, 10 epochs)
npm run train:quick

# Standard training (1000 games, 50 epochs)
npm run train:genetic:pytorch

# Production training (2000 games, 100 epochs)
npm run train:genetic:pytorch:production

# Advanced self-play training with MCTS
npm run train:self-play

# Quick self-play training (100 games, 10 epochs, 400 MCTS simulations)
npm run train:self-play:quick

# Production self-play training (2000 games, 100 epochs, 1200 MCTS simulations)
npm run train:self-play:production

# Advanced self-play with attention and residual connections
npm run train:self-play:advanced
```

### Advanced Usage

```bash
# Custom training with specific parameters
./ml/scripts/train.sh --backend pytorch --num-games 1500 --epochs 75

# Quick test with Rust backend
./ml/scripts/train.sh --backend rust --preset quick

# Production training with custom output
./ml/scripts/train.sh --preset production --output my_weights.json

# Advanced self-play training with custom parameters
./ml/scripts/train_self_play.sh --num-games 1500 --epochs 75 --mcts-simulations 1000

# Self-play with attention and residual connections
./ml/scripts/train_self_play.sh --use-attention --use-residual --preset production
```

### Training Presets

- **default**: 1000 games, 50 epochs, 32 batch size
- **quick**: 100 games, 10 epochs, 32 batch size
- **production**: 2000 games, 100 epochs, 64 batch size

### Backend Selection

- **auto**: Automatically selects PyTorch (if GPU available) or Rust
- **pytorch**: Uses PyTorch with **required** GPU acceleration (CUDA/MPS)
- **rust**: Uses pure Rust implementation with optimized CPU parallelization
- **self-play**: Advanced self-play training with MCTS exploration and PyTorch neural networks

### Self-Play Training Features

The new self-play training system includes:

- **Monte Carlo Tree Search (MCTS)**: Advanced exploration for better move selection
- **Dirichlet Noise**: Prevents overfitting and encourages exploration
- **Attention Layers**: Multi-head attention for processing game state features
- **Residual Connections**: Deep networks with skip connections
- **Curriculum Learning**: Progressive difficulty increase
- **Progressive Training**: Iterative improvement through multiple generations
- **Advanced Neural Networks**: Separate value and policy networks with modern architectures

### Performance Characteristics

| Backend | CPU Usage             | GPU Usage    | Best For                                |
| ------- | --------------------- | ------------ | --------------------------------------- |
| PyTorch | 1 core + GPU          | **Required** | High-performance training with GPU      |
| Rust    | All performance cores | None         | CPU-optimized training, no GPU required |

## 📊 Model Management

### Converting Weights

```bash
# Convert weights to unified format and copy to public directory
npm run load:ml-weights ml/data/weights/my_weights.json --copy-to-public

# Convert between formats
python3 ml/scripts/convert_weights.py input.json --format rust --output rust_weights.json
```

### Current Models

| Model            | Training Games | Epochs | MCTS Sims | Features                  | Status                    |
| ---------------- | -------------- | ------ | --------- | ------------------------- | ------------------------- |
| **Self-Play V1** | 1000           | 50     | 800       | MCTS, Attention, Residual | 🚧 **In Development**     |
| **PyTorch V5**   | 2000           | 100    | -         | Standard                  | ✅ **Latest Model**       |
| **ML-V2**        | 1000           | 50     | -         | Standard                  | ✅ **Strong Performance** |
| **ML-Fast**      | 1000           | 50     | -         | Standard                  | ✅ **Good Performance**   |
| **ML-V4**        | 5000           | 100    | -         | Standard                  | ✅ **Good Performance**   |
| **ML-Hybrid**    | 1000           | 50     | -         | Standard                  | ✅ **Hybrid Approach**    |

For detailed performance metrics, see [AI-MATRIX-RESULTS.md](../docs/AI-MATRIX-RESULTS.md).

## 🧬 Genetic Parameter Evolution

You can evolve and validate the genetic parameters for the classic AI:

```bash
# Evolve new genetic parameters
npm run evolve:genetic-params

# Validate evolved parameters
npm run validate:genetic-params
```

### Evolution Process

- **Population size:** 50 individuals
- **Generations:** 50 generations
- **Games per evaluation:** 100 games per individual
- **Evolution time:** ~42 minutes
- **Quality threshold:** Only saves parameters if they significantly outperform defaults

### Current Results

**Evolved Parameters Performance:**

- **Significant improvement** over default parameters
- **Validation confirmed:** 1000-game test confirms improvement

For detailed performance metrics, see [AI-MATRIX-RESULTS.md](../docs/AI-MATRIX-RESULTS.md).

## 🔧 Troubleshooting

### Common Issues

**GPU Not Found:**

```bash
# Check if PyTorch can see your GPU
python3 -c "import torch; print(torch.cuda.is_available())"
python3 -c "import torch; print(torch.backends.mps.is_available())"
```

**Training Too Slow:**

```bash
# Use Rust backend for CPU-only training
npm run train:genetic:quick

# Or reduce training parameters
./ml/scripts/train.sh --num-games 100 --epochs 10
```

**Out of Memory:**

```bash
# Reduce batch size
./ml/scripts/train.sh --batch-size 16

# Use smaller model
./ml/scripts/train.sh --preset quick
```

### Performance Tips

1. **Use GPU**: PyTorch training is 10-50x faster with GPU
2. **Apple Silicon**: Native Metal backend provides excellent performance
3. **Batch Size**: Larger batch sizes are faster but use more memory
4. **Games vs Epochs**: More games generally better than more epochs

## 📚 Further Reading

See [AI-SYSTEM.md](../docs/AI-SYSTEM.md) for detailed usage instructions and technical details about the AI system.
