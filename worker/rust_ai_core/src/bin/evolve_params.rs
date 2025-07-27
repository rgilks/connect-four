//! Genetic parameter evolution for Connect Four AI

use connect_four_ai_core::{genetic_params::GeneticParams, GameState, Player, AI};

use rayon::prelude::*;
use std::fs;

const POPULATION_SIZE: usize = 50;
const GENERATIONS: usize = 50;
const GAMES_PER_EVAL: usize = 100; // Back to original value for proper evolution
const MUTATION_RATE: f64 = 0.3;
const MUTATION_STRENGTH: f64 = 1.0;
const CROSSOVER_RATE: f64 = 0.5;

fn optimize_cpu_usage() {
    if cfg!(target_os = "macos") {
        let num_cores = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(8);
        let optimal_threads = (num_cores as f64 * 0.8) as usize;
        rayon::ThreadPoolBuilder::new()
            .num_threads(optimal_threads)
            .stack_size(8 * 1024 * 1024)
            .build_global()
            .unwrap_or_else(|_| {
                println!("Warning: Could not set optimal thread count, using default");
            });
        println!(
            "🍎 Apple Silicon detected: Using {} threads ({} cores available)",
            optimal_threads, num_cores
        );
    } else {
        let num_cores = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);
        rayon::ThreadPoolBuilder::new()
            .num_threads(num_cores)
            .stack_size(8 * 1024 * 1024)
            .build_global()
            .unwrap_or_else(|_| {
                println!("Warning: Could not set optimal thread count, using default");
            });
        println!("🖥️  Using {} threads for parallel processing", num_cores);
    }
}

// Tournament-style evaluation: evolved params vs default params
fn evaluate_params_tournament(evolved_params: &GeneticParams) -> f64 {
    let default_params = GeneticParams::default();

    let results: Vec<bool> = (0..GAMES_PER_EVAL)
        .into_par_iter()
        .map(|_| {
            let mut moves_played = 0;
            let max_moves = 42; // Maximum moves in Connect Four (6x7 board)

            // Randomly decide which player uses evolved parameters
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let evolved_is_player2 = rng.gen_bool(0.5);

            // Create neutral game state (doesn't matter which params we use for the game state itself)
            let mut game_state = GameState::new();

            while !game_state.is_game_over() && moves_played < max_moves {
                let current_player = game_state.current_player;
                let is_evolved_turn = if evolved_is_player2 {
                    current_player == Player::Player2
                } else {
                    current_player == Player::Player1
                };

                // Use different parameters for AI move calculation based on whose turn it is
                let ai_params = if is_evolved_turn {
                    evolved_params.clone()
                } else {
                    default_params.clone()
                };

                // Create a temporary state for AI move calculation
                let mut ai_state = GameState::with_genetic_params(ai_params);
                ai_state.board = game_state.board.clone();
                ai_state.current_player = game_state.current_player;

                let mut ai = AI::new();
                let (best_move, _) = ai.get_best_move(&ai_state, 5);

                if let Some(column) = best_move {
                    game_state.make_move(column).ok();
                } else {
                    // No valid moves, game is a draw
                    break;
                }
                moves_played += 1;
            }

            // Determine winner - evolved params win if they are the winner
            if let Some(winner) = game_state.get_winner() {
                if evolved_is_player2 {
                    winner == Player::Player2
                } else {
                    winner == Player::Player1
                }
            } else {
                // Game ended in draw - this should be rare in Connect Four
                // For draws, we'll assign based on who had the advantage
                // Use a neutral evaluation approach to avoid bias
                let mut evolved_state = GameState::with_genetic_params(evolved_params.clone());
                evolved_state.board = game_state.board.clone();
                evolved_state.current_player = game_state.current_player;
                let evolved_eval = evolved_state.evaluate();

                let mut default_state = GameState::with_genetic_params(default_params.clone());
                default_state.board = game_state.board.clone();
                default_state.current_player = game_state.current_player;
                let default_eval = default_state.evaluate();

                // Compare evaluations from both perspectives
                // If evolved params think they're winning AND default params agree, then evolved wins
                if evolved_is_player2 {
                    evolved_eval < 0 && default_eval < 0 // Both think Player2 is winning
                } else {
                    evolved_eval > 0 && default_eval > 0 // Both think Player1 is winning
                }
            }
        })
        .collect();

    let wins = results.iter().filter(|&&won| won).count();
    let fitness = wins as f64 / GAMES_PER_EVAL as f64;

    fitness
}

fn validate_against_default(evolved_params: &GeneticParams, num_games: usize) -> f64 {
    let default_params = GeneticParams::default();
    let results: Vec<bool> = (0..num_games)
        .into_par_iter()
        .map(|_| {
            let mut game_state = GameState::new();
            let mut moves_played = 0;
            let max_moves = 42;

            // Randomly decide which player uses evolved parameters
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let evolved_is_player2 = rng.gen_bool(0.5);

            while !game_state.is_game_over() && moves_played < max_moves {
                let current_player = game_state.current_player;
                let is_evolved_turn = if evolved_is_player2 {
                    current_player == Player::Player2
                } else {
                    current_player == Player::Player1
                };

                let ai_params = if is_evolved_turn {
                    evolved_params.clone()
                } else {
                    default_params.clone()
                };

                let mut ai_state = GameState::with_genetic_params(ai_params);
                ai_state.board = game_state.board.clone();
                ai_state.current_player = game_state.current_player;

                let mut ai = AI::new();
                let (best_move, _) = ai.get_best_move(&ai_state, 5);

                if let Some(column) = best_move {
                    game_state.make_move(column).ok();
                } else {
                    break;
                }
                moves_played += 1;
            }

            if let Some(winner) = game_state.get_winner() {
                if evolved_is_player2 {
                    winner == Player::Player2
                } else {
                    winner == Player::Player1
                }
            } else {
                // Game ended in draw - use neutral evaluation approach
                let mut evolved_state = GameState::with_genetic_params(evolved_params.clone());
                evolved_state.board = game_state.board.clone();
                evolved_state.current_player = game_state.current_player;
                let evolved_eval = evolved_state.evaluate();

                let mut default_state = GameState::with_genetic_params(default_params.clone());
                default_state.board = game_state.board.clone();
                default_state.current_player = game_state.current_player;
                let default_eval = default_state.evaluate();

                // Compare evaluations from both perspectives
                if evolved_is_player2 {
                    evolved_eval < 0 && default_eval < 0
                } else {
                    evolved_eval > 0 && default_eval > 0
                }
            }
        })
        .collect();

    let wins = results.iter().filter(|&&won| won).count();
    wins as f64 / num_games as f64
}

