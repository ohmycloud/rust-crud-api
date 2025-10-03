use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    AppState,
    handler::{create_game_handler, delete_game_handler, game_list_handler, get_game_handler},
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/api/games",
            post(create_game_handler).get(game_list_handler),
        )
        .route(
            "/api/games/{id}",
            get(get_game_handler).delete(delete_game_handler),
        )
        .with_state(app_state)
}
