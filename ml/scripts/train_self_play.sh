#!/bin/bash

# Advanced Self-Play Training Script for Connect Four
# Uses caffeinate to prevent system sleep during training

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to show usage
show_usage() {
    echo "Advanced Self-Play Training for Connect Four"
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --num-games N          Number of self-play games (default: 1000)"
    echo "  --epochs N             Number of training epochs (default: 50)"
    echo "  --batch-size N         Batch size (default: 32)"
    echo "  --learning-rate F      Learning rate (default: 0.001)"
    echo "  --mcts-simulations N   MCTS simulations per move (default: 800)"
    echo "  --use-attention        Enable attention layers"
    echo "  --use-residual         Enable residual connections"
    echo "  --output FILE          Output file name (default: ml_ai_weights_self_play.json)"
    echo "  --preset PRESET        Use preset configuration (quick, default, production)"
    echo "  --help                 Show this help message"
    echo ""
    echo "Presets:"
    echo "  quick        - 100 games, 10 epochs, 400 MCTS simulations"
    echo "  default      - 1000 games, 50 epochs, 800 MCTS simulations"
    echo "  production   - 2000 games, 100 epochs, 1200 MCTS simulations"
    echo ""
    echo "Examples:"
    echo "  $0 --preset quick"
    echo "  $0 --num-games 500 --epochs 25 --use-attention"
    echo "  $0 --preset production --use-attention --use-residual"
}

# Default values
NUM_GAMES=1000
EPOCHS=50
BATCH_SIZE=32
LEARNING_RATE=0.001
MCTS_SIMULATIONS=800
USE_ATTENTION=""
USE_RESIDUAL=""
OUTPUT_FILE="ml_ai_weights_self_play.json"
PRESET=""

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --num-games)
            NUM_GAMES="$2"
            shift 2
            ;;
        --epochs)
            EPOCHS="$2"
            shift 2
            ;;
        --batch-size)
            BATCH_SIZE="$2"
            shift 2
            ;;
        --learning-rate)
            LEARNING_RATE="$2"
            shift 2
            ;;
        --mcts-simulations)
            MCTS_SIMULATIONS="$2"
            shift 2
            ;;
        --use-attention)
            USE_ATTENTION="--use-attention"
            shift
            ;;
        --use-residual)
            USE_RESIDUAL="--use-residual"
            shift
            ;;
        --output)
            OUTPUT_FILE="$2"
            shift 2
            ;;
        --preset)
            PRESET="$2"
            shift 2
            ;;
        --help)
            show_usage
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# Apply presets
case $PRESET in
    quick)
        NUM_GAMES=100
        EPOCHS=10
        MCTS_SIMULATIONS=400
        print_status "Using quick preset: $NUM_GAMES games, $EPOCHS epochs, $MCTS_SIMULATIONS MCTS simulations"
        ;;
    production)
        NUM_GAMES=2000
        EPOCHS=100
        MCTS_SIMULATIONS=1200
        print_status "Using production preset: $NUM_GAMES games, $EPOCHS epochs, $MCTS_SIMULATIONS MCTS simulations"
        ;;
    default|"")
        print_status "Using default preset: $NUM_GAMES games, $EPOCHS epochs, $MCTS_SIMULATIONS MCTS simulations"
        ;;
    *)
        print_error "Unknown preset: $PRESET"
        show_usage
        exit 1
        ;;
esac

# Check if we're in the right directory
if [[ ! -f "package.json" ]] || [[ ! -d "ml" ]]; then
    print_error "Please run this script from the project root directory"
    exit 1
fi

# Check if Python is available
if ! command -v python3 &> /dev/null; then
    print_error "Python 3 is required but not installed"
    exit 1
fi

# Check if PyTorch is available
if ! python3 -c "import torch" &> /dev/null; then
    print_error "PyTorch is required but not installed. Run: pip install torch"
    exit 1
fi

# Check if Rust is available
if ! command -v cargo &> /dev/null; then
    print_error "Rust is required but not installed"
    exit 1
fi

# Check if the Rust project exists
if [[ ! -d "worker/rust_ai_core" ]]; then
    print_error "Rust AI core not found. Please ensure the project is properly set up"
    exit 1
fi

print_status "Starting advanced self-play training..."
print_status "Configuration:"
print_status "  Games: $NUM_GAMES"
print_status "  Epochs: $EPOCHS"
print_status "  Batch size: $BATCH_SIZE"
print_status "  Learning rate: $LEARNING_RATE"
print_status "  MCTS simulations: $MCTS_SIMULATIONS"
print_status "  Output file: $OUTPUT_FILE"
if [[ -n "$USE_ATTENTION" ]]; then
    print_status "  Attention layers: enabled"
fi
if [[ -n "$USE_RESIDUAL" ]]; then
    print_status "  Residual connections: enabled"
fi

# Build Rust project if needed
print_status "Building Rust project..."
cd worker/rust_ai_core
cargo build --release --features training
cd ../..

# Create training data directory
mkdir -p ml/data/weights

# Run training with caffeinate to prevent system sleep
print_status "Starting training with caffeinate..."
caffeinate -i python3 ml/scripts/train_self_play.py \
    --num-games "$NUM_GAMES" \
    --epochs "$EPOCHS" \
    --batch-size "$BATCH_SIZE" \
    --learning-rate "$LEARNING_RATE" \
    --mcts-simulations "$MCTS_SIMULATIONS" \
    --output "$OUTPUT_FILE" \
    $USE_ATTENTION \
    $USE_RESIDUAL

if [[ $? -eq 0 ]]; then
    print_success "Training completed successfully!"
    print_status "Model saved to: ml/data/weights/$OUTPUT_FILE"
    
    # Copy to public directory for web use
    if [[ -f "ml/data/weights/$OUTPUT_FILE" ]]; then
        cp "ml/data/weights/$OUTPUT_FILE" "public/ml/data/weights/"
        print_status "Model copied to public directory for web use"
    fi
else
    print_error "Training failed!"
    exit 1
fi 