use crate::models::project_model::{Project, NewProject, CreatableProject, ProjectDto};
use crate::models::user_model::User;
use crate::models::user_project_model::{NewUserProjects, UserProjects};
use crate::schema::{users, projects};
use diesel::{QueryDsl, SelectableHelper};
use diesel::RunQueryDsl;
use diesel::insert_into;
use crate::{schema, DbPool};
use actix_web::{web, get, HttpRequest, Responder, post};
use crate::diesel::ExpressionMethods;
use diesel::BelongingToDsl;

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

	let mut conn = pool.get().expect("couldn't get db connection from pool");

	// For now retrieving all users
	let all_users = users::table
		.select(User::as_select())
		.load::<User>(&mut conn)
		.expect("Error while trying to get Users");

	let projects_for_user: Vec<(Project, User)> = UserProjects::belonging_to(&all_users)
		.inner_join(projects::table)
		.select((Project::as_select(), UserProjects::as_select()))
		.load::<(Project, User)>(&mut conn)
		.expect("Error while trying to get Users");

	let fullProjects: Vec<ProjectDto> = projects_for_user
		.grouped_by(&all_users)
		.into_iter()
		.zip(projects_for_user)
		.map(|(p, u)| ProjectDto {
			id: p.id,
			created_at: p.created_at,
			currency: p.currency,
			name: p.name,
			users: u
		});

	web::Json(fullProjects)
}

#[get("/projects/users/{user_id}")]
pub async fn get_projects_by_user_id(pool: web::Data<DbPool>, path: web::Path<i32>) -> impl Responder {
	let mut conn = pool.get().expect("couldn't get db connection from pool");

	let path_user_id: i32 = path.into_inner();

	let target_project = users::table
		.filter(users::id.eq(path_user_id))
		.select(User::as_select())
		.get_result(&mut conn)
		.expect("Error while trying to get Project");

	let projects_for_user: Vec<Project> = UserProjects::belonging_to(&target_project)
		.inner_join(projects::table)
		.select(Project::as_select())
		.load(&mut conn)
		.expect("Error while trying to get Users");

	web::Json(projects_for_user)
}