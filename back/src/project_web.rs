use actix_web::{get, HttpRequest, post, Responder, web};
use diesel::{QueryDsl, SelectableHelper};
use diesel::BelongingToDsl;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

use crate::{DbPool, schema};
use crate::diesel::ExpressionMethods;
use crate::models::project_model::{CreatableProject, NewProject, Project};
use crate::models::user_model::User;
use crate::models::user_project_model::{NewUserProjects, UserProjects};
use crate::query_strings::project_query_string::ProjectQueryParams;
use crate::schema::{projects, users};

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
	let params = web::Query::<ProjectQueryParams>::from_query(_req.query_string()).unwrap();
	let mut conn = pool.get().expect("couldn't get db connection from pool");

	let projects: Vec<Project>;

	match params.user_id {
		None => projects = projects::table
			.load::<Project>(&mut conn)
			.expect("Error while trying to get Users"),
		Some(user_id) => {
			let target_project = users::table
				.filter(users::id.eq(user_id))
				.select(User::as_select())
				.get_result(&mut conn)
				.expect("Error while trying to get Project");

			projects = UserProjects::belonging_to(&target_project)
				.inner_join(projects::table)
				.select(Project::as_select())
				.load(&mut conn)
				.expect("Error while trying to get Users for project");
		}
	}

	web::Json(projects)
}