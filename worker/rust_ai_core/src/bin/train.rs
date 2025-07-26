use connect_four_ai_core::training::{
    evaluate_ai_performance, train_genetic_algorithm, TrainingConfig,
};
use std::env;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <train|evaluate> [config_file]", args[0]);
        println!();
        println!("Commands:");
        println!("  train     - Run genetic algorithm training");
        println!("  evaluate  - Evaluate AI performance");
        return Ok(());
    }

    let command = &args[1];

    match command.as_str() {
        "train" => {
            // Genetic algorithm training mode
            let population_size = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(50);
            let generations = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(100);
            let mutation_rate = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(0.1);
            let mutation_strength = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(0.2);
            let crossover_rate = args.get(6).and_then(|s| s.parse().ok()).unwrap_or(0.7);
            let games_per_evaluation = args.get(7).and_then(|s| s.parse().ok()).unwrap_or(10);

            println!("=== Connect Four AI Genetic Training ===");
            println!("Population size: {}", population_size);
            println!("Generations: {}", generations);
            println!("Mutation rate: {}", mutation_rate);
            println!("Mutation strength: {}", mutation_strength);
            println!("Crossover rate: {}", crossover_rate);
            println!("Games per evaluation: {}", games_per_evaluation);
            println!("========================================");

            let start_time = Instant::now();

            let config = TrainingConfig {
                population_size,
                generations,
                mutation_rate,
                mutation_strength,
                crossover_rate,
                tournament_size: 3,
                games_per_evaluation,
                max_moves_per_game: 42,
            };

            println!("\n🧬 Starting genetic algorithm training...");
            let result = train_genetic_algorithm(config);

            let total_time = start_time.elapsed();

            println!("\n=== Training Complete ===");
            println!("Total time: {:.2} seconds", total_time.as_secs_f64());
            println!("Best fitness: {:.3}", result.best_fitness);
            println!("Generations completed: {}", result.generation_history.len());
            println!("Best parameters:");
            println!(
                "  Center control weight: {:.3}",
                result.best_params.center_control_weight
            );
            println!(
                "  Piece count weight: {:.3}",
                result.best_params.piece_count_weight
            );
            println!("  Threat weight: {:.3}", result.best_params.threat_weight);
            println!(
                "  Mobility weight: {:.3}",
                result.best_params.mobility_weight
            );
            println!(
                "  Vertical control weight: {:.3}",
                result.best_params.vertical_control_weight
            );
            println!(
                "  Horizontal control weight: {:.3}",
                result.best_params.horizontal_control_weight
            );
            println!("========================");

            // Save the best parameters
            let output_file = "ml/data/genetic_params/evolved.json";
            if let Some(parent) = std::path::Path::new(output_file).parent() {
                std::fs::create_dir_all(parent)?;
            }
            result.best_params.save_to_file(output_file)?;
            println!("💾 Best parameters saved to: {}", output_file);
        }

        "evaluate" => {
            // AI performance evaluation mode
            let num_games = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(100);

            println!("=== Connect Four AI Performance Evaluation ===");
            println!("Games: {}", num_games);
            println!("=============================================");

            let start_time = Instant::now();

            // Load evolved parameters if available
            let evolved_params =
                connect_four_ai_core::genetic_params::GeneticParams::load_from_file(
                    "ml/data/genetic_params/evolved.json",
                )
                .unwrap_or_else(|_| connect_four_ai_core::genetic_params::GeneticParams::default());

            println!("\n📊 Evaluating AI performance...");
            let performance = evaluate_ai_performance(&evolved_params, num_games);

            let total_time = start_time.elapsed();

            println!("\n=== Evaluation Results ===");
            println!("Total time: {:.2} seconds", total_time.as_secs_f64());
            println!(
                "Win rate: {:.1}%",
                performance.get("win_rate").unwrap_or(&0.0) * 100.0
            );
            println!(
                "Draw rate: {:.1}%",
                performance.get("draw_rate").unwrap_or(&0.0) * 100.0
            );
            println!(
                "Loss rate: {:.1}%",
                performance.get("loss_rate").unwrap_or(&0.0) * 100.0
            );
            println!(
                "Average moves: {:.1}",
                performance.get("avg_moves").unwrap_or(&0.0)
            );
            println!(
                "Average time per move: {:.1}ms",
                performance.get("avg_time_ms").unwrap_or(&0.0)
            );
            println!("==========================");
        }

        _ => {
            println!("Unknown command: {}", command);
            println!("Available commands: train, evaluate");
        }
    }

    Ok(())
}
