use crate::models::user_model::{User, NewUser, PatchableUser, CreatableUser};
use actix_web::HttpResponse;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use crate::{schema, DbPool};
use actix_web::{web, get, HttpRequest, Responder, post, delete, patch};

#[post("/users")]
pub async fn create_user(pool: web::Data<DbPool>, creatable_users: web::Json<Vec<CreatableUser>>) -> impl Responder {
	let mut conn = pool.get().expect("couldn't get db connection from pool");

	let new_users: Vec<NewUser> = creatable_users.iter().map(|creatable_user| NewUser {
		name: creatable_user.name.to_string()
	}).collect();

	let created_user = diesel::insert_into(schema::users::table)
		.values(&new_users)
		.get_results::<User>(&mut conn)
		.expect("Error saving new post");
	
	web::Json(created_user)
}


#[get("/users")]
pub async fn get_users(pool: web::Data<DbPool>, _req: HttpRequest) -> impl Responder {
	use schema::users::dsl::*;

	let mut conn = pool.get().expect("couldn't get db connection from pool");

	let results = users.load::<User>(&mut conn)
		.expect("Error while trying to get Users");

	web::Json(results)
}


#[patch("/users")]
pub async fn update_user_name(pool: web::Data<DbPool>, payload: web::Json<PatchableUser>) -> impl Responder {
	use schema::users::dsl::{users, name};

	let mut conn = pool.get().expect("couldn't get db connection from pool");

	let updated_user = diesel::update(users.find(payload.user_id))
		.set(name.eq(&payload.name))
		.execute(&mut conn)
		.expect("Error while updating user amount");

	web::Json(updated_user)
}

#[delete("/users/{user_id}")]
pub async fn delete_user(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> HttpResponse {
	use schema::users::dsl::*;

	let mut conn = pool.get().expect("couldn't get db connection from pool");

	diesel::delete(users.find(user_id.into_inner()))
		.execute(&mut conn)
		.expect("Error deleting user");

		HttpResponse::Ok().finish()
}