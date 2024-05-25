use actix_web::{HttpRequest, HttpResponse};
use actix_web::{delete, get, post, patch, Responder, web};
use actix_web::web::Query;
use chrono::Utc;
use diesel::prelude::*;
use diesel::RunQueryDsl;

use crate::{DbPool, schema};
use crate::expenses::application::expense_application::{get_expense_app, get_expenses_app};
use crate::expenses::domain::expense_model::{CreatableExpense, Expense, NewExpense, PatchableExpense};
use crate::payments::domain::payment_model::{ExpenseDto, NewPayment};
use crate::query_strings::expense_query_string::ExpenseQueryParams;
use crate::schema::payments;
use crate::users::domain::user_model::UserAmount;

#[post("expenses")]
pub async fn create_expense(pool: web::Data<DbPool>, new_expense: web::Json<CreatableExpense>) -> impl Responder {
	use schema::expenses;

	let mut conn = pool.get().expect("couldn't get db connection from pool");

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

	let payers = Some(new_expense.payers.clone());
	let debtors = Some(new_expense.debtors.clone());

	let creatable_payments: Vec<NewPayment> = forge_creatable_payments(payers, debtors, created_expense.id);
	
	diesel::insert_into(payments::table)
		.values(&creatable_payments)
		.execute(&mut conn)
		.expect("Error adding payments");
	
	web::Json(created_expense)
}

#[get("expenses")]
pub async fn get_expenses(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
	let params: Query<ExpenseQueryParams> = web::Query::<ExpenseQueryParams>::from_query(req.query_string()).unwrap();

	let expense_dto: Vec<ExpenseDto> = get_expenses_app(pool, params).await;

	web::Json(expense_dto)
}

#[get("expenses/{expense_id}")]
pub async fn get_expense(pool: web::Data<DbPool>, expense_id: web::Path<i32>) -> impl Responder {
	let expense_dto: ExpenseDto = get_expense_app(pool, expense_id.into_inner()).await;

	web::Json(expense_dto)
}

#[patch("expenses/{expense_id}")]
pub async fn update_expense(pool: web::Data<DbPool>, path: web::Path<(i32, i32)>, payload: web::Json<PatchableExpense>) -> impl Responder {
	use schema::expenses::dsl::*;
	use schema::payments::dsl::*;

	let (_, path_expense_id): (i32, i32) = path.into_inner();

	let mut conn = pool.get().expect("couldn't get db connection from pool");
// https://stackoverflow.com/questions/72249171/rust-diesel-conditionally-update-fields

	// TODO handle null values for each prop by making an update for each prop (i.e. 4 set(), one for each prop)
	let values = (
		schema::expenses::columns::amount.eq(payload.clone().amount.unwrap()),
		name.eq(payload.clone().name.unwrap()),
		description.eq(payload.clone().description.unwrap()),
		expense_type.eq(payload.clone().expense_type.unwrap()),
	);

	let updated_user = diesel::update(expenses.find(path_expense_id))
		.set(values)
		.execute(&mut conn)
		.expect("Error while updating user amount");

	let editable_payments: Vec<NewPayment> = forge_creatable_payments(payload.clone().payers, payload.clone().debtors, path_expense_id);

	for editable_payment in editable_payments {
		diesel::update(payments.find(editable_payment.expense_id))
			.set(editable_payment)
			.execute(&mut conn)
			.expect("Error updating payments");
	}

	web::Json(updated_user)
}

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

fn forge_creatable_payments(payers_option: Option<Vec<UserAmount>>, debtors_option: Option<Vec<UserAmount>>, created_expense_id: i32) -> Vec<NewPayment> {
	let mut debtors: Vec<UserAmount> = vec![];
	if let Some(debtors_unwrapped) = debtors_option {
		debtors = debtors_unwrapped;
	}

	let mut payers: Vec<UserAmount> = vec![];
	if let Some(payers_unwrapped) = payers_option {
		payers = payers_unwrapped;
	}

	let creatable_debtors: Vec<NewPayment> = debtors.into_iter().map(|d| NewPayment {
		amount: d.amount,
		expense_id: created_expense_id,
		user_id: d.user_id,
		is_debt: true
	}).collect();

	let creatable_payers: Vec<NewPayment> = payers.into_iter().map(|p| NewPayment {
		amount: p.amount,
		expense_id: created_expense_id,
		user_id: p.user_id,
		is_debt: false
	}).collect();

	return [creatable_debtors, creatable_payers].concat();
}
