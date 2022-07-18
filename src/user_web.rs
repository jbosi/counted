use crate::models::PatchableUserAmount;
use crate::models::{User, NewUser};
use actix_web::HttpResponse;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use crate::{schema, DbPool};
use actix_web::{web, get, HttpRequest, Responder, post, delete, patch};

#[post("/users")]
pub async fn create_user(pool: web::Data<DbPool>, new_user: web::Json<NewUser>) -> impl Responder {
	use schema::users;

	let conn = pool.get().expect("couldn't get db connection from pool");

	let new_user = NewUser {
		name: new_user.name.to_string(),
		balance: new_user.balance
	};

	let created_user = diesel::insert_into(users::table)
		.values(&new_user)
		.get_result::<User>(&conn)
		.expect("Error saving new post");
	
	web::Json(created_user)
}


#[get("/users")]
pub async fn get_users(pool: web::Data<DbPool>, _req: HttpRequest) -> impl Responder {
	use schema::users::dsl::*;

	let conn = pool.get().expect("couldn't get db connection from pool");

	let results = users.load::<User>(&conn)
		.expect("Error while trying to get Users");

	web::Json(results)
}


#[patch("/users")]
pub async fn update_user_amount(pool: web::Data<DbPool>, payload: web::Json<PatchableUserAmount>) -> impl Responder {
	use schema::users::dsl::{users, balance};

	let conn = pool.get().expect("couldn't get db connection from pool");

	let updated_user = diesel::update(users.find(payload.user_id))
		.set(balance.eq(payload.amount))
		.execute(&conn)
		.expect("Error while updating user amount");

	web::Json(updated_user)
}

#[delete("/users/{user_id}")]
pub async fn delete_user(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> HttpResponse {
	use schema::users::dsl::*;

	let conn = pool.get().expect("couldn't get db connection from pool");

	diesel::delete(users.find(user_id.into_inner()))
		.execute(&conn)
		.expect("Error deleting user");

		HttpResponse::Ok().finish()
}