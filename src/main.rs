mod docs;
mod error;
mod handlers;
mod models;
mod services;

use crate::services::scraper::ScraperService;
use axum::{Router, routing::get};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().pretty().init();
    let scraper_service = Arc::new(ScraperService::new());

    let app = Router::new()
        .route("/api-docs/openapi.json", get(docs::openapi_json))
        .route("/scalar", get(docs::scalar_docs))
        .route("/titles/search", get(handlers::title_search_handler))
        .route("/titles/{id}", get(handlers::title_details_handler))
        .route(
            "/titles/{id}/episodes",
            get(handlers::title_episodes_handler),
        )
        .route("/titles/{id}/cast", get(handlers::title_cast_handler))
        .route("/titles/{id}/photos", get(handlers::title_photos_handler))
        .route("/titles/{id}/reviews", get(handlers::title_reviews_handler))
        .route("/people/search", get(handlers::people_search_handler))
        .route("/articles/search", get(handlers::article_search_handler))
        .with_state(scraper_service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on http://localhost:3000/scalar");
    axum::serve(listener, app).await.unwrap();
}
