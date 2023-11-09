use actix_web::{HttpRequest, HttpResponse};
use actix_web::{delete, get, patch, post, Responder, web};
use diesel::BelongingToDsl;
use diesel::prelude::*;
use diesel::RunQueryDsl;

use crate::{DbPool, schema};
use crate::models::project_model::Project;
use crate::models::user_model::{CreatableUser, NewUser, PatchableUser, User};
use crate::models::user_project_model::{NewUserProjects, UserProjects};
use crate::query_strings::users_query_string::UsersQueryParams;
use crate::schema::{projects, users};

#[post("/users")]
pub async fn create_users(pool: web::Data<DbPool>, creatable_users: web::Json<Vec<CreatableUser>>) -> impl Responder {
	let mut conn = pool.get().expect("couldn't get db connection from pool");

	let new_users: Vec<NewUser> = creatable_users.iter().map(|creatable_user| NewUser {
		name: creatable_user.name.to_string()
	}).collect();

	let created_users = diesel::insert_into(users::table)
		.values(&new_users)
		.get_results::<User>(&mut conn)
		.expect("Error saving new post");

	let users_to_associate_to_projects: Vec<CreatableUser> = creatable_users
		.clone()
		.into_iter()
		.filter(|u| u.project_id.is_some()).collect();

	users_to_associate_to_projects.iter().map(|u| {
		let user = created_users.iter().find(|cu| cu.name == u.name).unwrap();
		NewUserProjects {
			project_id: u.project_id.unwrap(),
			user_id: user.id
		}
	}).for_each(|u| {
		diesel::insert_into(schema::user_projects::table)
			.values(u)
			.execute(&mut conn)
			.expect("associating users with projects");
});

	
	web::Json(created_users)
}


#[get("/users")]
pub async fn get_users(pool: web::Data<DbPool>, _req: HttpRequest) -> impl Responder {
	let mut conn = pool.get().expect("couldn't get db connection from pool");

	let params = web::Query::<UsersQueryParams>::from_query(_req.query_string()).unwrap();

	let results: Vec<User>;

	match params.project_id {
		None => {
			results = users::table.load::<User>(&mut conn)
				.expect("Error while trying to get Users");
		}
		Some(project_id) => {
			let target_project = projects::table
				.filter(projects::id.eq(project_id))
				.select(Project::as_select())
				.get_result(&mut conn)
				.expect("Error while trying to get Project");

			results = UserProjects::belonging_to(&target_project)
				.inner_join(users::table)
				.select(User::as_select())
				.load(&mut conn)
				.expect("Error while trying to get Users");
		}
	}

	web::Json(results)
}

#[patch("/users/{user_id}")]
pub async fn update_user_name(pool: web::Data<DbPool>, user_id: web::Path<i32>, payload: web::Json<PatchableUser>) -> impl Responder {
	use schema::users::dsl::{name, users};

	let mut conn = pool.get().expect("couldn't get db connection from pool");

	let updated_user = diesel::update(users.find(user_id.into_inner()))
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

