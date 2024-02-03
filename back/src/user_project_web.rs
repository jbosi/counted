use std::collections::HashMap;
use std::vec::IntoIter;
use actix_web::{get, HttpRequest, post, Responder, web};
use diesel::{QueryDsl, SelectableHelper};
use diesel::BelongingToDsl;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::query_builder::AsQuery;
use diesel::query_dsl::methods::GroupByDsl;
use diesel::RunQueryDsl;
use uuid::Uuid;
use itertools::{GroupBy, Itertools};

use crate::{DbPool, schema};
use crate::models::user_project_model::{NewUserProjects, UserProjects};
use crate::projects::domain::project_model::{CreatableProject, NewProject, Project, ProjectDto};
use crate::schema::{projects, user_projects, users};
use crate::schema::user_projects::project_id;
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


// #[get("/user-projects")]
// pub async fn get_user_projects(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
// 	let params = web::Query::<UserProjectsQueryParams>::from_query(req.query_string()).unwrap();
//
// 	let mut conn = pool.get().expect("couldn't get db connection from pool");
//
// 	let projects: Vec<Project> = projects::table
// 		.load::<Project>(&mut conn)
// 		.expect("Error while trying to get UserProjects - project step");
//
// 	let users: Vec<User> = users::table
// 		.load::<User>(&mut conn)
// 		.expect("Error while trying to get UserProjects - user step");
//
// 	// let user_by_user_projects: (User, Vec<UserProjects>) =
//
// 	// let user_by_user_project: Vec<(UserProjects, User)> = UserProjects::belonging_to(&projects)
// 	// 	.left_outer_join(users::table.on(users::id.eq()))
// 	// 	.filter(users::id.eq(params.user_id))
// 	// 	.select((UserProjects::as_select(), User::as_select()))
// 	// 	.load::<(UserProjects, User)>(&mut conn)
// 	// 	.expect("Error while trying to get UserProjects - userproject step ");
//
// 	let projects_and_user_projects_for_user: Vec<(UserProjects, Project)> = users::table
// 		.inner_join(user_projects::table.inner_join(projects::table))
// 		// .left_outer_join(users::table.on(users::id.eq(user_projects::user_id)))
// 		.filter(users::id.eq(params.user_id))
// 		.select((UserProjects::as_select(), Project::as_select()))
// 		.load::<(UserProjects, Project)>(&mut conn)
// 		.expect("Error while trying to get UserProjects - user_projects step ");
//
// 	// let user_ids_by_project_id: HashMap<Uuid, Vec<i32>> = projects_and_user_projects_for_user
// 	// 	.iter()
// 	// 	.fold(HashMap::new(), |mut acc, (up, p)| acc.insert(p.id, [up.user_id]))
// 	// 	.collect();
//
// 	// let user_ids_by_project_id: HashMap<Uuid, Vec<i32>> = projects_and_user_projects_for_user
// 	// 	.fold(HashMap::new(), |mut acc, (up, p)| acc.insert(p.id, [up.user_id]))
// 	// 	.collect();
//
// 	let projects_group = projects_and_user_projects_for_user
// 		// .grouped_by(&users)
// 		.into_iter()
// 		.group_by(|(up, p)| p.id);
// 		// .zip(projects)
//
// 	let mut full_projects: Vec<ProjectDto> = Vec::new();
//
// 	for (p_id, user_projects) in &projects_group {
// 		let current_project = projects.iter().find(|p| p.id == p_id).unwrap();
// 		let users: Vec<i32> = user_projects.collect();
//
// 		full_projects.push(ProjectDto {
// 			id: current_project.id,
// 			created_at: current_project.created_at,
// 			currency: current_project.currency,
// 			name: current_project.name,
// 			users: users
// 		})
// 	}
//
// 	web::Json(full_projects)
// }
