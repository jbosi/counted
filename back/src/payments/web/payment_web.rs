use actix_web::{get, Responder, web};
use actix_web::HttpRequest;
use actix_web::web::Query;

use crate::DbPool;
use crate::payments::application::payment_application::get_payments_app;
use crate::payments::domain::payment_model::Payment;
use crate::payments::domain::payment_query_params::PaymentQueryParams;

#[utoipa::path(
	responses(
		(status = 200, description = "Payments found"),
	),
	params(
		("project_id" = Uuid, Path, description = "get payments"),
	)
)]
#[get("payments")]
pub async fn get_payments(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
	let params: Query<PaymentQueryParams> = Query::<PaymentQueryParams>::from_query(req.query_string()).unwrap();
	let payment_list: Vec<Payment> = get_payments_app(pool, params).await;

	web::Json(payment_list)
}