fn crossover(parent1: &GeneticParams, parent2: &GeneticParams) -> GeneticParams {
    // Use the built-in crossover method from the GeneticParams struct
    parent1.crossover(parent2, CROSSOVER_RATE)
}

fn mutate(params: &mut GeneticParams) {
    // Use the built-in mutation method from the GeneticParams struct
    *params = params.random_mutation(MUTATION_RATE, MUTATION_STRENGTH);
}

fn main() {
    optimize_cpu_usage();
    println!("🧬 Starting Connect Four AI parameter evolution");
    println!("Population size: {}", POPULATION_SIZE);
    println!("Generations: {}", GENERATIONS);
    println!("Games per evaluation: {}", GAMES_PER_EVAL);

    let mut population: Vec<GeneticParams> = (0..POPULATION_SIZE)
        .map(|_| GeneticParams::random())
        .collect();

    let mut best_fitness = 0.0;
    let mut best_params = GeneticParams::default();

    for generation in 0..GENERATIONS {
        println!("\n🔄 Generation {}", generation + 1);

        // Evaluate fitness for all individuals
        let fitness_scores: Vec<f64> = population
            .par_iter()
            .map(|params| evaluate_params_tournament(params))
            .collect();

        // Find best individual
        let (best_idx, &best_score) = fitness_scores
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();

        if best_score > best_fitness {
            best_fitness = best_score;
            best_params = population[best_idx].clone();
            println!("🏆 New best fitness: {:.3}", best_fitness);
        }

        println!(
            "📊 Average fitness: {:.3}",
            fitness_scores.iter().sum::<f64>() / fitness_scores.len() as f64
        );
        println!("🏆 Best fitness: {:.3}", best_score);

        // Selection: Keep top 20% and tournament select the rest
        let elite_count = POPULATION_SIZE / 5;
        let mut new_population = Vec::new();

        // Keep elite individuals
        let mut indexed_fitness: Vec<(usize, f64)> = fitness_scores
            .iter()
            .enumerate()
            .map(|(i, &f)| (i, f))
            .collect();
        indexed_fitness.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

        for i in 0..elite_count {
            new_population.push(population[indexed_fitness[i].0].clone());
        }

        // Tournament selection for the rest
        while new_population.len() < POPULATION_SIZE {
            let tournament_size = 3;
            let mut tournament = Vec::new();

            for _ in 0..tournament_size {
                let idx = rand::random::<usize>() % POPULATION_SIZE;
                tournament.push((idx, fitness_scores[idx]));
            }

            let winner_idx = tournament
                .iter()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap()
                .0;
            new_population.push(population[winner_idx].clone());
        }

        // Crossover and mutation
        for i in elite_count..POPULATION_SIZE {
            if rand::random::<f64>() < CROSSOVER_RATE {
                let parent1_idx = rand::random::<usize>() % POPULATION_SIZE;
                let parent2_idx = rand::random::<usize>() % POPULATION_SIZE;
                new_population[i] = crossover(&population[parent1_idx], &population[parent2_idx]);
            }
            mutate(&mut new_population[i]);
        }

        population = new_population;
    }

    println!("\n🎯 Evolution complete!");
    println!("🏆 Best fitness achieved: {:.3}", best_fitness);
    println!("📋 Best parameters:");
    println!("  Win score: {}", best_params.win_score);
    println!("  Loss score: {}", best_params.loss_score);
    println!("  Center column value: {}", best_params.center_column_value);
    println!(
        "  Adjacent center value: {}",
        best_params.adjacent_center_value
    );
    println!("  Outer column value: {}", best_params.outer_column_value);
    println!("  Edge column value: {}", best_params.edge_column_value);
    println!("  Row height weight: {:.3}", best_params.row_height_weight);
    println!(
        "  Center control weight: {:.3}",
        best_params.center_control_weight
    );
    println!(
        "  Piece count weight: {:.3}",
        best_params.piece_count_weight
    );
    println!("  Threat weight: {:.3}", best_params.threat_weight);
    println!("  Mobility weight: {:.3}", best_params.mobility_weight);
    println!(
        "  Vertical control weight: {:.3}",
        best_params.vertical_control_weight
    );
    println!(
        "  Horizontal control weight: {:.3}",
        best_params.horizontal_control_weight
    );
    println!("  Defensive weight: {:.3}", best_params.defensive_weight);

    // Validate against default parameters
    let validation_score = validate_against_default(&best_params, 1000);
    println!(
        "✅ Validation score: {:.3} (vs default params)",
        validation_score
    );

    // Save evolved parameters
    let evolved_json = serde_json::to_string_pretty(&best_params).unwrap();
    fs::write("../../ml/data/genetic_params/evolved.json", evolved_json).unwrap();
    println!("💾 Evolved parameters saved to ../../ml/data/genetic_params/evolved.json");
}
