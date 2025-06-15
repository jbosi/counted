//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;



// pub mod entities;

// use axum::{
//     extract::{Path, State},
//     http::StatusCode,
//     response::IntoResponse,
//     routing::{get, post},
//     Json, Router,
// };
// use rust_decimal::Decimal;
// use shared::{
//     CreateExpensePayload, CreateUserPayload, Expense, ExpenseSummary, FullGroupDetails, Group, User,
//     UserBalance,
// };
// use sqlx::{PgPool};
// use std::net::SocketAddr;
// use tower_http::services::ServeDir;
// use tracing::{info, Level};
// use tracing_subscriber::FmtSubscriber;
// use uuid::Uuid;

// use crate::entities::entities::DbUser;

// // On utilise les structs de 'shared' mais on a besoin de `FromRow` ici pour les requêtes DB.
// // On peut dériver FromRow sur des "newtypes" ou des copies locales si nécessaire,
// // mais pour la simplicité, on utilise des tuples pour les requêtes complexes.

// type AppState = PgPool;

// --- GESTIONNAIRES D'API (API Handlers) ---

// async fn get_group_details(
//     State(pool): State<AppState>,
//     Path(share_token): Path<Uuid>,
// ) -> Result<Json<FullGroupDetails>, AppError> {
//     let group: Group = sqlx::query_as("SELECT * FROM groups WHERE share_token = $1")
//         .bind(share_token)
//         .fetch_one(&pool)
//         .await?;

//     let users: Vec<User> =
//         sqlx::query_as("SELECT id, group_id, name FROM users WHERE group_id = $1 ORDER BY name")
//             .bind(group.id)
//             .fetch_all(&pool)
//             .await?;

//     let expenses: Vec<ExpenseSummary> = sqlx::query_as(
//         r#"
//         SELECT e.id, e.description, e.amount, e.paid_by_user_id, u.name as paid_by_user_name
//         FROM expenses e JOIN users u ON e.paid_by_user_id = u.id
//         WHERE e.group_id = $1 ORDER BY e.date DESC
//         "#,
//     )
//     .bind(group.id)
//     .fetch_all(&pool)
//     .await?;

//     let mut balances: Vec<UserBalance> = Vec::new();
//     for user in &users {
//         let total_paid: Option<Decimal> =
//             sqlx::query_scalar("SELECT SUM(amount) FROM expenses WHERE paid_by_user_id = $1")
//                 .bind(user.id)
//                 .fetch_one(&pool)
//                 .await?;
//         let total_share: Option<Decimal> =
//             sqlx::query_scalar("SELECT SUM(share) FROM expense_participants WHERE user_id = $1")
//                 .bind(user.id)
//                 .fetch_one(&pool)
//                 .await?;
//         let paid = total_paid.unwrap_or_default();
//         let share = total_share.unwrap_or_default();
//         balances.push(UserBalance {
//             user_id: user.id,
//             user_name: user.name.clone(),
//             total_paid: paid,
//             total_share: share,
//             balance: paid - share,
//         });
//     }

//     Ok(Json(FullGroupDetails {
//         group,
//         users,
//         expenses,
//         balances,
//     }))
// }

// async fn add_user_to_group(
//     State(pool): State<AppState>,
//     Path(group_id): Path<i32>,
//     Json(payload): Json<CreateUserPayload>,
// ) -> Result<Json<User>, AppError> {
//     let user: DbUser =
//         sqlx::query_as("INSERT INTO users (group_id, name) VALUES ($1, $2) RETURNING id, group_id, name")
//             .bind(group_id)
//             .bind(payload.name)
//             .fetch_one(&pool)
//             .await?;
//     Ok(Json(User::from(user)))
// }

// async fn add_expense(
//     State(pool): State<AppState>,
//     Path(group_id): Path<i32>,
//     Json(payload): Json<CreateExpensePayload>,
// ) -> Result<Json<Expense>, AppError> {
//     if payload.participant_ids.is_empty() {
//         return Err(AppError::BadRequest(
//             "Une dépense doit avoir au moins un participant.".to_string(),
//         ));
//     }

//     let participant_count = Decimal::from(payload.participant_ids.len());
//     let share_per_person = payload.amount / participant_count;

//     let mut tx = pool.begin().await?;
//     let expense: Expense = sqlx::query_as(
//         "INSERT INTO expenses (group_id, description, amount, paid_by_user_id) VALUES ($1, $2, $3, $4) RETURNING *",
//     )
//     .bind(group_id)
//     .bind(&payload.description)
//     .bind(payload.amount)
//     .bind(payload.paid_by_user_id)
//     .fetch_one(&mut *tx)
//     .await?;

//     for user_id in &payload.participant_ids {
//         sqlx::query("INSERT INTO expense_participants (expense_id, user_id, share) VALUES ($1, $2, $3)")
//             .bind(expense.id)
//             .bind(user_id)
//             .bind(share_per_person)
//             .execute(&mut *tx)
//             .await?;
//     }
//     tx.commit().await?;

//     Ok(Json(expense))
// }

// --- MAIN ---
// #[tokio::main]
// async fn main() {
//     let subscriber = FmtSubscriber::builder()
//         .with_max_level(Level::INFO)
//         .finish();
//     tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

//     dotenvy::dotenv().expect("Le fichier .env n'a pas pu être chargé");
//     let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL doit être défini");

//     let pool = PgPool::connect(&db_url)
//         .await
//         .expect("Impossible de se connecter à la base de données");

//     let api_router = Router::new()
//         // .route("/group/:share_token", get(get_group_details))
//         // .route("/group/:group_id/expenses", post(add_expense))
//         .route("/group/:group_id/users", post(add_user_to_group));

//     // Le backend sert l'API sous `/api` et les fichiers statiques du frontend pour toutes les autres routes.
//     let app = Router::new()
//         .nest("/api", api_router.with_state(pool))
//         .fallback_service(ServeDir::new("../frontend/dist"));

//     let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
//     info!("Le serveur écoute sur http://{}", addr);

//     let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
//     axum::serve(listener, app.into_make_service())
//         .await
//         .unwrap();
// }

// // --- GESTION DES ERREURS ---
// #[derive(Debug, thiserror::Error)]
// enum AppError {
//     #[error(transparent)]
//     SqlxError(#[from] sqlx::Error),
//     #[error("Erreur de requête : {0}")]
//     BadRequest(String),
// }

// impl IntoResponse for AppError {
//     fn into_response(self) -> axum::response::Response {
//         let (status, error_message) = match self {
//             AppError::SqlxError(sqlx::Error::RowNotFound) => {
//                 (StatusCode::NOT_FOUND, "La ressource n'a pas été trouvée.".to_string())
//             }
//             AppError::SqlxError(e) => {
//                 eprintln!("Erreur SQL: {:?}", e);
//                 (StatusCode::INTERNAL_SERVER_ERROR, "Une erreur interne est survenue.".to_string())
//             }
//             AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
//         };

//         (status, Json(serde_json::json!({ "error": error_message }))).into_response()
//     }
// }

/// Echo the user input on the server.
#[server(Echo)]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
