use connect_four_ai_core::{Cell, GameState, Player};

#[test]
fn test_evaluation_debug() {
    println!("🔍 Evaluation Function Debug");
    println!("============================");

    // Test 1: Empty board
    let game_state = GameState::new();
    let eval = game_state.evaluate();
    println!("Empty board: {}", eval);
    assert_eq!(eval, 0, "Empty board should evaluate to 0");

    // Test 2: Single center piece
    let mut game_state = GameState::new();
    game_state.make_move(3).unwrap(); // Player1 plays center
    let eval = game_state.evaluate();
    println!("Player1 center piece: {}", eval);

    // Test 3: Two center pieces
    game_state.make_move(3).unwrap(); // Player2 plays center
    let eval = game_state.evaluate();
    println!("Two center pieces: {}", eval);

    // Test 4: Winning position for Player1
    let mut game_state = GameState::new();
    game_state.make_move(0).unwrap(); // P1
    game_state.make_move(1).unwrap(); // P2
    game_state.make_move(0).unwrap(); // P1
    game_state.make_move(1).unwrap(); // P2
    game_state.make_move(0).unwrap(); // P1

    println!("\nWinning position for Player1:");
    print_board(&game_state);

    let eval = game_state.evaluate();
    println!("Evaluation: {}", eval);

    // Test 5: Test individual evaluation components
    test_evaluation_components(&game_state);
}

fn test_evaluation_components(game_state: &GameState) {
    println!("\n📊 Evaluation Components:");
    println!("-------------------------");

    // Position scores
    let pos_p1 = game_state.position_score(Player::Player1);
    let pos_p2 = game_state.position_score(Player::Player2);
    println!(
        "Position - P1: {}, P2: {}, Diff: {}",
        pos_p1,
        pos_p2,
        pos_p2 - pos_p1
    );

    // Center control
    let center_p1 = game_state.center_control_score(Player::Player1);
    let center_p2 = game_state.center_control_score(Player::Player2);
    println!(
        "Center - P1: {}, P2: {}, Diff: {}",
        center_p1,
        center_p2,
        center_p2 - center_p1
    );

    // Threat scores
    let threat_p1 = game_state.threat_score(Player::Player1);
    let threat_p2 = game_state.threat_score(Player::Player2);
    println!(
        "Threat - P1: {}, P2: {}, Diff: {}",
        threat_p1,
        threat_p2,
        threat_p2 - threat_p1
    );

    // Piece counts
    let pieces_p1 = game_state.pieces_count(Player::Player1);
    let pieces_p2 = game_state.pieces_count(Player::Player2);
    println!(
        "Pieces - P1: {}, P2: {}, Diff: {}",
        pieces_p1,
        pieces_p2,
        pieces_p2 - pieces_p1
    );
}

fn print_board(game_state: &GameState) {
    for row in (0..6).rev() {
        print!("|");
        for col in 0..7 {
            match game_state.board[col][row] {
                Cell::Empty => print!(" "),
                Cell::Player1 => print!("X"),
                Cell::Player2 => print!("O"),
            }
            print!("|");
        }
        println!();
    }
    println!(" 0 1 2 3 4 5 6");
}
