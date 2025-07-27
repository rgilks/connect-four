//! Genetic parameter evolution for Connect Four AI

use connect_four_ai_core::{genetic_params::GeneticParams, GameState, Player, AI};

use rayon::prelude::*;
use std::fs;
use std::io::Write;

const POPULATION_SIZE: usize = 50; // Increased for better exploration
const GENERATIONS: usize = 100; // Increased for more thorough evolution
const GAMES_PER_EVAL: usize = 50; // Reduced from 100 since depth 5 provides much better evaluation quality
const MUTATION_RATE: f64 = 0.8; // Increased from 0.6 for more exploration
const MUTATION_STRENGTH: f64 = 3.0; // Increased from 2.0 for more exploration
const CROSSOVER_RATE: f64 = 0.7; // Increased from 0.5 for more diversity
const SEARCH_DEPTH: u8 = 5; // Increased to depth 5 for better evaluation and to reduce perfect scores

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
        println!("🖥️  Using {} threads for maximum performance", num_cores);
    }
}

// Single game evaluation for tournament tie-breaking
fn evaluate_single_game(evolved_params: &GeneticParams, opponent_params: &GeneticParams) -> f64 {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // Randomly decide which player uses evolved parameters
    let evolved_is_player2 = rng.gen_bool(0.5);

    // Create neutral game state
    let mut game_state = GameState::new();

    while !game_state.is_game_over() {
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
            opponent_params.clone()
        };

        // Create a temporary state for AI move calculation
        let mut ai_state = GameState::with_genetic_params(ai_params);
        ai_state.board = game_state.board.clone();
        ai_state.current_player = game_state.current_player;

        let mut ai = AI::new();
        let (best_move, _) = ai.get_best_move(&ai_state, SEARCH_DEPTH);

        // Make the move
        if let Some(move_col) = best_move {
            game_state.make_move(move_col).unwrap();
        } else {
            break; // No valid moves
        }
    }

    // Determine the winner with small noise to break ties
    let base_score = match game_state.get_winner() {
        Some(Player::Player1) => {
            if evolved_is_player2 {
                0.0 // Evolved lost
            } else {
                1.0 // Evolved won
            }
        }
        Some(Player::Player2) => {
            if evolved_is_player2 {
                1.0 // Evolved won
            } else {
                0.0 // Evolved lost
            }
        }
        None => 0.5, // Draw
    };

    // Add small noise to break ties and make evolution more robust
    let noise: f64 = rng.gen_range(-0.001..0.001);
    let final_score = base_score + noise;
    final_score.max(0.0).min(1.0)
}

