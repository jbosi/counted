use axum::{routing::get, Router, Json};
use shared::User;

async fn get_user() -> Json<User> {
    Json(User {
        id: 1,
        username: "dioxus_user".to_string(),
        email: "user@example.com".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/user", get(get_user));

    println!("Server running on http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}