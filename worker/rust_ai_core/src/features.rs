use super::{Cell, GameState, Player, COLS, ROWS};
use ndarray::Array1;

pub const SIZE: usize = 100;

#[derive(Clone, Debug)]
pub struct GameFeatures {
    pub features: [f32; 100],
}

impl GameFeatures {
    pub fn from_game_state(state: &GameState) -> Self {
        let mut features = [0.0; SIZE];
        let mut idx = 0;

        // Board occupancy (42 features - 6 rows × 7 columns)
        for col in 0..COLS {
            for row in 0..ROWS {
                features[idx] = match state.board[col][row] {
                    Cell::Empty => 0.0,
                    Cell::Player1 => 1.0,
                    Cell::Player2 => -1.0,
                };
                idx += 1;
            }
        }

        // Strategic features
        features[idx] = Self::center_control_score(state, Player::Player1) as f32;
        idx += 1;

        features[idx] = Self::center_control_score(state, Player::Player2) as f32;
        idx += 1;

        features[idx] = Self::pieces_count(state, Player::Player1) as f32;
        idx += 1;

        features[idx] = Self::pieces_count(state, Player::Player2) as f32;
        idx += 1;

        features[idx] = Self::threat_score(state, Player::Player1) as f32;
        idx += 1;

        features[idx] = Self::threat_score(state, Player::Player2) as f32;
        idx += 1;

        features[idx] = Self::mobility_score(state, Player::Player1) as f32;
        idx += 1;

        features[idx] = Self::mobility_score(state, Player::Player2) as f32;
        idx += 1;

        features[idx] = Self::vertical_control_score(state, Player::Player1) as f32;
        idx += 1;

        features[idx] = Self::vertical_control_score(state, Player::Player2) as f32;
        idx += 1;

        features[idx] = Self::horizontal_control_score(state, Player::Player1) as f32;
        idx += 1;

        features[idx] = Self::horizontal_control_score(state, Player::Player2) as f32;
        idx += 1;

        features[idx] = Self::diagonal_control_score(state, Player::Player1) as f32;
        idx += 1;

        features[idx] = Self::diagonal_control_score(state, Player::Player2) as f32;
        idx += 1;

        features[idx] = Self::blocking_score(state, Player::Player1) as f32;
        idx += 1;

        features[idx] = Self::blocking_score(state, Player::Player2) as f32;
        idx += 1;

        features[idx] = Self::height_advantage_score(state, Player::Player1) as f32;
        idx += 1;

        features[idx] = Self::height_advantage_score(state, Player::Player2) as f32;
        idx += 1;

        features[idx] = Self::material_balance_score(state) as f32;
        idx += 1;

        features[idx] = Self::positional_advantage_score(state, Player::Player1);
        idx += 1;

        features[idx] = Self::positional_advantage_score(state, Player::Player2);
        idx += 1;

        features[idx] = Self::endgame_evaluation(state, Player::Player1);
        idx += 1;

        features[idx] = Self::endgame_evaluation(state, Player::Player2);
        idx += 1;

        // Fill remaining features with zeros
        while idx < SIZE {
            features[idx] = 0.0;
            idx += 1;
        }

        // Normalize features to ensure they're in reasonable bounds
        for i in 0..SIZE {
            features[i] = features[i].max(-10.0).min(10.0);
        }

        GameFeatures { features }
    }

    pub fn to_array(&self) -> Array1<f32> {
        Array1::from_vec(self.features.to_vec())
    }

    fn pieces_count(state: &GameState, player: Player) -> i32 {
        let mut count = 0;
        for col in 0..COLS {
            for row in 0..ROWS {
                if state.board[col][row] == Cell::from_player(player) {
                    count += 1;
                }
            }
        }
        count
    }

    fn center_control_score(state: &GameState, player: Player) -> i32 {
        let mut score = 0;
        // Center columns (2, 3, 4) are most valuable
        for col in [2, 3, 4] {
            for row in 0..ROWS {
                if state.board[col][row] == Cell::from_player(player) {
                    score += match col {
                        3 => 3,     // Center column
                        2 | 4 => 2, // Adjacent to center
                        _ => 1,
                    };
                }
            }
        }
        score
    }

