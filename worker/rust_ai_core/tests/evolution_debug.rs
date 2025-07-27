use connect_four_ai_core::{genetic_params::GeneticParams, GameState, Player, AI};
use std::time::Instant;

#[test]
fn test_evolution_debug_detailed() {
    println!("=== Evolution Debug Test - Detailed Analysis ===");

    // Test default vs evolved parameters
    let default_params = GeneticParams::default();
    let evolved_params = GeneticParams::random(); // Use random for testing

    println!("Default params: {:?}", default_params);
    println!("Evolved params: {:?}", evolved_params);

    // Test individual game outcomes
    println!("\n=== Testing Individual Game Outcomes ===");
    let mut evolved_wins = 0;
    let mut default_wins = 0;
    let mut draws = 0;
    let games = 10;

    for game_num in 0..games {
        println!("\n--- Game {} ---", game_num + 1);

        let mut game_state = GameState::new();
        let mut moves_played = 0;
        let max_moves = 42;

        // Randomly decide which player uses evolved parameters
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let evolved_is_player2 = rng.gen_bool(0.5);
        println!(
            "Evolved params are Player{}",
            if evolved_is_player2 { "2" } else { "1" }
        );

        while !game_state.is_game_over() && moves_played < max_moves {
            let current_player = game_state.current_player;
            let is_evolved_turn = if evolved_is_player2 {
                current_player == Player::Player2
            } else {
                current_player == Player::Player1
            };

            // Use different parameters based on whose turn it is
            let test_params = if is_evolved_turn {
                evolved_params.clone()
            } else {
                default_params.clone()
            };

            // Create a new game state with the test parameters
            let mut test_state = GameState::with_genetic_params(test_params);
            test_state.board = game_state.board.clone();
            test_state.current_player = game_state.current_player;

            let mut ai = AI::new();
            let start_time = Instant::now();
            let (best_move, _) = ai.get_best_move(&test_state, 5);
            let end_time = Instant::now();
            let move_time = end_time.duration_since(start_time).as_millis();

            if let Some(column) = best_move {
                println!(
                    "Move {}: Player{} places in column {} ({}ms, params: {})",
                    moves_played + 1,
                    if current_player == Player::Player1 {
                        "1"
                    } else {
                        "2"
                    },
                    column,
                    move_time,
                    if is_evolved_turn {
                        "evolved"
                    } else {
                        "default"
                    }
                );

                if game_state.make_move(column).is_err() {
                    println!("  ERROR: Invalid move!");
                    break;
                }
            } else {
                println!("Move {}: No valid move available", moves_played + 1);
                break;
            }

            moves_played += 1;
        }

        // Determine winner
        if let Some(winner) = game_state.get_winner() {
            println!(
                "Game {}: {:?} wins after {} moves",
                game_num + 1,
                winner,
                moves_played
            );
            if evolved_is_player2 {
                if winner == Player::Player2 {
                    evolved_wins += 1;
                } else {
                    default_wins += 1;
                }
            } else {
                if winner == Player::Player1 {
                    evolved_wins += 1;
                } else {
                    default_wins += 1;
                }
            }
        } else {
            println!("Game {}: Draw after {} moves", game_num + 1, moves_played);

            // Evaluate final position
            let evolved_eval = {
                let mut evolved_state = GameState::with_genetic_params(evolved_params.clone());
                evolved_state.board = game_state.board.clone();
                evolved_state.current_player = game_state.current_player;
                evolved_state.evaluate()
            };

            let default_eval = {
                let mut default_state = GameState::with_genetic_params(default_params.clone());
                default_state.board = game_state.board.clone();
                default_state.current_player = game_state.current_player;
                default_state.evaluate()
            };

            println!(
                "  Final evaluation - Evolved: {}, Default: {}",
                evolved_eval, default_eval
            );

            // Check if both agree on the winner
            let evolved_thinks_evolved_wins = if evolved_is_player2 {
                evolved_eval < 0
            } else {
                evolved_eval > 0
            };

            let default_thinks_evolved_wins = if evolved_is_player2 {
                default_eval < 0
            } else {
                default_eval > 0
            };

            println!(
                "  Evolved thinks evolved wins: {}",
                evolved_thinks_evolved_wins
            );
            println!(
                "  Default thinks evolved wins: {}",
                default_thinks_evolved_wins
            );

            if evolved_thinks_evolved_wins && default_thinks_evolved_wins {
                evolved_wins += 1;
                println!("  -> Evolved wins (both agree)");
            } else if !evolved_thinks_evolved_wins && !default_thinks_evolved_wins {
                default_wins += 1;
                println!("  -> Default wins (both agree)");
            } else {
                draws += 1;
                println!("  -> Draw (disagreement)");
            }
        }
    }

    println!("\n=== Final Results ===");
    println!("Evolved wins: {}", evolved_wins);
    println!("Default wins: {}", default_wins);
    println!("Draws: {}", draws);
    println!(
        "Evolved win rate: {:.1}%",
        (evolved_wins as f64 / games as f64) * 100.0
    );

    // Test evaluation bias
    println!("\n=== Testing Evaluation Bias ===");
    let test_state = GameState::new();

    let evolved_eval = {
        let mut evolved_state = GameState::with_genetic_params(evolved_params.clone());
        evolved_state.board = test_state.board.clone();
        evolved_state.current_player = test_state.current_player;
        evolved_state.evaluate()
    };

    let default_eval = {
        let mut default_state = GameState::with_genetic_params(default_params.clone());
        default_state.board = test_state.board.clone();
        default_state.current_player = test_state.current_player;
        default_state.evaluate()
    };

    println!("Empty board evaluation:");
    println!("  Evolved params: {}", evolved_eval);
    println!("  Default params: {}", default_eval);
    println!("  Difference: {}", evolved_eval - default_eval);

    if evolved_eval != default_eval {
        println!("  ⚠️  WARNING: Different evaluation of empty board!");
    }
}

