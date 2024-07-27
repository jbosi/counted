use actix_web::web;
use actix_web::web::Query;
use diesel::prelude::*;
use diesel::prelude::*;
use diesel::QueryDsl;
use diesel::r2d2::ConnectionManager;
use diesel::RunQueryDsl;
use r2d2::PooledConnection;

use crate::{DbPool, schema};
use crate::diesel::ExpressionMethods;
use crate::expenses::domain::expense_model::{Expense, PatchableExpense};
use crate::query_strings::expense_query_string::ExpenseQueryParams;
use crate::schema::expenses::{description, expense_type, name};
use crate::schema::expenses::dsl::expenses;

pub async fn get_expenses(pool: web::Data<DbPool>, params: Query<ExpenseQueryParams>) -> Vec<Expense> {
    use schema::expenses::dsl::*;

    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let mut query = expenses.into_boxed();

    if let Some(project_id_unwrapped) = params.project_id {
        query = query.filter(project_id.eq(project_id_unwrapped))
    }

    let expense_list: Vec<Expense> = query
        .load::<Expense>(&mut conn)
        .expect("Error while trying to get Expenses");

    return expense_list;
}

pub async fn get_expense(pool: web::Data<DbPool>, expense_id: i32) -> Expense {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    return schema::expenses::table
        .find(expense_id)
        .get_result(&mut conn)
        .expect("Error while trying to get Expenses");
}

pub async fn patch_expense(conn: &mut PooledConnection<ConnectionManager<PgConnection>>, path_expense_id: i32, payload: PatchableExpense) -> () {
    if let Some(name_unwrapped) = payload.clone().name {
        diesel::update(expenses.find(path_expense_id))
            .set(name.eq(name_unwrapped))
            .execute(conn)
            .expect("Error while updating user name");
    }

    if let Some(amount_unwrapped) = payload.clone().amount {
        diesel::update(expenses.find(path_expense_id))
            .set(schema::expenses::columns::amount.eq(amount_unwrapped))
            .execute(conn)
            .expect("Error while updating user amount");
    }

    if let Some(description_unwrapped) = payload.clone().description {
        diesel::update(expenses.find(path_expense_id))
            .set(description.eq(description_unwrapped))
            .execute(conn)
            .expect("Error while updating user description");
    }

    if let Some(expense_type_unwrapped) = payload.clone().expense_type {
        diesel::update(expenses.find(path_expense_id))
            .set(expense_type.eq(expense_type_unwrapped))
            .execute(conn)
            .expect("Error while updating user expense_type");
    }
}
