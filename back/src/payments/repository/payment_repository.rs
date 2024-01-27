use actix_web::web;
use actix_web::web::Query;
use diesel::{QueryDsl, RunQueryDsl};

use crate::{DbPool, schema};
use crate::payments::domain::payment_model::Payment;
use crate::payments::domain::payment_query_params::PaymentQueryParams;

pub async fn get_payments(pool: web::Data<DbPool>, params: Query<PaymentQueryParams>) -> Vec<Payment> {
    use schema::payments::dsl::*;

    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let payments_list: Vec<Payment>;

    match params.user_id {
        None => {
            payments_list = payments
                .load::<Payment>(&mut conn)
                .expect("Error while trying to get Payment");
        }
        Some(user_id) => {
            payments_list = payments
                .filter(payments::user_id.eq(user_id))
                .load::<Payment>(&mut conn)
                .expect("Error while trying to get Payment");
        }
    }


    return payments_list;
}