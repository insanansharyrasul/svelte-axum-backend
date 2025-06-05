use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize)]
pub struct ApiResponse {
    pub message: String,
}

#[derive(Serialize, Clone, FromRow)]
pub struct User {
    pub id: u32,
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
}
