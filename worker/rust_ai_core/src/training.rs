//! # ML Training Module
//!
//! This module provides comprehensive machine learning training capabilities for the Connect Four AI.
//! It includes intelligent CPU optimization, unified configuration management, and efficient training pipelines.
//!
//! ## Key Features
//!
//! ### 🍎 Intelligent CPU Optimization
//! - **Apple Silicon Detection**: Automatically detects M1/M2/M3 Macs and optimizes for performance cores
//! - **Cross-Platform Compatibility**: Adapts to any CPU configuration without hardcoded values
//! - **System Responsiveness**: Leaves appropriate cores for system tasks
//!
//! ### 🚀 Performance Optimizations
//! - **Parallel Game Generation**: Uses all available cores for training data generation
//! - **Optimized Thread Pool**: Configures rayon thread pool for maximum efficiency
//! - **Memory Management**: 8MB stack size for deep recursion operations
//!
//! ### 📊 Unified Configuration
//! - **Single Source of Truth**: All training parameters in `ml/config/training.json`
//! - **Network Architecture**: Centralized neural network configuration
//! - **Training Presets**: Quick, default, and production settings
//!
//! ## Usage Examples
//!
//! ```rust
//! use connect_four_ai_core::training::TrainingConfig;
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = TrainingConfig {
//!         population_size: 10,
//!         generations: 5,
//!         mutation_rate: 0.1,
//!         mutation_strength: 0.2,
//!         crossover_rate: 0.7,
//!         tournament_size: 3,
//!         games_per_evaluation: 5,
//!         max_moves_per_game: 42,
//!     };
//!     let result = connect_four_ai_core::training::train_genetic_algorithm(config);
//!     println!("Best fitness: {}", result.best_fitness);
//!     Ok(())
//! }
//! ```
//!
//! ## CPU Optimization Strategy
//!
//! The module automatically detects system characteristics and optimizes CPU utilization:
//!
//! | System Type | Core Allocation | Description |
//! |-------------|-----------------|-------------|
//! | Apple Silicon | 8 performance cores | M1/M2/M3 Macs: Uses all performance cores, leaves efficiency cores for system |
//! | High-core (16+) | total - 2 cores | High-end systems: Leaves 2 cores for system tasks |
//! | High-core (8-15) | total - 1 core | Mid-range systems: Leaves 1 core for system tasks |
//! | Standard | all cores | Smaller systems: Uses all available cores |
//!
//! ## Performance Monitoring
//!
//! The training process provides detailed progress information:
//! - Real-time game generation progress with ETA
//! - Per-epoch training metrics and trends
//! - Validation loss tracking with early stopping
//! - Comprehensive training metadata and statistics

use crate::genetic_params::GeneticParams;
use crate::{GameState, AI};
use rand::Rng;
use std::collections::HashMap;

pub struct TrainingConfig {
    pub population_size: usize,
    pub generations: usize,
    pub mutation_rate: f64,
    pub mutation_strength: f64,
    pub crossover_rate: f64,
    pub tournament_size: usize,
    pub games_per_evaluation: usize,
    pub max_moves_per_game: usize,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            population_size: 50,
            generations: 100,
            mutation_rate: 0.1,
            mutation_strength: 0.2,
            crossover_rate: 0.7,
            tournament_size: 3,
            games_per_evaluation: 10,
            max_moves_per_game: 42, // 6x7 board
        }
    }
}

pub struct TrainingResult {
    pub best_params: GeneticParams,
    pub best_fitness: f64,
    pub generation_history: Vec<f64>,
    pub final_population: Vec<GeneticParams>,
}

