use actix_web::web;
use actix_web::web::Query;
use diesel::prelude::*;

use crate::DbPool;
use crate::expenses::domain::expense_model::Expense;
use crate::expenses::repository::expense_repository::get_expenses;
use crate::query_strings::expenses_query_string::ExpensesQueryParams;
use crate::payments::application::payment_application::get_payments_app;
use crate::payments::domain::payment_model::{ExpenseDto, Payment};

pub async fn get_expenses_app(pool: web::Data<DbPool>, params: Query<ExpensesQueryParams>) -> Vec<ExpenseDto> {
    let expenses: Vec<Expense> = get_expenses(pool.clone(), params).await;
    let payments: Vec<Payment> = get_payments_app(pool.clone()).await;

    return expenses
        .iter()
        .map(|expense| ExpenseDto {
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
        })
        .collect();
}