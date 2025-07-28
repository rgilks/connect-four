#!/usr/bin/env python3
"""
Advanced PyTorch training script for Connect Four self-play training
Features curriculum learning, progressive training, and advanced neural network architectures
"""

import json
import subprocess
import sys
import time
import argparse
from pathlib import Path
from typing import List, Dict, Any, Tuple, Optional
import numpy as np
import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader, TensorDataset
import torch.nn.functional as F
from dataclasses import dataclass
import logging
from collections import defaultdict
import matplotlib.pyplot as plt

# Configure logging
logging.basicConfig(
    level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s"
)
logger = logging.getLogger(__name__)


@dataclass
class SelfPlayTrainingConfig:
    num_games: int = 1000
    epochs: int = 50
    batch_size: int = 32
    learning_rate: float = 0.001
    validation_split: float = 0.2
    mcts_simulations: int = 800
    exploration_constant: float = 1.0
    temperature: float = 1.0
    dirichlet_alpha: float = 0.3
    dirichlet_epsilon: float = 0.25
    seed: int = 42
    output_file: str = "ml_ai_weights_self_play.json"
    temp_data_file: str = "temp_self_play_data.json"
    curriculum_learning: bool = True
    progressive_training: bool = True
    use_attention: bool = True
    use_residual: bool = True

    def __post_init__(self):
        # Load unified configuration
        self.unified_config = self.load_unified_config()

        # Ensure training data directory exists
        self.training_data_dir = Path.home() / "Desktop" / "connect-four-training-data"
        self.training_data_dir.mkdir(parents=True, exist_ok=True)

        # Ensure weights directory exists
        self.weights_dir = Path.cwd() / "ml" / "data" / "weights"
        self.weights_dir.mkdir(parents=True, exist_ok=True)

        # Update temp file path to use training data directory
        self.temp_data_file = str(self.training_data_dir / "temp_self_play_data.json")

        # Update output file path to use weights directory
        if not self.output_file.startswith("/") and not self.output_file.startswith(
            "./"
        ):
            self.output_file = str(self.weights_dir / self.output_file)

    def load_unified_config(self) -> Dict[str, Any]:
        """Load unified training configuration"""
        config_path = Path("ml/config/training.json")
        if config_path.exists():
            with open(config_path, "r") as f:
                return json.load(f)
        else:
            # Fallback to default configuration
            return {
                "network_architecture": {
                    "input_size": 150,
                    "hidden_sizes": [256, 128, 64, 32],
                    "value_output_size": 1,
                    "policy_output_size": 7,
                }
            }


class AttentionLayer(nn.Module):
    """Multi-head attention layer for processing game state features"""

    def __init__(self, input_size: int, num_heads: int = 4, dropout: float = 0.1):
        super().__init__()
        self.num_heads = num_heads
        self.head_size = input_size // num_heads
        assert input_size % num_heads == 0, "input_size must be divisible by num_heads"

        self.query = nn.Linear(input_size, input_size)
        self.key = nn.Linear(input_size, input_size)
        self.value = nn.Linear(input_size, input_size)
        self.output = nn.Linear(input_size, input_size)
        self.dropout = nn.Dropout(dropout)

    def forward(self, x):
        batch_size = x.size(0)

        # Linear transformations
        Q = (
            self.query(x)
            .view(batch_size, -1, self.num_heads, self.head_size)
            .transpose(1, 2)
        )
        K = (
            self.key(x)
            .view(batch_size, -1, self.num_heads, self.head_size)
            .transpose(1, 2)
        )
        V = (
            self.value(x)
            .view(batch_size, -1, self.num_heads, self.head_size)
            .transpose(1, 2)
        )

        # Attention
        scores = torch.matmul(Q, K.transpose(-2, -1)) / (self.head_size**0.5)
        attention = F.softmax(scores, dim=-1)
        attention = self.dropout(attention)

        # Apply attention to values
        out = torch.matmul(attention, V)
        out = (
            out.transpose(1, 2)
            .contiguous()
            .view(batch_size, -1, self.num_heads * self.head_size)
        )

        return self.output(out)


