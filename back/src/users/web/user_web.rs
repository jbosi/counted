use actix_web::{HttpRequest, HttpResponse};
use actix_web::{delete, get, patch, post, Responder, web};
use diesel::prelude::*;

use crate::DbPool;
use crate::query_strings::user_query_string::UserQueryParams;
use crate::users::application::user_application::{create_users_app, delete_user_app, get_users_app, patch_user_app};
use crate::users::domain::user_model::{CreatableUser, PatchableUser, User};

#[post("/users")]
pub async fn create_users(pool: web::Data<DbPool>, creatable_users: web::Json<Vec<CreatableUser>>) -> impl Responder {
	let created_users: Vec<User> = create_users_app(pool, creatable_users).await;

	web::Json(created_users)
}


#[get("/users")]
pub async fn get_users(pool: web::Data<DbPool>, _req: HttpRequest) -> impl Responder {
	let params = web::Query::<UserQueryParams>::from_query(_req.query_string()).unwrap();

	let created_users: Vec<User> = get_users_app(pool, params).await;

	web::Json(created_users)
}

#[patch("/users/{user_id}")]
pub async fn update_user_name(pool: web::Data<DbPool>, user_id: web::Path<i32>, payload: web::Json<PatchableUser>) -> impl Responder {
	patch_user_app(pool, user_id.into_inner(), payload.into_inner()).await;

	HttpResponse::Ok().finish()
}

#[delete("/users/{user_id}")]
pub async fn delete_user(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> HttpResponse {
	delete_user_app(pool, user_id.into_inner()).await;

	HttpResponse::Ok().finish()
}