// Tournament-style evaluation: evolved params vs multiple opponents for better evaluation
fn evaluate_params_tournament(
    evolved_params: &GeneticParams,
    opponent_params: &GeneticParams,
) -> f64 {
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

                // Create multiple opponent strategies for more challenging evaluation
                let opponent_strategy = rng.gen_range(0..3);
                let current_opponent_params = match opponent_strategy {
                    0 => opponent_params.clone(),  // Previous generation's best
                    1 => GeneticParams::default(), // Default parameters
                    2 => {
                        // Aggressive opponent
                        let mut aggressive = opponent_params.clone();
                        aggressive.threat_weight *= 1.5;
                        aggressive.center_control_weight *= 1.3;
                        aggressive
                    }
                    _ => opponent_params.clone(),
                };

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
                        current_opponent_params.clone()
                    };

                    // Create a temporary state for AI move calculation
                    let mut ai_state = GameState::with_genetic_params(ai_params);
                    ai_state.board = game_state.board.clone();
                    ai_state.current_player = game_state.current_player;

                    let mut ai = AI::new();
                    let (best_move, _) = ai.get_best_move(&ai_state, SEARCH_DEPTH);

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

                    let mut opponent_state =
                        GameState::with_genetic_params(opponent_params.clone());
                    opponent_state.board = game_state.board.clone();
                    opponent_state.current_player = game_state.current_player;
                    let opponent_eval = opponent_state.evaluate();

                    // Compare evaluations from both perspectives
                    let evolved_won = if evolved_is_player2 {
                        evolved_eval < 0 && opponent_eval < 0 // Both think Player2 is winning
                    } else {
                        evolved_eval > 0 && opponent_eval > 0 // Both think Player1 is winning
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
                    let (best_move, _) = ai.get_best_move(&ai_state, SEARCH_DEPTH);

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

fn print_params_diff(current: &GeneticParams, previous: &GeneticParams, generation: usize) {
    println!(
        "  📊 Generation {} parameter changes ({} → {}):",
        generation, previous.id, current.id
    );
    println!(
        "    Win score: {} → {} ({:+})",
        previous.win_score,
        current.win_score,
        current.win_score - previous.win_score
    );
    println!(
        "    Loss score: {} → {} ({:+})",
        previous.loss_score,
        current.loss_score,
        current.loss_score - previous.loss_score
    );
    println!(
        "    Center column value: {} → {} ({:+})",
        previous.center_column_value,
        current.center_column_value,
        current.center_column_value - previous.center_column_value
    );
    println!(
        "    Adjacent center value: {} → {} ({:+})",
        previous.adjacent_center_value,
        current.adjacent_center_value,
        current.adjacent_center_value - previous.adjacent_center_value
    );
    println!(
        "    Outer column value: {} → {} ({:+})",
        previous.outer_column_value,
        current.outer_column_value,
        current.outer_column_value - previous.outer_column_value
    );
    println!(
        "    Edge column value: {} → {} ({:+})",
        previous.edge_column_value,
        current.edge_column_value,
        current.edge_column_value - previous.edge_column_value
    );
    println!(
        "    Row height weight: {:.3} → {:.3} ({:+.3})",
        previous.row_height_weight,
        current.row_height_weight,
        current.row_height_weight - previous.row_height_weight
    );
    println!(
        "    Center control weight: {:.3} → {:.3} ({:+.3})",
        previous.center_control_weight,
        current.center_control_weight,
        current.center_control_weight - previous.center_control_weight
    );
    println!(
        "    Piece count weight: {:.3} → {:.3} ({:+.3})",
        previous.piece_count_weight,
        current.piece_count_weight,
        current.piece_count_weight - previous.piece_count_weight
    );
    println!(
        "    Threat weight: {:.3} → {:.3} ({:+.3})",
        previous.threat_weight,
        current.threat_weight,
        current.threat_weight - previous.threat_weight
    );
    println!(
        "    Mobility weight: {:.3} → {:.3} ({:+.3})",
        previous.mobility_weight,
        current.mobility_weight,
        current.mobility_weight - previous.mobility_weight
    );
    println!(
        "    Vertical control weight: {:.3} → {:.3} ({:+.3})",
        previous.vertical_control_weight,
        current.vertical_control_weight,
        current.vertical_control_weight - previous.vertical_control_weight
    );
    println!(
        "    Horizontal control weight: {:.3} → {:.3} ({:+.3})",
        previous.horizontal_control_weight,
        current.horizontal_control_weight,
        current.horizontal_control_weight - previous.horizontal_control_weight
    );
    println!(
        "    Defensive weight: {:.3} → {:.3} ({:+.3})",
        previous.defensive_weight,
        current.defensive_weight,
        current.defensive_weight - previous.defensive_weight
    );
}

fn crossover(parent1: &GeneticParams, parent2: &GeneticParams) -> GeneticParams {
    // Use the built-in crossover method from the GeneticParams struct
    parent1.crossover(parent2, CROSSOVER_RATE)
}

fn calculate_population_diversity(population: &[GeneticParams]) -> f64 {
    if population.len() < 2 {
        return 0.0;
    }

    let mut total_diversity = 0.0;
    let mut comparisons = 0;

    for i in 0..population.len() {
        for j in (i + 1)..population.len() {
            let diff = calculate_params_difference(&population[i], &population[j]);
            total_diversity += diff;
            comparisons += 1;
        }
    }

    if comparisons > 0 {
        total_diversity / comparisons as f64
    } else {
        0.0
    }
}

fn calculate_params_difference(params1: &GeneticParams, params2: &GeneticParams) -> f64 {
    let mut total_diff = 0.0;

    // Use smaller normalization factors to make differences more visible
    total_diff += ((params1.win_score - params2.win_score).abs() as f64) / 1000.0;
    total_diff += ((params1.loss_score - params2.loss_score).abs() as f64) / 1000.0;
    total_diff += ((params1.center_column_value - params2.center_column_value).abs() as f64) / 50.0;
    total_diff +=
        ((params1.adjacent_center_value - params2.adjacent_center_value).abs() as f64) / 25.0;
    total_diff += ((params1.outer_column_value - params2.outer_column_value).abs() as f64) / 10.0;
    total_diff += ((params1.edge_column_value - params2.edge_column_value).abs() as f64) / 5.0;
    total_diff += (params1.row_height_weight - params2.row_height_weight).abs() / 1.0;
    total_diff += (params1.center_control_weight - params2.center_control_weight).abs() / 1.0;
    total_diff += (params1.piece_count_weight - params2.piece_count_weight).abs() / 1.0;
    total_diff += (params1.threat_weight - params2.threat_weight).abs() / 2.0;
    total_diff += (params1.mobility_weight - params2.mobility_weight).abs() / 1.0;
    total_diff += (params1.vertical_control_weight - params2.vertical_control_weight).abs() / 1.0;
    total_diff +=
        (params1.horizontal_control_weight - params2.horizontal_control_weight).abs() / 1.0;
    total_diff += (params1.defensive_weight - params2.defensive_weight).abs() / 1.0;

    total_diff / 14.0 // Average across all parameters
}

fn log_generation_parameters(
    generation: usize,
    best_params: &GeneticParams,
    avg_fitness: f64,
    diversity: f64,
    csv_file: &mut std::fs::File,
) -> Result<(), std::io::Error> {
    // Write CSV header if this is the first generation
    if generation == 0 {
        writeln!(
            csv_file,
            "generation,fitness,diversity,win_score,loss_score,center_column_value,adjacent_center_value,outer_column_value,edge_column_value,row_height_weight,center_control_weight,piece_count_weight,threat_weight,mobility_weight,vertical_control_weight,horizontal_control_weight,defensive_weight"
        )?;
    }

    // Write parameter values for this generation
    writeln!(
        csv_file,
        "{},{:.6},{:.6},{},{},{},{},{},{},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6}",
        generation,
        avg_fitness,
        diversity,
        best_params.win_score,
        best_params.loss_score,
        best_params.center_column_value,
        best_params.adjacent_center_value,
        best_params.outer_column_value,
        best_params.edge_column_value,
        best_params.row_height_weight,
        best_params.center_control_weight,
        best_params.piece_count_weight,
        best_params.threat_weight,
        best_params.mobility_weight,
        best_params.vertical_control_weight,
        best_params.horizontal_control_weight,
        best_params.defensive_weight
    )?;

    Ok(())
}

fn log_convergence_analysis(
    generation: usize,
    best_params: &GeneticParams,
    previous_best_params: &GeneticParams,
    csv_file: &mut std::fs::File,
) -> Result<(), std::io::Error> {
    // Calculate parameter changes
    let win_score_change = best_params.win_score - previous_best_params.win_score;
    let loss_score_change = best_params.loss_score - previous_best_params.loss_score;
    let center_column_change =
        best_params.center_column_value - previous_best_params.center_column_value;
    let adjacent_center_change =
        best_params.adjacent_center_value - previous_best_params.adjacent_center_value;
    let outer_column_change =
        best_params.outer_column_value - previous_best_params.outer_column_value;
    let edge_column_change = best_params.edge_column_value - previous_best_params.edge_column_value;
    let row_height_change = best_params.row_height_weight - previous_best_params.row_height_weight;
    let center_control_change =
        best_params.center_control_weight - previous_best_params.center_control_weight;
    let piece_count_change =
        best_params.piece_count_weight - previous_best_params.piece_count_weight;
    let threat_change = best_params.threat_weight - previous_best_params.threat_weight;
    let mobility_change = best_params.mobility_weight - previous_best_params.mobility_weight;
    let vertical_control_change =
        best_params.vertical_control_weight - previous_best_params.vertical_control_weight;
    let horizontal_control_change =
        best_params.horizontal_control_weight - previous_best_params.horizontal_control_weight;
    let defensive_change = best_params.defensive_weight - previous_best_params.defensive_weight;

    // Write convergence CSV header if this is the first generation
    if generation == 1 {
        writeln!(
            csv_file,
            "generation,win_score_change,loss_score_change,center_column_change,adjacent_center_change,outer_column_change,edge_column_change,row_height_change,center_control_change,piece_count_change,threat_change,mobility_change,vertical_control_change,horizontal_control_change,defensive_change"
        )?;
    }

    // Write parameter changes for this generation
    writeln!(
        csv_file,
        "{},{},{},{},{},{},{},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6}",
        generation,
        win_score_change,
        loss_score_change,
        center_column_change,
        adjacent_center_change,
        outer_column_change,
        edge_column_change,
        row_height_change,
        center_control_change,
        piece_count_change,
        threat_change,
        mobility_change,
        vertical_control_change,
        horizontal_control_change,
        defensive_change
    )?;

    Ok(())
}

fn inject_diversity(population: &mut Vec<GeneticParams>, target_diversity: f64) {
    let current_diversity = calculate_population_diversity(population);

    if current_diversity < target_diversity {
        println!(
            "    🔄 Injecting diversity (current: {:.3}, target: {:.3})",
            current_diversity, target_diversity
        );

        // Replace some individuals with completely random ones
        let num_to_replace = (population.len() as f64 * 0.4) as usize; // Increased from 30% to 40%

        for _ in 0..num_to_replace {
            let replace_idx = rand::random::<usize>() % population.len();
            population[replace_idx] = GeneticParams::random();
        }

        // Apply extra mutation to existing individuals with higher strength
        for individual in population.iter_mut() {
            if rand::random::<f64>() < 0.7 {
                // Increased from 0.5 to 0.7
                *individual = individual.random_mutation(0.9, 6.0); // Increased strength from 4.0 to 6.0
            }
        }

        println!(
            "    ✅ Injected diversity: replaced {} individuals, applied extra mutations",
            num_to_replace
        );
    }
}

fn main() {
    optimize_cpu_usage();
    println!("🧬 Starting Connect Four AI parameter evolution");
    println!("Population size: {}", POPULATION_SIZE);
    println!("Generations: {}", GENERATIONS);
    println!("Games per evaluation: {}", GAMES_PER_EVAL);
    println!("Search depth: {}", SEARCH_DEPTH);
    println!("⚠️  NOTE: Each generation plays against the previous generation's best, not default params");

    // Show starting parameters (default)
    let default_params = GeneticParams::default();
    println!("\n🎯 Starting parameters (default):");
    println!("  Win score: {}", default_params.win_score);
    println!("  Loss score: {}", default_params.loss_score);
    println!(
        "  Center column value: {}",
        default_params.center_column_value
    );
    println!(
        "  Adjacent center value: {}",
        default_params.adjacent_center_value
    );
    println!(
        "  Outer column value: {}",
        default_params.outer_column_value
    );
    println!("  Edge column value: {}", default_params.edge_column_value);
    println!(
        "  Row height weight: {:.3}",
        default_params.row_height_weight
    );
    println!(
        "  Center control weight: {:.3}",
        default_params.center_control_weight
    );
    println!(
        "  Piece count weight: {:.3}",
        default_params.piece_count_weight
    );
    println!("  Threat weight: {:.3}", default_params.threat_weight);
    println!("  Mobility weight: {:.3}", default_params.mobility_weight);
    println!(
        "  Vertical control weight: {:.3}",
        default_params.vertical_control_weight
    );
    println!(
        "  Horizontal control weight: {:.3}",
        default_params.horizontal_control_weight
    );
    println!("  Defensive weight: {:.3}", default_params.defensive_weight);

    // Create a more diverse initial population
    let mut population: Vec<GeneticParams> = Vec::new();

    // Add default parameters as one individual
    population.push(GeneticParams::default());

    // Add individuals with extreme parameter values for better exploration
    for i in 0..8 {
        let mut extreme_params = GeneticParams::default();
        extreme_params.id = format!("extreme_{}", i);
        extreme_params.generation = 0;
        extreme_params.win_score = 3000 + i * 1500;
        extreme_params.loss_score = -20000 + i * 1500;
        extreme_params.center_column_value = 20 + i * 25;
        extreme_params.adjacent_center_value = 10 + i * 12;
        extreme_params.outer_column_value = 2 + i * 3;
        extreme_params.edge_column_value = 1 + i;
        extreme_params.row_height_weight = 0.1 + i as f64 * 0.3;
        extreme_params.center_control_weight = 0.0 + i as f64 * 0.5;
        extreme_params.piece_count_weight = 0.0 + i as f64 * 0.4;
        extreme_params.threat_weight = 0.1 + i as f64 * 0.8;
        extreme_params.mobility_weight = 0.0 + i as f64 * 0.4;
        extreme_params.vertical_control_weight = 0.1 + i as f64 * 0.5;
        extreme_params.horizontal_control_weight = 0.1 + i as f64 * 0.5;
        extreme_params.defensive_weight = 0.1 + i as f64 * 0.5;
        population.push(extreme_params);
    }

    // Add individuals with very different strategic focuses
    for i in 0..8 {
        let mut strategy_params = GeneticParams::default();
        strategy_params.id = format!("strategy_{}", i);
        strategy_params.generation = 0;
        match i {
            0 => {
                // Aggressive center control
                strategy_params.center_column_value = 300;
                strategy_params.adjacent_center_value = 200;
                strategy_params.center_control_weight = 5.0;
                strategy_params.threat_weight = 0.5;
            }
            1 => {
                // Defensive play
                strategy_params.defensive_weight = 5.0;
                strategy_params.threat_weight = 3.0;
                strategy_params.center_control_weight = 0.5;
            }
            2 => {
                // Mobility focus
                strategy_params.mobility_weight = 3.0;
                strategy_params.piece_count_weight = 0.1;
                strategy_params.center_control_weight = 0.5;
            }
            3 => {
                // Threat detection
                strategy_params.threat_weight = 5.0;
                strategy_params.defensive_weight = 2.0;
                strategy_params.mobility_weight = 0.5;
            }
            4 => {
                // Positional play
                strategy_params.row_height_weight = 3.0;
                strategy_params.vertical_control_weight = 3.0;
                strategy_params.horizontal_control_weight = 3.0;
            }
            5 => {
                // Balanced but different
                strategy_params.center_control_weight = 1.0;
                strategy_params.piece_count_weight = 2.0;
                strategy_params.threat_weight = 1.5;
                strategy_params.mobility_weight = 1.5;
            }
            6 => {
                // Extreme values
                strategy_params.win_score = 20000;
                strategy_params.loss_score = -20000;
                strategy_params.center_column_value = 500;
                strategy_params.threat_weight = 10.0;
            }
            7 => {
                // Minimal values
                strategy_params.win_score = 5000;
                strategy_params.loss_score = -5000;
                strategy_params.center_column_value = 50;
                strategy_params.threat_weight = 0.1;
            }
            _ => unreachable!(),
        }
        population.push(strategy_params);
    }

    // Fill the rest with random individuals
    while population.len() < POPULATION_SIZE {
        population.push(GeneticParams::random());
    }

    // Create CSV files for parameter tracking
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let params_csv_path = format!("evolution_params_{}.csv", timestamp);
    let convergence_csv_path = format!("evolution_convergence_{}.csv", timestamp);

    let mut params_csv_file =
        std::fs::File::create(&params_csv_path).expect("Failed to create parameters CSV file");
    let mut convergence_csv_file = std::fs::File::create(&convergence_csv_path)
        .expect("Failed to create convergence CSV file");

    println!("📊 Parameter tracking enabled:");
    println!("  Parameters: {}", params_csv_path);
    println!("  Convergence: {}", convergence_csv_path);

    let mut best_fitness = 0.0;
    let mut best_params = GeneticParams::default();
    let mut previous_best_params = GeneticParams::default(); // Track previous generation's best
    let mut hall_of_fame_fitness = 0.0;
    let mut generations_without_improvement = 0;
    let mut consecutive_stagnation_count = 0; // Track consecutive generations with stagnation

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
        println!(
            "    🎯 Opponent parameters for this generation ({}):",
            previous_best_params.id
        );
        println!("      Win score: {}", previous_best_params.win_score);
        println!(
            "      Center column value: {}",
            previous_best_params.center_column_value
        );
        println!(
            "      Center control weight: {:.3}",
            previous_best_params.center_control_weight
        );
        println!(
            "      Threat weight: {:.3}",
            previous_best_params.threat_weight
        );

        let fitness_scores: Vec<f64> = population
            .par_iter()
            .enumerate()
            .map(|(idx, params)| {
                let fitness = evaluate_params_tournament(params, &previous_best_params);

                // Log progress every 5 individuals
                if (idx + 1) % 5 == 0 || idx == 0 {
                    println!(
                        "      {}/{} evaluated (fitness: {:.3})",
                        idx + 1,
                        POPULATION_SIZE,
                        fitness
                    );
                }

                // Log perfect scores immediately
                if fitness >= 1.0 {
                    println!(
                        "      ⚠️  PERFECT SCORE at individual {} ({}): {:.3}",
                        idx + 1,
                        params.id,
                        fitness
                    );
                }

                fitness
            })
            .collect();
        println!("    Parallel evaluation finished!");

        let eval_time = start_time.elapsed();
        println!("  Evaluation completed in {:.1}s", eval_time.as_secs_f64());

        // Find best individual with tie-breaking for perfect scores
        let (best_idx, &best_score) = if fitness_scores.iter().filter(|&&f| f >= 1.0).count() > 1 {
            // Multiple perfect scores - need tie-breaking
            let perfect_indices: Vec<usize> = fitness_scores
                .iter()
                .enumerate()
                .filter(|(_, &f)| f >= 1.0)
                .map(|(idx, _)| idx)
                .collect();

            println!(
                "    🔍 Tie-breaking between {} perfect candidates...",
                perfect_indices.len()
            );

            // Check if the perfect candidates are actually different from each other
            let mut all_same = true;
            let first_perfect = &population[perfect_indices[0]];
            for &idx in &perfect_indices[1..] {
                let diff = calculate_params_difference(first_perfect, &population[idx]);
                if diff > 0.01 {
                    all_same = false;
                    break;
                }
            }

            if all_same {
                println!("    ⚠️  All perfect candidates are identical - picking first one");
                (perfect_indices[0], &fitness_scores[perfect_indices[0]])
            } else {
                println!("    ✅ Perfect candidates are different - running tournament");

                // Evaluate perfect candidates against each other in parallel mini-tournaments
                // Collect tournament results efficiently in parallel with round-robin format
                let tournament_games = 50; // Reduced from 200 since depth 4 provides better evaluation quality

                println!(
                    "    🎮 Starting tie-break tournament ({} games per candidate)...",
                    tournament_games
                );
                // Create all individual games to parallelize at the game level
                let mut all_games = Vec::new();
                for &candidate_idx in &perfect_indices {
                    let opponents_per_game = perfect_indices.len() - 1; // Exclude self
                    let games_per_opponent = tournament_games / opponents_per_game; // At least 1
                    let remaining_games = tournament_games % opponents_per_game;

                    // Add base games against each opponent
                    for &opponent_idx in &perfect_indices {
                        if opponent_idx != candidate_idx {
                            for _ in 0..games_per_opponent {
                                all_games.push((candidate_idx, opponent_idx));
                            }
                        }
                    }

                    // Add remaining games randomly
                    for _ in 0..remaining_games {
                        let opponent_idx =
                            perfect_indices[rand::random::<usize>() % perfect_indices.len()];
                        if opponent_idx != candidate_idx {
                            all_games.push((candidate_idx, opponent_idx));
                        }
                    }
                }

                println!(
                    "    🎯 Parallelizing {} individual games across all cores...",
                    all_games.len()
                );

                // Run all games in parallel at the individual game level
                let game_results: Vec<(usize, usize, f64)> = all_games
                    .par_iter()
                    .map(|(candidate_idx, opponent_idx)| {
                        // Use a simpler single-game evaluation for tournament
                        let fitness = evaluate_single_game(
                            &population[*candidate_idx],
                            &population[*opponent_idx],
                        );
                        (*candidate_idx, *opponent_idx, fitness)
                    })
                    .collect();

                // Aggregate results by candidate
                let mut candidate_results = std::collections::HashMap::new();
                for (candidate_idx, opponent_idx, fitness) in game_results {
                    let entry =
                        candidate_results
                            .entry(candidate_idx)
                            .or_insert((0, 0, Vec::new()));
                    entry.0 += 1; // total games
                    if fitness > 0.5 {
                        entry.1 += 1; // wins
                    }
                    entry.2.push((opponent_idx, fitness));
                }

                // Convert to tournament results format
                let tournament_results: Vec<(usize, usize, usize, Vec<(usize, f64)>)> =
                    perfect_indices
                        .iter()
                        .map(|&idx| {
                            let default_result = (0, 0, Vec::new());
                            let (total_games, wins, game_results) =
                                candidate_results.get(&idx).unwrap_or(&default_result);
                            (idx, *wins, *total_games, game_results.clone())
                        })
                        .collect();

                // Log results after parallel computation is complete
                let tournament_scores: Vec<f64> = tournament_results
                    .iter()
                    .map(|(idx, wins, total_games, _game_results)| {
                        let tournament_score = if *total_games > 0 {
                            *wins as f64 / *total_games as f64
                        } else {
                            0.0
                        };

                        // Skip individual game logging for performance (too many games now)

                        // Log candidate summary
                        println!(
                            "    📊 Candidate {} ({}) tournament complete: {}/{} wins ({:.1}%)",
                            idx + 1,
                            population[*idx].id,
                            wins,
                            total_games,
                            tournament_score * 100.0
                        );

                        tournament_score
                    })
                    .collect();

                // Find the best tournament score
                let (best_tournament_idx, &best_tournament_score) = tournament_scores
                    .iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                    .unwrap();

                let best_perfect_idx = perfect_indices[best_tournament_idx];

                println!(
                    "    🏆 Tie-break winner: individual {} ({}) (tournament score: {:.3})",
                    best_perfect_idx + 1,
                    population[best_perfect_idx].id,
                    best_tournament_score
                );
                (best_perfect_idx, &fitness_scores[best_perfect_idx])
            }
        } else {
            // Normal case - just pick the best
            fitness_scores
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap()
        };

        // Find runner-up for reporting
        let (runner_up_idx, runner_up_score) = {
            let mut fitness_with_indices: Vec<(usize, f64)> = fitness_scores
                .iter()
                .enumerate()
                .map(|(idx, &score)| (idx, score))
                .collect();
            fitness_with_indices.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
            (fitness_with_indices[1].0, fitness_with_indices[1].1)
        };

        let runner_up_params = &population[runner_up_idx];

        if best_score > best_fitness {
            let improvement = best_score - best_fitness;
            best_fitness = best_score;
            best_params = population[best_idx].clone();
            generations_without_improvement = 0;
            println!(
                "🏆 New best fitness: {:.3} (+{:.3}) - ID: {} (gen {})",
                best_fitness, improvement, best_params.id, best_params.generation
            );

            // Update hall of fame if this is the best ever
            if best_score > hall_of_fame_fitness {
                hall_of_fame_fitness = best_score;
                println!(
                    "🌟 New hall of fame entry! Fitness: {:.3}",
                    hall_of_fame_fitness
                );
            }
        } else {
            generations_without_improvement += 1;
            if generations_without_improvement == 5 {
                println!("⚠️  No improvement for 5 generations");
            } else if generations_without_improvement == 10 {
                println!("⚠️  No improvement for 10 generations - evolution may be stagnating");
            }
        }

        // Get the current generation's winner
        let current_winner = &population[best_idx];

        // Check if the current generation's winner is different from the previous generation's winner
        let params_diff = calculate_params_difference(current_winner, &previous_best_params);
        if params_diff < 0.01 {
            // Adjusted threshold for new normalization
            println!(
                "⚠️  WARNING: Current generation winner is nearly identical to previous generation winner (diff: {:.3})",
                params_diff
            );
            println!("    This suggests the evolution is not making meaningful progress!");
        } else {
            println!(
                "✅ Current generation winner is different from previous generation winner (diff: {:.3})",
                params_diff
            );
        }

        // Always show current generation winner parameters and changes from previous generation
        if generation > 0 {
            print_params_diff(current_winner, &previous_best_params, generation + 1);
        } else {
            // For generation 1, show changes from default parameters
            print_params_diff(current_winner, &default_params, generation + 1);
        }

        // Show runner-up information
        if runner_up_idx != best_idx {
            println!(
                "🥈 Runner-up: individual {} ({}) - fitness: {:.3}",
                runner_up_idx + 1,
                runner_up_params.id,
                runner_up_score
            );
            println!("   Win score: {}", runner_up_params.win_score);
            println!("   Loss score: {}", runner_up_params.loss_score);
            println!(
                "   Center column value: {}",
                runner_up_params.center_column_value
            );
            println!(
                "   Adjacent center value: {}",
                runner_up_params.adjacent_center_value
            );
            println!(
                "   Outer column value: {}",
                runner_up_params.outer_column_value
            );
            println!(
                "   Edge column value: {}",
                runner_up_params.edge_column_value
            );
            println!(
                "   Row height weight: {:.3}",
                runner_up_params.row_height_weight
            );
            println!(
                "   Center control weight: {:.3}",
                runner_up_params.center_control_weight
            );
            println!(
                "   Piece count weight: {:.3}",
                runner_up_params.piece_count_weight
            );
            println!("   Threat weight: {:.3}", runner_up_params.threat_weight);
            println!(
                "   Mobility weight: {:.3}",
                runner_up_params.mobility_weight
            );
            println!(
                "   Vertical control weight: {:.3}",
                runner_up_params.vertical_control_weight
            );
            println!(
                "   Horizontal control weight: {:.3}",
                runner_up_params.horizontal_control_weight
            );
            println!(
                "   Defensive weight: {:.3}",
                runner_up_params.defensive_weight
            );
        }

        let avg_fitness = fitness_scores.iter().sum::<f64>() / fitness_scores.len() as f64;
        let min_fitness = fitness_scores.iter().fold(1.0_f64, |a, &b| a.min(b));
        let high_fitness_count = fitness_scores.iter().filter(|&&f| f > 0.7).count();
        let perfect_fitness_count = fitness_scores.iter().filter(|&&f| f >= 1.0).count();

        println!(
            "📊 Fitness stats: avg={:.3}, min={:.3}, best={:.3}, high(>0.7)={}/{}, perfect={}/{}",
            avg_fitness,
            min_fitness,
            best_score,
            high_fitness_count,
            POPULATION_SIZE,
            perfect_fitness_count,
            POPULATION_SIZE
        );

        // Log diversity information
        let diversity = calculate_population_diversity(&population);
        println!("🌱 Population diversity: {:.3}", diversity);

        // Log parameters to CSV for graphing
        if let Err(e) = log_generation_parameters(
            generation + 1,
            current_winner,
            avg_fitness,
            diversity,
            &mut params_csv_file,
        ) {
            eprintln!("⚠️  Failed to log generation parameters: {}", e);
        }

        // Log convergence analysis
        if generation > 0 {
            if let Err(e) = log_convergence_analysis(
                generation + 1,
                current_winner,
                &previous_best_params,
                &mut convergence_csv_file,
            ) {
                eprintln!("⚠️  Failed to log convergence analysis: {}", e);
            }
        }

        // Analysis of why this generation might be performing well
        if perfect_fitness_count > POPULATION_SIZE / 2 {
            let perfect_percentage = (perfect_fitness_count * 100) / POPULATION_SIZE;
            println!(
                "⚠️  WARNING: {}% of population has perfect scores - this suggests:",
                perfect_percentage
            );
            println!("    1. The opponent (previous generation) might be too weak");
            println!("    2. The evaluation might be too easy");
            println!("    3. The search depth might be insufficient");
            println!(
                "    4. Population diversity is too low (current: {:.3})",
                diversity
            );
        }

        // Selection: Use tournament selection with diversity preservation
        let mut new_population = Vec::new();

        // Keep only the absolute best individual (elitism = 1)
        let best_individual = population[best_idx].clone();
        new_population.push(best_individual.clone());

        // Check if we're stagnating (current generation winner is too similar to previous generation winner)
        let params_diff = calculate_params_difference(current_winner, &previous_best_params);
        let is_stagnating = params_diff < 0.01; // Adjusted threshold for new normalization

        if is_stagnating {
            consecutive_stagnation_count += 1;
            println!(
                "    ⚠️  STAGNATION DETECTED: Best individual diff = {:.4} (threshold: 0.01) - Consecutive: {}",
                params_diff, consecutive_stagnation_count
            );
            println!("    🔧 Applying aggressive mutation strategy...");
        } else {
            consecutive_stagnation_count = 0;
        }

        // Adjust mutation parameters based on stagnation
        let current_mutation_strength = if is_stagnating {
            MUTATION_STRENGTH * 2.0 // Double mutation strength when stagnating
        } else {
            MUTATION_STRENGTH
        };

        let current_mutation_rate = if is_stagnating {
            (MUTATION_RATE * 1.5).min(1.0) // Increase mutation rate when stagnating, but cap at 1.0
        } else {
            MUTATION_RATE
        };

        // Generate the rest through tournament selection with forced diversity
        let mut used_indices = std::collections::HashSet::new();
        used_indices.insert(best_idx); // Don't reuse the best individual

        // Focus more mutations on the best candidate when stagnating
        let best_focused_ratio = if is_stagnating { 0.6 } else { 0.3 }; // 60% vs 30% focus on best
        let num_best_focused = ((POPULATION_SIZE - 1) as f64 * best_focused_ratio) as usize;

        if is_stagnating {
            println!(
                "    🎯 Creating {} offspring from best individual ({}% focus)",
                num_best_focused,
                (best_focused_ratio * 100.0) as i32
            );
        }

        // Create some offspring specifically from the best individual
        for _ in 0..num_best_focused {
            let mut offspring = best_individual.clone();

            // Apply stronger mutation to the best individual
            offspring = offspring.random_mutation(current_mutation_rate, current_mutation_strength);

            // Additional random mutation for extra diversity
            if rand::random::<f64>() < 0.5 {
                offspring = offspring.random_mutation(0.7, current_mutation_strength * 1.5);
            }

            new_population.push(offspring);
        }

        // Fill the rest through tournament selection
        while new_population.len() < POPULATION_SIZE {
            // Tournament selection with diversity constraint
            let tournament_size = 5; // Increased from 3 for better selection pressure
            let mut tournament = Vec::new();

            // Select tournament participants, avoiding recently used individuals
            for _ in 0..tournament_size {
                let mut attempts = 0;
                let max_attempts = 20;
                let mut idx;

                loop {
                    idx = rand::random::<usize>() % POPULATION_SIZE;
                    attempts += 1;

                    // Prefer unused individuals, but allow reuse after max attempts
                    if !used_indices.contains(&idx) || attempts >= max_attempts {
                        break;
                    }
                }

                tournament.push((idx, fitness_scores[idx]));
            }

            let winner_idx = tournament
                .iter()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap()
                .0;

            // Create offspring through crossover and mutation
            let mut offspring = population[winner_idx].clone();

            // Always apply crossover with a different parent for diversity
            let parent2_idx = loop {
                let idx = rand::random::<usize>() % POPULATION_SIZE;
                if idx != winner_idx {
                    break idx;
                }
            };
            offspring = crossover(&offspring, &population[parent2_idx]);

            // Apply stronger mutation to ensure diversity
            offspring = offspring.random_mutation(current_mutation_rate, current_mutation_strength);

            // Additional random mutation for extra diversity
            if rand::random::<f64>() < 0.3 {
                offspring = offspring.random_mutation(0.5, current_mutation_strength);
            }

            new_population.push(offspring);
            used_indices.insert(winner_idx);
        }

        // Crossover and mutation already applied during tournament selection

        // Check population diversity and inject diversity if needed
        let current_diversity = calculate_population_diversity(&new_population);
        let target_diversity = if is_stagnating { 0.2 } else { 0.1 }; // Higher target when stagnating

        if current_diversity < target_diversity || is_stagnating {
            inject_diversity(&mut new_population, target_diversity);
        }

        // Check for early stopping due to stagnation
        if consecutive_stagnation_count >= 3 {
            println!(
                "\n🛑 EARLY STOPPING: Stagnation detected for {} consecutive generations",
                consecutive_stagnation_count
            );
            println!("   The evolution has converged and is no longer making meaningful progress.");
            println!("   Best fitness achieved: {:.3}", best_fitness);
            break;
        }

        // Update previous best for next generation (track current generation's winner)
        previous_best_params = current_winner.clone();
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
    println!("🧬 Best individual lineage:");
    println!("  ID: {}", best_params.id);
    println!("  Generation: {}", best_params.generation);
    if !best_params.parent_ids.is_empty() {
        println!("  Parents: {}", best_params.parent_ids.join(" → "));
    } else {
        println!("  Parents: None (initial population)");
    }
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
    let validation_score = validate_against_default(&best_params, 500); // Reduced from 2000 since depth 4 provides better evaluation
    println!(
        "✅ Validation score: {:.3} (vs default params)",
        validation_score
    );

    // Save evolved parameters
    let evolved_json = serde_json::to_string_pretty(&best_params).unwrap();
    fs::write("../../ml/data/genetic_params/evolved.json", evolved_json).unwrap();
    println!("💾 Evolved parameters saved to ../../ml/data/genetic_params/evolved.json");

    // Convergence analysis summary
    println!("\n📊 Convergence Analysis:");
    println!("  Parameter tracking files created:");
    println!("    - Parameters: {}", params_csv_path);
    println!("    - Convergence: {}", convergence_csv_path);
    println!("  Use these files to create graphs showing:");
    println!("    - Parameter evolution over generations");
    println!("    - Convergence rates for each parameter");
    println!("    - Fitness improvement trends");
    println!("    - Population diversity changes");
    println!("  Expected convergence patterns:");
    println!("    - Parameters should stabilize after ~20-30 generations");
    println!("    - Fitness should plateau around 0.8-0.9");
    println!("    - Diversity should decrease as population converges");
    println!("    - Large parameter swings suggest insufficient evaluation");
}