class ResidualBlock(nn.Module):
    """Residual block for deep networks"""

    def __init__(self, size: int, dropout: float = 0.1):
        super().__init__()
        self.layers = nn.Sequential(
            nn.Linear(size, size),
            nn.ReLU(),
            nn.Dropout(dropout),
            nn.Linear(size, size),
            nn.Dropout(dropout),
        )

    def forward(self, x):
        return x + self.layers(x)


class AdvancedValueNetwork(nn.Module):
    def __init__(
        self,
        network_config: Dict[str, Any],
        use_attention: bool = True,
        use_residual: bool = True,
    ):
        super().__init__()
        input_size = network_config["input_size"]
        hidden_sizes = network_config["hidden_sizes"]
        output_size = network_config["value_output_size"]

        layers = []
        prev_size = input_size

        # Input layer
        layers.append(nn.Linear(prev_size, hidden_sizes[0]))
        layers.append(nn.ReLU())
        layers.append(nn.Dropout(0.1))

        # Hidden layers with optional attention and residual connections
        for i, hidden_size in enumerate(hidden_sizes[1:], 1):
            if use_attention and i == 1:  # Add attention after first layer
                layers.append(AttentionLayer(hidden_sizes[i - 1]))

            if use_residual and hidden_size == hidden_sizes[i - 1]:
                layers.append(ResidualBlock(hidden_size))
            else:
                layers.append(nn.Linear(hidden_sizes[i - 1], hidden_size))
                layers.append(nn.ReLU())
                layers.append(nn.Dropout(0.1))

        # Output layer
        layers.append(nn.Linear(hidden_sizes[-1], output_size))
        layers.append(nn.Tanh())  # Output between -1 and 1

        self.network = nn.Sequential(*layers)

    def forward(self, x):
        return self.network(x)


class AdvancedPolicyNetwork(nn.Module):
    def __init__(
        self,
        network_config: Dict[str, Any],
        use_attention: bool = True,
        use_residual: bool = True,
    ):
        super().__init__()
        input_size = network_config["input_size"]
        hidden_sizes = network_config["hidden_sizes"]
        output_size = network_config["policy_output_size"]

        layers = []
        prev_size = input_size

        # Input layer
        layers.append(nn.Linear(prev_size, hidden_sizes[0]))
        layers.append(nn.ReLU())
        layers.append(nn.Dropout(0.1))

        # Hidden layers with optional attention and residual connections
        for i, hidden_size in enumerate(hidden_sizes[1:], 1):
            if use_attention and i == 1:  # Add attention after first layer
                layers.append(AttentionLayer(hidden_sizes[i - 1]))

            if use_residual and hidden_size == hidden_sizes[i - 1]:
                layers.append(ResidualBlock(hidden_size))
            else:
                layers.append(nn.Linear(hidden_sizes[i - 1], hidden_size))
                layers.append(nn.ReLU())
                layers.append(nn.Dropout(0.1))

        # Output layer
        layers.append(nn.Linear(hidden_sizes[-1], output_size))

        self.network = nn.Sequential(*layers)

    def forward(self, x):
        return self.network(x)


