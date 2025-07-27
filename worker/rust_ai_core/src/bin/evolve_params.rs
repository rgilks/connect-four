//! Genetic parameter evolution for Connect Four AI

use connect_four_ai_core::{genetic_params::GeneticParams, GameState, Player, AI};

use rayon::prelude::*;
use std::fs;

const POPULATION_SIZE: usize = 30; // Reduced from 50 for more exploration
const GENERATIONS: usize = 50;
const GAMES_PER_EVAL: usize = 50; // Reduced from 100 for more noise/exploration
const MUTATION_RATE: f64 = 0.6; // Increased from 0.4 for more exploration
const MUTATION_STRENGTH: f64 = 2.0; // Increased from 1.2 for more exploration
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
        println!("🖥️  Using {} threads for maximum performance", num_cores);
    }
}

// Tournament-style evaluation: evolved params vs opponent params (previous generation's best)
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
                        opponent_params.clone()
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

fn print_params_diff(current: &GeneticParams, previous: &GeneticParams, generation: usize) {
    println!("  📊 Generation {} parameter changes:", generation);
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

    // Add some individuals with extreme parameter values
    for i in 0..5 {
        let mut extreme_params = GeneticParams::default();
        extreme_params.win_score = 5000 + i * 2000;
        extreme_params.loss_score = -15000 + i * 2000;
        extreme_params.center_column_value = 50 + i * 30;
        extreme_params.threat_weight = 0.5 + i as f64 * 0.5;
        extreme_params.center_control_weight = 0.0 + i as f64 * 1.0;
        population.push(extreme_params);
    }

    // Add some individuals with very different strategies
    for i in 0..5 {
        let mut strategy_params = GeneticParams::default();
        strategy_params.center_column_value = 200 + i * 20;
        strategy_params.adjacent_center_value = 150 + i * 15;
        strategy_params.outer_column_value = 30 + i * 5;
        strategy_params.defensive_weight = 3.0 + i as f64 * 0.5;
        strategy_params.mobility_weight = 0.0 + i as f64 * 0.8;
        population.push(strategy_params);
    }

    // Fill the rest with random individuals
    while population.len() < POPULATION_SIZE {
        population.push(GeneticParams::random());
    }

    let mut best_fitness = 0.0;
    let mut best_params = GeneticParams::default();
    let mut previous_best_params = GeneticParams::default(); // Track previous generation's best
    let mut hall_of_fame_fitness = 0.0;
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
                        "      ⚠️  PERFECT SCORE at individual {}: {:.3}",
                        idx + 1,
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

            // Evaluate perfect candidates against each other in parallel mini-tournaments
            // Collect tournament results efficiently in parallel with round-robin format
            let tournament_games = 20; // Reduced from 50 to 20

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
                    let fitness = evaluate_params_tournament(
                        &population[*candidate_idx],
                        &population[*opponent_idx],
                    );
                    (*candidate_idx, *opponent_idx, fitness)
                })
                .collect();

            // Aggregate results by candidate
            let mut candidate_results = std::collections::HashMap::new();
            for (candidate_idx, opponent_idx, fitness) in game_results {
                let entry = candidate_results
                    .entry(candidate_idx)
                    .or_insert((0, 0, Vec::new()));
                entry.0 += 1; // total games
                if fitness > 0.5 {
                    entry.1 += 1; // wins
                }
                entry.2.push((opponent_idx, fitness));
            }

            // Convert to tournament results format
            let tournament_results: Vec<(usize, usize, usize, Vec<(usize, f64)>)> = perfect_indices
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
                .map(|(idx, wins, total_games, game_results)| {
                    let tournament_score = if *total_games > 0 {
                        *wins as f64 / *total_games as f64
                    } else {
                        0.0
                    };

                    // Log individual game results
                    for (opponent_idx, fitness) in game_results {
                        if *fitness > 0.5 {
                            println!(
                                "      🟢 Candidate {} vs {}: WIN (fitness: {:.3})",
                                idx + 1,
                                opponent_idx + 1,
                                fitness
                            );
                        } else {
                            println!(
                                "      🔴 Candidate {} vs {}: LOSS (fitness: {:.3})",
                                idx + 1,
                                opponent_idx + 1,
                                fitness
                            );
                        }
                    }

                    // Log candidate summary
                    println!(
                        "    📊 Candidate {} tournament complete: {}/{} wins ({:.1}%)",
                        idx + 1,
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
                "    🏆 Tie-break winner: individual {} (tournament score: {:.3})",
                best_perfect_idx + 1,
                best_tournament_score
            );
            (best_perfect_idx, &fitness_scores[best_perfect_idx])
        } else {
            // Normal case - just pick the best
            fitness_scores
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap()
        };

        if best_score > best_fitness {
            let improvement = best_score - best_fitness;
            best_fitness = best_score;
            best_params = population[best_idx].clone();
            generations_without_improvement = 0;
            println!(
                "🏆 New best fitness: {:.3} (+{:.3})",
                best_fitness, improvement
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

        // Always show current best parameters and changes from previous generation
        if generation > 0 {
            print_params_diff(&best_params, &previous_best_params, generation + 1);
        } else {
            // For generation 1, show changes from default parameters
            print_params_diff(&best_params, &default_params, generation + 1);
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
        }

        // Selection: Keep top 20% and tournament select the rest
        let elite_count = POPULATION_SIZE / 10; // Reduced from /5 to /10 (10% instead of 20%)
        let mut new_population = Vec::new();

        // Keep elite individuals
        let mut indexed_fitness: Vec<(usize, f64)> = fitness_scores
            .iter()
            .enumerate()
            .map(|(i, &f)| (i, f))
            .collect();
        indexed_fitness.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

        for i in 0..elite_count {
            let mut elite_individual = population[indexed_fitness[i].0].clone();
            // Apply very light mutation to elite individuals to maintain diversity
            elite_individual = elite_individual.random_mutation(0.15, 0.5); // 15% chance, 0.5 strength
            new_population.push(elite_individual);
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

            // Create a copy and apply crossover/mutation to add diversity
            let mut selected_individual = population[winner_idx].clone();

            // Apply crossover with another random individual
            if rand::random::<f64>() < CROSSOVER_RATE {
                let parent2_idx = rand::random::<usize>() % POPULATION_SIZE;
                selected_individual = crossover(&selected_individual, &population[parent2_idx]);
            }

            // Apply mutation
            mutate(&mut selected_individual);

            new_population.push(selected_individual);
        }

        // Crossover and mutation already applied during tournament selection

        // Update previous best for next generation
        previous_best_params = best_params.clone();
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
