use actix_web::web;
use actix_web::web::Query;
use diesel::prelude::*;

use crate::DbPool;
use crate::projects::domain::project_model::{CreatableProject, NewProject, Project};
use crate::projects::repository::project_repository::{create_project, get_projects};
use crate::query_strings::project_query_string::ProjectQueryParams;

pub async fn get_projects_app(pool: web::Data<DbPool>, params: Query<ProjectQueryParams>) -> Vec<Project> {
    return get_projects(pool.clone(), params).await;
}

pub async fn create_project_app(pool: web::Data<DbPool>, creatable_project: web::Json<CreatableProject>) -> Project {
    let new_project = NewProject {
        name: creatable_project.name.to_string(),
        currency: "Euro".to_string(),
        // total_expenses: 0.0
    };

    return create_project(pool, creatable_project, new_project).await;
}