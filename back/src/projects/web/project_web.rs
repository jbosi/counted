use actix_web::{get, HttpRequest, post, Responder, web};
use diesel::prelude::*;

use crate::DbPool;
use crate::projects::application::project_application::{create_project_app, get_projects_app};
use crate::projects::domain::project_model::{CreatableProject, Project};
use crate::query_strings::project_query_string::ProjectQueryParams;

#[post("/projects")]
pub async fn create_project(pool: web::Data<DbPool>, creatable_project: web::Json<CreatableProject>) -> impl Responder {
	let created_project: Project = create_project_app(pool, creatable_project).await;

	web::Json(created_project)
}


#[get("/projects")]
pub async fn get_projects(pool: web::Data<DbPool>, _req: HttpRequest) -> impl Responder {
	let params = web::Query::<ProjectQueryParams>::from_query(_req.query_string()).unwrap();

	let projects = get_projects_app(pool, params).await;

	web::Json(projects)
}