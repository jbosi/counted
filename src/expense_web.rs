use crate::models::NewPayment;
use crate::models::{Expense, NewExpense};
use actix_web::HttpResponse;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use crate::{schema, DbPool};
use actix_web::{web, get, HttpRequest, Responder, post, delete, patch};

#[post("/expenses")]
pub async fn create_expense(pool: web::Data<DbPool>, new_expense: web::Json<NewExpense>) -> impl Responder {
	use schema::expenses;

	let mut conn = pool.get().expect("couldn't get db connection from pool");

	let payers = new_expense.payers.into_iter();
	let debtors = new_expense.debtors.into_iter();

	let expense: NewExpense = new_expense.into_inner();
	// Créer la dépense
	let created_expense = diesel::insert_into(expenses::table)
			.values(&expense)
			.get_result::<Expense>(&mut conn)
			.expect("Error saving new post");

	// ainsi que les paiements
	let creatable_payments = payers.map(|p| NewPayment {
		amount: p.amount,
	});
	
	
	web::Json(created_expense)
}


#[get("/expenses")]
pub async fn get_expense(pool: web::Data<DbPool>, _req: HttpRequest) -> impl Responder {
	use schema::expenses::dsl::*;

	let conn = pool.get().expect("couldn't get db connection from pool");

	let results = expenses.load::<Expense>(&mut conn)
		.expect("Error while trying to get Expenses");

	web::Json(results)
}


// #[patch("/expenses")]
// pub async fn update_expense(pool: web::Data<DbPool>, payload: web::Json<PatchableExpense>) -> impl Responder {
// 	use schema::expenses::dsl::{expenses, date, amount, description, expense_type, name};

// 	let conn = pool.get().expect("couldn't get db connection from pool");

// 	let updated_user = diesel::update(expenses.find(payload.expense_id))
// 		.set(balance.eq(payload.amount))
// 		.execute(&conn)
// 		.expect("Error while updating user amount");

// 	web::Json(updated_user)
// }

#[delete("/expenses/{expense_id}")]
pub async fn delete_expense(pool: web::Data<DbPool>, expense_id: web::Path<i32>) -> HttpResponse {
	use schema::expenses::dsl::*;

	let mut conn = pool.get().expect("couldn't get db connection from pool");

	diesel::delete(expenses.find(expense_id.into_inner()))
		.execute(&mut conn)
		.expect("Error deleting expense");

		HttpResponse::Ok().finish()
}