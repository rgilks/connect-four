//! Genetic parameter evolution for Connect Four AI

use connect_four_ai_core::{genetic_params::GeneticParams, GameState, Player, AI};

use rayon::prelude::*;
use std::fs;

const POPULATION_SIZE: usize = 50;
const GENERATIONS: usize = 50;
const GAMES_PER_EVAL: usize = 100; // Reduced for faster evolution while still detecting perfect scores
const MUTATION_RATE: f64 = 0.3;
const MUTATION_STRENGTH: f64 = 1.0;
const CROSSOVER_RATE: f64 = 0.5;

fn optimize_cpu_usage() {
    if cfg!(target_os = "macos") {
        let num_cores = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(8);
        // Use only performance cores (typically 6-8 on Apple Silicon)
        let performance_cores = std::cmp::min(num_cores, 8);
        rayon::ThreadPoolBuilder::new()
            .num_threads(performance_cores)
            .stack_size(8 * 1024 * 1024)
            .build_global()
            .unwrap_or_else(|_| {
                println!("Warning: Could not set optimal thread count, using default");
            });
        println!(
            "🍎 Apple Silicon detected: Using {} performance cores ({} total cores available)",
            performance_cores, num_cores
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
        println!(
            "🖥️  Using {} threads for maximum performance",
            num_cores
        );
    }
}

// Tournament-style evaluation: evolved params vs default params
fn evaluate_params_tournament(evolved_params: &GeneticParams) -> f64 {
    let default_params = GeneticParams::default();

    // Use chunked parallel processing for better performance with large sample sizes
    let chunk_size = 100;
    let num_chunks = (GAMES_PER_EVAL + chunk_size - 1) / chunk_size;

    let total_wins: usize = (0..num_chunks)
        .into_par_iter()
        .map(|chunk_idx| {
            let start = chunk_idx * chunk_size;
            let end = std::cmp::min(start + chunk_size, GAMES_PER_EVAL);
            let chunk_size_actual = end - start;

            let mut chunk_wins = 0;

            for _ in 0..chunk_size_actual {
                let mut moves_played = 0;
                let max_moves = 42; // Maximum moves in Connect Four (6x7 board)

                // Randomly decide which player uses evolved parameters
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let evolved_is_player2 = rng.gen_bool(0.5);

                // Create neutral game state
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
                    let evolved_won = if evolved_is_player2 {
                        winner == Player::Player2
                    } else {
                        winner == Player::Player1
                    };

                    if evolved_won {
                        chunk_wins += 1;
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
                    let evolved_won = if evolved_is_player2 {
                        evolved_eval < 0 && default_eval < 0 // Both think Player2 is winning
                    } else {
                        evolved_eval > 0 && default_eval > 0 // Both think Player1 is winning
                    };

                    if evolved_won {
                        chunk_wins += 1;
                    }
                }
            }

            chunk_wins
        })
        .sum();

    let fitness = total_wins as f64 / GAMES_PER_EVAL as f64;
    fitness
}

fn validate_against_default(evolved_params: &GeneticParams, num_games: usize) -> f64 {
    let default_params = GeneticParams::default();

    // Use chunked parallel processing for better performance
    let chunk_size = 100;
    let num_chunks = (num_games + chunk_size - 1) / chunk_size;

    let total_wins: usize = (0..num_chunks)
        .into_par_iter()
        .map(|chunk_idx| {
            let start = chunk_idx * chunk_size;
            let end = std::cmp::min(start + chunk_size, num_games);
            let chunk_size_actual = end - start;

            let mut chunk_wins = 0;

            for _ in 0..chunk_size_actual {
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
                    let evolved_won = if evolved_is_player2 {
                        winner == Player::Player2
                    } else {
                        winner == Player::Player1
                    };

                    if evolved_won {
                        chunk_wins += 1;
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
                    let evolved_won = if evolved_is_player2 {
                        evolved_eval < 0 && default_eval < 0
                    } else {
                        evolved_eval > 0 && default_eval > 0
                    };

                    if evolved_won {
                        chunk_wins += 1;
                    }
                }
            }

            chunk_wins
        })
        .sum();

    total_wins as f64 / num_games as f64
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
    let mut generations_without_improvement = 0;

    for generation in 0..GENERATIONS {
        println!("\n🔄 Generation {}", generation + 1);

                // Evaluate fitness for all individuals with progress reporting
        println!(
            "  Evaluating {} individuals ({} games each) in parallel...",
            POPULATION_SIZE, GAMES_PER_EVAL
        );
        let start_time = std::time::Instant::now();
        
        // Use parallel processing with progress tracking
        println!("    Starting parallel evaluation...");
        let fitness_scores: Vec<f64> = population
            .par_iter()
            .enumerate()
            .map(|(idx, params)| {
                let fitness = evaluate_params_tournament(params);
                
                // Log progress every 5 individuals
                if (idx + 1) % 5 == 0 || idx == 0 {
                    println!("      {}/{} evaluated (fitness: {:.3})", idx + 1, POPULATION_SIZE, fitness);
                }
                
                // Log perfect scores immediately
                if fitness >= 1.0 {
                    println!("      ⚠️  PERFECT SCORE at individual {}: {:.3}", idx + 1, fitness);
                }
                
                fitness
            })
            .collect();
        println!("    Parallel evaluation finished!");
        
        let eval_time = start_time.elapsed();
        println!("  Evaluation completed in {:.1}s", eval_time.as_secs_f64());

        // Find best individual
        let (best_idx, &best_score) = fitness_scores
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();

        if best_score > best_fitness {
            best_fitness = best_score;
            best_params = population[best_idx].clone();
            generations_without_improvement = 0;
            println!(
                "🏆 New best fitness: {:.3} (+{:.3})",
                best_fitness,
                best_score - best_fitness
            );
        } else {
            generations_without_improvement += 1;
            if generations_without_improvement == 5 {
                println!("⚠️  No improvement for 5 generations");
            } else if generations_without_improvement == 10 {
                println!("⚠️  No improvement for 10 generations - evolution may be stagnating");
            }
        }

                let avg_fitness = fitness_scores.iter().sum::<f64>() / fitness_scores.len() as f64;
        let min_fitness = fitness_scores.iter().fold(1.0_f64, |a, &b| a.min(b));
        let high_fitness_count = fitness_scores.iter().filter(|&&f| f > 0.7).count();
        let perfect_fitness_count = fitness_scores.iter().filter(|&&f| f >= 1.0).count();
        
        println!(
            "📊 Fitness stats: avg={:.3}, min={:.3}, best={:.3}, high(>0.7)={}/{}, perfect={}/{}",
            avg_fitness, min_fitness, best_score, high_fitness_count, POPULATION_SIZE, perfect_fitness_count, POPULATION_SIZE
        );

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

        // Generation summary
        if (generation + 1) % 10 == 0 {
            println!(
                "📈 Generation {} complete - Best so far: {:.3}",
                generation + 1,
                best_fitness
            );
        }
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
