use actix_web::HttpRequest;
use actix_web::{get, Responder, web};

use crate::{DbPool, schema};
use crate::payments::application::payment_application::get_payments_app;
use crate::payments::domain::payment_model::Payment;

#[get("payments")]
pub async fn get_payments(pool: web::Data<DbPool>, _req: HttpRequest) -> impl Responder {
	use schema::payments::dsl::*;

	let payment_list: Vec<Payment> = get_payments_app(pool).await;

	web::Json(payment_list)
}