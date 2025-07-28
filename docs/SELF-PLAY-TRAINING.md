# Self-Play Training System

This document describes the advanced self-play training system for the Connect Four AI, which uses Monte Carlo Tree Search (MCTS) and modern neural network architectures to create powerful AI models.

## Overview

The self-play training system combines several advanced techniques:

- **Monte Carlo Tree Search (MCTS)**: For intelligent exploration and move selection
- **Neural Networks**: Separate value and policy networks with modern architectures
- **Attention Mechanisms**: Multi-head attention for processing game state features
- **Residual Connections**: Deep networks with skip connections
- **Dirichlet Noise**: Prevents overfitting and encourages exploration
- **Curriculum Learning**: Progressive difficulty increase
- **Progressive Training**: Iterative improvement through multiple generations

## Architecture

### Neural Networks

The system uses two separate neural networks:

1. **Value Network**: Evaluates board positions and outputs a value between -1 and 1
2. **Policy Network**: Predicts move probabilities for each column (7 outputs)

Both networks can optionally use:

- **Attention Layers**: Multi-head attention for processing game state features
- **Residual Connections**: Skip connections for training deep networks

### MCTS Integration

MCTS is used during self-play to:

- Explore promising moves more thoroughly
- Balance exploration vs exploitation
- Generate high-quality training data
- Improve move selection beyond simple neural network predictions

## Quick Start

### Basic Self-Play Training

```bash
# Quick test (100 games, 10 epochs, 400 MCTS simulations)
npm run train:self-play:quick

# Standard training (1000 games, 50 epochs, 800 MCTS simulations)
npm run train:self-play

# Production training (2000 games, 100 epochs, 1200 MCTS simulations)
npm run train:self-play:production

# Advanced training with attention and residual connections
npm run train:self-play:advanced
```

### Custom Training

```bash
# Custom parameters
./ml/scripts/train_self_play.sh \
  --num-games 1500 \
  --epochs 75 \
  --batch-size 64 \
  --learning-rate 0.0005 \
  --mcts-simulations 1000 \
  --use-attention \
  --use-residual

# Custom output file
./ml/scripts/train_self_play.sh \
  --preset production \
  --output my_custom_model.json
```

## Configuration

### Training Parameters

| Parameter              | Default | Description                           |
| ---------------------- | ------- | ------------------------------------- |
| `num_games`            | 1000    | Number of self-play games to generate |
| `epochs`               | 50      | Number of training epochs             |
| `batch_size`           | 32      | Training batch size                   |
| `learning_rate`        | 0.001   | Learning rate for optimization        |
| `mcts_simulations`     | 800     | MCTS simulations per move             |
| `exploration_constant` | 1.0     | UCB exploration constant              |
| `temperature`          | 1.0     | Temperature for move selection        |
| `dirichlet_alpha`      | 0.3     | Dirichlet noise alpha parameter       |
| `dirichlet_epsilon`    | 0.25    | Dirichlet noise epsilon parameter     |

### Network Architecture

The default network architecture is defined in `ml/config/training.json`:

```json
{
  "network_architecture": {
    "input_size": 150,
    "hidden_sizes": [256, 128, 64, 32],
    "value_output_size": 1,
    "policy_output_size": 7
  }
}
```

### Presets

| Preset       | Games | Epochs | MCTS Sims | Description                  |
| ------------ | ----- | ------ | --------- | ---------------------------- |
| `quick`      | 100   | 10     | 400       | Fast testing and development |
| `default`    | 1000  | 50     | 800       | Standard training            |
| `production` | 2000  | 100    | 1200      | High-quality model training  |

## Training Process

### 1. Self-Play Data Generation

The system generates training data by:

1. **Game Simulation**: Playing games using MCTS + neural network
2. **Move Selection**: Using MCTS with Dirichlet noise for exploration
3. **Data Collection**: Recording board states, moves, and game results
4. **Value Assignment**: Backpropagating game results to all positions

### 2. Neural Network Training

The training process:

1. **Data Preparation**: Converting self-play data to PyTorch format
2. **Network Training**: Training value and policy networks simultaneously
3. **Validation**: Monitoring loss on validation set
4. **Early Stopping**: Stopping when validation loss stops improving
5. **Model Saving**: Saving the best model weights

### 3. Iterative Improvement

The system supports iterative training:

1. Train initial model with random weights
2. Generate self-play data using current model
3. Retrain model on new data
4. Repeat to improve performance

## Advanced Features

### Attention Mechanisms

Attention layers help the network focus on relevant parts of the game state:

