use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::projects::domain::project_model::Project;
use crate::schema::user_projects;
use crate::users::domain::user_model::User;

#[derive(Identifiable, Selectable, Queryable, Associations, Debug, Serialize, Deserialize, Clone)]
#[diesel(belongs_to(Project))]
#[diesel(belongs_to(User))]
#[diesel(table_name = user_projects)]
#[diesel(primary_key(project_id, user_id))]
pub struct UserProjects {
	pub project_id: Uuid,
	pub user_id: i32,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_projects)]
pub struct NewUserProjects {
	pub project_id: Uuid,
	pub user_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatableUserProject {
	pub project_id: Uuid,
	pub user_id: i32,
}