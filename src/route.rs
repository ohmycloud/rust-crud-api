use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{AppState, handler::create_game_handler};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/game", post(create_game_handler))
        .with_state(app_state)
}
