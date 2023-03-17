use crate::models::payment_model::{NewPayment};
use crate::models::expense_model::{Expense, NewExpense, CreatableExpense, PatchableExpense};
use crate::schema::payments;
use actix_web::HttpResponse;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use crate::{schema, DbPool};
use actix_web::{web, get, Responder, post, delete, patch};

#[post("projects/{project_id}/expenses/")]
pub async fn create_expense(pool: web::Data<DbPool>, new_expense: web::Json<CreatableExpense>, path: web::Path<i32>) -> impl Responder {
	use schema::expenses;
	let path_project_id: i32 = path.into_inner();

	let mut conn = pool.get().expect("couldn't get db connection from pool");

	let payers = new_expense.payers.clone().into_iter();
	let debtors = new_expense.debtors.clone().into_iter();

	let expense: NewExpense = NewExpense {
		name: new_expense.clone().name,
		amount: new_expense.amount,
		description: new_expense.clone().description,
		expense_type: new_expense.clone().expense_type,
		author_id: new_expense.author_id,
		project_id: path_project_id
	};

	let created_expense: Expense = diesel::insert_into(expenses::table)
		.values(&expense)
		.get_result::<Expense>(&mut conn)
		.expect("Error saving new post");

	let creatable_debtors: Vec<NewPayment> = debtors.map(|d| NewPayment {
		amount: d.amount,
		expense_id: created_expense.id,
		user_id: d.user_id,
		is_debt: true
	}).collect();

	let creatable_payors: Vec<NewPayment> = payers.map(|p| NewPayment {
		amount: p.amount,
		expense_id: created_expense.id,
		user_id: p.user_id,
		is_debt: false
	}).collect();

	let creatable_payments: Vec<NewPayment> = [creatable_debtors, creatable_payors].concat();
	
	diesel::insert_into(payments::table)
		.values(&creatable_payments)
		.execute(&mut conn)
		.expect("Error adding payments");
	
	web::Json(created_expense)
}

// On veut les expenses relatives à un projet et pouvoir éventuellement filtrer sur un user
#[get("projects/{project_id}/expenses/")]
pub async fn get_expense(pool: web::Data<DbPool>, path: web::Path<i32>) -> impl Responder {
	use schema::expenses::dsl::*;
	let path_project_id: i32 = path.into_inner();

	let mut conn = pool.get().expect("couldn't get db connection from pool");

	let expense_list = expenses
		.filter(project_id.eq(path_project_id))
		.load::<Expense>(&mut conn)
		.expect("Error while trying to get Expenses");
	
	web::Json(expense_list)
}


// #[patch("projects/{project_id}/expenses/{expense_id}")]
// pub async fn update_expense(pool: web::Data<DbPool>, path: web::Path<(i32, i32)>, payload: web::Json<PatchableExpense>) -> impl Responder {
// 	use schema::expenses::dsl::*;

// 	let (path_project_id, path_expense_id): (i32, i32) = path.into_inner();

// 	let mut conn = pool.get().expect("couldn't get db connection from pool");

// 	let updated_user = diesel::update(expenses.find(path_expense_id))
// 		.set({
// 			if (Some(payload.amount)) {
// 				amount.eq(payload.amount)
// 			} if (Some(payload.name)) {
// 				name.eq(payload.name)
// 			} if (Some(payload.description)) {
// 				description.eq(payload.description)
// 			} if (Some(payload.expense_type)) {
// 				description.eq(payload.expense_type)
// 			} if (Some(payload.debtors)) {
// 				description.eq(payload.debtors)
// 			} if (Some(payload.payers)) {
// 				description.eq(payload.payers)
// 			}
// 		})
// 		.execute(&conn)
// 		.expect("Error while updating user amount");

// 	web::Json(updated_user)
// }

#[delete("projects/{project_id}/expenses/{expense_id}")]
pub async fn delete_expense(pool: web::Data<DbPool>, path: web::Path<(i32, i32)>) -> HttpResponse {
	use schema::expenses::dsl::*;

	let (path_project_id, path_expense_id): (i32, i32) = path.into_inner();

	let mut conn = pool.get().expect("couldn't get db connection from pool");

	diesel::delete(expenses.find(path_expense_id))
		.execute(&mut conn)
		.expect("Error deleting expense");

		HttpResponse::Ok().finish()
}