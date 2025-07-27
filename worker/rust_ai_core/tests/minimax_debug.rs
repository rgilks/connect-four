use connect_four_ai_core::genetic_params::GeneticParams;
use connect_four_ai_core::{GameState, HeuristicAI, AI};

#[test]
fn test_transposition_table_state() {
    println!("=== Transposition Table State Test ===");

    // Create a simple position
    let mut game = GameState::new();
    game.make_move(3).unwrap(); // Player1 moves to center

    let mut ai = AI::new();

    println!("Testing the same position multiple times:");
    println!(
        "Position: Player1 moved to center, current player: {:?}",
        game.current_player
    );
    println!("Evaluation: {}", game.evaluate());

    // Test depth 3 multiple times without clearing
    for i in 1..=5 {
        let (best_move, evaluations) = ai.get_best_move(&game, 3);
        println!(
            "Run {} (no clear): Best move {:?}, Score: {:.2}, Nodes: {}, Cache hits: {}",
            i, best_move, evaluations[0].score, ai.nodes_evaluated, ai.transposition_hits
        );
    }

    // Clear and test again
    ai.clear_transposition_table();
    println!("\nAfter clearing transposition table:");

    for i in 1..=3 {
        let (best_move, evaluations) = ai.get_best_move(&game, 3);
        println!(
            "Run {} (after clear): Best move {:?}, Score: {:.2}, Nodes: {}, Cache hits: {}",
            i, best_move, evaluations[0].score, ai.nodes_evaluated, ai.transposition_hits
        );
    }
}

#[test]
fn test_multiple_games_state() {
    println!("=== Multiple Games State Test ===");

    let mut ai = AI::new();

    // Play several games and see if state accumulates
    for game_num in 1..=3 {
        println!("\n=== Game {} ===", game_num);

        let mut game = GameState::new();
        let mut moves = 0;

        while !game.is_game_over() && moves < 10 {
            let (best_move, evaluations) = ai.get_best_move(&game, 3);
            println!(
                "Move {}: Player {:?}, Best move: {:?}, Score: {:.2}, Nodes: {}, Cache hits: {}",
                moves + 1,
                game.current_player,
                best_move,
                evaluations[0].score,
                ai.nodes_evaluated,
                ai.transposition_hits
            );

            if let Some(col) = best_move {
                game.make_move(col).unwrap();
            } else {
                break;
            }
            moves += 1;
        }

        // Don't clear between games to see if state persists
        println!(
            "End of game {}: Transposition table size: {}",
            game_num,
            ai.get_transposition_table_size()
        );
    }

    // Now clear and play one more game
    ai.clear_transposition_table();
    println!("\n=== Game 4 (after clear) ===");

    let mut game = GameState::new();
    let mut moves = 0;

    while !game.is_game_over() && moves < 5 {
        let (best_move, evaluations) = ai.get_best_move(&game, 3);
        println!(
            "Move {}: Player {:?}, Best move: {:?}, Score: {:.2}, Nodes: {}, Cache hits: {}",
            moves + 1,
            game.current_player,
            best_move,
            evaluations[0].score,
            ai.nodes_evaluated,
            ai.transposition_hits
        );

        if let Some(col) = best_move {
            game.make_move(col).unwrap();
        } else {
            break;
        }
        moves += 1;
    }
}

