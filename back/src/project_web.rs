use crate::models::project_model::{Project, NewProject, CreatableProject};
use crate::models::user_project_model::NewUserProjects;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::insert_into;
use uuid::Uuid;
use crate::{schema, DbPool};
use actix_web::{web, get, HttpRequest, Responder, post};
use crate::diesel::ExpressionMethods;

#[post("/projects")]
pub async fn create_project(pool: web::Data<DbPool>, creatable_project: web::Json<CreatableProject>) -> impl Responder {
	use schema::projects::dsl::*;
	use schema::user_projects::dsl::*;
	let mut conn = pool.get().expect("couldn't get db connection from pool");

	let new_project = NewProject {
		name: creatable_project.name.to_string(),
		currency: "Euro".to_string(),
		// total_expenses: 0.0
	};

	let created_project: Project = insert_into(projects)
		.values(new_project)
		.get_result(&mut conn)
		.expect("Error while adding project");

	let user_project_values: Vec<NewUserProjects> = creatable_project.users.clone()
		.into_iter()
		.map(|u_id| NewUserProjects { project_id: created_project.id, user_id: u_id })
		.collect();

	insert_into(user_projects)
		.values(user_project_values)
		.execute(&mut conn)
		.expect("Error while adding project and users to the join table");


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

// #[get("/projects/{project_id}/users")]
// pub async fn get_project_users(pool: web::Data<DbPool>, path: web::Path<Uuid>) -> impl Responder {
// 	use schema::projects::dsl::*;

// 	let project_id: Uuid = path.into_inner();

// 	let mut conn = pool.get().expect("couldn't get db connection from pool");

// 	let results = projects
// 		.filter(id.eq(project_id))
// 		.load::<Project>(&mut conn)
// 		.expect("Error while trying to get Projects");

// 	web::Json(results)
// }