use connect_four_ai_core::{GameState, MoveEvaluation, Player, AI};
use console_error_panic_hook;
use js_sys::Date;
use serde::{Deserialize, Serialize};
use worker::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AI_SEARCH_DEPTH: u8 = 4;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    timestamp: String,
    version: String,
}

#[derive(Deserialize)]
struct ConnectFourGameState {
    board: Vec<Vec<String>>,
    current_player: String,
}

#[derive(Serialize)]
struct AIResponse {
    r#move: Option<u8>,
    evaluation: i32,
    thinking: String,
    timings: Timings,
    diagnostics: Diagnostics,
}

#[derive(Serialize)]
struct Timings {
    ai_move_calculation: u32,
    total_handler_time: u32,
}

#[derive(Serialize)]
struct Diagnostics {
    search_depth: u8,
    valid_moves: Vec<u8>,
    move_evaluations: Vec<MoveEvaluationWasm>,
    transposition_hits: usize,
    nodes_evaluated: u64,
}

#[derive(Serialize)]
struct MoveEvaluationWasm {
    column: u8,
    score: f32,
    move_type: String,
}

impl From<&MoveEvaluation> for MoveEvaluationWasm {
    fn from(eval: &MoveEvaluation) -> Self {
        MoveEvaluationWasm {
            column: eval.column,
            score: eval.score,
            move_type: eval.move_type.clone(),
        }
    }
}

fn cors_headers_with_origin(origin: &Option<String>, env: &Env) -> Result<Headers> {
    let mut headers = Headers::new();
    headers.set(
        "Access-Control-Allow-Origin",
        &origin.clone().unwrap_or_else(|| "*".to_string()),
    )?;
    headers.set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")?;
    headers.set("Access-Control-Allow-Headers", "Content-Type")?;
    headers.set("Access-Control-Max-Age", "86400")?;
    Ok(headers)
}

fn is_development(env: &Env) -> bool {
    match env.var("ENVIRONMENT") {
        Ok(val) => val.to_string() == "development",
        Err(_) => false,
    }
}

async fn handle_ai_move(mut req: Request, start_time: f64, env: &Env) -> Result<Response> {
    let is_dev = is_development(env);

    let game_state_request: ConnectFourGameState = req.json().await?;

    console_log!(
        "[AI] Player: {}, Board size: {}x{}",
        game_state_request.current_player,
        game_state_request.board.len(),
        if !game_state_request.board.is_empty() {
            game_state_request.board[0].len()
        } else {
            0
        }
    );

    let ai_start = js_sys::Date::now();
    let game_state = convert_request_to_game_state(&game_state_request);

    if is_dev {
        console_log!(
            "[AI] Dev mode: Game state converted, current player: {:?}",
            game_state.current_player
        );
    }

    let mut ai = AI::new();
    let (ai_move, move_evaluations) = ai.get_best_move(&game_state, AI_SEARCH_DEPTH);
    let evaluation = game_state.evaluate();

    let move_evaluations_wasm: Vec<MoveEvaluationWasm> =
        move_evaluations.iter().map(|eval| eval.into()).collect();

    let ai_end = js_sys::Date::now();
    let end_time = js_sys::Date::now();

    let response = AIResponse {
        r#move: ai_move,
        evaluation,
        thinking: format!(
            "AI (depth {}) chose move {:?} with score {:.1}. Evaluated {} nodes, {} cache hits.",
            AI_SEARCH_DEPTH,
            ai_move,
            move_evaluations.first().map(|m| m.score).unwrap_or(0.0),
            ai.nodes_evaluated,
            ai.transposition_hits
        ),
        timings: Timings {
            ai_move_calculation: ((ai_end - ai_start) as u32).max(1),
            total_handler_time: ((end_time - start_time) as u32).max(1),
        },
        diagnostics: Diagnostics {
            search_depth: AI_SEARCH_DEPTH,
            valid_moves: game_state.get_valid_moves(),
            move_evaluations: move_evaluations_wasm,
            transposition_hits: ai.transposition_hits as usize,
            nodes_evaluated: ai.nodes_evaluated as u64,
        },
    };

    console_log!(
        "[AI] Response: move={:?}, eval={}, time={}ms, nodes={}, cache_hits={}",
        ai_move,
        evaluation,
        response.timings.ai_move_calculation,
        ai.nodes_evaluated,
        ai.transposition_hits
    );

    if is_dev && !move_evaluations.is_empty() {
        console_log!("[AI] Dev mode: Top 3 move evaluations:");
        for (i, eval) in move_evaluations.iter().take(3).enumerate() {
            console_log!(
                "  {}: column={}, score={:.1}, type={}",
                i + 1,
                                    eval.column,
                eval.score,
                eval.move_type
            );
        }
    }

    Response::from_json(&response)
}

fn convert_request_to_game_state(request: &ConnectFourGameState) -> GameState {
    let mut game_state = GameState::new();

    game_state.current_player = if request.current_player == "Player1" {
        Player::Player1
    } else {
        Player::Player2
    };

    // Convert the board from the request format to our internal format
    for (col, column) in request.board.iter().enumerate() {
        for (row, cell) in column.iter().enumerate() {
            if col < 7 && row < 6 {
                game_state.board[col][row] = match cell.as_str() {
                    "Player1" => connect_four_ai_core::Cell::Player1,
                    "Player2" => connect_four_ai_core::Cell::Player2,
                    _ => connect_four_ai_core::Cell::Empty,
                };
            }
        }
    }

    game_state
}

fn handle_health() -> Result<Response> {
    let response = HealthResponse {
        status: "healthy".to_string(),
        timestamp: Date::now().to_string(),
        version: VERSION.to_string(),
    };

    Response::from_json(&response)
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let origin = req.headers().get("Origin").ok().flatten();
    let cors_headers = cors_headers_with_origin(&origin, &env)?;

    if req.method() == Method::Options {
        return Response::empty()
            .map(|resp| resp.with_headers(cors_headers))
            .map_err(|e| e.into());
    }

    let start_time = js_sys::Date::now();
    let url = req.url()?;
    let path = url.path();

    let response = match path {
        "/health" => handle_health()?,
        "/ai/move" => handle_ai_move(req, start_time, &env).await?,
        _ => Response::error("Not Found", 404)?,
    };

    Ok(response.with_headers(cors_headers))
}
