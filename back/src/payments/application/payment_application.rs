use actix_web::web;

use crate::DbPool;
use crate::payments::domain::payment_model::Payment;
use crate::payments::repository::payment_repository::get_payments;

pub async fn get_payments_app(pool: web::Data<DbPool>) -> Vec<Payment> {
    let payments: Vec<Payment> = get_payments(pool).await;
    return payments;
}