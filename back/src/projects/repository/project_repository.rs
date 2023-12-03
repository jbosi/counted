use actix_web::web;
use actix_web::web::Query;
use diesel::{insert_into, RunQueryDsl};
use diesel::{QueryDsl, SelectableHelper};
use diesel::BelongingToDsl;
use diesel::prelude::*;

use crate::{DbPool, schema};
use crate::diesel::ExpressionMethods;
use crate::models::user_project_model::{NewUserProjects, UserProjects};
use crate::projects::domain::project_model::{NewProject, Project, CreatableProject};
use crate::query_strings::project_query_string::ProjectQueryParams;
use crate::schema::{projects, users};
use crate::users::domain::user_model::User;

pub async fn get_projects(pool: web::Data<DbPool>, params: Query<ProjectQueryParams>) -> Vec<Project> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let project_list: Vec<Project>;

    match params.user_id {
        None => project_list = projects::table
            .load::<Project>(&mut conn)
            .expect("Error while trying to get Users"),
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