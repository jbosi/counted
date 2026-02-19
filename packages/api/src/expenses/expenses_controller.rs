use chrono::Local;
use dioxus::{fullstack::Json, prelude::*};
use uuid::Uuid;

#[cfg(feature = "server")]
use crate::expenses::expenses_repository;
#[cfg(feature = "server")]
use crate::payments::payments_repository;
use shared::{CreatableExpense, EditableExpense, Expense, NewPayment, UserAmount};

#[post("/api/v1/expenses")]
pub async fn add_expense(Json(expense): Json<CreatableExpense>) -> Result<Expense, ServerFnError> {
    validate_expense(&expense.name, &expense.payers, &expense.debtors)?;

    let created_expense_id = expenses_repository::add_expense(expense.clone()).await?;

    let creatable_payments: Vec<NewPayment> = forge_creatable_payments_from_expense(
        expense.payers.clone(),
        expense.debtors.clone(),
        created_expense_id,
    );

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

#[put("/api/v1/expenses")]
pub async fn edit_expense(Json(expense): Json<EditableExpense>) -> Result<Expense, ServerFnError> {
    validate_expense(&expense.name, &expense.payers, &expense.debtors)?;

    expenses_repository::edit_expense(expense.clone()).await?;

    payments_repository::delete_payments_by_expense_id(expense.id).await?;

    let creatable_payments: Vec<NewPayment> = forge_creatable_payments_from_expense(
        expense.payers.clone(),
        expense.debtors.clone(),
        expense.id,
    );

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

#[get("/api/v1/projects/{project_id}/expenses")]
pub async fn get_expenses_by_project_id(project_id: Uuid) -> Result<Vec<Expense>, ServerFnError> {
    let expenses = expenses_repository::get_expenses_by_project_id(project_id).await?;

    Ok(expenses)
}

#[get("/api/v1/expenses/{expense_id}")]
pub async fn get_expense_by_id(expense_id: i32) -> Result<Expense, ServerFnError> {
    let expense = expenses_repository::get_expense_by_id(expense_id).await?;

    Ok(expense)
}

#[delete("/api/v1/expenses/{expense_id}")]
pub async fn delete_expense(expense_id: i32) -> Result<(), ServerFnError> {
    payments_repository::delete_payments_by_expense_id(expense_id).await?;

    expenses_repository::delete_expense(expense_id).await?;

    Ok(())
}

fn validate_expense(
    name: &str,
    payers: &[UserAmount],
    debtors: &[UserAmount],
) -> Result<(), ServerFnError> {
    if name.is_empty() {
        return Err(ServerFnError::new("name cannot be empty"));
    }
    if payers.is_empty() {
        return Err(ServerFnError::new("payers cannot be empty"));
    }
    if debtors.is_empty() {
        return Err(ServerFnError::new("debtors cannot be empty"));
    }
    Ok(())
}

fn forge_creatable_payments_from_expense(
    payers: Vec<UserAmount>,
    debtors: Vec<UserAmount>,
    created_expense_id: i32,
) -> Vec<NewPayment> {
    let creatable_debtors: Vec<NewPayment> = debtors
        .into_iter()
        .filter(|d| d.amount != 0.0)
        .map(|d| NewPayment {
            amount: d.amount,
            expense_id: created_expense_id,
            user_id: d.user_id,
            is_debt: true,
        })
        .collect();

    let creatable_payers: Vec<NewPayment> = payers
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
