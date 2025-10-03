use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::{
    AppState,
    model::GameModel,
    schema::{GameSchema, UpdateGameSchema},
};

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

pub async fn get_game_handler(
    Path(game_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let query_result = sqlx::query_as!(
        GameModel,
        r#"
        SELECT * FROM games WHERE id = $1
        "#,
        &game_id
    )
    .fetch_one(&state.db_pool)
    .await;

    match query_result {
        Ok(game) => {
            let game_response = json!({
                "status": "success",
                "data": json!({
                    "game": game
                })
            });

            Ok(Json(game_response))
        }
        Err(sqlx::Error::RowNotFound) => {
            let error_response = json!({
                "status": "fail",
                "message": format!("Game with ID {} not found", game_id)
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", e)})),
        )),
    }
}

pub async fn delete_game_handler(
    Path(game_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let query_result = sqlx::query_as!(
        GameModel,
        r#"
        DELETE FROM games WHERE id = $1 RETURNING *
        "#,
        &game_id
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => {
            let error_response = json!({
                "status": "fail",
                "message": format!("Game with ID {} not found", game_id)
            });
            (StatusCode::NOT_FOUND, Json(error_response))
        }
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", err)})),
        ),
    })?;

    let response = json!({
        "status": "success",
        "message": "Game deleted successfully",
        "data": json!({
            "deleted_game": query_result
        })
    });

    Ok(Json(response))
}

pub async fn update_game_handler(
    Path(game_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateGameSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let query_result = sqlx::query_as!(
        GameModel,
        r#"
        SELECT * FROM games WHERE id = $1
        "#,
        &game_id
    )
    .fetch_one(&state.db_pool)
    .await;

    let game = match query_result {
        Ok(game) => game,
        Err(sqlx::Error::RowNotFound) => {
            let error_response = json!({
                "status": "fail",
                "message": format!("Game with ID {} not found", game_id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(err) => {
            let error_response = json!({
                "status": "error",
                "message": format!("{:?}", err)
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }
    };

    let new_name = payload.name.as_ref().unwrap_or(&game.name);
    let new_creator = payload.creator.as_ref().unwrap_or(&game.creator);
    let new_plays = payload.plays.unwrap_or(game.plays);

    let updated_game = sqlx::query_as!(
        GameModel,
        r#"
        UPDATE games SET name = $1, creator = $2, plays = $3 WHERE id = $4 RETURNING *
        "#,
        &new_name,
        &new_creator,
        &new_plays,
        &game_id
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error", "message": format!("{:?}", err)})),
        )
    })?;

    let response = json!({
        "status": "success",
        "message": "Game updated successfully",
        "data": json!({
            "player": updated_game
        })
    });

    Ok(Json(response))
}