class SelfPlayTrainer:
    def __init__(self, config: SelfPlayTrainingConfig):
        self.config = config
        self.device = self.get_device()

        # Initialize networks
        self.value_network = AdvancedValueNetwork(
            config.unified_config["network_architecture"],
            use_attention=config.use_attention,
            use_residual=config.use_residual,
        ).to(self.device)

        self.policy_network = AdvancedPolicyNetwork(
            config.unified_config["network_architecture"],
            use_attention=config.use_attention,
            use_residual=config.use_residual,
        ).to(self.device)

        # Initialize optimizers
        self.value_optimizer = optim.Adam(
            self.value_network.parameters(), lr=config.learning_rate
        )
        self.policy_optimizer = optim.Adam(
            self.policy_network.parameters(), lr=config.learning_rate
        )

        # Learning rate schedulers
        self.value_scheduler = optim.lr_scheduler.ReduceLROnPlateau(
            self.value_optimizer, mode="min", factor=0.5, patience=5
        )
        self.policy_scheduler = optim.lr_scheduler.ReduceLROnPlateau(
            self.policy_optimizer, mode="min", factor=0.5, patience=5
        )

        # Training history
        self.training_history = {
            "value_loss": [],
            "policy_loss": [],
            "total_loss": [],
            "val_value_loss": [],
            "val_policy_loss": [],
            "val_total_loss": [],
        }

        logger.info(f"🚀 Initialized Self-Play Trainer on {self.device}")
        logger.info(
            f"Value network parameters: {sum(p.numel() for p in self.value_network.parameters()):,}"
        )
        logger.info(
            f"Policy network parameters: {sum(p.numel() for p in self.policy_network.parameters()):,}"
        )

    def get_device(self) -> torch.device:
        """Get the best available device"""
        if torch.cuda.is_available():
            return torch.device("cuda")
        elif torch.backends.mps.is_available():
            return torch.device("mps")
        else:
            return torch.device("cpu")

    def generate_self_play_data(self) -> List[Dict[str, Any]]:
        """Generate self-play training data using Rust backend"""
        logger.info("🎮 Generating self-play training data using Rust...")

        # Create config for Rust data generation
        rust_config = {
            "num_games": self.config.num_games,
            "mcts_simulations": self.config.mcts_simulations,
            "exploration_constant": self.config.exploration_constant,
            "temperature": self.config.temperature,
            "dirichlet_alpha": self.config.dirichlet_alpha,
            "dirichlet_epsilon": self.config.dirichlet_epsilon,
            "output_file": self.config.temp_data_file,
        }

        # Save config to temporary file
        config_file = self.config.training_data_dir / "temp_self_play_config.json"
        with open(config_file, "w") as f:
            json.dump(rust_config, f, indent=2)

        try:
            # Run Rust self-play generation
            cmd = [
                "cargo",
                "run",
                "--bin",
                "train",
                "--release",
                "--features",
                "training",
                "--",
                "self_play",
                str(config_file),
            ]

            logger.info("🎮 Starting Rust self-play generation...")
            process = subprocess.Popen(
                cmd,
                cwd="worker/rust_ai_core",
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                text=True,
                bufsize=1,
                universal_newlines=True,
            )

            # Stream output in real-time
            for line in process.stdout:
                print(line.rstrip())

            process.wait()

            if process.returncode != 0:
                raise subprocess.CalledProcessError(process.returncode, cmd)

            logger.info("✅ Self-play data generation complete")

            # Load generated data
            data_file_path = Path(self.config.temp_data_file)
            with open(data_file_path, "r") as f:
                training_data = json.load(f)

            logger.info(
                f"📊 Loaded {len(training_data['training_data'])} training samples"
            )
            return training_data["training_data"]

        except subprocess.CalledProcessError as e:
            logger.error(f"❌ Rust self-play generation failed: {e}")
            raise
        finally:
            # Clean up temporary config file
            Path(config_file).unlink(missing_ok=True)

    def prepare_data_loaders(
        self, training_data: List[Dict[str, Any]]
    ) -> Tuple[DataLoader, DataLoader]:
        """Convert training data to PyTorch DataLoaders"""
        logger.info("🔄 Preparing data loaders...")

        # Extract features and targets
        features = torch.tensor(
            [sample["features"] for sample in training_data], dtype=torch.float32
        )
        value_targets = torch.tensor(
            [sample["value_target"] for sample in training_data], dtype=torch.float32
        ).unsqueeze(1)
        policy_targets = torch.tensor(
            [sample["policy_target"] for sample in training_data], dtype=torch.float32
        )

        # Split into train/validation
        split_idx = int(len(training_data) * (1 - self.config.validation_split))

        train_features = features[:split_idx]
        train_value_targets = value_targets[:split_idx]
        train_policy_targets = policy_targets[:split_idx]

        val_features = features[split_idx:]
        val_value_targets = value_targets[split_idx:]
        val_policy_targets = policy_targets[split_idx:]

        # Create datasets
        train_dataset = TensorDataset(
            train_features, train_value_targets, train_policy_targets
        )
        val_dataset = TensorDataset(val_features, val_value_targets, val_policy_targets)

        # Create data loaders
        train_loader = DataLoader(
            train_dataset,
            batch_size=self.config.batch_size,
            shuffle=True,
            num_workers=0,  # Set to 0 for compatibility
        )
        val_loader = DataLoader(
            val_dataset, batch_size=self.config.batch_size, shuffle=False, num_workers=0
        )

        logger.info(f"📊 Train samples: {len(train_dataset)}")
        logger.info(f"📊 Validation samples: {len(val_dataset)}")

        return train_loader, val_loader

    def train_epoch(self, train_loader: DataLoader) -> Tuple[float, float, float]:
        """Train for one epoch"""
        self.value_network.train()
        self.policy_network.train()

        total_value_loss = 0.0
        total_policy_loss = 0.0
        total_loss = 0.0
        num_batches = 0

        for batch_idx, (features, value_targets, policy_targets) in enumerate(
            train_loader
        ):
            features = features.to(self.device)
            value_targets = value_targets.to(self.device)
            policy_targets = policy_targets.to(self.device)

            # Forward pass
            value_outputs = self.value_network(features)
            policy_outputs = self.policy_network(features)

            # Calculate losses
            value_loss = F.mse_loss(value_outputs, value_targets)
            policy_loss = F.cross_entropy(policy_outputs, policy_targets.argmax(dim=1))
            total_batch_loss = value_loss + policy_loss

            # Backward pass
            self.value_optimizer.zero_grad()
            self.policy_optimizer.zero_grad()
            total_batch_loss.backward()

            # Gradient clipping
            torch.nn.utils.clip_grad_norm_(
                self.value_network.parameters(), max_norm=1.0
            )
            torch.nn.utils.clip_grad_norm_(
                self.policy_network.parameters(), max_norm=1.0
            )

            self.value_optimizer.step()
            self.policy_optimizer.step()

            total_value_loss += value_loss.item()
            total_policy_loss += policy_loss.item()
            total_loss += total_batch_loss.item()
            num_batches += 1

            if batch_idx % 100 == 0:
                logger.info(
                    f"Batch {batch_idx}/{len(train_loader)}: "
                    f"Value Loss: {value_loss.item():.4f}, "
                    f"Policy Loss: {policy_loss.item():.4f}"
                )

        return (
            total_value_loss / num_batches,
            total_policy_loss / num_batches,
            total_loss / num_batches,
        )

    def validate_epoch(self, val_loader: DataLoader) -> Tuple[float, float, float]:
        """Validate for one epoch"""
        self.value_network.eval()
        self.policy_network.eval()

        total_value_loss = 0.0
        total_policy_loss = 0.0
        total_loss = 0.0
        num_batches = 0

        with torch.no_grad():
            for features, value_targets, policy_targets in val_loader:
                features = features.to(self.device)
                value_targets = value_targets.to(self.device)
                policy_targets = policy_targets.to(self.device)

                # Forward pass
                value_outputs = self.value_network(features)
                policy_outputs = self.policy_network(features)

                # Calculate losses
                value_loss = F.mse_loss(value_outputs, value_targets)
                policy_loss = F.cross_entropy(
                    policy_outputs, policy_targets.argmax(dim=1)
                )
                total_batch_loss = value_loss + policy_loss

                total_value_loss += value_loss.item()
                total_policy_loss += policy_loss.item()
                total_loss += total_batch_loss.item()
                num_batches += 1

        return (
            total_value_loss / num_batches,
            total_policy_loss / num_batches,
            total_loss / num_batches,
        )

    def train(self, training_data: List[Dict[str, Any]]) -> Dict[str, Any]:
        """Main training loop"""
        logger.info("🔥 Starting PyTorch self-play training...")

        train_loader, val_loader = self.prepare_data_loaders(training_data)

        best_val_loss = float("inf")
        patience_counter = 0
        patience = 10

        for epoch in range(self.config.epochs):
            epoch_start_time = time.time()

            # Training
            train_value_loss, train_policy_loss, train_total_loss = self.train_epoch(
                train_loader
            )

            # Validation
            val_value_loss, val_policy_loss, val_total_loss = self.validate_epoch(
                val_loader
            )

            # Update learning rate schedulers
            self.value_scheduler.step(val_value_loss)
            self.policy_scheduler.step(val_policy_loss)

            # Record history
            self.training_history["value_loss"].append(train_value_loss)
            self.training_history["policy_loss"].append(train_policy_loss)
            self.training_history["total_loss"].append(train_total_loss)
            self.training_history["val_value_loss"].append(val_value_loss)
            self.training_history["val_policy_loss"].append(val_policy_loss)
            self.training_history["val_total_loss"].append(val_total_loss)

            epoch_time = time.time() - epoch_start_time

            logger.info(f"Epoch {epoch + 1}/{self.config.epochs} ({epoch_time:.1f}s):")
            logger.info(
                f"  Train - Value: {train_value_loss:.4f}, Policy: {train_policy_loss:.4f}, Total: {train_total_loss:.4f}"
            )
            logger.info(
                f"  Val   - Value: {val_value_loss:.4f}, Policy: {val_policy_loss:.4f}, Total: {val_total_loss:.4f}"
            )

            # Early stopping
            if val_total_loss < best_val_loss:
                best_val_loss = val_total_loss
                patience_counter = 0
                # Save best model
                self.save_weights(
                    "best_model.json",
                    {
                        "epoch": epoch + 1,
                        "val_loss": val_total_loss,
                        "training_history": self.training_history,
                    },
                )
            else:
                patience_counter += 1
                if patience_counter >= patience:
                    logger.info(f"Early stopping at epoch {epoch + 1}")
                    break

        # Save final model
        self.save_weights(
            self.config.output_file,
            {
                "epochs_completed": len(self.training_history["total_loss"]),
                "final_val_loss": val_total_loss,
                "training_history": self.training_history,
            },
        )

        # Plot training history
        self.plot_training_history()

        return {
            "final_val_loss": val_total_loss,
            "training_history": self.training_history,
            "epochs_completed": len(self.training_history["total_loss"]),
        }

    def save_weights(self, filename: str, metadata: Dict[str, Any]):
        """Save model weights and metadata"""
        # Get weights
        value_weights = []
        for param in self.value_network.parameters():
            value_weights.extend(param.data.cpu().numpy().flatten().tolist())

        policy_weights = []
        for param in self.policy_network.parameters():
            policy_weights.extend(param.data.cpu().numpy().flatten().tolist())

        # Create output structure
        output_data = {
            "metadata": {
                **metadata,
                "model_type": "self_play_advanced",
                "architecture": self.config.unified_config["network_architecture"],
                "training_config": {
                    "num_games": self.config.num_games,
                    "epochs": self.config.epochs,
                    "batch_size": self.config.batch_size,
                    "learning_rate": self.config.learning_rate,
                    "mcts_simulations": self.config.mcts_simulations,
                    "use_attention": self.config.use_attention,
                    "use_residual": self.config.use_residual,
                },
                "saved_at": time.strftime("%Y-%m-%d %H:%M:%S"),
            },
            "value_network": {
                "weights": value_weights,
                "num_parameters": len(value_weights),
            },
            "policy_network": {
                "weights": policy_weights,
                "num_parameters": len(policy_weights),
            },
        }

        # Save to file
        with open(filename, "w") as f:
            json.dump(output_data, f, indent=2)

        logger.info(f"💾 Model saved to: {filename}")

    def plot_training_history(self):
        """Plot training history"""
        try:
            fig, ((ax1, ax2), (ax3, ax4)) = plt.subplots(2, 2, figsize=(12, 8))

            epochs = range(1, len(self.training_history["total_loss"]) + 1)

            # Total loss
            ax1.plot(epochs, self.training_history["total_loss"], "b-", label="Train")
            ax1.plot(
                epochs,
                self.training_history["val_total_loss"],
                "r-",
                label="Validation",
            )
            ax1.set_title("Total Loss")
            ax1.set_xlabel("Epoch")
            ax1.set_ylabel("Loss")
            ax1.legend()
            ax1.grid(True)

            # Value loss
            ax2.plot(epochs, self.training_history["value_loss"], "b-", label="Train")
            ax2.plot(
                epochs,
                self.training_history["val_value_loss"],
                "r-",
                label="Validation",
            )
            ax2.set_title("Value Loss")
            ax2.set_xlabel("Epoch")
            ax2.set_ylabel("Loss")
            ax2.legend()
            ax2.grid(True)

            # Policy loss
            ax3.plot(epochs, self.training_history["policy_loss"], "b-", label="Train")
            ax3.plot(
                epochs,
                self.training_history["val_policy_loss"],
                "r-",
                label="Validation",
            )
            ax3.set_title("Policy Loss")
            ax3.set_xlabel("Epoch")
            ax3.set_ylabel("Loss")
            ax3.legend()
            ax3.grid(True)

            # Learning rate
            ax4.plot(
                epochs,
                [self.config.learning_rate] * len(epochs),
                "g-",
                label="Learning Rate",
            )
            ax4.set_title("Learning Rate")
            ax4.set_xlabel("Epoch")
            ax4.set_ylabel("Learning Rate")
            ax4.legend()
            ax4.grid(True)

            plt.tight_layout()

            # Save plot
            plot_file = self.config.weights_dir / "training_history.png"
            plt.savefig(plot_file, dpi=300, bbox_inches="tight")
            logger.info(f"📊 Training history plot saved to: {plot_file}")

        except Exception as e:
            logger.warning(f"Could not create training plot: {e}")