#[test]
fn test_evaluation_components() {
    println!("=== Testing Evaluation Components ===");

    let default_params = GeneticParams::default();
    let test_state = GameState::new();

    // Test individual components
    println!("Empty board component analysis:");

    // Center control
    let center_p1 = test_state.center_control_score(Player::Player1);
    let center_p2 = test_state.center_control_score(Player::Player2);
    let center_score = center_p1 - center_p2;
    println!(
        "  Center control: {} (P1: {}, P2: {})",
        center_score, center_p1, center_p2
    );

    // Threat detection
    let threat_p1 = test_state.threat_score(Player::Player1);
    let threat_p2 = test_state.threat_score(Player::Player2);
    let threat_score = threat_p1 - threat_p2;
    println!(
        "  Threat detection: {} (P1: {}, P2: {})",
        threat_score, threat_p1, threat_p2
    );

    // Piece count
    let piece_p1 = test_state.pieces_count(Player::Player1);
    let piece_p2 = test_state.pieces_count(Player::Player2);
    let piece_score = piece_p1 - piece_p2;
    println!(
        "  Piece count: {} (P1: {}, P2: {})",
        piece_score, piece_p1, piece_p2
    );

    // Mobility
    let mobility_p1 = test_state.mobility_score(Player::Player1);
    let mobility_p2 = test_state.mobility_score(Player::Player2);
    let mobility_score = mobility_p1 - mobility_p2;
    println!(
        "  Mobility: {} (P1: {}, P2: {})",
        mobility_score, mobility_p1, mobility_p2
    );

    // Vertical control
    let vertical_p1 = test_state.vertical_control_score(Player::Player1);
    let vertical_p2 = test_state.vertical_control_score(Player::Player2);
    let vertical_score = vertical_p1 - vertical_p2;
    println!(
        "  Vertical control: {} (P1: {}, P2: {})",
        vertical_score, vertical_p1, vertical_p2
    );

    // Horizontal control
    let horizontal_p1 = test_state.horizontal_control_score(Player::Player1);
    let horizontal_p2 = test_state.horizontal_control_score(Player::Player2);
    let horizontal_score = horizontal_p1 - horizontal_p2;
    println!(
        "  Horizontal control: {} (P1: {}, P2: {})",
        horizontal_score, horizontal_p1, horizontal_p2
    );

    // Defensive
    let defensive_p1 = test_state.defensive_score(Player::Player1);
    let defensive_p2 = test_state.defensive_score(Player::Player2);
    let defensive_score = defensive_p1 - defensive_p2;
    println!(
        "  Defensive: {} (P1: {}, P2: {})",
        defensive_score, defensive_p1, defensive_p2
    );

    // Calculate weighted scores
    let center_weighted = center_score * default_params.center_control_weight as i32;
    let threat_weighted = threat_score * default_params.threat_weight as i32;
    let piece_weighted = piece_score * default_params.piece_count_weight as i32;
    let mobility_weighted = mobility_score * default_params.mobility_weight as i32;
    let vertical_weighted = vertical_score * default_params.vertical_control_weight as i32;
    let horizontal_weighted = horizontal_score * default_params.horizontal_control_weight as i32;
    let defensive_weighted = defensive_score * default_params.defensive_weight as i32;

    println!("\nWeighted component scores:");
    println!(
        "  Center control: {} * {} = {}",
        center_score, default_params.center_control_weight, center_weighted
    );
    println!(
        "  Threat detection: {} * {} = {}",
        threat_score, default_params.threat_weight, threat_weighted
    );
    println!(
        "  Piece count: {} * {} = {}",
        piece_score, default_params.piece_count_weight, piece_weighted
    );
    println!(
        "  Mobility: {} * {} = {}",
        mobility_score, default_params.mobility_weight, mobility_weighted
    );
    println!(
        "  Vertical control: {} * {} = {}",
        vertical_score, default_params.vertical_control_weight, vertical_weighted
    );
    println!(
        "  Horizontal control: {} * {} = {}",
        horizontal_score, default_params.horizontal_control_weight, horizontal_weighted
    );
    println!(
        "  Defensive: {} * {} = {}",
        defensive_score, default_params.defensive_weight, defensive_weighted
    );

    let total = center_weighted
        + threat_weighted
        + piece_weighted
        + mobility_weighted
        + vertical_weighted
        + horizontal_weighted
        + defensive_weighted;
    println!("  Total: {}", total);
}

#[test]
fn test_threat_bias() {
    println!("=== Testing Threat Bias ===");

    let test_state = GameState::new();

    // Test total threat scores
    let total_threat_p1 = test_state.threat_score(Player::Player1);
    let total_threat_p2 = test_state.threat_score(Player::Player2);
    println!("Total threat scores (empty board):");
    println!("  Player1: {}", total_threat_p1);
    println!("  Player2: {}", total_threat_p2);
    println!("  Difference: {}", total_threat_p1 - total_threat_p2);

    if total_threat_p1 != total_threat_p2 {
        println!("  ⚠️  WARNING: Threat scores are different for Player1 vs Player2!");
    }

    // Test mobility scores
    let mobility_p1 = test_state.mobility_score(Player::Player1);
    let mobility_p2 = test_state.mobility_score(Player::Player2);
    println!("\nMobility scores (empty board):");
    println!("  Player1: {}", mobility_p1);
    println!("  Player2: {}", mobility_p2);
    println!("  Difference: {}", mobility_p1 - mobility_p2);

    if mobility_p1 != mobility_p2 {
        println!("  ⚠️  WARNING: Mobility scores are different for Player1 vs Player2!");
    }
}
