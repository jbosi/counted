use actix_web::web;
use actix_web::web::Query;
use diesel::{QueryDsl, RunQueryDsl};
use diesel::prelude::*;

use crate::{DbPool, schema};
use crate::payments::domain::payment_model::{NewPayment, Payment};
use crate::payments::domain::payment_query_params::PaymentQueryParams;

pub async fn get_payments(pool: web::Data<DbPool>, params: Query<PaymentQueryParams>) -> Vec<Payment> {
    use schema::payments::dsl::*;

    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let mut query = payments.into_boxed();

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

pub async fn create_payments(pool: web::Data<DbPool>, new_payments: Vec<NewPayment>) -> () {
    use schema::payments::dsl::*;
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    for new_payment in new_payments {
    	diesel::update(payments.find(new_payment.expense_id))
    		.set(new_payment)
    		.execute(&mut conn)
    		.expect("Error updating payments"); // TODO set payment id in error
    }
}

// risque de perte de donnée quand même donc il faut un mécanisme de undo si il y a un souci ?
pub async fn delete_payments_by_expense_id(pool: web::Data<DbPool>, expense_id: i32) -> () {
    use schema::payments::dsl::*;
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    diesel::delete(payments.filter(expense_id.eq(expense_id)))
        .execute(&mut conn)
        .expect("Error removing payments");
}