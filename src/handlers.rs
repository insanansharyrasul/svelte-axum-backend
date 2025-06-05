use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::Json as JsonResponse,
};
use sqlx::SqlitePool;

use crate::models::{ApiResponse, CreateUser, User};

pub type SharedState = SqlitePool;

pub async fn api_hello() -> JsonResponse<ApiResponse> {
    JsonResponse(ApiResponse {
        message: "Hello from Axum backend with SQLite!".to_string(),
    })
}

pub async fn get_users(
    State(pool): State<SharedState>,
) -> Result<JsonResponse<Vec<User>>, StatusCode> {
    let users = sqlx::query_as::<_, User>("SELECT id, name FROM users")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(JsonResponse(users))
}

pub async fn create_user(
    State(pool): State<SharedState>,
    Json(payload): Json<CreateUser>,
) -> Result<JsonResponse<User>, StatusCode> {
    let result = sqlx::query("INSERT INTO users (name) VALUES (?)")
        .bind(&payload.name)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_id = result.last_insert_rowid();

    let user = User {
        id: user_id as u32,
        name: payload.name,
    };

    Ok(JsonResponse(user))
}
