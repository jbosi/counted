use actix_web::web;
use actix_web::web::Query;
use diesel::prelude::*;
use diesel::RunQueryDsl;

use crate::{DbPool, schema};
use crate::expenses::domain::expense_model::Expense;
use crate::query_strings::expense_query_string::ExpenseQueryParams;
use crate::schema::expenses::table;

pub async fn get_expenses(pool: web::Data<DbPool>, params: Query<ExpenseQueryParams>) -> Vec<Expense> {
    use schema::expenses::dsl::*;

    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let query: table = expenses;

    match params.project_id {
        None => {},
        Some(project_id_unwrapped) => query.filter(project_id.eq(project_id_unwrapped))
    };

    let expense_list: Vec<Expense> = query
        .load::<Expense>(&mut conn)
        .expect("Error while trying to get Expenses");

    return expense_list;
}