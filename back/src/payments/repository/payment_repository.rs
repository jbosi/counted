use actix_web::web;
use actix_web::web::Query;
use diesel::{QueryDsl, RunQueryDsl};
use diesel::prelude::*;

use crate::{DbPool, schema};
use crate::payments::domain::payment_model::Payment;
use crate::payments::domain::payment_query_params::PaymentQueryParams;
use crate::schema::expenses::dsl::expenses;

pub async fn get_payments(pool: web::Data<DbPool>, params: Query<PaymentQueryParams>) -> Vec<Payment> {
    use schema::payments::dsl::*;

    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let mut query = expenses.into_boxed();

    if let Some(user_id_unwrapped) = params.user_id {
        query = query.filter(user_id.eq(user_id_unwrapped))
    }

    if let Some(expense_id_unwrapped) = params.expense_id {
        query = query.filter(expense_id.eq(expense_id_unwrapped))
    }

    let payments_list: Vec<Payment> = query
        .load::<Payment>(&mut conn)
        .expect("Error while trying to get Payment");

    return payments_list;
}