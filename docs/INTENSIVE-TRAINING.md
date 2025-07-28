# Intensive Training Guide

This guide covers the intensive training script for creating high-performance AI models through extended self-play training sessions.

## Overview

The intensive training script (`npm run train:intensive`) is designed for creating the best possible AI models by running extended training sessions with enhanced parameters. It's optimized for 6-hour training runs with caffeinate to prevent system sleep.

## Training Configuration

### Parameters

- **Games**: 5,000 self-play games (5x default)
- **Epochs**: 200 training epochs (4x default)
- **Batch Size**: 128 (4x larger for better gradient estimates)
- **Learning Rate**: 0.0005 (half default for stability)
- **MCTS Simulations**: 2,000 per move (2.5x more for better evaluation)
- **Attention Layers**: Enabled for pattern recognition
- **Residual Connections**: Enabled for deeper networks
- **Output**: `ml_ai_weights_intensive_13min.json`

### Key Features

- **Caffeinate Integration**: Prevents system sleep during training
- **Enhanced Neural Network**: Attention and residual connections
- **Curriculum Learning**: Progressive difficulty increase
- **Early Stopping**: Prevents overfitting
- **Comprehensive Logging**: Training progress and metrics

## Usage

### Start Intensive Training

```bash
# Start 6-hour intensive training session
npm run train:intensive
```

This command:

1. Uses `caffeinate -i` to prevent system sleep
2. Runs Python training script with enhanced parameters
3. Saves results to `ml/data/weights/ml_ai_weights_intensive_13min.json`

### Monitor Training Progress

```bash
# Monitor training logs in real-time
npm run train:intensive:monitor
```

This tails the training log file to show:

- Game generation progress
- Training epoch metrics
- Validation performance
- Time estimates

### Manual Command

You can also run the training manually with custom parameters:

```bash
caffeinate -i python3 ml/scripts/train_self_play.py \
  --num-games 5000 \
  --epochs 200 \
  --batch-size 128 \
  --learning-rate 0.0005 \
  --mcts-simulations 2000 \
  --use-attention \
  --use-residual \
  --output ml_ai_weights_custom.json
```

## Training Process

### Phase 1: Data Generation (2-3 hours)

- Generates 5,000 high-quality self-play games
- Uses MCTS with 2,000 simulations per move
- Saves games to `~/Desktop/connect-four-training-data/`

### Phase 2: Model Training (3-4 hours)

- Trains for 200 epochs with early stopping
- Uses curriculum learning for progressive difficulty
- Monitors validation loss to prevent overfitting

### Phase 3: Model Evaluation

- Generates training history plots
- Saves final model weights
- Creates performance metrics

## Output Files

### Model Weights

- `ml/data/weights/ml_ai_weights_intensive_13min.json` - Final trained model

### Training Data

- `~/Desktop/connect-four-training-data/temp_self_play_data.json` - Training games
- `~/Desktop/connect-four-training-data/training.log` - Training logs

### Visualizations

- `ml/data/weights/training_history.png` - Training progress plots

## Performance Expectations

### Training Time

- **Total**: ~6 hours
- **Data Generation**: 2-3 hours
- **Model Training**: 3-4 hours

### Model Quality

- Significantly better than standard training
- Improved strategic play
- Better endgame performance
- More consistent decision making

### System Requirements

- **RAM**: 8GB+ recommended
- **Storage**: 2GB+ free space
- **CPU**: Multi-core recommended
- **GPU**: Optional but beneficial

## Troubleshooting

### Common Issues

**Training stops unexpectedly**

- Check system resources (RAM, disk space)
- Ensure caffeinate is working properly
- Monitor system temperature

**Poor training performance**

- Reduce batch size if out of memory
- Lower learning rate if unstable
- Check training data quality

**System becomes unresponsive**

- Training is CPU intensive
- Consider running overnight
- Monitor system resources

### Recovery

If training is interrupted:

1. Check for partial output files
2. Resume with reduced parameters if needed
3. Use existing training data if available

## Best Practices

### Timing

- Run during off-peak hours
- Ensure stable power supply
- Avoid other intensive tasks

### Monitoring

- Use `npm run train:intensive:monitor` to watch progress
- Check system resources periodically
- Save intermediate results if possible

### Validation

- Test the trained model thoroughly
- Compare against previous models
- Document performance improvements

## Integration

After training completes:

```bash
# Load the new model weights
npm run load:ml-weights

# Test the new model
npm run test:ai-comparison

# Deploy if satisfied with performance
npm run deploy
```

## Advanced Customization

### Modify Training Parameters

Edit `ml/config/training.json` to customize:

- Network architecture
- Training parameters
- MCTS settings

### Custom Training Scripts

Create custom training scripts by modifying:

- `ml/scripts/train_self_play.py` - Main training script
- `ml/scripts/train_self_play.sh` - Shell wrapper

### Experimentation

Try different configurations:

- Vary number of games and epochs
- Adjust learning rates and batch sizes
- Test different network architectures