#[test]
fn test_evaluation_debug() {
    println!("=== Evaluation Debug Test ===");

    // Create a simple position
    let mut game = GameState::new();
    game.make_move(3).unwrap(); // Player1 moves to center

    println!("Position after Player1 moves to center:");
    println!("Current player: {:?}", game.current_player);

    // Test with default parameters
    let default_params = GeneticParams::default();
    let mut game_default = GameState::with_genetic_params(default_params);
    game_default.board = game.board.clone();
    game_default.current_player = game.current_player;

    println!("\nWith DEFAULT parameters:");
    println!("Evaluation: {}", game_default.evaluate());

    // Test with evolved parameters
    let evolved_params = GeneticParams::load_from_file("ml/data/genetic_params/evolved.json")
        .unwrap_or_else(|_| GeneticParams::default());
    let mut game_evolved = GameState::with_genetic_params(evolved_params.clone());
    game_evolved.board = game.board.clone();
    game_evolved.current_player = game.current_player;

    println!("With EVOLVED parameters:");
    println!("Evaluation: {}", game_evolved.evaluate());

    // Let's see what the individual components are
    println!("\n=== Component Analysis ===");

    // Position evaluation
    let mut pos_score = 0;
    for col in 0..7 {
        let column_value = match col {
            3 => evolved_params.center_column_value,
            2 | 4 => evolved_params.adjacent_center_value,
            1 | 5 => evolved_params.outer_column_value,
            0 | 6 => evolved_params.edge_column_value,
            _ => evolved_params.edge_column_value,
        };

        for row in 0..6 {
            match game.board[col][row] {
                connect_four_ai_core::Cell::Player1 => {
                    let score = (column_value as f64
                        * (6 - row) as f64
                        * evolved_params.row_height_weight) as i32;
                    pos_score += score;
                    if col == 3 && row == 5 {
                        // The piece we just placed
                        println!("Player1 piece at (3,5): column_value={}, row_factor={}, weight={}, score={}", 
                                column_value, 6-row, evolved_params.row_height_weight, score);
                    }
                }
                connect_four_ai_core::Cell::Player2 => {
                    let score = (column_value as f64
                        * (6 - row) as f64
                        * evolved_params.row_height_weight) as i32;
                    pos_score -= score;
                }
                connect_four_ai_core::Cell::Empty => {}
            }
        }
    }

    println!("Position score component: {}", pos_score);

    // Other components
    let center_control_p1 =
        game_evolved.center_control_score(connect_four_ai_core::Player::Player1);
    let center_control_p2 =
        game_evolved.center_control_score(connect_four_ai_core::Player::Player2);
    let center_weight = evolved_params.center_control_weight as i32;
    let center_score = center_control_p1 * center_weight - center_control_p2 * center_weight;

    println!(
        "Center control: P1={}, P2={}, weight={}, score={}",
        center_control_p1, center_control_p2, center_weight, center_score
    );

    let threat_p1 = game_evolved.threat_score(connect_four_ai_core::Player::Player1);
    let threat_p2 = game_evolved.threat_score(connect_four_ai_core::Player::Player2);
    let threat_weight = evolved_params.threat_weight as i32;
    let threat_score = threat_p1 * threat_weight - threat_p2 * threat_weight;

    println!(
        "Threat score: P1={}, P2={}, weight={}, score={}",
        threat_p1, threat_p2, threat_weight, threat_score
    );
}

#[test]
fn test_evaluation_consistency() {
    println!("=== Evaluation Consistency Test ===");

    // Test with default parameters
    let default_params = GeneticParams::default();
    let mut game_default = GameState::with_genetic_params(default_params);
    game_default.make_move(3).unwrap();

    println!("With DEFAULT parameters:");
    println!("Evaluation: {}", game_default.evaluate());

    // Test with evolved parameters
    let evolved_params = GeneticParams::load_from_file("ml/data/genetic_params/evolved.json")
        .unwrap_or_else(|_| GeneticParams::default());
    let mut game_evolved = GameState::with_genetic_params(evolved_params);
    game_evolved.make_move(3).unwrap();

    println!("With EVOLVED parameters:");
    println!("Evaluation: {}", game_evolved.evaluate());

    // Test minimax with both
    let mut ai = AI::new();

    println!("\n=== Testing with DEFAULT parameters ===");
    for depth in 1..=3 {
        ai.clear_transposition_table();
        let (best_move, evaluations) = ai.get_best_move(&game_default, depth);
        println!(
            "Depth {}: Best move {:?}, Score: {:.2}",
            depth, best_move, evaluations[0].score
        );
    }

    println!("\n=== Testing with EVOLVED parameters ===");
    for depth in 1..=3 {
        ai.clear_transposition_table();
        let (best_move, evaluations) = ai.get_best_move(&game_evolved, depth);
        println!(
            "Depth {}: Best move {:?}, Score: {:.2}",
            depth, best_move, evaluations[0].score
        );
    }
}

