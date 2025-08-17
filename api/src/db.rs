#[cfg(feature = "sqlx")]
use sqlx::{PgPool, Pool, Postgres};
#[cfg(feature = "sqlx")]
use tokio::sync::OnceCell;

#[cfg(feature = "server")]
static DB: OnceCell<Pool<Postgres>> = OnceCell::const_new();

#[cfg(feature = "server")]
async fn get_pool() -> Pool<Postgres> {
    dotenvy::dotenv().expect("Le fichier .env n'a pas pu être chargé");
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL doit être défini");

    let pool = PgPool::connect(&db_url).await.expect("Unable to connect to the database");

    pool
}

#[cfg(feature = "server")]
pub async fn get_db() -> Pool<Postgres> {
    DB.get_or_init(get_pool).await.clone()
}
