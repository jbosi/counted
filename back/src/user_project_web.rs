use actix_web::{get, HttpRequest, post, Responder, web};
use diesel::{QueryDsl, SelectableHelper};
use diesel::BelongingToDsl;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

use crate::{DbPool, schema};
use crate::models::user_project_model::{NewUserProjects, UserProjects};
use crate::projects::domain::project_model::{CreatableProject, NewProject, Project, UserProjectDto};
use crate::schema::{projects, users};
use crate::users::domain::user_model::User;

#[post("/user-projects")]
pub async fn create_project(pool: web::Data<DbPool>, creatable_project: web::Json<CreatableProject>) -> impl Responder {
	use schema::projects::dsl::*;
	use schema::user_projects::dsl::*;
	let mut conn = pool.get().expect("couldn't get db connection from pool");

	let new_project = NewProject {
		name: creatable_project.name.to_string(),
		currency: "Euro".to_string(),
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


#[get("/user-projects")]
pub async fn get_user_projects(pool: web::Data<DbPool>, _req: HttpRequest) -> impl Responder {
	// let params = web::Query::<UserProjectsQueryParams>::from_query(_req.query_string()).unwrap();

	let mut conn = pool.get().expect("couldn't get db connection from pool");

	let projects: Vec<Project> = projects::table
		.load::<Project>(&mut conn)
		.expect("Error while trying to get UserProjects - project step");

	let users: Vec<User> = users::table
		.select(User::as_select())
		.load::<User>(&mut conn)
		.expect("Error while trying to get UserProjects - user step");

	let user_by_user_project: Vec<(UserProjects, User)> = UserProjects::belonging_to(&projects)
		.inner_join(users::table)
		.select((UserProjects::as_select(), User::as_select()))
		.load::<(UserProjects, User)>(&mut conn)
		.expect("Error while trying to get UserProjects - userproject step ");

	let full_projects: Vec<UserProjectDto> = user_by_user_project
		.grouped_by(&users)
		.into_iter()
		.zip(projects)
		.map(|(g, p)| UserProjectDto {
			id: p.id,
			created_at: p.created_at,
			currency: p.currency,
			name: p.name,
			users: g.into_iter().map(|(_up, u)| u.id).collect()
		})
		.collect();

	web::Json(full_projects)
}