#[test]
fn test_simple_minimax_consistency() {
    println!("=== Simple Minimax Consistency Test ===");

    // Create a very simple position - just one move made
    let mut game = GameState::new();
    game.make_move(3).unwrap(); // Player1 moves to center

    println!("Simple position after Player1 moves to center:");
    println!("Current player: {:?}", game.current_player);
    println!("Evaluation: {}", game.evaluate());

    let mut ai = AI::new();

    // Test depths 1-4 on this simple position
    for depth in 1..=4 {
        ai.clear_transposition_table();
        let (best_move, evaluations) = ai.get_best_move(&game, depth);

        println!("\n=== Depth {} ===", depth);
        println!("Best move: {:?}", best_move);
        println!("Nodes evaluated: {}", ai.nodes_evaluated);

        // Show all evaluations sorted by score
        let mut sorted_evals = evaluations.clone();
        sorted_evals.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        for (i, eval) in sorted_evals.iter().enumerate() {
            println!(
                "  {}: Column {}: Score {:.2}",
                i + 1,
                eval.column,
                eval.score
            );
        }
    }
}

#[test]
fn test_minimax_depth_comparison() {
    println!("=== Minimax Depth Comparison Test ===");

    // Create a simple position where deeper search should find a better move
    let mut game = GameState::new();

    // Make some moves to create a position where depth matters
    game.make_move(3).unwrap(); // Player1 center
    game.make_move(3).unwrap(); // Player2 center
    game.make_move(2).unwrap(); // Player1 left of center
    game.make_move(4).unwrap(); // Player2 right of center

    println!("Board after 4 moves:");
    println!("Current player: {:?}", game.current_player);
    println!("Evaluation: {}", game.evaluate());

    let mut ai = AI::new();

    // Test different depths
    for depth in 1..=6 {
        ai.clear_transposition_table();
        let (best_move_1, evaluations_1) = ai.get_best_move(&game, depth);
        let nodes_1 = ai.nodes_evaluated;

        println!("\n=== Depth {} ===", depth);
        println!("Best move: {:?}", best_move_1);
        println!("Nodes evaluated: {}", nodes_1);
        for eval in &evaluations_1[..evaluations_1.len().min(3)] {
            println!("Column {}: Score {:.2}", eval.column, eval.score);
        }
    }
}

#[test]
fn test_minimax_alpha_beta_bug() {
    println!("=== Alpha-Beta Bug Test ===");

    let mut game = GameState::new();
    let mut ai = AI::new();

    // Test a position where alpha-beta should work correctly
    game.make_move(3).unwrap();

    println!("Testing depth 3 with alpha-beta:");
    let (best_move, _evaluations) = ai.get_best_move(&game, 3);
    println!("Best move: {:?}", best_move);
    println!("Nodes evaluated: {}", ai.nodes_evaluated);

    // Now test without alpha-beta (by modifying the function temporarily)
    // This would require modifying the minimax function to disable pruning
    println!("The issue might be in the alpha-beta implementation");
}

