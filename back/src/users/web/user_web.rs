use actix_web::{HttpRequest, HttpResponse};
use actix_web::{delete, get, patch, post, Responder, web};

use crate::DbPool;
use crate::query_strings::user_query_string::UserQueryParams;
use crate::users::application::user_application::{create_users_app, delete_user_app, get_user_app, get_users_app, patch_user_app};
use crate::users::domain::user_model::{CreatableUser, PatchableUser, User};

#[post("/users")]
pub async fn create_users(pool: web::Data<DbPool>, creatable_users: web::Json<Vec<CreatableUser>>) -> impl Responder {
	let created_users: Vec<User> = create_users_app(pool, creatable_users).await;

	web::Json(created_users)
}

#[utoipa::path(
	responses(
		(status = 200, description = "Users list", body = Vec<User>),
	),
	params(
		("UserQueryParams", description = "filter by project id"),
	)
)]
#[get("/users")]
pub async fn get_users(pool: web::Data<DbPool>, _req: HttpRequest) -> impl Responder {
	let params = web::Query::<UserQueryParams>::from_query(_req.query_string()).unwrap();

	let users: Vec<User> = get_users_app(pool, params).await;

	web::Json(users)
}

#[utoipa::path(
	responses(
		(status = 200, description = "Get User", body = User),
	),
	params(
		("user_id", description = "get requested user id"),
	)
)]
#[get("/users/{user_id}")]
pub async fn get_user(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> impl Responder {
	let user: User = get_user_app(pool, user_id.into_inner()).await;

	web::Json(user)
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

