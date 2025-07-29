use super::genetic_params::GeneticParams;
use super::{GameState, HeuristicAI, AI};
use super::ml_ai::MLAI;
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen;

#[wasm_bindgen]
pub struct ConnectFourAI {
    ai: AI,
    heuristic_ai: HeuristicAI,
    ml_ai: MLAI,
}

#[wasm_bindgen]
impl ConnectFourAI {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        ConnectFourAI {
            ai: AI::new(),
            heuristic_ai: HeuristicAI::new(),
            ml_ai: MLAI::new(),
        }
    }

    pub fn get_best_move(&mut self, board_state: &JsValue, depth: u8) -> Result<JsValue, JsValue> {
        let state: GameState = serde_wasm_bindgen::from_value(board_state.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let (best_move, evaluations) = self.ai.get_best_move(&state, depth);

        let result = serde_json::json!({
            "move": best_move,
            "evaluations": evaluations,
            "nodes_evaluated": self.ai.nodes_evaluated,
            "transposition_hits": self.ai.transposition_hits,
        });

        Ok(serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))?)
    }

    pub fn get_heuristic_move(&mut self, board_state: &JsValue) -> Result<JsValue, JsValue> {
        let state: GameState = serde_wasm_bindgen::from_value(board_state.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let (best_move, evaluations) = self.heuristic_ai.get_best_move(&state);

        let result = serde_json::json!({
            "move": best_move,
            "evaluations": evaluations,
            "nodes_evaluated": self.heuristic_ai.nodes_evaluated,
        });

        Ok(serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))?)
    }

    pub fn get_ml_move(&mut self, board_state: &JsValue) -> Result<JsValue, JsValue> {
        let state: GameState = serde_wasm_bindgen::from_value(board_state.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Check if game is over
        if state.is_game_over() {
            return Err(JsValue::from_str("Game is already over"));
        }

        // Get valid moves
        let valid_moves = state.get_valid_moves();
        if valid_moves.is_empty() {
            return Err(JsValue::from_str("No valid moves available"));
        }

        // Use ML AI's direct get_best_move method
        let ml_response = self.ml_ai.get_best_move(&state);

        // Convert move_evaluations to the expected format
        let move_evaluations: Vec<serde_json::Value> = ml_response.diagnostics.move_evaluations
            .iter()
            .map(|eval| serde_json::json!({
                "column": eval.column,
                "score": eval.score,
                "moveType": eval.move_type
            }))
            .collect();

        let result = serde_json::json!({
            "move": ml_response.r#move.map(|m| m as u32),
            "evaluation": ml_response.evaluation,
            "thinking": ml_response.thinking,
            "diagnostics": {
                "validMoves": ml_response.diagnostics.valid_moves,
                "moveEvaluations": move_evaluations,
                "valueNetworkOutput": ml_response.diagnostics.value_network_output,
                "policyNetworkOutputs": ml_response.diagnostics.policy_network_outputs
            }
        });

        Ok(serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))?)
    }

    pub fn evaluate_position(&self, board_state: &JsValue) -> Result<f32, JsValue> {
        let state: GameState = serde_wasm_bindgen::from_value(board_state.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(state.evaluate() as f32)
    }

    pub fn evaluate_position_ml(&self, board_state: &JsValue) -> Result<f32, JsValue> {
        let state: GameState = serde_wasm_bindgen::from_value(board_state.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(self.ml_ai.evaluate_position(&state))
    }

    pub fn get_valid_moves(&self, board_state: &JsValue) -> Result<JsValue, JsValue> {
        let state: GameState = serde_wasm_bindgen::from_value(board_state.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let moves = state.get_valid_moves();
        Ok(serde_wasm_bindgen::to_value(&moves).map_err(|e| JsValue::from_str(&e.to_string()))?)
    }

    pub fn make_move(&self, board_state: &JsValue, column: u8) -> Result<JsValue, JsValue> {
        let mut state: GameState = serde_wasm_bindgen::from_value(board_state.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        match state.make_move(column) {
            Ok(()) => {
                let result = serde_json::json!({
                    "success": true,
                    "new_state": state,
                });
                Ok(serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))?)
            }
            Err(e) => {
                let result = serde_json::json!({
                    "success": false,
                    "error": e,
                });
                Ok(serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))?)
            }
        }
    }

    pub fn is_game_over(&self, board_state: &JsValue) -> Result<bool, JsValue> {
        let state: GameState = serde_wasm_bindgen::from_value(board_state.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(state.is_game_over())
    }

    pub fn get_winner(&self, board_state: &JsValue) -> Result<JsValue, JsValue> {
        let state: GameState = serde_wasm_bindgen::from_value(board_state.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let winner = state.get_winner();
        Ok(serde_wasm_bindgen::to_value(&winner).map_err(|e| JsValue::from_str(&e.to_string()))?)
    }

    pub fn create_new_game(&self) -> Result<JsValue, JsValue> {
        let state = GameState::new();
        Ok(serde_wasm_bindgen::to_value(&state).map_err(|e| JsValue::from_str(&e.to_string()))?)
    }

    pub fn create_game_with_params(&self, params: &JsValue) -> Result<JsValue, JsValue> {
        let genetic_params: GeneticParams = serde_wasm_bindgen::from_value(params.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let state = GameState::with_genetic_params(genetic_params);
        Ok(serde_wasm_bindgen::to_value(&state).map_err(|e| JsValue::from_str(&e.to_string()))?)
    }

    pub fn clear_transposition_table(&mut self) {
        self.ai.clear_transposition_table();
    }

    pub fn get_transposition_table_size(&self) -> usize {
        self.ai.get_transposition_table_size()
    }

    pub fn load_ml_weights(
        &mut self,
        value_weights: &JsValue,
        policy_weights: &JsValue,
    ) -> Result<(), JsValue> {
        let value_weights: Vec<f32> = serde_wasm_bindgen::from_value(value_weights.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let policy_weights: Vec<f32> = serde_wasm_bindgen::from_value(policy_weights.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        self.ml_ai.load_weights(&value_weights, &policy_weights);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_ai_creation() {
        let ai = ConnectFourAI::new();
        assert_eq!(ai.get_transposition_table_size(), 0);
    }

    #[wasm_bindgen_test]
    fn test_new_game_creation() {
        let ai = ConnectFourAI::new();
        let game_state = ai.create_new_game().unwrap();
        assert!(!game_state.is_undefined());
    }

    #[wasm_bindgen_test]
    fn test_valid_moves_empty_board() {
        let ai = ConnectFourAI::new();
        let game_state = ai.create_new_game().unwrap();
        let valid_moves = ai.get_valid_moves(&game_state).unwrap();
        assert!(!valid_moves.is_undefined());
    }

    #[wasm_bindgen_test]
    fn test_make_move() {
        let ai = ConnectFourAI::new();
        let game_state = ai.create_new_game().unwrap();
        let result = ai.make_move(&game_state, 3).unwrap();
        assert!(!result.is_undefined());
    }

    #[wasm_bindgen_test]
    fn test_game_over_detection() {
        let ai = ConnectFourAI::new();
        let game_state = ai.create_new_game().unwrap();
        let is_over = ai.is_game_over(&game_state).unwrap();
        assert!(!is_over); // Empty board should not be game over
    }

    #[wasm_bindgen_test]
    fn test_winner_detection() {
        let ai = ConnectFourAI::new();
        let game_state = ai.create_new_game().unwrap();
        let winner = ai.get_winner(&game_state).unwrap();
        assert!(!winner.is_undefined());
    }

    #[wasm_bindgen_test]
    fn test_position_evaluation() {
        let ai = ConnectFourAI::new();
        let game_state = ai.create_new_game().unwrap();
        let evaluation = ai.evaluate_position(&game_state).unwrap();
        assert!(evaluation.is_finite());
    }

    #[wasm_bindgen_test]
    fn test_ml_position_evaluation() {
        let ai = ConnectFourAI::new();
        let game_state = ai.create_new_game().unwrap();
        let evaluation = ai.evaluate_position_ml(&game_state).unwrap();
        assert!(evaluation.is_finite());
    }

    #[wasm_bindgen_test]
    fn test_heuristic_move() {
        let ai = ConnectFourAI::new();
        let game_state = ai.create_new_game().unwrap();
        let result = ai.get_heuristic_move(&game_state).unwrap();
        assert!(!result.is_undefined());
    }

    #[wasm_bindgen_test]
    fn test_ml_move() {
        let ai = ConnectFourAI::new();
        let game_state = ai.create_new_game().unwrap();
        let result = ai.get_ml_move(&game_state).unwrap();
        assert!(!result.is_undefined());
    }
}
