#!/usr/bin/env python3
"""
Simple PyTorch training script for Connect Four
Uses a lightweight neural network appropriate for the game's simplicity
"""

import json
import sys
import time
import argparse
from pathlib import Path
from typing import List, Dict, Any, Tuple
import numpy as np
import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader, TensorDataset
import logging
import matplotlib.pyplot as plt

logging.basicConfig(
    level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s"
)
logger = logging.getLogger(__name__)


class SimpleValueNetwork(nn.Module):
    """Simple value network for Connect Four"""

    def __init__(self, input_size: int = 42):
        super().__init__()
        # Simple 2-layer network: input -> 64 -> 32 -> 1
        self.layers = nn.Sequential(
            nn.Linear(input_size, 64),
            nn.ReLU(),
            nn.Dropout(0.1),
            nn.Linear(64, 32),
            nn.ReLU(),
            nn.Dropout(0.1),
            nn.Linear(32, 1),
            nn.Tanh(),  # Output between -1 and 1
        )

    def forward(self, x):
        return self.layers(x)


class SimplePolicyNetwork(nn.Module):
    """Simple policy network for Connect Four"""

    def __init__(self, input_size: int = 42):
        super().__init__()
        # Simple 2-layer network: input -> 64 -> 32 -> 7
        self.layers = nn.Sequential(
            nn.Linear(input_size, 64),
            nn.ReLU(),
            nn.Dropout(0.1),
            nn.Linear(64, 32),
            nn.ReLU(),
            nn.Dropout(0.1),
            nn.Linear(32, 7),  # 7 possible moves
            nn.Softmax(dim=1),
        )

    def forward(self, x):
        return self.layers(x)


