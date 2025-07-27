use connect_four_ai_core::{genetic_params::GeneticParams, GameState, Player, AI};

#[test]
fn test_evolution_debug() {
    println!("=== Evolution Debug Test ===");

    // Test default vs evolved parameters
    let default_params = GeneticParams::default();
    let evolved_params = GeneticParams::random(); // Use random for testing

    println!("Default params: {:?}", default_params);
    println!("Evolved params: {:?}", evolved_params);

    // Test if Player2 has a systematic advantage
    println!("\n=== Testing Player2 Advantage ===");
    let mut p2_wins = 0;
    let games = 20;

    for game_num in 0..games {
        let mut game_state = GameState::new();
        let mut moves_played = 0;
        let max_moves = 42;

        while !game_state.is_game_over() && moves_played < max_moves {
            let _current_player = game_state.current_player;

            // Both players use DEFAULT parameters
            let mut test_state = GameState::with_genetic_params(default_params.clone());
            test_state.board = game_state.board.clone();
            test_state.current_player = game_state.current_player;

            let mut ai = AI::new(); // Fresh AI for each move
            let (best_move, _) = ai.get_best_move(&test_state, 3);

            if let Some(column) = best_move {
                game_state.make_move(column).ok();
            } else {
                break;
            }
            moves_played += 1;
        }

        // Determine winner
        if let Some(winner) = game_state.get_winner() {
            if winner == Player::Player2 {
                p2_wins += 1;
            }
            println!("Game {}: {:?} wins", game_num, winner);
        } else {
            println!("Game {}: Draw", game_num);
        }
    }

    println!(
        "Player2 wins: {}/{} ({:.1}%)",
        p2_wins,
        games,
        (p2_wins as f64 / games as f64) * 100.0
    );

    // Test with shared AI (potential transposition table issue)
    println!("\n=== Testing with Shared AI ===");
    let mut shared_ai = AI::new();
    let mut p2_wins_shared = 0;

    for game_num in 0..10 {
        let mut game_state = GameState::new();
        let mut moves_played = 0;
        let max_moves = 42;

        while !game_state.is_game_over() && moves_played < max_moves {
            let mut test_state = GameState::with_genetic_params(default_params.clone());
            test_state.board = game_state.board.clone();
            test_state.current_player = game_state.current_player;

            let (best_move, _) = shared_ai.get_best_move(&test_state, 3);

            if let Some(column) = best_move {
                game_state.make_move(column).ok();
            } else {
                break;
            }
            moves_played += 1;
        }

        if let Some(winner) = game_state.get_winner() {
            if winner == Player::Player2 {
                p2_wins_shared += 1;
            }
            println!("Shared AI Game {}: {:?} wins", game_num, winner);
        } else {
            println!("Shared AI Game {}: Draw", game_num);
        }
    }

    println!(
        "Player2 wins with shared AI: {}/10 ({:.1}%)",
        p2_wins_shared,
        (p2_wins_shared as f64 / 10.0) * 100.0
    );

    // Test with evolved vs default parameters
    println!("\n=== Testing Evolved vs Default Parameters ===");
    let mut wins = 0;
    let games = 20;

    for game_num in 0..games {
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
            let (best_move, _) = ai.get_best_move(&test_state, 3);

            if let Some(column) = best_move {
                game_state.make_move(column).ok();
            } else {
                break;
            }
            moves_played += 1;
        }

        // Determine winner - evolved params win if they are the winner
        let evolved_wins = if let Some(winner) = game_state.get_winner() {
            if evolved_is_player2 {
                winner == Player::Player2
            } else {
                winner == Player::Player1
            }
        } else {
            // Game ended in draw, evaluate final position using evolved parameters
            let mut evolved_state = GameState::with_genetic_params(evolved_params.clone());
            evolved_state.board = game_state.board.clone();
            evolved_state.current_player = game_state.current_player;
            let evolved_eval = evolved_state.evaluate();
            if evolved_is_player2 {
                evolved_eval < 0
            } else {
                evolved_eval > 0
            }
        };

        if evolved_wins {
            wins += 1;
        }

        println!("Game {}: Evolved wins = {}", game_num, evolved_wins);
    }

    let win_rate = wins as f64 / games as f64;
    println!("Final win rate: {:.3}", win_rate);
}
