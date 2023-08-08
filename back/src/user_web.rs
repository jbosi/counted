use crate::models::project_model::Project;
use crate::models::user_model::{User, NewUser, PatchableUser, CreatableUser};
use crate::models::user_project_model::UserProjects;
use actix_web::HttpResponse;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use uuid::Uuid;
use crate::{schema, DbPool};
use actix_web::{web, get, Responder, post, delete, patch};
use diesel::Queryable;
use crate::schema::{users, projects};
use diesel::BelongingToDsl;
use super::schema::user_projects;

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
pub async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
	let mut conn = pool.get().expect("couldn't get db connection from pool");

	let results = users::table.load::<User>(&mut conn)
		.expect("Error while trying to get Users");

	web::Json(results)
}

#[get("/users/projects/{project_id}")]
pub async fn get_users_by_project_id(pool: web::Data<DbPool>, path: web::Path<Uuid>) -> impl Responder {
	let mut conn = pool.get().expect("couldn't get db connection from pool");

	let path_project_id: Uuid = path.into_inner();

	let target_project = projects::table
		.filter(projects::id.eq(path_project_id))
		.select(Project::as_select())
		.get_result(&mut conn)
		.expect("Error while trying to get Project");

	let users_in_project: Vec<User> = UserProjects::belonging_to(&target_project)
		.inner_join(users::table)
		.select(User::as_select())
		.load(&mut conn)
		.expect("Error while trying to get Users");

	web::Json(users_in_project)
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

#[derive(Identifiable, Queryable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(primary_key(id))]
struct UserProject {
	id: Uuid,
	project_id: Uuid,
	user_id: i32,
}