```python
class AttentionLayer(nn.Module):
    def __init__(self, input_size: int, num_heads: int = 4):
        # Multi-head attention implementation
        pass
```

### Residual Connections

Residual connections help train deeper networks:

```python
class ResidualBlock(nn.Module):
    def forward(self, x):
        return x + self.layers(x)  # Skip connection
```

### MCTS Configuration

MCTS parameters can be tuned for different scenarios:

- **High simulations**: Better move quality, slower training
- **Low simulations**: Faster training, potentially lower quality
- **Exploration constant**: Balance between exploration and exploitation

## Performance Monitoring

### Training Metrics

The system tracks:

- **Value Loss**: How well the network evaluates positions
- **Policy Loss**: How well the network predicts moves
- **Total Loss**: Combined loss for both networks
- **Validation Loss**: Performance on held-out data

### Training Plots

Training history is automatically plotted and saved to `ml/data/weights/training_history.png`, showing:

- Loss curves over time
- Learning rate changes
- Validation performance

## Model Management

### Saving Models

Models are saved in JSON format with:

- Network weights
- Training metadata
- Architecture information
- Performance metrics

### Loading Models

Models can be loaded for:

- Continued training
- Evaluation
- Game play
- Model comparison

### Model Conversion

Models can be converted between formats:

```bash
# Convert to Rust format
python ml/scripts/convert_weights.py model.json --format rust

# Convert to PyTorch format
python ml/scripts/convert_weights.py model.json --format pytorch
```

## Troubleshooting

### Common Issues

**GPU Not Available**

```bash
# Check GPU availability
python3 -c "import torch; print(torch.cuda.is_available())"
python3 -c "import torch; print(torch.backends.mps.is_available())"
```

**Out of Memory**

```bash
# Reduce batch size
./ml/scripts/train_self_play.sh --batch-size 16

# Use smaller model
./ml/scripts/train_self_play.sh --preset quick
```

**Training Too Slow**

```bash
# Use fewer MCTS simulations
./ml/scripts/train_self_play.sh --mcts-simulations 400

# Use quick preset
./ml/scripts/train_self_play.sh --preset quick
```

### Performance Tips

1. **Use GPU**: PyTorch training is significantly faster with GPU
2. **Apple Silicon**: Native Metal backend provides excellent performance
3. **Batch Size**: Larger batch sizes are faster but use more memory
4. **MCTS Simulations**: More simulations = better quality but slower training

## Integration with Game

### Using Trained Models

Trained models are automatically:

1. Saved to `ml/data/weights/`
2. Copied to `public/ml/data/weights/` for web use
3. Available in the game interface

### Model Selection

The game can use different models:

- **Self-Play Models**: Advanced models trained with MCTS
- **Standard Models**: Models trained with traditional methods
- **Hybrid Models**: Combination of different approaches

## Future Enhancements

### Planned Features

- **Progressive Training**: Train against previous model versions
- **Ensemble Methods**: Combine multiple models for better performance
- **Advanced Architectures**: Transformer-based networks
- **Multi-GPU Training**: Distributed training across multiple GPUs
- **Automated Hyperparameter Tuning**: Bayesian optimization for parameters

### Research Directions

- **AlphaZero-style Training**: Pure self-play without human knowledge
- **Curriculum Learning**: Progressive difficulty increase
- **Meta-Learning**: Learning to learn new game variants
- **Explainability**: Understanding model decision-making

## Examples

### Complete Training Pipeline

```bash
# 1. Quick test
npm run train:self-play:quick

# 2. Standard training
npm run train:self-play

# 3. Advanced training
npm run train:self-play:advanced

# 4. Production training
npm run train:self-play:production
```

### Custom Training Session

```bash
# Train with custom parameters
./ml/scripts/train_self_play.sh \
  --num-games 3000 \
  --epochs 150 \
  --batch-size 128 \
  --learning-rate 0.0001 \
  --mcts-simulations 1600 \
  --use-attention \
  --use-residual \
  --output super_advanced_model.json
```

### Evaluation

```bash
# Test model performance
cd worker/rust_ai_core
cargo run --bin train evaluate 100
```

## Conclusion

The self-play training system provides a powerful framework for creating advanced Connect Four AI models. By combining MCTS exploration with modern neural network architectures, it can generate models that significantly outperform traditional approaches.

The system is designed to be:

- **Easy to use**: Simple commands for common tasks
- **Flexible**: Customizable parameters for different needs
- **Scalable**: Can train models of varying complexity
- **Robust**: Handles errors and provides helpful diagnostics

For questions or issues, please refer to the main project documentation or create an issue in the repository.
