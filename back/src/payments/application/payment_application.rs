use actix_web::web;
use actix_web::web::Query;

use crate::DbPool;
use crate::payments::domain::payment_model::Payment;
use crate::payments::domain::payment_query_params::PaymentQueryParams;
use crate::payments::repository::payment_repository::get_payments;

pub async fn get_payments_app(pool: web::Data<DbPool>, params: Query<PaymentQueryParams>) -> Vec<Payment> {
    let payments: Vec<Payment> = get_payments(pool).await;
    return payments;
}