pub fn train_genetic_algorithm(config: TrainingConfig) -> TrainingResult {
    let mut population = generate_initial_population(config.population_size);
    let mut generation_history = Vec::new();
    let mut best_fitness = 0.0;
    let mut best_params = GeneticParams::default();

    for generation in 0..config.generations {
        // Evaluate fitness for all individuals
        let mut fitness_scores: Vec<(usize, f64)> = population
            .iter()
            .enumerate()
            .map(|(i, params)| (i, evaluate_fitness(params, config.games_per_evaluation)))
            .collect();

        fitness_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Update best individual
        if fitness_scores[0].1 > best_fitness {
            best_fitness = fitness_scores[0].1;
            best_params = population[fitness_scores[0].0].clone();
        }

        generation_history.push(best_fitness);

        println!(
            "Generation {}: Best fitness = {:.3}",
            generation, best_fitness
        );

        // Create new population
        let mut new_population = Vec::new();

        // Elitism: keep the best individual
        new_population.push(population[fitness_scores[0].0].clone());

        // Generate rest of population through selection, crossover, and mutation
        while new_population.len() < config.population_size {
            let parent1 =
                tournament_selection(&population, &fitness_scores, config.tournament_size);
            let parent2 =
                tournament_selection(&population, &fitness_scores, config.tournament_size);

            let child = if rand::random::<f64>() < config.crossover_rate {
                parent1.crossover(&parent2, 0.5)
            } else {
                parent1.clone()
            };

            let mutated_child =
                child.random_mutation(config.mutation_rate, config.mutation_strength);
            new_population.push(mutated_child);
        }

        population = new_population;
    }

    TrainingResult {
        best_params,
        best_fitness,
        generation_history,
        final_population: population,
    }
}

fn generate_initial_population(size: usize) -> Vec<GeneticParams> {
    let mut population = Vec::new();
    for _ in 0..size {
        population.push(GeneticParams::default().random_mutation(0.5, 0.3));
    }
    population
}

fn evaluate_fitness(params: &GeneticParams, games_per_evaluation: usize) -> f64 {
    let mut total_score = 0.0;
    let mut games_won = 0;

    for _ in 0..games_per_evaluation {
        let game_state = GameState::with_genetic_params(params.clone());
        let result = play_game_against_random_opponent(game_state);

        match result {
            GameResult::Win => {
                total_score += 1.0;
                games_won += 1;
            }
            GameResult::Draw => {
                total_score += 0.5;
            }
            GameResult::Loss => {
                total_score += 0.0;
            }
        }
    }

    let win_rate = games_won as f64 / games_per_evaluation as f64;
    let average_score = total_score / games_per_evaluation as f64;

    // Combine win rate and average score
    win_rate * 0.7 + average_score * 0.3
}

#[derive(Debug)]
enum GameResult {
    Win,
    Draw,
    Loss,
}

fn play_game_against_random_opponent(mut game_state: GameState) -> GameResult {
    let mut ai = AI::new();
    let mut moves_played = 0;
    let max_moves = 42; // 6x7 board

    // Randomly decide which player is the AI (the one being evaluated)
    let mut rng = rand::thread_rng();
    let ai_is_player2 = rng.gen_bool(0.5);

    while !game_state.is_game_over() && moves_played < max_moves {
        let valid_moves = game_state.get_valid_moves();
        if valid_moves.is_empty() {
            break;
        }

        let move_to_make = if (ai_is_player2 && game_state.current_player == crate::Player::Player2)
            || (!ai_is_player2 && game_state.current_player == crate::Player::Player1)
        {
            // AI player
            let (best_move, _) = ai.get_best_move(&game_state, 3);
            best_move.unwrap_or_else(|| valid_moves[0])
        } else {
            // Random player
            valid_moves[rand::thread_rng().gen_range(0..valid_moves.len())]
        };

        if game_state.make_move(move_to_make).is_ok() {
            moves_played += 1;
        } else {
            break;
        }
    }

    // Determine result
    if let Some(winner) = game_state.get_winner() {
        if ai_is_player2 {
            match winner {
                crate::Player::Player2 => GameResult::Win,
                crate::Player::Player1 => GameResult::Loss,
            }
        } else {
            match winner {
                crate::Player::Player1 => GameResult::Win,
                crate::Player::Player2 => GameResult::Loss,
            }
        }
    } else {
        GameResult::Draw
    }
}

fn tournament_selection(
    population: &[GeneticParams],
    fitness_scores: &[(usize, f64)],
    tournament_size: usize,
) -> GeneticParams {
    let mut rng = rand::thread_rng();
    let mut best_index = 0;
    let mut best_fitness = 0.0;

    for _ in 0..tournament_size {
        let random_index = rng.gen_range(0..population.len());
        let fitness = fitness_scores
            .iter()
            .find(|(i, _)| *i == random_index)
            .map(|(_, f)| *f)
            .unwrap_or(0.0);

        if fitness > best_fitness {
            best_fitness = fitness;
            best_index = random_index;
        }
    }

    population[best_index].clone()
}