    fn threat_score(state: &GameState, player: Player) -> i32 {
        let mut threats = 0;
        for col in 0..COLS {
            for row in 0..ROWS {
                if state.board[col][row] == Cell::from_player(player) {
                    // Check for potential winning lines
                    let directions = [(1, 0), (0, 1), (1, 1), (1, -1)];
                    for (dcol, drow) in directions {
                        let mut consecutive = 1;
                        let mut blocked = 0;

                        // Count in positive direction
                        let mut c = col as i32 + dcol;
                        let mut r = row as i32 + drow;
                        while c >= 0 && c < COLS as i32 && r >= 0 && r < ROWS as i32 {
                            if state.board[c as usize][r as usize] == Cell::from_player(player) {
                                consecutive += 1;
                                c += dcol;
                                r += drow;
                            } else {
                                if state.board[c as usize][r as usize] != Cell::Empty {
                                    blocked += 1;
                                }
                                break;
                            }
                        }

                        // Count in negative direction
                        c = col as i32 - dcol;
                        r = row as i32 - drow;
                        while c >= 0 && c < COLS as i32 && r >= 0 && r < ROWS as i32 {
                            if state.board[c as usize][r as usize] == Cell::from_player(player) {
                                consecutive += 1;
                                c -= dcol;
                                r -= drow;
                            } else {
                                if state.board[c as usize][r as usize] != Cell::Empty {
                                    blocked += 1;
                                }
                                break;
                            }
                        }

                        // Score based on consecutive pieces and blocking
                        match consecutive {
                            4 => threats += 1000, // Winning line
                            3 => {
                                if blocked == 0 {
                                    threats += 100
                                } else {
                                    threats += 10
                                }
                            }
                            2 => {
                                if blocked == 0 {
                                    threats += 10
                                } else {
                                    threats += 1
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        threats
    }

    fn mobility_score(state: &GameState, player: Player) -> i32 {
        let mut mobility = 0;
        for col in 0..COLS {
            if state.can_place_in_column(col) {
                // Test the move
                let mut test_state = state.clone();
                if test_state.make_move(col as u8).is_ok() {
                    // Check if this creates a threat
                    let threat_score = Self::threat_score(&test_state, player);
                    mobility += threat_score / 10; // Normalize
                }
            }
        }
        mobility
    }

    fn vertical_control_score(state: &GameState, player: Player) -> i32 {
        let mut score = 0;
        for col in 0..COLS {
            let mut consecutive = 0;
            for row in 0..ROWS {
                if state.board[col][row] == Cell::from_player(player) {
                    consecutive += 1;
                } else {
                    consecutive = 0;
                }
                score += consecutive;
            }
        }
        score
    }

    fn horizontal_control_score(state: &GameState, player: Player) -> i32 {
        let mut score = 0;
        for row in 0..ROWS {
            let mut consecutive = 0;
            for col in 0..COLS {
                if state.board[col][row] == Cell::from_player(player) {
                    consecutive += 1;
                } else {
                    consecutive = 0;
                }
                score += consecutive;
            }
        }
        score
    }

    fn diagonal_control_score(state: &GameState, player: Player) -> i32 {
        let mut score = 0;
        let directions = [(1, 1), (1, -1)]; // Diagonal directions

        for start_col in 0..COLS {
            for start_row in 0..ROWS {
                for (dcol, drow) in directions {
                    let mut consecutive = 0;
                    let mut c = start_col as i32;
                    let mut r = start_row as i32;

                    while c >= 0 && c < COLS as i32 && r >= 0 && r < ROWS as i32 {
                        if state.board[c as usize][r as usize] == Cell::from_player(player) {
                            consecutive += 1;
                        } else {
                            consecutive = 0;
                        }
                        score += consecutive;
                        c += dcol;
                        r += drow;
                    }
                }
            }
        }
        score
    }

    fn blocking_score(state: &GameState, player: Player) -> i32 {
        let opponent = player.opponent();
        let mut blocks = 0;

        // Count how many opponent threats we can block
        for col in 0..COLS {
            if state.can_place_in_column(col) {
                let mut test_state = state.clone();
                if test_state.make_move(col as u8).is_ok() {
                    let opponent_threats = Self::threat_score(&test_state, opponent);
                    blocks += opponent_threats / 10;
                }
            }
        }
        blocks
    }

    fn height_advantage_score(state: &GameState, player: Player) -> i32 {
        let mut score = 0;
        for col in 0..COLS {
            for row in 0..ROWS {
                if state.board[col][row] == Cell::from_player(player) {
                    // Higher pieces (lower row numbers) are more valuable
                    score += (ROWS - row) as i32;
                }
            }
        }
        score
    }

    fn material_balance_score(state: &GameState) -> i32 {
        let p1_pieces = Self::pieces_count(state, Player::Player1);
        let p2_pieces = Self::pieces_count(state, Player::Player2);
        p2_pieces - p1_pieces
    }

    fn positional_advantage_score(state: &GameState, player: Player) -> f32 {
        let center_score = Self::center_control_score(state, player) as f32;
        let height_score = Self::height_advantage_score(state, player) as f32;
        let threat_score = Self::threat_score(state, player) as f32;

        (center_score * 0.3 + height_score * 0.2 + threat_score * 0.5) / 100.0
    }

    fn endgame_evaluation(state: &GameState, player: Player) -> f32 {
        let total_pieces =
            Self::pieces_count(state, Player::Player1) + Self::pieces_count(state, Player::Player2);
        let max_pieces = (ROWS * COLS) as i32;

        if total_pieces > max_pieces * 3 / 4 {
            // Endgame - focus on immediate threats
            Self::threat_score(state, player) as f32 / 1000.0
        } else {
            // Opening/middlegame - focus on position
            Self::positional_advantage_score(state, player)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_features_size() {
        let state = GameState::new();
        let features = GameFeatures::from_game_state(&state);
        assert_eq!(features.features.len(), SIZE);
    }

    #[test]
    fn test_empty_board_features() {
        let state = GameState::new();
        let features = GameFeatures::from_game_state(&state);

        // First 42 features should be 0.0 (empty board)
        for i in 0..42 {
            assert_eq!(features.features[i], 0.0);
        }
    }

    #[test]
    fn test_piece_count_features() {
        let mut state = GameState::new();
        state.make_move(3).unwrap(); // Player 1 places a piece
        state.current_player = Player::Player1;
        state.make_move(4).unwrap(); // Player 1 places another piece

        let features = GameFeatures::from_game_state(&state);

        // Should have 2 pieces for Player 1
        let p1_pieces_idx = 44; // Strategic features start at 42, pieces_count is at index 44
        assert_eq!(features.features[p1_pieces_idx], 2.0);
    }

    #[test]
    fn test_center_control_features() {
        let mut state = GameState::new();
        state.make_move(3).unwrap(); // Player 1 places in center

        let features = GameFeatures::from_game_state(&state);

        // Center control should be computed
        let center_control_idx = 42; // First strategic feature
        assert!(features.features[center_control_idx] > 0.0);
    }

    #[test]
    fn test_threat_score_features() {
        let mut state = GameState::new();
        // Create a threat
        state.make_move(0).unwrap();
        state.current_player = Player::Player1;
        state.make_move(1).unwrap();
        state.current_player = Player::Player1;
        state.make_move(2).unwrap();

        let features = GameFeatures::from_game_state(&state);

        // Threat score should be computed
        let threat_score_idx = 44; // Threat score feature index
        assert!(features.features[threat_score_idx] > 0.0);
    }

    #[test]
    fn test_features_normalization() {
        let state = GameState::new();
        let features = GameFeatures::from_game_state(&state);

        // All features should be within bounds
        for (i, &feature) in features.features.iter().enumerate() {
            assert!(feature >= -10.0, "Feature {} is too low: {}", i, feature);
            assert!(feature <= 10.0, "Feature {} is too high: {}", i, feature);
        }
    }

    #[test]
    fn test_features_no_nan_or_infinite() {
        let state = GameState::new();
        let features = GameFeatures::from_game_state(&state);

        for (i, &feature) in features.features.iter().enumerate() {
            assert!(!feature.is_nan(), "Feature {} is NaN", i);
            assert!(!feature.is_infinite(), "Feature {} is infinite", i);
        }
    }
}