#[test]
fn test_deep_search_advantage() {
    println!("=== Deep Search Advantage Test ===");

    // Create a position where deeper search should definitely find a better move
    // This is a position where there's a forced win that requires 3 moves to see
    let mut game = GameState::new();

    // Set up a position where Player1 can force a win in 3 moves
    // This requires depth 6 to see the full sequence
    game.make_move(3).unwrap(); // Player1 center
    game.make_move(2).unwrap(); // Player2 left
    game.make_move(3).unwrap(); // Player1 center again
    game.make_move(4).unwrap(); // Player2 right
    game.make_move(3).unwrap(); // Player1 center again
    game.make_move(1).unwrap(); // Player2 left
    game.make_move(2).unwrap(); // Player1 left
    game.make_move(5).unwrap(); // Player2 right
    game.make_move(4).unwrap(); // Player1 right
    game.make_move(0).unwrap(); // Player2 edge

    println!("Position after 10 moves:");
    println!("Current player: {:?}", game.current_player);
    println!("Evaluation: {}", game.evaluate());
    println!("Game over: {}", game.is_game_over());
    if let Some(winner) = game.get_winner() {
        println!("Winner: {:?}", winner);
    }

    let mut ai = AI::new();

    // Test different depths to see if deeper search finds better moves
    for depth in 1..=8 {
        ai.clear_transposition_table();
        let start = std::time::Instant::now();
        let (best_move, evaluations) = ai.get_best_move(&game, depth);
        let duration = start.elapsed();

        println!("\n=== Depth {} ===", depth);
        println!("Best move: {:?}", best_move);
        println!("Time: {:?}", duration);
        println!("Nodes evaluated: {}", ai.nodes_evaluated);

        // Show top 3 moves
        let mut sorted_evals = evaluations.clone();
        sorted_evals.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        for (i, eval) in sorted_evals.iter().enumerate().take(3) {
            println!(
                "  {}: Column {}: Score {:.2}",
                i + 1,
                eval.column,
                eval.score
            );
        }

        // Check if this move leads to a win
        if let Some(col) = best_move {
            let mut test_game = game.clone();
            if test_game.make_move(col).is_ok() {
                if test_game.is_game_over() {
                    if let Some(winner) = test_game.get_winner() {
                        println!("  -> This move leads to {:?} winning!", winner);
                    }
                }
            }
        }
    }
}

