use connect_four_ai_core::training::{
    evaluate_ai_performance, train_genetic_algorithm, TrainingConfig,
};
use connect_four_ai_core::{
    ml_ai::MLAI,
    self_play::{SelfPlayConfig, SelfPlayTrainer},
    GameState,
};
use serde_json;
use std::env;
use std::fs;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!(
            "Usage: {} <train|evaluate|generate_data|self_play> [config_file]",
            args[0]
        );
        println!();
        println!("Commands:");
        println!("  train         - Run genetic algorithm training");
        println!("  evaluate      - Evaluate AI performance");
        println!("  generate_data - Generate self-play training data");
        println!("  self_play     - Run advanced self-play with MCTS");
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

        "generate_data" => {
            // Self-play data generation mode
            let default_config = "ml/config/training.json".to_string();
            let config_file = args.get(2).unwrap_or(&default_config);

            println!("=== Connect Four Self-Play Data Generation ===");
            println!("Config file: {}", config_file);
            println!("=============================================");

            let start_time = Instant::now();

            // Load configuration
            let config_content = fs::read_to_string(config_file)?;
            let config: serde_json::Value = serde_json::from_str(&config_content)?;

            let num_games = config["training_defaults"]["num_games"]
                .as_u64()
                .unwrap_or(1000) as usize;
            let depth = config["training_defaults"]["depth"].as_u64().unwrap_or(3) as usize;
            let output_file = config["output_formats"]["unified"]
                .as_str()
                .unwrap_or("ml/data/weights/self_play_data.json");

            println!("Number of games: {}", num_games);
            println!("Search depth: {}", depth);
            println!("Output file: {}", output_file);

            // Generate self-play data
            println!("\n🎮 Generating self-play training data...");
            let training_data = generate_self_play_data(num_games, depth)?;

            // Save training data
            if let Some(parent) = std::path::Path::new(output_file).parent() {
                std::fs::create_dir_all(parent)?;
            }

            let output_data = serde_json::json!({
                "metadata": {
                    "num_games": num_games,
                    "depth": depth,
                    "generated_at": chrono::Utc::now().to_rfc3339(),
                    "version": "1.0"
                },
                "training_data": training_data
            });

            fs::write(output_file, serde_json::to_string_pretty(&output_data)?)?;

            let total_time = start_time.elapsed();
            println!("\n=== Data Generation Complete ===");
            println!("Total time: {:.2} seconds", total_time.as_secs_f64());
            println!("Games generated: {}", num_games);
            println!("Training samples: {}", training_data.len());
            println!("Output file: {}", output_file);
            println!("================================");
        }

        "self_play" => {
            // Advanced self-play with MCTS
            let default_config = "ml/config/training.json".to_string();
            let config_file = args.get(2).unwrap_or(&default_config);

            println!("=== Connect Four Advanced Self-Play Training ===");
            println!("Config file: {}", config_file);
            println!("================================================");

            let start_time = Instant::now();

            // Load configuration
            let config_content = fs::read_to_string(config_file)?;
            let config: serde_json::Value = serde_json::from_str(&config_content)?;

            let num_games = config["training_defaults"]["num_games"]
                .as_u64()
                .unwrap_or(1000) as usize;
            let mcts_simulations = config["training_defaults"]
                .get("mcts_simulations")
                .and_then(|v| v.as_u64())
                .unwrap_or(800) as usize;
            let output_file = config["output_formats"]["unified"]
                .as_str()
                .unwrap_or("ml/data/weights/advanced_self_play_data.json");

            println!("Number of games: {}", num_games);
            println!("MCTS simulations: {}", mcts_simulations);
            println!("Output file: {}", output_file);

            // Create self-play configuration
            let mut self_play_config = SelfPlayConfig::default();
            self_play_config.num_games = num_games;
            self_play_config.mcts_simulations = mcts_simulations;

            // Run advanced self-play training
            println!("\n🎮 Starting advanced self-play training with MCTS...");
            let mut trainer = SelfPlayTrainer::new(self_play_config);
            let training_data = trainer.generate_training_data();

            // Save training data
            trainer.save_training_data(&training_data, output_file)?;

            let total_time = start_time.elapsed();
            println!("\n=== Advanced Self-Play Complete ===");
            println!("Total time: {:.2} seconds", total_time.as_secs_f64());
            println!("Games played: {}", num_games);
            println!("Training samples: {}", training_data.len());
            println!("Output file: {}", output_file);
            println!("===================================");
        }

        _ => {
            println!("Unknown command: {}", command);
            println!("Available commands: train, evaluate, generate_data, self_play");
        }
    }

    Ok(())
}

