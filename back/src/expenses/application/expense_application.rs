use actix_web::web;
use actix_web::web::Query;

use crate::DbPool;
use crate::expenses::domain::expense_model::Expense;
use crate::expenses::repository::expense_repository::{get_expense, get_expenses};
use crate::query_strings::expense_query_string::ExpenseQueryParams;
use crate::payments::application::payment_application::get_payments_app;
use crate::payments::domain::payment_model::{ExpenseDto, Payment};
use crate::payments::domain::payment_query_params::PaymentQueryParams;

pub async fn get_expenses_app(pool: web::Data<DbPool>, params: Query<ExpenseQueryParams>) -> Vec<ExpenseDto> {
    let expenses: Vec<Expense> = get_expenses(pool.clone(), params.clone()).await;
    let payments_params: Query<PaymentQueryParams> = Query(
        PaymentQueryParams {
            user_id: params.user_id,
            expense_id: None,
        }
    );
    let payments: Vec<Payment> = get_payments_app(pool.clone(), payments_params).await;

    return expenses
        .iter()
        .map(|expense| to_expense_dto(expense, &payments))
        .filter(|expense| !expense.payments.is_empty()) // Filter expenses where there is no payment bound
        .collect();
}

pub async fn get_expense_app(pool: web::Data<DbPool>, expense_id: i32) -> ExpenseDto {
    let expense: Expense = get_expense(pool.clone(), expense_id).await;
    let payments_params: Query<PaymentQueryParams> = Query(
        PaymentQueryParams {
            user_id: None,
            expense_id: Some(expense_id)
        }
    );
    let payments: Vec<Payment> = get_payments_app(pool.clone(), payments_params).await;

    return to_expense_dto(&expense, &payments);
}

fn to_expense_dto(expense: &Expense, payments: &Vec<Payment>) -> ExpenseDto {
    return ExpenseDto {
        author_id: expense.author_id,
        id: expense.id,
        amount: expense.amount,
        date: expense.date,
        description: expense.description.clone(),
        expense_type: expense.expense_type.clone(),
        name: expense.name.clone(),
        project_id: expense.project_id,
        payments: payments
            .clone()
            .into_iter()
            .filter(|&payment| payment.expense_id == expense.id)
            .collect::<Vec<Payment>>()
    }
}