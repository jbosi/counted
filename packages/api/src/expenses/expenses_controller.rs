use chrono::Local;
use dioxus::{fullstack::Json, prelude::*};
use uuid::Uuid;

#[cfg(feature = "server")]
use crate::expenses::expenses_repository;
#[cfg(feature = "server")]
use crate::payments::payments_repository;
use shared::{CreatableExpense, EditableExpense, Expense, NewPayment, UserAmount};

#[post("/api/expenses")]
pub async fn add_expense(Json(expense): Json<CreatableExpense>) -> Result<Expense, ServerFnError> {
    let created_expense_id = expenses_repository::add_expense(expense.clone()).await?;

    let payers = Some(expense.clone().payers);
    let debtors = Some(expense.clone().debtors);

    let creatable_payments: Vec<NewPayment> =
        forge_creatable_payments_from_expense(payers, debtors, created_expense_id);

    payments_repository::add_payments(creatable_payments).await?;

    let created_expense = Expense {
        id: created_expense_id,
        name: expense.name,
        amount: expense.amount,
        expense_type: expense.expense_type,
        project_id: expense.project_id,
        author_id: expense.author_id,
        description: expense.description,
        created_at: Local::now().naive_local(),
        date: expense.date,
    };

    Ok(created_expense)
}

#[put("/api/expenses")]
pub async fn edit_expense(Json(expense): Json<EditableExpense>) -> Result<Expense, ServerFnError> {
    expenses_repository::edit_expense(expense.clone()).await?;

    payments_repository::delete_payments_by_expense_id(expense.id).await?;

    let payers = Some(expense.payers.clone());
    let debtors = Some(expense.debtors.clone());

    let creatable_payments: Vec<NewPayment> =
        forge_creatable_payments_from_expense(payers, debtors, expense.id);

    payments_repository::add_payments(creatable_payments).await?;

    let updated_expense = Expense {
        id: expense.id,
        name: expense.name,
        amount: expense.amount,
        expense_type: expense.expense_type,
        project_id: expense.project_id,
        author_id: expense.author_id,
        description: expense.description,
        created_at: Local::now().naive_local(), // TODO
        date: expense.date,
    };

    Ok(updated_expense)
}

#[get("/api/projects/{project_id}/expenses")]
pub async fn get_expenses_by_project_id(project_id: Uuid) -> Result<Vec<Expense>, ServerFnError> {
    let expenses = expenses_repository::get_expenses_by_project_id(project_id).await?;

    Ok(expenses)
}

#[get("/api/expenses/{expense_id}")]
pub async fn get_expense_by_id(expense_id: i32) -> Result<Expense, ServerFnError> {
    let expense = expenses_repository::get_expense_by_id(expense_id).await?;

    Ok(expense)
}

#[delete("/api/expenses/{expense_id}")]
pub async fn delete_expense(expense_id: i32) -> Result<(), ServerFnError> {
    payments_repository::delete_payments_by_expense_id(expense_id).await?;

    expenses_repository::delete_expense(expense_id).await?;

    Ok(())
}

fn forge_creatable_payments_from_expense(
    payers: Option<Vec<UserAmount>>,
    debtors: Option<Vec<UserAmount>>,
    created_expense_id: i32,
) -> Vec<NewPayment> {
    let mut debtors_result: Vec<UserAmount> = vec![];
    if let Some(debtors_unwrapped) = debtors {
        debtors_result = debtors_unwrapped;
    }

    let mut payers_result: Vec<UserAmount> = vec![];
    if let Some(payers_unwrapped) = payers {
        payers_result = payers_unwrapped;
    }

    let creatable_debtors: Vec<NewPayment> = debtors_result
        .into_iter()
        .filter(|d| d.amount != 0.0)
        .map(|d| NewPayment {
            amount: d.amount,
            expense_id: created_expense_id,
            user_id: d.user_id,
            is_debt: true,
        })
        .collect();

    let creatable_payers: Vec<NewPayment> = payers_result
        .into_iter()
        .filter(|d| d.amount != 0.0)
        .map(|p| NewPayment {
            amount: p.amount,
            expense_id: created_expense_id,
            user_id: p.user_id,
            is_debt: false,
        })
        .collect();

    [creatable_debtors, creatable_payers].concat()
}
