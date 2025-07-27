use connect_four_ai_core::{GameState, AI};

#[test]
fn test_minimax_debug() {
    println!("=== Minimax Debug Test ===");
    
    let mut game = GameState::new();
    let mut ai = AI::new();
    
    // Test a simple position
    println!("Empty board:");
    println!("Current player: {:?}", game.current_player);
    println!("Evaluation: {}", game.evaluate());
    
    // Make a move and see what happens
    game.make_move(3).unwrap();
    println!("\nAfter Player1 moves to column 3:");
    println!("Current player: {:?}", game.current_player);
    println!("Evaluation: {}", game.evaluate());
    
    // Test get_best_move at different depths
    println!("\n=== Testing get_best_move Depth 1 ===");
    let (best_move, evaluations) = ai.get_best_move(&game, 1);
    println!("Best move: {:?}", best_move);
    for eval in evaluations {
        println!("Column {}: Score {}", eval.column, eval.score);
    }
    
    println!("\n=== Testing get_best_move Depth 2 ===");
    let (best_move, evaluations) = ai.get_best_move(&game, 2);
    println!("Best move: {:?}", best_move);
    for eval in evaluations {
        println!("Column {}: Score {}", eval.column, eval.score);
    }
    
    println!("\n=== Testing get_best_move Depth 3 ===");
    let (best_move, evaluations) = ai.get_best_move(&game, 3);
    println!("Best move: {:?}", best_move);
    for eval in evaluations {
        println!("Column {}: Score {}", eval.column, eval.score);
    }
}
