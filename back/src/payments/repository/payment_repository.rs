use actix_web::web;
use diesel::RunQueryDsl;

use crate::{DbPool, schema};
use crate::payments::domain::payment_model::Payment;

pub async fn get_payments(pool: web::Data<DbPool>) -> Vec<Payment> {
    use schema::payments::dsl::*;

    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let payments_list = payments
        .load::<Payment>(&mut conn)
        .expect("Error while trying to get Payment");

    return payments_list;
}