#[test]
fn test_winning_sequence_detection() {
    println!("=== Winning Sequence Detection Test ===");

    // Create a simple position where there's a clear winning sequence
    let mut game = GameState::new();

    // Set up a position where Player1 can win in 2 moves
    game.make_move(3).unwrap(); // Player1 center
    game.make_move(2).unwrap(); // Player2 left
    game.make_move(3).unwrap(); // Player1 center
    game.make_move(4).unwrap(); // Player2 right
    game.make_move(3).unwrap(); // Player1 center - should see win coming

    println!("Position after 5 moves:");
    println!("Current player: {:?}", game.current_player);
    println!("Evaluation: {}", game.evaluate());

    let mut ai = AI::new();

    // Test if different depths can see the winning sequence
    for depth in 1..=6 {
        ai.clear_transposition_table();
        let (best_move, evaluations) = ai.get_best_move(&game, depth);

        println!("\n=== Depth {} ===", depth);
        println!("Best move: {:?}", best_move);
        println!("Nodes evaluated: {}", ai.nodes_evaluated);

        // Show all evaluations
        for (i, eval) in evaluations.iter().enumerate() {
            println!(
                "  {}: Column {}: Score {:.2}",
                i + 1,
                eval.column,
                eval.score
            );
        }

        // Check if this move leads to a win
        if let Some(col) = best_move {
            let mut test_game = game.clone();
            if test_game.make_move(col).is_ok() {
                if test_game.is_game_over() {
                    if let Some(winner) = test_game.get_winner() {
                        println!("  -> This move leads to {:?} winning!", winner);
                    }
                } else {
                    // Check if opponent can block
                    let opponent_moves = test_game.get_valid_moves();
                    let mut can_block = false;
                    for opp_move in opponent_moves {
                        let mut test_game2 = test_game.clone();
                        if test_game2.make_move(opp_move).is_ok() {
                            if test_game2.is_game_over() {
                                if let Some(winner) = test_game2.get_winner() {
                                    if winner == test_game.current_player.opponent() {
                                        can_block = true;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    if can_block {
                        println!("  -> Opponent can block this move");
                    } else {
                        println!("  -> This move creates a threat");
                    }
                }
            }
        }
    }
}

#[test]
fn test_evaluation_quality_at_depth() {
    println!("=== Evaluation Quality at Depth Test ===");

    // Test if the evaluation function is good enough for deep search
    let mut game = GameState::new();

    // Create a position where evaluation should be meaningful
    game.make_move(3).unwrap(); // Player1 center
    game.make_move(2).unwrap(); // Player2 left
    game.make_move(4).unwrap(); // Player1 right
    game.make_move(3).unwrap(); // Player2 center

    println!("Position after 4 moves:");
    println!("Current player: {:?}", game.current_player);
    println!("Evaluation: {}", game.evaluate());

    let mut ai = AI::new();

    // Test how evaluation changes with depth
    for depth in 1..=6 {
        ai.clear_transposition_table();
        let (best_move, evaluations) = ai.get_best_move(&game, depth);

        println!("\n=== Depth {} ===", depth);
        println!("Best move: {:?}", best_move);
        println!("Nodes evaluated: {}", ai.nodes_evaluated);

        // Show evaluation range
        let scores: Vec<f32> = evaluations.iter().map(|e| e.score).collect();
        let min_score = scores.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max_score = scores.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let avg_score = scores.iter().sum::<f32>() / scores.len() as f32;

        println!(
            "Score range: {:.2} to {:.2}, avg: {:.2}",
            min_score, max_score, avg_score
        );

        // Check if scores are reasonable
        if max_score - min_score < 1.0 {
            println!("  ⚠️  Very small score range - evaluation might be too flat");
        } else if max_score - min_score > 10000.0 {
            println!("  ⚠️  Very large score range - evaluation might be too extreme");
        } else {
            println!("  ✅ Score range looks reasonable");
        }
    }
}

#[test]
fn test_emm_depth1_vs_heuristic() {
    println!("=== EMM-Depth1 vs Heuristic AI Comparison ===");

    // Create a simple position
    let mut game = GameState::new();
    game.make_move(3).unwrap(); // Player1 center
    game.make_move(2).unwrap(); // Player2 left
    game.make_move(4).unwrap(); // Player1 right

    println!("Position after 3 moves:");
    println!("Current player: {:?}", game.current_player);
    println!("Evaluation: {}", game.evaluate());

    let mut emm_ai = AI::new();
    let mut heuristic_ai = HeuristicAI::new();

    // Test EMM-Depth1
    emm_ai.clear_transposition_table();
    let (emm_move, emm_evaluations) = emm_ai.get_best_move(&game, 1);

    println!("\n=== EMM-Depth1 ===");
    println!("Best move: {:?}", emm_move);
    println!("Nodes evaluated: {}", emm_ai.nodes_evaluated);
    for (i, eval) in emm_evaluations.iter().enumerate() {
        println!(
            "  {}: Column {}: Score {:.2}",
            i + 1,
            eval.column,
            eval.score
        );
    }

    // Test Heuristic AI
    let (heuristic_move, heuristic_evaluations) = heuristic_ai.get_best_move(&game);

    println!("\n=== Heuristic AI ===");
    println!("Best move: {:?}", heuristic_move);
    println!("Nodes evaluated: {}", heuristic_ai.nodes_evaluated);
    for (i, eval) in heuristic_evaluations.iter().enumerate() {
        println!(
            "  {}: Column {}: Score {:.2}",
            i + 1,
            eval.column,
            eval.score
        );
    }

    // Compare results
    println!("\n=== Comparison ===");
    if emm_move == heuristic_move {
        println!("✅ Same best move: {:?}", emm_move);
    } else {
        println!(
            "❌ Different moves: EMM={:?}, Heuristic={:?}",
            emm_move, heuristic_move
        );
    }

    // Compare all evaluations
    let mut emm_scores: Vec<(u8, f32)> = emm_evaluations
        .iter()
        .map(|e| (e.column, e.score))
        .collect();
    let mut heuristic_scores: Vec<(u8, f32)> = heuristic_evaluations
        .iter()
        .map(|e| (e.column, e.score))
        .collect();

    emm_scores.sort_by(|a, b| a.0.cmp(&b.0));
    heuristic_scores.sort_by(|a, b| a.0.cmp(&b.0));

    println!("Score comparison:");
    for (emm, heuristic) in emm_scores.iter().zip(heuristic_scores.iter()) {
        if (emm.1 - heuristic.1).abs() < 0.01 {
            println!(
                "  Column {}: EMM={:.2}, Heuristic={:.2} ✅",
                emm.0, emm.1, heuristic.1
            );
        } else {
            println!(
                "  Column {}: EMM={:.2}, Heuristic={:.2} ❌",
                emm.0, emm.1, heuristic.1
            );
        }
    }

    // Test with a position that has immediate wins/blocks
    println!("\n=== Testing with immediate win/block position ===");
    let mut game2 = GameState::new();
    game2.make_move(3).unwrap(); // Player1 center
    game2.make_move(3).unwrap(); // Player2 center
    game2.make_move(3).unwrap(); // Player1 center
    game2.make_move(2).unwrap(); // Player2 left
    game2.make_move(4).unwrap(); // Player1 right
    game2.make_move(4).unwrap(); // Player2 right
    game2.make_move(5).unwrap(); // Player1 right

    println!("Position after 7 moves:");
    println!("Current player: {:?}", game2.current_player);
    println!("Evaluation: {}", game2.evaluate());

    emm_ai.clear_transposition_table();
    let (emm_move2, _emm_evaluations2) = emm_ai.get_best_move(&game2, 1);
    let (heuristic_move2, _heuristic_evaluations2) = heuristic_ai.get_best_move(&game2);

    println!("\nEMM-Depth1: {:?}", emm_move2);
    println!("Heuristic: {:?}", heuristic_move2);

    if emm_move2 == heuristic_move2 {
        println!("✅ Same move in tactical position");
    } else {
        println!("❌ Different moves in tactical position");
    }
}

#[test]
fn test_simple_rules_vs_sophisticated_evaluation() {
    println!("=== Simple Rules vs Sophisticated Evaluation ===");

    // Create a position where deeper search should find a better move
    let mut game = GameState::new();

    // Set up a position where there's a forced win that requires 3 moves to see
    game.make_move(3).unwrap(); // Player1 center
    game.make_move(2).unwrap(); // Player2 left
    game.make_move(3).unwrap(); // Player1 center
    game.make_move(4).unwrap(); // Player2 right
    game.make_move(3).unwrap(); // Player1 center
    game.make_move(1).unwrap(); // Player2 left
    game.make_move(2).unwrap(); // Player1 left
    game.make_move(5).unwrap(); // Player2 right
    game.make_move(4).unwrap(); // Player1 right
    game.make_move(0).unwrap(); // Player2 edge

    println!("Position after 10 moves:");
    println!("Current player: {:?}", game.current_player);
    println!("Evaluation: {}", game.evaluate());

    let mut ai = AI::new();

    // Test different depths
    for depth in 1..=6 {
        ai.clear_transposition_table();
        let (best_move, evaluations) = ai.get_best_move(&game, depth);

        println!("\n=== Depth {} ===", depth);
        println!("Best move: {:?}", best_move);
        println!("Nodes evaluated: {}", ai.nodes_evaluated);

        // Show top 3 moves
        let mut sorted_evals = evaluations.clone();
        sorted_evals.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        for (i, eval) in sorted_evals.iter().enumerate().take(3) {
            println!(
                "  {}: Column {}: Score {:.2}",
                i + 1,
                eval.column,
                eval.score
            );
        }

        // Check if this move leads to a win
        if let Some(col) = best_move {
            let mut test_game = game.clone();
            if test_game.make_move(col).is_ok() {
                if test_game.is_game_over() {
                    if let Some(winner) = test_game.get_winner() {
                        println!("  -> This move leads to {:?} winning!", winner);
                    }
                } else {
                    // Check if it creates a strong threat
                    let threat_score = test_game.threat_score(game.current_player);
                    if threat_score > 0 {
                        println!("  -> This move creates {} threats", threat_score);
                    }
                }
            }
        }
    }
}

#[test]
fn test_evaluation_quality_comparison() {
    println!("=== Evaluation Quality Comparison ===");

    // Create a position where evaluation quality matters
    let mut game = GameState::new();

    // Set up a position where the evaluation function should be tested
    game.make_move(3).unwrap(); // Player1 center
    game.make_move(2).unwrap(); // Player2 left
    game.make_move(4).unwrap(); // Player1 right
    game.make_move(3).unwrap(); // Player2 center

    println!("Position after 4 moves:");
    println!("Current player: {:?}", game.current_player);
    println!("Raw evaluation: {}", game.evaluate());

    // Show what the evaluation function is actually measuring
    println!("\n=== Evaluation Components ===");

    let center_control_p1 = game.center_control_score(connect_four_ai_core::Player::Player1);
    let center_control_p2 = game.center_control_score(connect_four_ai_core::Player::Player2);
    println!(
        "Center control: P1={}, P2={}",
        center_control_p1, center_control_p2
    );

    let threat_p1 = game.threat_score(connect_four_ai_core::Player::Player1);
    let threat_p2 = game.threat_score(connect_four_ai_core::Player::Player2);
    println!("Threats: P1={}, P2={}", threat_p1, threat_p2);

    let mobility_p1 = game.mobility_score(connect_four_ai_core::Player::Player1);
    let mobility_p2 = game.mobility_score(connect_four_ai_core::Player::Player2);
    println!("Mobility: P1={}, P2={}", mobility_p1, mobility_p2);

    let vertical_p1 = game.vertical_control_score(connect_four_ai_core::Player::Player1);
    let vertical_p2 = game.vertical_control_score(connect_four_ai_core::Player::Player2);
    println!("Vertical control: P1={}, P2={}", vertical_p1, vertical_p2);

    let horizontal_p1 = game.horizontal_control_score(connect_four_ai_core::Player::Player1);
    let horizontal_p2 = game.horizontal_control_score(connect_four_ai_core::Player::Player2);
    println!(
        "Horizontal control: P1={}, P2={}",
        horizontal_p1, horizontal_p2
    );

    // Test how different depths evaluate this position
    let mut ai = AI::new();

    println!("\n=== Depth Comparison ===");
    for depth in 1..=4 {
        ai.clear_transposition_table();
        let (best_move, evaluations) = ai.get_best_move(&game, depth);

        println!(
            "Depth {}: Best move {:?}, Score: {:.2}",
            depth, best_move, evaluations[0].score
        );
    }
}

#[test]
fn test_why_deeper_should_be_better() {
    println!("=== Why Deeper Search Should Be Better ===");

    // Create a position where deeper search reveals a better move
    let mut game = GameState::new();

    // Set up a position where there's a subtle advantage that requires deeper search
    game.make_move(3).unwrap(); // Player1 center
    game.make_move(2).unwrap(); // Player2 left
    game.make_move(4).unwrap(); // Player1 right
    game.make_move(3).unwrap(); // Player2 center
    game.make_move(2).unwrap(); // Player1 left
    game.make_move(4).unwrap(); // Player2 right
    game.make_move(1).unwrap(); // Player1 left
    game.make_move(5).unwrap(); // Player2 right

    println!("Position after 8 moves:");
    println!("Current player: {:?}", game.current_player);
    println!("Evaluation: {}", game.evaluate());

    let mut ai = AI::new();

    // Test if deeper search finds different moves
    for depth in 1..=6 {
        ai.clear_transposition_table();
        let (best_move, evaluations) = ai.get_best_move(&game, depth);

        println!("\n=== Depth {} ===", depth);
        println!("Best move: {:?}", best_move);
        println!("Nodes evaluated: {}", ai.nodes_evaluated);

        // Show all moves and their scores
        for (i, eval) in evaluations.iter().enumerate() {
            println!(
                "  {}: Column {}: Score {:.2}",
                i + 1,
                eval.column,
                eval.score
            );
        }

        // Check if this move creates a winning sequence
        if let Some(col) = best_move {
            let mut test_game = game.clone();
            if test_game.make_move(col).is_ok() {
                // Check if opponent can block effectively
                let opponent_moves = test_game.get_valid_moves();
                let mut best_opponent_score = f32::NEG_INFINITY;

                for opp_move in opponent_moves {
                    let mut opp_test = test_game.clone();
                    if opp_test.make_move(opp_move).is_ok() {
                        let score = opp_test.evaluate() as f32;
                        if score > best_opponent_score {
                            best_opponent_score = score;
                        }
                    }
                }

                println!(
                    "  -> After opponent's best response: {:.2}",
                    best_opponent_score
                );
            }
        }
    }
}
