use crate::models::payment_model::{NewPayment, Payment, ExpensePayments};
use crate::models::expense_model::{Expense, NewExpense, CreatableExpense};
use crate::schema::payments;
use actix_web::{HttpRequest, HttpResponse};
use chrono::Utc;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use uuid::Uuid;
use crate::{schema, DbPool};
use actix_web::{web, get, Responder, post, delete};
use crate::query_strings::expenses_query_string::ExpensesQueryParams;

#[post("expenses")]
pub async fn create_expense(pool: web::Data<DbPool>, new_expense: web::Json<CreatableExpense>) -> impl Responder {
	use schema::expenses;

	let mut conn = pool.get().expect("couldn't get db connection from pool");

	let payers = new_expense.payers.clone().into_iter();
	let debtors = new_expense.debtors.clone().into_iter();


	let expense: NewExpense = NewExpense {
		name: new_expense.clone().name,
		amount: new_expense.amount,
		date: Utc::now().date_naive(),
		description: new_expense.clone().description,
		expense_type: new_expense.clone().expense_type,
		author_id: new_expense.author_id,
		project_id: new_expense.project_id
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
#[get("expenses")]
pub async fn get_expense(pool: web::Data<DbPool>, _req: HttpRequest) -> impl Responder {
	let params = web::Query::<ExpensesQueryParams>::from_query(_req.query_string()).unwrap();
	use schema::expenses::dsl::*;

	let mut conn = pool.get().expect("couldn't get db connection from pool");

	let expense_list = expenses
		.filter(project_id.eq(params.project_id))
		.load::<Expense>(&mut conn)
		.expect("Error while trying to get Expenses");
	
	web::Json(expense_list)
}

#[get("expensepayments")]
pub async fn get_expense_payments(pool: web::Data<DbPool>, _req: HttpRequest) -> impl Responder {
	use schema::expenses::dsl::*;
	use schema::payments::dsl::*;
	let params = web::Query::<ExpensesQueryParams>::from_query(_req.query_string()).unwrap();

	let mut conn = pool.get().expect("couldn't get db connection from pool");

	let expense_list = expenses
		.filter(project_id.eq(params.project_id))
		.load::<Expense>(&mut conn)
		.expect("Error while trying to get Expenses");

	let payments_list = payments
		.load::<Payment>(&mut conn)
		.expect("Error while trying to get Payment");

	let expense_payments: Vec<ExpensePayments> = expense_list
		.iter()
		.map(|expense| ExpensePayments {
			author_id: expense.author_id,
			id: expense.id,
			amount: expense.amount,
			date: expense.date,
			description: expense.description.clone(),
			expense_type: expense.expense_type.clone(),
			name: expense.name.clone(),
			project_id: expense.project_id,
			payments: payments_list
				.clone()
				.into_iter()
				.filter(|&payment| payment.expense_id == expense.id)
				.collect::<Vec<Payment>>()
		})
		.collect();

	web::Json(expense_payments)
}

// #[patch("expenses/{expense_id}")]
// pub async fn update_expense(pool: web::Data<DbPool>, path: web::Path<(i32, i32)>, payload: web::Json<PatchableExpense>) -> impl Responder {
// 	use schema::expenses::dsl::*;

// 	let (path_project_id, path_expense_id): (i32, i32) = path.into_inner();

// 	let mut conn = pool.get().expect("couldn't get db connection from pool");
// // https://stackoverflow.com/questions/72249171/rust-diesel-conditionally-update-fields
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

#[delete("expenses/{expense_id}")]
pub async fn delete_expense(pool: web::Data<DbPool>, path: web::Path<i32>) -> HttpResponse {
	use schema::expenses::dsl::*;

	let path_expense_id: i32 = path.into_inner();

	let mut conn = pool.get().expect("couldn't get db connection from pool");

	diesel::delete(expenses.find(path_expense_id))
		.execute(&mut conn)
		.expect("Error deleting expense");

		HttpResponse::Ok().finish()
}