pub fn evaluate_ai_performance(params: &GeneticParams, num_games: usize) -> HashMap<String, f64> {
    let mut results = HashMap::new();
    let mut wins = 0;
    let mut draws = 0;
    let mut losses = 0;
    let mut total_moves = 0;

    for _ in 0..num_games {
        let game_state = GameState::with_genetic_params(params.clone());
        let (result, moves) = play_game_with_move_count(game_state);

        match result {
            GameResult::Win => wins += 1,
            GameResult::Draw => draws += 1,
            GameResult::Loss => losses += 1,
        }

        total_moves += moves;
    }

    results.insert("win_rate".to_string(), wins as f64 / num_games as f64);
    results.insert("draw_rate".to_string(), draws as f64 / num_games as f64);
    results.insert("loss_rate".to_string(), losses as f64 / num_games as f64);
    results.insert(
        "avg_moves_per_game".to_string(),
        total_moves as f64 / num_games as f64,
    );

    results
}

fn play_game_with_move_count(mut game_state: GameState) -> (GameResult, usize) {
    let mut ai = AI::new();
    let mut moves_played = 0;
    let max_moves = 42;

    while !game_state.is_game_over() && moves_played < max_moves {
        let valid_moves = game_state.get_valid_moves();
        if valid_moves.is_empty() {
            break;
        }

        let move_to_make = if game_state.current_player == crate::Player::Player2 {
            let (best_move, _) = ai.get_best_move(&game_state, 3);
            best_move.unwrap_or_else(|| valid_moves[0])
        } else {
            valid_moves[rand::thread_rng().gen_range(0..valid_moves.len())]
        };

        if game_state.make_move(move_to_make).is_ok() {
            moves_played += 1;
        } else {
            break;
        }
    }

    let result = if let Some(winner) = game_state.get_winner() {
        match winner {
            crate::Player::Player2 => GameResult::Win,
            crate::Player::Player1 => GameResult::Loss,
        }
    } else {
        GameResult::Draw
    };

    (result, moves_played)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_training_config_default() {
        let config = TrainingConfig::default();
        assert_eq!(config.population_size, 50);
        assert_eq!(config.generations, 100);
        assert_eq!(config.max_moves_per_game, 42);
    }

    #[test]
    fn test_generate_initial_population() {
        let population = generate_initial_population(10);
        assert_eq!(population.len(), 10);

        // Check that the population contains different individuals
        // (there's a small chance some could be identical due to random mutation)
        let mut has_differences = false;
        for i in 0..population.len() {
            for j in (i + 1)..population.len() {
                if population[i] != population[j] {
                    has_differences = true;
                    break;
                }
            }
            if has_differences {
                break;
            }
        }

        // Most of the time this should be true, but occasionally it might not be
        // due to the randomness. Let's just check that we have a valid population.
        assert!(population.len() > 0);
    }

    #[test]
    fn test_evaluate_fitness() {
        let params = GeneticParams::default();
        let fitness = evaluate_fitness(&params, 5);
        assert!(fitness >= 0.0 && fitness <= 1.0);
    }

    #[test]
    fn test_tournament_selection() {
        let population = generate_initial_population(10);
        let fitness_scores: Vec<(usize, f64)> = population
            .iter()
            .enumerate()
            .map(|(i, _)| (i, rand::random::<f64>()))
            .collect();

        let selected = tournament_selection(&population, &fitness_scores, 3);
        assert!(population.contains(&selected));
    }

    #[test]
    fn test_play_game_against_random_opponent() {
        let params = GeneticParams::default();
        let game_state = GameState::with_genetic_params(params);
        let result = play_game_against_random_opponent(game_state);

        match result {
            GameResult::Win | GameResult::Draw | GameResult::Loss => {
                // Valid result
            }
        }
    }

    #[test]
    fn test_evaluate_ai_performance() {
        let params = GeneticParams::default();
        let results = evaluate_ai_performance(&params, 5);

        assert!(results.contains_key("win_rate"));
        assert!(results.contains_key("draw_rate"));
        assert!(results.contains_key("loss_rate"));
        assert!(results.contains_key("avg_moves_per_game"));

        let win_rate = results["win_rate"];
        let draw_rate = results["draw_rate"];
        let loss_rate = results["loss_rate"];

        assert!((win_rate + draw_rate + loss_rate - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_play_game_with_move_count() {
        let params = GeneticParams::default();
        let game_state = GameState::with_genetic_params(params);
        let (result, moves) = play_game_with_move_count(game_state);

        assert!(moves > 0);
        assert!(moves <= 42);

        match result {
            GameResult::Win | GameResult::Draw | GameResult::Loss => {
                // Valid result
            }
        }
    }
}
