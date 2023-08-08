use diesel::Queryable;
use uuid::Uuid;
use diesel::prelude::*;
use crate::schema::{user_project};
use crate::models::project_model::Project;
use crate::models::user_model::User;
use serde::{Serialize, Deserialize};

#[derive(Identifiable, Selectable, Queryable, Associations, Debug, Clone, Serialize, Deserialize)]
#[diesel(belongs_to(Project))]
#[diesel(belongs_to(User))]
#[diesel(table_name = user_project)]
pub struct UserProject {
	pub id: Uuid,
	pub project_id: Uuid,
	pub user_id: i32,
}
