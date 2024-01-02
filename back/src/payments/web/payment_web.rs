use actix_web::{get, Responder, web};
use actix_web::HttpRequest;

use crate::DbPool;
use crate::payments::application::payment_application::get_payments_app;
use crate::payments::domain::payment_model::Payment;

#[utoipa::path(
	responses(
		(status = 200, description = "Payments found"),
	),
	params(
		("project_id" = Uuid, Path, description = "get payments"),
	)
)]
#[get("payments")]
pub async fn get_payments(pool: web::Data<DbPool>, _req: HttpRequest) -> impl Responder {
	let payment_list: Vec<Payment> = get_payments_app(pool).await;

	web::Json(payment_list)
}