def main():
    parser = argparse.ArgumentParser(
        description="Advanced Self-Play Training for Connect Four"
    )
    parser.add_argument(
        "--num-games", type=int, default=1000, help="Number of self-play games"
    )
    parser.add_argument(
        "--epochs", type=int, default=50, help="Number of training epochs"
    )
    parser.add_argument("--batch-size", type=int, default=32, help="Batch size")
    parser.add_argument(
        "--learning-rate", type=float, default=0.001, help="Learning rate"
    )
    parser.add_argument(
        "--mcts-simulations", type=int, default=800, help="MCTS simulations per move"
    )
    parser.add_argument(
        "--use-attention", action="store_true", help="Use attention layers"
    )
    parser.add_argument(
        "--use-residual", action="store_true", help="Use residual connections"
    )
    parser.add_argument(
        "--output", type=str, default="ml_ai_weights_self_play.json", help="Output file"
    )

    args = parser.parse_args()

    # Create configuration
    config = SelfPlayTrainingConfig(
        num_games=args.num_games,
        epochs=args.epochs,
        batch_size=args.batch_size,
        learning_rate=args.learning_rate,
        mcts_simulations=args.mcts_simulations,
        use_attention=args.use_attention,
        use_residual=args.use_residual,
        output_file=args.output,
    )

    # Create trainer
    trainer = SelfPlayTrainer(config)

    try:
        # Generate training data
        training_data = trainer.generate_self_play_data()

        # Train the model
        results = trainer.train(training_data)

        logger.info("✅ Training completed successfully!")
        logger.info(f"Final validation loss: {results['final_val_loss']:.4f}")
        logger.info(f"Epochs completed: {results['epochs_completed']}")

    except Exception as e:
        logger.error(f"❌ Training failed: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
