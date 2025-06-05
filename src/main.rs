use axum::http::{HeaderValue, Method};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

mod database;
mod handlers;
mod models;
mod routes;

use database::init_database;
use routes::create_routes;

#[tokio::main]
async fn main() {
    let pool = init_database().await;

    let app = create_routes().with_state(pool).layer(
        ServiceBuilder::new().layer(
            CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([axum::http::header::CONTENT_TYPE]),
        ),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    println!("API available at http://localhost:3000/api/");
    axum::serve(listener, app).await.unwrap();
}
