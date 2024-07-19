use actix_web::web;
use actix_web::web::Query;

use crate::DbPool;
use crate::payments::domain::payment_model::{NewPayment, Payment};
use crate::payments::domain::payment_query_params::PaymentQueryParams;
use crate::payments::repository::payment_repository::get_payments;
use crate::users::domain::user_model::UserAmount;

pub async fn get_payments_app(pool: web::Data<DbPool>, params: Query<PaymentQueryParams>) -> Vec<Payment> {
    let payments: Vec<Payment> = get_payments(pool, params).await;
    return payments;
}

pub fn forge_creatable_payments_from_expense(payers: Option<Vec<UserAmount>>, debtors: Option<Vec<UserAmount>>, created_expense_id: i32) -> Vec<NewPayment> {
    let mut debtors_result: Vec<UserAmount> = vec![];
    if let Some(debtors_unwrapped) = debtors {
        debtors_result = debtors_unwrapped;
    }

    let mut payers_result: Vec<UserAmount> = vec![];
    if let Some(payers_unwrapped) = payers {
        payers_result = payers_unwrapped;
    }

    let creatable_debtors: Vec<NewPayment> = debtors_result.into_iter().map(|d| NewPayment {
        amount: d.amount,
        expense_id: created_expense_id,
        user_id: d.user_id,
        is_debt: true
    }).collect();

    let creatable_payers: Vec<NewPayment> = payers_result.into_iter().map(|p| NewPayment {
        amount: p.amount,
        expense_id: created_expense_id,
        user_id: p.user_id,
        is_debt: false
    }).collect();

    return [creatable_debtors, creatable_payers].concat();
}