fn generate_self_play_data(
    num_games: usize,
    _depth: usize,
) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let mut training_data = Vec::new();
    let mut ai = MLAI::new();

    // Try to load existing weights if available
    if let Ok(weights) = fs::read_to_string("ml/data/weights/ml_ai_weights.json") {
        if let Ok(weights_data) = serde_json::from_str::<serde_json::Value>(&weights) {
            if let (Some(value_weights), Some(policy_weights)) = (
                weights_data["value_network"]["weights"].as_array(),
                weights_data["policy_network"]["weights"].as_array(),
            ) {
                let value_weights: Vec<f32> = value_weights
                    .iter()
                    .filter_map(|w| w.as_f64().map(|x| x as f32))
                    .collect();
                let policy_weights: Vec<f32> = policy_weights
                    .iter()
                    .filter_map(|w| w.as_f64().map(|x| x as f32))
                    .collect();

                ai.load_weights(&value_weights, &policy_weights);
                println!("✅ Loaded existing weights for self-play");
            }
        }
    }

    for game_idx in 0..num_games {
        if game_idx % 100 == 0 {
            println!("🎮 Generating game {}/{}", game_idx + 1, num_games);
        }

        let mut game_state = GameState::new();
        let mut game_moves = Vec::new();
        let mut game_features = Vec::new();
        let mut game_values = Vec::new();
        let mut game_policies = Vec::new();

        // Play the game
        while !game_state.is_game_over() {
            let features =
                connect_four_ai_core::features::GameFeatures::from_game_state(&game_state);
            let features_array = features.to_array();

            // Get AI evaluation
            let response = ai.get_best_move(&game_state);
            let valid_moves = game_state.get_valid_moves();

            // Create policy target (one-hot encoding of the chosen move)
            let mut policy_target = vec![0.0; 7];
            if let Some(chosen_move) = response.r#move {
                if valid_moves.contains(&chosen_move) {
                    policy_target[chosen_move as usize] = 1.0;
                }
            }

            // Store training data for this position
            game_features.push(features_array.to_vec());
            game_values.push(response.evaluation);
            game_policies.push(policy_target);
            game_moves.push(response.r#move);

            // Make the move
            if let Some(mv) = response.r#move {
                if game_state.make_move(mv).is_err() {
                    break;
                }
            } else {
                break;
            }
        }

        // Determine game result and create final targets
        let game_result = if game_state.get_winner().is_some() {
            if game_state.get_winner().unwrap() == connect_four_ai_core::Player::Player1 {
                1.0 // Win
            } else {
                -1.0 // Loss
            }
        } else {
            0.0 // Draw
        };

        // Create training samples from the game
        for (i, ((features, value), policy)) in game_features
            .iter()
            .zip(game_values.iter())
            .zip(game_policies.iter())
            .enumerate()
        {
            // Adjust value based on game result and move number
            let move_discount = 0.95_f32.powi(i as i32);
            let adjusted_value = game_result * move_discount;

            training_data.push(serde_json::json!({
                "features": features,
                "value_target": adjusted_value,
                "policy_target": policy,
                "game_idx": game_idx,
                "move_idx": i,
                "game_result": game_result,
                "original_value": value
            }));
        }
    }

    Ok(training_data)
}
