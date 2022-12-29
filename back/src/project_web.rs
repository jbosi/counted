use crate::models::{Project, NewProject};
use actix_web::HttpResponse;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use crate::{schema, DbPool};
use actix_web::{web, get, HttpRequest, Responder, post};

#[post("/projects")]
pub async fn create_project(pool: web::Data<DbPool>, new_project: web::Json<NewProject>) -> impl Responder {
	let mut conn = pool.get().expect("couldn't get db connection from pool");

	let new_project = NewProject {
		name: new_project.name.to_string(),
		users: new_project.users
	};

	let created_project = diesel::insert_into(schema::projects::table)
		.values(&new_project)
		.get_result::<Project>(&mut conn)
		.expect("Error saving new post");
	
	web::Json(created_project)
}


#[get("/projects")]
pub async fn get_projects(pool: web::Data<DbPool>, _req: HttpRequest) -> impl Responder {
	use schema::projects::dsl::*;

	let mut conn = pool.get().expect("couldn't get db connection from pool");

	let results = projects.load::<Project>(&mut conn)
		.expect("Error while trying to get Projects");

	web::Json(results)
}
