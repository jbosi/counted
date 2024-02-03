use actix_web::web;
use actix_web::web::Query;
use diesel::query_builder::AsQuery;
use itertools::Itertools;
use uuid::Uuid;

use crate::DbPool;
use crate::models::user_project_model::UserProjects;
use crate::projects::domain::project_model::{CreatableProject, NewProject, Project, ProjectDto};
use crate::projects::repository::project_repository::{create_project, get_all_projects, get_project, get_projects_and_user_projects_for_user};
use crate::query_strings::project_query_string::ProjectQueryParams;

pub async fn get_projects_app(pool: web::Data<DbPool>, params: Query<ProjectQueryParams>) -> Vec<ProjectDto> {
    let all_projects: Vec<Project> = get_all_projects(pool.clone()).await;
    let mut projects_dto: Vec<ProjectDto> = Vec::new();

    let projects_and_user_projects_for_user: Vec<(UserProjects, Project)> = get_projects_and_user_projects_for_user(pool, params).await;

    let projects_group = projects_and_user_projects_for_user
        // .grouped_by(&users)
        .into_iter()
        .group_by(|(_up, p)| p.id);
    // .zip(projects)

    for (p_id, user_projects) in &projects_group {
        let current_project = all_projects.iter().find(|p| p.id == p_id).unwrap().clone();
        let users: Vec<i32> = user_projects
            .map(|(up, _p)| up.user_id)
            .collect();

        projects_dto.push(ProjectDto {
            id: current_project.id,
            created_at: current_project.created_at,
            currency: current_project.currency,
            name: current_project.name,
            users
        })
    }

    return projects_dto;
}

pub async fn get_project_app(pool: web::Data<DbPool>, project_id: Uuid) -> Project {
    return get_project(pool.clone(), project_id).await;
}
pub async fn create_project_app(pool: web::Data<DbPool>, creatable_project: web::Json<CreatableProject>) -> Project {
    let new_project = NewProject {
        name: creatable_project.name.to_string(),
        currency: "Euro".to_string(),
        // total_expenses: 0.0
    };

    return create_project(pool, creatable_project, new_project).await;
}