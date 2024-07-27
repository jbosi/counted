use actix_web::{get, Responder, web};
use actix_web::HttpRequest;
use actix_web::web::Query;

use crate::DbPool;


#[get("balance")]
pub async fn get_balance(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
	let balance: Vec<Balance> = get_balance_app(pool, params).await;

	web::Json(balance)
}