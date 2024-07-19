use actix_web::{HttpRequest, HttpResponse};
use actix_web::{delete, get, patch, post, Responder, web};
use actix_web::web::Query;
use chrono::Utc;
use diesel::prelude::*;
use diesel::RunQueryDsl;

use crate::{DbPool, schema};
use crate::expenses::application::expense_application::{get_expense_app, get_expenses_app, patch_expense_app};
use crate::expenses::domain::expense_model::{CreatableExpense, Expense, NewExpense, PatchableExpense};
use crate::payments::application::payment_application::forge_creatable_payments_from_expense;
use crate::payments::domain::payment_model::{ExpenseDto, NewPayment};
use crate::payments::repository::payment_repository::{create_payments, delete_payments_by_expense_id};
use crate::query_strings::expense_query_string::ExpenseQueryParams;
use crate::schema::expenses::dsl::expenses;
use crate::schema::payments;

#[post("/expenses")]
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

	let payers = Some(new_expense.clone().payers);
	let debtors = Some(new_expense.clone().debtors);

	let creatable_payments: Vec<NewPayment> = forge_creatable_payments_from_expense(payers, debtors, created_expense.id);
	
	diesel::insert_into(payments::table)
		.values(&creatable_payments)
		.execute(&mut conn)
		.expect("Error adding payments");
	
	web::Json(created_expense)
}

#[get("/expenses")]
pub async fn get_expenses(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
	let params: Query<ExpenseQueryParams> = web::Query::<ExpenseQueryParams>::from_query(req.query_string()).unwrap();

	let expense_dto: Vec<ExpenseDto> = get_expenses_app(pool, params).await;

	web::Json(expense_dto)
}

#[get("/expenses/{expense_id}")]
pub async fn get_expense(pool: web::Data<DbPool>, expense_id: web::Path<i32>) -> impl Responder {
	let expense_dto: ExpenseDto = get_expense_app(pool, expense_id.into_inner()).await;

	web::Json(expense_dto)
}

#[patch("/expenses/{expense_id}")]
pub async fn update_expense(pool: web::Data<DbPool>, path: web::Path<i32>, payload: web::Json<PatchableExpense>) -> impl Responder {

	let path_expense_id: i32 = path.into_inner();

	patch_expense_app(pool.clone(), path_expense_id, payload.clone()).await;

	// La solution la plus raisonnable mais perfectible me semble être de supprimer tous les payments de la dépense cible pour les recréer
	delete_payments_by_expense_id(pool.clone(), path_expense_id).await;

	let new_payments: Vec<NewPayment> = forge_creatable_payments_from_expense(payload.clone().payers, payload.clone().debtors, path_expense_id);
	create_payments(pool, new_payments).await;

	HttpResponse::Ok().finish()
}

#[delete("/expenses/{expense_id}")]
pub async fn delete_expense(pool: web::Data<DbPool>, path: web::Path<i32>) -> HttpResponse {
	use schema::expenses::dsl::*;

	let path_expense_id: i32 = path.into_inner();

	let mut conn = pool.get().expect("couldn't get db connection from pool");

	diesel::delete(expenses.find(path_expense_id))
		.execute(&mut conn)
		.expect("Error deleting expense");

		HttpResponse::Ok().finish()
}
