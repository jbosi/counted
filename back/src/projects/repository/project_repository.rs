use actix_web::web;
use actix_web::web::Query;
use diesel::{insert_into, RunQueryDsl};
use diesel::{QueryDsl, SelectableHelper};
use diesel::BelongingToDsl;
use uuid::Uuid;

use crate::{DbPool, schema};
use crate::diesel::ExpressionMethods;
use crate::models::user_project_model::{NewUserProjects, UserProjects};
use crate::projects::domain::project_model::{CreatableProject, NewProject, Project};
use crate::query_strings::project_query_string::ProjectQueryParams;
use crate::schema::{projects, user_projects, users};
use crate::users::domain::user_model::User;

pub async fn get_projects(pool: web::Data<DbPool>, params: Query<ProjectQueryParams>) -> Vec<Project> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let project_list: Vec<Project>;

    match params.user_id {
        None => project_list = projects::table
            .load::<Project>(&mut conn)
            .expect("Error while trying to get Projects"),
        Some(user_id) => {
            let target_project = users::table
                .filter(users::id.eq(user_id))
                .select(User::as_select())
                .get_result(&mut conn)
                .expect("Error while trying to get Project");

            project_list = UserProjects::belonging_to(&target_project)
                .inner_join(projects::table)
                .select(Project::as_select())
                .load(&mut conn)
                .expect("Error while trying to get Users for project");
        }
    }

    return project_list;
}

pub async fn get_all_projects(pool: web::Data<DbPool>) -> Vec<Project> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    return projects::table
        .load::<Project>(&mut conn)
        .expect("Error while trying to get Projects");
}
pub async fn get_projects_and_user_projects_for_user(pool: web::Data<DbPool>, params: Query<ProjectQueryParams>) -> Vec<(UserProjects, Project)> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    return users::table.inner_join(user_projects::table.inner_join(projects::table))
        // .left_outer_join(users::table.on(users::id.eq(user_projects::user_id)))
        .filter(users::id.eq(params.user_id.unwrap()))
        .select((UserProjects::as_select(), Project::as_select()))
        .load::<(UserProjects, Project)>(&mut conn)
        .expect("Error while trying to get UserProjects - user_projects step ");
}

pub async fn create_project(pool: web::Data<DbPool>, creatable_project: web::Json<CreatableProject>, new_project: NewProject) -> Project {
    use schema::projects::dsl::*;
    use schema::user_projects::dsl::*;

    let mut conn = pool.get().expect("couldn't get db connection from pool");

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

    return created_project;
}

pub async fn get_project(pool: web::Data<DbPool>, project_id: Uuid) -> Project {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    return projects::table
        .find(project_id)
        .get_result(&mut conn)
        .expect("Error while trying to get Project");
}
