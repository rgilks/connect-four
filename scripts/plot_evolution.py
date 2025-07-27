#!/usr/bin/env python3
"""
Plot evolution data from genetic algorithm CSV files.
Usage: python scripts/plot_evolution.py <params_csv_file> [convergence_csv_file]
"""

import sys
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
from pathlib import Path


def plot_parameters(params_file):
    """Plot parameter evolution over generations."""
    df = pd.read_csv(params_file)

    # Create a large figure with subplots
    fig, axes = plt.subplots(4, 4, figsize=(20, 16))
    fig.suptitle("Genetic Algorithm Parameter Evolution", fontsize=16)

    # Parameter groups for better organization
    param_groups = {
        "Scores": ["win_score", "loss_score"],
        "Position Values": [
            "center_column_value",
            "adjacent_center_value",
            "outer_column_value",
            "edge_column_value",
        ],
        "Weights": [
            "row_height_weight",
            "center_control_weight",
            "piece_count_weight",
            "threat_weight",
            "mobility_weight",
            "vertical_control_weight",
            "horizontal_control_weight",
            "defensive_weight",
        ],
    }

    # Plot fitness and diversity
    axes[0, 0].plot(df["generation"], df["fitness"], "b-", linewidth=2)
    axes[0, 0].set_title("Fitness Over Generations")
    axes[0, 0].set_ylabel("Fitness")
    axes[0, 0].grid(True, alpha=0.3)

    axes[0, 1].plot(df["generation"], df["diversity"], "g-", linewidth=2)
    axes[0, 1].set_title("Population Diversity")
    axes[0, 1].set_ylabel("Diversity")
    axes[0, 1].grid(True, alpha=0.3)

    # Plot scores
    for i, param in enumerate(["win_score", "loss_score"]):
        row, col = 0, 2 + i
        axes[row, col].plot(df["generation"], df[param], "r-", linewidth=2)
        axes[row, col].set_title(f"{param.replace('_', ' ').title()}")
        axes[row, col].grid(True, alpha=0.3)

    # Plot position values
    for i, param in enumerate(
        [
            "center_column_value",
            "adjacent_center_value",
            "outer_column_value",
            "edge_column_value",
        ]
    ):
        row, col = 1, i
        axes[row, col].plot(df["generation"], df[param], "purple", linewidth=2)
        axes[row, col].set_title(f"{param.replace('_', ' ').title()}")
        axes[row, col].grid(True, alpha=0.3)

    # Plot weights
    weight_params = [
        "row_height_weight",
        "center_control_weight",
        "piece_count_weight",
        "threat_weight",
        "mobility_weight",
        "vertical_control_weight",
        "horizontal_control_weight",
        "defensive_weight",
    ]

    for i, param in enumerate(weight_params):
        row, col = 2 + (i // 4), i % 4
        axes[row, col].plot(df["generation"], df[param], "orange", linewidth=2)
        axes[row, col].set_title(f"{param.replace('_', ' ').title()}")
        axes[row, col].grid(True, alpha=0.3)

    # Set x-axis labels for bottom row
    for col in range(4):
        axes[3, col].set_xlabel("Generation")

    plt.tight_layout()
    return fig


def plot_convergence(convergence_file):
    """Plot parameter convergence (changes over generations)."""
    df = pd.read_csv(convergence_file)

    # Create a large figure with subplots
    fig, axes = plt.subplots(4, 4, figsize=(20, 16))
    fig.suptitle("Parameter Convergence Analysis", fontsize=16)

    # Plot score changes
    for i, param in enumerate(["win_score_change", "loss_score_change"]):
        row, col = 0, i
        axes[row, col].plot(df["generation"], df[param], "r-", linewidth=2)
        axes[row, col].set_title(f"{param.replace('_', ' ').title()}")
        axes[row, col].grid(True, alpha=0.3)
        axes[row, col].axhline(y=0, color="black", linestyle="--", alpha=0.5)

    # Plot position value changes
    for i, param in enumerate(
        [
            "center_column_change",
            "adjacent_center_change",
            "outer_column_change",
            "edge_column_change",
        ]
    ):
        row, col = 0, 2 + i
        axes[row, col].plot(df["generation"], df[param], "purple", linewidth=2)
        axes[row, col].set_title(f"{param.replace('_', ' ').title()}")
        axes[row, col].grid(True, alpha=0.3)
        axes[row, col].axhline(y=0, color="black", linestyle="--", alpha=0.5)

    # Plot weight changes
    weight_changes = [
        "row_height_change",
        "center_control_change",
        "piece_count_change",
        "threat_change",
        "mobility_change",
        "vertical_control_change",
        "horizontal_control_change",
        "defensive_change",
    ]

    for i, param in enumerate(weight_changes):
        row, col = 1 + (i // 4), i % 4
        axes[row, col].plot(df["generation"], df[param], "orange", linewidth=2)
        axes[row, col].set_title(f"{param.replace('_', ' ').title()}")
        axes[row, col].grid(True, alpha=0.3)
        axes[row, col].axhline(y=0, color="black", linestyle="--", alpha=0.5)

    # Set x-axis labels for bottom row
    for col in range(4):
        axes[3, col].set_xlabel("Generation")

    plt.tight_layout()
    return fig


def plot_convergence_summary(params_file, convergence_file=None):
    """Create a summary plot showing convergence patterns."""
    df_params = pd.read_csv(params_file)

    fig, axes = plt.subplots(2, 2, figsize=(15, 12))
    fig.suptitle("Evolution Convergence Summary", fontsize=16)

    # 1. Fitness and diversity
    ax1 = axes[0, 0]
    ax1.plot(
        df_params["generation"],
        df_params["fitness"],
        "b-",
        linewidth=3,
        label="Fitness",
    )
    ax1.set_title("Fitness Evolution")
    ax1.set_ylabel("Fitness")
    ax1.grid(True, alpha=0.3)
    ax1.legend()

    ax1_twin = ax1.twinx()
    ax1_twin.plot(
        df_params["generation"],
        df_params["diversity"],
        "g-",
        linewidth=2,
        alpha=0.7,
        label="Diversity",
    )
    ax1_twin.set_ylabel("Diversity", color="g")
    ax1_twin.legend(loc="upper right")

    # 2. Parameter stability (standard deviation over time)
    ax2 = axes[0, 1]
    weight_params = [
        "row_height_weight",
        "center_control_weight",
        "piece_count_weight",
        "threat_weight",
        "mobility_weight",
        "vertical_control_weight",
        "horizontal_control_weight",
        "defensive_weight",
    ]

    # Calculate rolling standard deviation for each weight parameter
    window_size = min(5, len(df_params) // 4)  # Adaptive window size
    for param in weight_params:
        rolling_std = df_params[param].rolling(window=window_size, center=True).std()
        ax2.plot(
            df_params["generation"],
            rolling_std,
            label=param.replace("_", " ").title(),
            alpha=0.7,
        )

    ax2.set_title(f"Parameter Stability (Rolling Std Dev, window={window_size})")
    ax2.set_ylabel("Standard Deviation")
    ax2.grid(True, alpha=0.3)
    ax2.legend(bbox_to_anchor=(1.05, 1), loc="upper left")

    # 3. Convergence rate analysis
    if convergence_file and Path(convergence_file).exists():
        df_conv = pd.read_csv(convergence_file)
        ax3 = axes[1, 0]

        # Calculate magnitude of changes
        change_cols = [col for col in df_conv.columns if col.endswith("_change")]
        total_change = df_conv[change_cols].abs().sum(axis=1)

        ax3.plot(df_conv["generation"], total_change, "r-", linewidth=2)
        ax3.set_title("Total Parameter Change Magnitude")
        ax3.set_ylabel("Sum of Absolute Changes")
        ax3.set_xlabel("Generation")
        ax3.grid(True, alpha=0.3)

        # Add trend line
        if len(total_change) > 5:
            z = np.polyfit(df_conv["generation"], total_change, 1)
            p = np.poly1d(z)
            ax3.plot(
                df_conv["generation"],
                p(df_conv["generation"]),
                "r--",
                alpha=0.8,
                label=f"Trend: {z[0]:.4f}x + {z[1]:.4f}",
            )
            ax3.legend()

    # 4. Parameter correlation heatmap (final generation)
    ax4 = axes[1, 1]
    final_params = df_params.iloc[-1][weight_params + ["win_score", "loss_score"]]

    # Create correlation matrix for final generation vs initial
    initial_params = df_params.iloc[0][weight_params + ["win_score", "loss_score"]]
    correlation_data = pd.DataFrame({"Initial": initial_params, "Final": final_params})

    sns.heatmap(correlation_data.corr(), annot=True, cmap="coolwarm", center=0, ax=ax4)
    ax4.set_title("Initial vs Final Parameter Correlation")

    plt.tight_layout()
    return fig


def main():
    if len(sys.argv) < 2:
        print(
            "Usage: python scripts/plot_evolution.py <params_csv_file> [convergence_csv_file]"
        )
        sys.exit(1)

    params_file = sys.argv[1]
    convergence_file = sys.argv[2] if len(sys.argv) > 2 else None

    if not Path(params_file).exists():
        print(f"Error: {params_file} not found")
        sys.exit(1)

    # Set style
    plt.style.use("seaborn-v0_8")
    sns.set_palette("husl")

    # Create plots
    print("Creating parameter evolution plot...")
    fig1 = plot_parameters(params_file)
    fig1.savefig(
        f"{Path(params_file).stem}_parameters.png", dpi=300, bbox_inches="tight"
    )

    if convergence_file and Path(convergence_file).exists():
        print("Creating convergence analysis plot...")
        fig2 = plot_convergence(convergence_file)
        fig2.savefig(
            f"{Path(convergence_file).stem}_convergence.png",
            dpi=300,
            bbox_inches="tight",
        )

        print("Creating convergence summary plot...")
        fig3 = plot_convergence_summary(params_file, convergence_file)
        fig3.savefig(
            f"{Path(params_file).stem}_summary.png", dpi=300, bbox_inches="tight"
        )

    print("Plots saved! Check the generated PNG files.")
    print("\nExpected convergence patterns:")
    print("- Parameters should stabilize after ~20-30 generations")
    print("- Fitness should plateau around 0.8-0.9")
    print("- Diversity should decrease as population converges")
    print("- Large parameter swings suggest insufficient evaluation")


if __name__ == "__main__":
    import numpy as np

    main()
