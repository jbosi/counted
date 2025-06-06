use axum::{routing::get, Router, Json};
use shared::User;
use std::env;
use dotenvy::dotenv;

async fn get_user() -> Json<User> {
    Json(User {
        id: 1,
        username: "dioxus_user".to_string(),
        email: "user@example.com".to_string(),
    })
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // charge le .env
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let app = Router::new().route("/user", get(get_user));

    println!("Server running on http://localhost:3000");
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}