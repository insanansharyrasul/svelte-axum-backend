use axum::{
    Router,
    routing::{get, post},
};
use tower_http::services::ServeDir;

use crate::handlers::{SharedState, api_hello, create_user, get_users};

pub fn create_routes() -> Router<SharedState> {
    Router::new()
        .route("/api/hello", get(api_hello))
        .route("/api/users", get(get_users))
        .route("/api/users", post(create_user))
        .fallback_service(ServeDir::new("dist"))
}