class SimpleTrainer:
    def __init__(self, config: Dict[str, Any]):
        self.config = config
        self.device = torch.device("cuda" if torch.cuda.is_available() else "cpu")

        # Initialize networks
        self.value_network = SimpleValueNetwork().to(self.device)
        self.policy_network = SimplePolicyNetwork().to(self.device)

        # Optimizers
        self.value_optimizer = optim.Adam(
            self.value_network.parameters(), lr=config["learning_rate"]
        )
        self.policy_optimizer = optim.Adam(
            self.policy_network.parameters(), lr=config["learning_rate"]
        )

        # Loss functions
        self.value_loss_fn = nn.MSELoss()
        self.policy_loss_fn = nn.CrossEntropyLoss()

        # Training history
        self.training_history = {
            "value_loss": [],
            "policy_loss": [],
            "total_loss": [],
            "val_value_loss": [],
            "val_policy_loss": [],
            "val_total_loss": [],
        }

    def generate_simple_training_data(self) -> List[Dict[str, Any]]:
        """Generate simple training data using basic game scenarios"""
        logger.info("Generating simple training data...")

        training_data = []

        # Generate data from basic Connect Four scenarios
        for _ in range(self.config["num_games"]):
            # Create simple game scenarios
            game_data = self._create_simple_game_scenario()
            training_data.extend(game_data)

        logger.info(f"Generated {len(training_data)} training samples")
        return training_data

    def _create_simple_game_scenario(self) -> List[Dict[str, Any]]:
        """Create simple game scenarios for training"""
        scenarios = []

        # Scenario 1: Empty board
        empty_board = np.zeros(42, dtype=np.float32)
        scenarios.append(
            {
                "state": empty_board,
                "value_target": 0.0,  # Neutral position
                "policy_target": np.array(
                    [1 / 7] * 7, dtype=np.float32
                ),  # Uniform distribution
            }
        )

        # Scenario 2: Near win for player 1
        near_win_board = np.zeros(42, dtype=np.float32)
        # Create a near-win scenario (3 in a row)
        near_win_board[0] = 1.0
        near_win_board[1] = 1.0
        near_win_board[2] = 1.0
        scenarios.append(
            {
                "state": near_win_board,
                "value_target": 0.8,  # Strong advantage
                "policy_target": np.array(
                    [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0], dtype=np.float32
                ),  # Winning move
            }
        )

        # Scenario 3: Blocking scenario
        block_board = np.zeros(42, dtype=np.float32)
        # Create a blocking scenario
        block_board[7] = -1.0
        block_board[8] = -1.0
        block_board[9] = -1.0
        scenarios.append(
            {
                "state": block_board,
                "value_target": -0.6,  # Disadvantage
                "policy_target": np.array(
                    [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0], dtype=np.float32
                ),  # Blocking move
            }
        )

        return scenarios

    def prepare_data_loaders(
        self, training_data: List[Dict[str, Any]]
    ) -> Tuple[DataLoader, DataLoader]:
        """Prepare data loaders for training"""
        # Split data
        split_idx = int(len(training_data) * (1 - self.config["validation_split"]))
        train_data = training_data[:split_idx]
        val_data = training_data[split_idx:]

        # Prepare tensors
        train_states = torch.tensor(
            [d["state"] for d in train_data], dtype=torch.float32
        )
        train_values = torch.tensor(
            [d["value_target"] for d in train_data], dtype=torch.float32
        )
        train_policies = torch.tensor(
            [d["policy_target"] for d in train_data], dtype=torch.float32
        )

        val_states = torch.tensor([d["state"] for d in val_data], dtype=torch.float32)
        val_values = torch.tensor(
            [d["value_target"] for d in val_data], dtype=torch.float32
        )
        val_policies = torch.tensor(
            [d["policy_target"] for d in val_data], dtype=torch.float32
        )

        # Create datasets
        train_dataset = TensorDataset(train_states, train_values, train_policies)
        val_dataset = TensorDataset(val_states, val_values, val_policies)

        # Create loaders
        train_loader = DataLoader(
            train_dataset, batch_size=self.config["batch_size"], shuffle=True
        )
        val_loader = DataLoader(
            val_dataset, batch_size=self.config["batch_size"], shuffle=False
        )

        return train_loader, val_loader

    def train_epoch(self, train_loader: DataLoader) -> Tuple[float, float, float]:
        """Train for one epoch"""
        self.value_network.train()
        self.policy_network.train()

        total_value_loss = 0.0
        total_policy_loss = 0.0
        total_loss = 0.0
        num_batches = 0

        for states, values, policies in train_loader:
            states = states.to(self.device)
            values = values.to(self.device)
            policies = policies.to(self.device)

            # Forward pass
            value_outputs = self.value_network(states).squeeze()
            policy_outputs = self.policy_network(states)

            # Calculate losses
            value_loss = self.value_loss_fn(value_outputs, values)
            policy_loss = self.policy_loss_fn(policy_outputs, policies)
            total_batch_loss = value_loss + policy_loss

            # Backward pass
            self.value_optimizer.zero_grad()
            self.policy_optimizer.zero_grad()
            total_batch_loss.backward()
            self.value_optimizer.step()
            self.policy_optimizer.step()

            total_value_loss += value_loss.item()
            total_policy_loss += policy_loss.item()
            total_loss += total_batch_loss.item()
            num_batches += 1

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
            for states, values, policies in val_loader:
                states = states.to(self.device)
                values = values.to(self.device)
                policies = policies.to(self.device)

                # Forward pass
                value_outputs = self.value_network(states).squeeze()
                policy_outputs = self.policy_network(states)

                # Calculate losses
                value_loss = self.value_loss_fn(value_outputs, values)
                policy_loss = self.policy_loss_fn(policy_outputs, policies)
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
        logger.info("Starting training...")

        train_loader, val_loader = self.prepare_data_loaders(training_data)

        best_val_loss = float("inf")
        patience_counter = 0

        for epoch in range(self.config["epochs"]):
            # Training
            train_value_loss, train_policy_loss, train_total_loss = self.train_epoch(
                train_loader
            )

            # Validation
            val_value_loss, val_policy_loss, val_total_loss = self.validate_epoch(
                val_loader
            )

            # Record history
            self.training_history["value_loss"].append(train_value_loss)
            self.training_history["policy_loss"].append(train_policy_loss)
            self.training_history["total_loss"].append(train_total_loss)
            self.training_history["val_value_loss"].append(val_value_loss)
            self.training_history["val_policy_loss"].append(val_policy_loss)
            self.training_history["val_total_loss"].append(val_total_loss)

            # Log progress
            if epoch % 5 == 0:
                logger.info(f"Epoch {epoch + 1}/{self.config['epochs']}")
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
            else:
                patience_counter += 1
                if patience_counter >= 10:  # Early stopping patience
                    logger.info(f"Early stopping at epoch {epoch + 1}")
                    break

        logger.info("Training completed!")
        return {
            "final_train_loss": train_total_loss,
            "final_val_loss": val_total_loss,
            "best_val_loss": best_val_loss,
            "epochs_completed": len(self.training_history["total_loss"]),
        }

    def save_weights(self, filename: str, metadata: Dict[str, Any]):
        """Save model weights"""
        logger.info(f"Saving weights to {filename}")

        # Get network weights
        value_weights = {}
        for name, param in self.value_network.named_parameters():
            value_weights[name] = param.data.cpu().numpy().tolist()

        policy_weights = {}
        for name, param in self.policy_network.named_parameters():
            policy_weights[name] = param.data.cpu().numpy().tolist()

        # Create output structure
        output = {
            "metadata": {
                "model_type": "simple_connect_four",
                "architecture": {
                    "input_size": 42,
                    "hidden_sizes": [64, 32],
                    "value_output_size": 1,
                    "policy_output_size": 7,
                },
                "training_config": {
                    "epochs": self.config["epochs"],
                    "batch_size": self.config["batch_size"],
                    "learning_rate": self.config["learning_rate"],
                    "use_attention": False,
                    "use_residual": False,
                },
                **metadata,
            },
            "value_network": value_weights,
            "policy_network": policy_weights,
        }

        # Save to file
        with open(filename, "w") as f:
            json.dump(output, f, indent=2)

        logger.info(f"Model saved successfully!")


def main():
    parser = argparse.ArgumentParser(description="Simple Connect Four ML Training")
    parser.add_argument(
        "--output", default="ml_ai_weights_simple.json", help="Output file name"
    )
    parser.add_argument(
        "--epochs", type=int, default=20, help="Number of training epochs"
    )
    parser.add_argument("--batch-size", type=int, default=32, help="Batch size")
    parser.add_argument(
        "--learning-rate", type=float, default=0.001, help="Learning rate"
    )
    parser.add_argument(
        "--num-games", type=int, default=500, help="Number of training games"
    )

    args = parser.parse_args()

    # Configuration
    config = {
        "epochs": args.epochs,
        "batch_size": args.batch_size,
        "learning_rate": args.learning_rate,
        "num_games": args.num_games,
        "validation_split": 0.2,
    }

    # Create trainer
    trainer = SimpleTrainer(config)

    # Generate training data
    training_data = trainer.generate_simple_training_data()

    # Train model
    training_results = trainer.train(training_data)

    # Save model
    output_path = Path("ml/data/weights") / args.output
    output_path.parent.mkdir(parents=True, exist_ok=True)
    trainer.save_weights(str(output_path), training_results)

    logger.info("Training completed successfully!")


if __name__ == "__main__":
    main()
