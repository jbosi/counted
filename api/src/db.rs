#[cfg(feature = "sqlx")]
use sqlx::{PgPool, Pool, Postgres};

#[cfg(feature = "server")]
pub async fn get_db() -> Pool<Postgres> {
    dotenvy::dotenv().expect("Le fichier .env n'a pas pu être chargé");
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL doit être défini");

    let pool = PgPool::connect(&db_url)
        .await
        .expect("Unable to connect to the database");

    pool
}