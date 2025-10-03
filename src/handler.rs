use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::{Value, json};

use crate::{AppState, model::GameModel, schema::GameSchema};

pub async fn hello_world() -> impl IntoResponse {
    let json_response = json!({
        "status": "ok",
        "message": "Hello, World!"
    });
    Json(json_response)
}

pub async fn create_game_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<GameSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let id = uuid::Uuid::new_v4();
    let game = sqlx::query_as!(
        GameModel,
        r#"
        INSERT INTO games (id, name, creator, plays) VALUES ($1, $2, $3, $4) RETURNING *
        "#,
        &id,
        &payload.name,
        &payload.creator,
        &payload.plays
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|err| err.to_string());

    if let Err(err) = game {
        if err.to_string().contains("duplicate key value") {
            let error_response = json!({
                "status": "error",
                "message": "Game already exists"
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": "Internal server error"
            })),
        ));
    }

    let game_response = json!({
        "status": "success",
        "data": json!({
            "game": game
        })
    });

    Ok(Json(game_response))
}

pub async fn game_list_handler(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let games = sqlx::query_as!(
        GameModel,
        r#"
        SELECT * FROM games ORDER BY name
        "#,
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|err| {
        let error_response = json!({
            "status": "error",
            "message": format!("Database error: {}", err)
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let games_response = json!({
        "status": "ok",
        "count": games.len(),
        "notes": games
    });

    Ok(Json(games_response))
}
