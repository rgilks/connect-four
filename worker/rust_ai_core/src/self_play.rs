use crate::features::GameFeatures;
use crate::{mcts::MCTS, ml_ai::MLAI, GameState, Player};
use rayon::prelude::*;
use serde_json;
use std::fs;
use std::time::Instant;

#[derive(Clone)]
pub struct SelfPlayConfig {
    pub num_games: usize,
    pub mcts_simulations: usize,
    pub exploration_constant: f32,
    pub temperature: f32,
    pub dirichlet_alpha: f32,
    pub dirichlet_epsilon: f32,
    pub save_every: usize,
}

impl Default for SelfPlayConfig {
    fn default() -> Self {
        Self {
            num_games: 1000,
            mcts_simulations: 800,
            exploration_constant: 1.0,
            temperature: 1.0,
            dirichlet_alpha: 0.3,
            dirichlet_epsilon: 0.25,
            save_every: 100,
        }
    }
}

pub struct SelfPlayTrainer {
    pub config: SelfPlayConfig,
    pub ai: MLAI,
    pub mcts: MCTS,
}

impl SelfPlayTrainer {
    pub fn new(config: SelfPlayConfig) -> Self {
        // Optimize CPU usage for M1 Macs
        SelfPlayTrainer::optimize_cpu_usage();

        let mut ai = MLAI::new();

        // Try to load existing weights
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
                    println!("✅ Loaded existing weights for self-play training");
                }
            }
        }

        let mcts = MCTS::new(config.exploration_constant, config.mcts_simulations);

        Self { config, ai, mcts }
    }

    fn optimize_cpu_usage() {
        // Use a static flag to ensure we only configure the thread pool once
        static INITIALIZED: std::sync::Once = std::sync::Once::new();

        INITIALIZED.call_once(|| {
            // Detect system architecture and optimize thread pool
            let num_cores = std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4);

            // For M1 Macs (8 performance cores), use all performance cores for maximum training speed
            let optimal_threads = if cfg!(target_arch = "aarch64") && cfg!(target_os = "macos") {
                // Apple Silicon - use all performance cores
                std::cmp::min(num_cores, 8)
            } else if num_cores >= 16 {
                // High-core systems - leave 2 cores for system
                num_cores.saturating_sub(2)
            } else if num_cores >= 8 {
                // Mid-range systems - leave 1 core for system
                num_cores.saturating_sub(1)
            } else {
                // Smaller systems - use all cores
                num_cores
            };

            // Configure rayon thread pool for optimal performance
            rayon::ThreadPoolBuilder::new()
                .num_threads(optimal_threads)
                .stack_size(8 * 1024 * 1024) // 8MB stack for deep recursion
                .build_global()
                .unwrap_or_else(|_| {
                    println!("⚠️  Could not configure optimal thread pool, using default");
                });

            println!(
                "🍎 CPU Optimization: Using {} performance cores on {} total cores",
                optimal_threads, num_cores
            );
        });
    }

    pub fn generate_training_data(&mut self) -> Vec<serde_json::Value> {
        println!(
            "🎮 Starting self-play training with {} games",
            self.config.num_games
        );
        println!(
            "MCTS simulations per move: {}",
            self.config.mcts_simulations
        );
        println!("Temperature: {}", self.config.temperature);
        println!(
            "🔄 Using parallel processing with batch size: {}",
            std::cmp::min(10, self.config.num_games)
        );

        let start_time = Instant::now();

        // Use parallel iterator for game generation
        let mut training_data = Vec::new();

        // Process games in parallel batches
        let batch_size = std::cmp::min(10, self.config.num_games); // Adapt batch size to number of games
        println!(
            "🔄 Starting batch processing with {} games per batch",
            batch_size
        );
        for (batch_idx, batch_start) in (0..self.config.num_games).step_by(batch_size).enumerate() {
            let batch_end = std::cmp::min(batch_start + batch_size, self.config.num_games);
            // Only log batch start for every 10th batch or if total games <= 100
            if self.config.num_games <= 100 || batch_idx % 10 == 0 {
                println!(
                    "🔄 Processing batch {}: games {} to {}",
                    batch_idx + 1,
                    batch_start,
                    batch_end - 1
                );
            }
            let batch_games: Vec<Vec<serde_json::Value>> = (batch_start..batch_end)
                .into_par_iter()
                .map(|game_idx| {
                    // Show progress for every game when total games is small, or every 10% otherwise
                    let progress_interval = if self.config.num_games <= 10 {
                        1 // Show progress for every game
                    } else {
                        std::cmp::max(self.config.num_games / 10, 10)
                    };
                    if game_idx % progress_interval == 0 {
                        let elapsed = start_time.elapsed();
                        let games_per_sec = (game_idx + 1) as f64 / elapsed.as_secs_f64();
                        let percent_complete = ((game_idx + 1) * 100) / self.config.num_games;
                        println!(
                            "🎮 Progress: {}% ({}/{}) - {:.1} games/sec",
                            percent_complete,
                            game_idx + 1,
                            self.config.num_games,
                            games_per_sec
                        );
                    }

                    // Create a new trainer instance for each thread
                    // Only log game start/completion for every 100th game or if total games <= 100
                    if self.config.num_games <= 100 || game_idx % 100 == 0 {
                        println!("🎮 Starting game {}", game_idx + 1);
                    }
                    let mut thread_trainer = SelfPlayTrainer::new(self.config.clone());
                    let result = thread_trainer.play_game(game_idx);
                    if self.config.num_games <= 100 || game_idx % 100 == 0 {
                        println!("✅ Completed game {}", game_idx + 1);
                    }
                    result
                })
                .collect();

            // Flatten batch results
            for game_data in batch_games {
                training_data.extend(game_data);
            }
            // Only log batch completion for every 10th batch or if total games <= 100
            if self.config.num_games <= 100 || batch_idx % 10 == 0 {
                println!("✅ Batch {} complete, total samples: {}", batch_idx + 1, training_data.len());
            }
        }

        let total_time = start_time.elapsed();
        println!("🎉 Self-play generation complete!");
        println!("⏱️  Total time: {:.2} seconds", total_time.as_secs_f64());
        println!("🎮 Games played: {}", self.config.num_games);
        println!("📊 Training samples: {}", training_data.len());
        println!(
            "📈 Average samples per game: {:.1}",
            training_data.len() as f64 / self.config.num_games as f64
        );
        println!("🚀 Ready for neural network training...");

        training_data
    }

    fn play_game(&mut self, game_idx: usize) -> Vec<serde_json::Value> {
        let mut game_state = GameState::new();
        let mut game_data = Vec::new();
        let mut move_count = 0;

        // Only log game start for every 100th game or if total games <= 100
        if self.config.num_games <= 100 || game_idx % 100 == 0 {
            println!(
                "  🎯 Game {}: Starting with {} MCTS simulations",
                game_idx + 1,
                self.config.mcts_simulations
            );
        }

        while !game_state.is_game_over() {
            let features = GameFeatures::from_game_state(&game_state);
            let features_array = features.to_array();

            // Get current policy (unused but kept for potential future use)
            let _current_policy = self.get_policy(&game_state);

            // Get MCTS move probabilities
            // Only log MCTS progress for every 50th move or if total games <= 10
            let should_log_moves =
                self.config.num_games <= 10 || (game_idx % 100 == 0 && move_count % 50 == 0);
            if should_log_moves {
                println!(
                    "  🎯 Game {}: Move {}, running MCTS search...",
                    game_idx + 1,
                    move_count + 1
                );
            }

            let mcts_start = std::time::Instant::now();
            let (best_move, move_probs) = {
                let value_fn = |state: &GameState| self.ai.evaluate_position(state);
                let policy_fn = |state: &GameState| {
                    let features = GameFeatures::from_game_state(state);
                    let features_array = features.to_array();
                    let (_, policy_network) = self.ai.get_networks();
                    let policy_output = policy_network.forward(&features_array);

                    let mut policy = Vec::new();
                    let mut sum = 0.0;
                    for i in 0..7 {
                        let prob = policy_output[i].exp();
                        policy.push(prob);
                        sum += prob;
                    }
                    if sum > 0.0 {
                        for prob in &mut policy {
                            *prob /= sum;
                        }
                    } else {
                        for prob in &mut policy {
                            *prob = 1.0 / 7.0;
                        }
                    }
                    policy
                };

                self.mcts.search(game_state.clone(), &value_fn, &policy_fn)
            };
            let mcts_duration = mcts_start.elapsed();

            // Only log MCTS completion for the same conditions as start
            if should_log_moves {
                println!(
                    "  ✅ Game {}: Move {}, MCTS completed in {:.2}s",
                    game_idx + 1,
                    move_count + 1,
                    mcts_duration.as_secs_f64()
                );
            }

            // Add Dirichlet noise for exploration
            let noisy_probs = self.add_dirichlet_noise(&move_probs);

            // Store training data
            game_data.push(serde_json::json!({
                "features": features_array.to_vec(),
                "policy_target": noisy_probs,
                "game_idx": game_idx,
                "move_idx": move_count,
                "player": if game_state.current_player == Player::Player1 { "player1" } else { "player2" }
            }));

            // Make the move
            if game_state.make_move(best_move).is_err() {
                break;
            }
            move_count += 1;
        }

        // Determine game result
        let game_result = if let Some(winner) = game_state.get_winner() {
            match winner {
                Player::Player1 => 1.0,
                Player::Player2 => -1.0,
            }
        } else {
            0.0 // Draw
        };

        // Update all training samples with the final result
        for sample in &mut game_data {
            let move_idx = sample["move_idx"].as_u64().unwrap_or(0) as usize;
            let move_discount = 0.95_f32.powi(move_idx as i32);
            let adjusted_value = game_result * move_discount;

            sample["value_target"] = serde_json::Value::Number(
                serde_json::Number::from_f64(adjusted_value as f64).unwrap(),
            );
            sample["game_result"] = serde_json::Value::Number(
                serde_json::Number::from_f64(game_result as f64).unwrap(),
            );
        }

        game_data
    }

    fn get_policy(&self, state: &GameState) -> Vec<f32> {
        let features = GameFeatures::from_game_state(state);
        let features_array = features.to_array();

        // Get policy from neural network
        let (_, policy_network) = self.ai.get_networks();
        let policy_output = policy_network.forward(&features_array);

        // Convert to probabilities
        let mut policy = Vec::new();
        let mut sum = 0.0;

        for i in 0..7 {
            let prob = policy_output[i].exp();
            policy.push(prob);
            sum += prob;
        }

        // Normalize
        if sum > 0.0 {
            for prob in &mut policy {
                *prob /= sum;
            }
        } else {
            // Fallback to uniform distribution
            for prob in &mut policy {
                *prob = 1.0 / 7.0;
            }
        }

        policy
    }

    fn add_dirichlet_noise(&self, probs: &[f32]) -> Vec<f32> {
        use rand::thread_rng;
        use rand::Rng;

        let mut rng = thread_rng();

        // Generate Dirichlet-like noise using gamma distribution approximation
        let mut noise = Vec::new();
        let mut sum = 0.0;

        for _ in 0..probs.len() {
            let gamma_sample = rng.gen_range(0.1..1.0); // Simplified gamma-like distribution
            noise.push(gamma_sample);
            sum += gamma_sample;
        }

        // Normalize noise
        for noise_val in &mut noise {
            *noise_val /= sum;
        }

        let mut result = Vec::new();
        for (i, &prob) in probs.iter().enumerate() {
            let noisy_prob = (1.0 - self.config.dirichlet_epsilon) * prob
                + self.config.dirichlet_epsilon * noise[i];
            result.push(noisy_prob);
        }

        // Renormalize
        let sum: f32 = result.iter().sum();
        if sum > 0.0 {
            for prob in &mut result {
                *prob /= sum;
            }
        }

        result
    }

    pub fn save_training_data(
        &self,
        training_data: &[serde_json::Value],
        filename: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let output_data = serde_json::json!({
            "metadata": {
                "num_games": self.config.num_games,
                "mcts_simulations": self.config.mcts_simulations,
                "exploration_constant": self.config.exploration_constant,
                "temperature": self.config.temperature,
                "dirichlet_alpha": self.config.dirichlet_alpha,
                "dirichlet_epsilon": self.config.dirichlet_epsilon,
                "generated_at": chrono::Utc::now().to_rfc3339(),
                "version": "2.0"
            },
            "training_data": training_data
        });

        if let Some(parent) = std::path::Path::new(filename).parent() {
            std::fs::create_dir_all(parent)?;
        }

        fs::write(filename, serde_json::to_string_pretty(&output_data)?)?;
        println!("💾 Training data saved to: {}", filename);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_play_config_default() {
        let config = SelfPlayConfig::default();
        assert_eq!(config.num_games, 1000);
        assert_eq!(config.mcts_simulations, 800);
        assert_eq!(config.exploration_constant, 1.0);
        assert_eq!(config.temperature, 1.0);
    }

    #[test]
    fn test_self_play_trainer_creation() {
        let config = SelfPlayConfig::default();
        let trainer = SelfPlayTrainer::new(config);
        assert_eq!(trainer.config.num_games, 1000);
    }

    #[test]
    fn test_dirichlet_noise() {
        let config = SelfPlayConfig::default();
        let trainer = SelfPlayTrainer::new(config);

        let probs = vec![0.5, 0.3, 0.2];
        let noisy_probs = trainer.add_dirichlet_noise(&probs);

        assert_eq!(noisy_probs.len(), 3);
        assert!((noisy_probs.iter().sum::<f32>() - 1.0).abs() < 0.001);
    }
}
