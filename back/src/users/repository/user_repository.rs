use actix_web::web;
use actix_web::web::Query;
use diesel::RunQueryDsl;
use diesel::{QueryDsl, SelectableHelper};
use diesel::BelongingToDsl;
use diesel::prelude::*;

use crate::{DbPool, schema};
use crate::diesel::ExpressionMethods;
use crate::models::user_project_model::{NewUserProjects, UserProjects};
use crate::projects::domain::project_model::Project;
use crate::query_strings::user_query_string::UserQueryParams;
use crate::schema::{projects, users};
use crate::users::domain::user_model::{CreatableUser, NewUser, PatchableUser, User};

pub async fn get_users(pool: web::Data<DbPool>, params: Query<UserQueryParams>) -> Vec<User> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let user_list: Vec<User>;

    match params.project_id {
        None => {
            user_list = users::table.load::<User>(&mut conn)
                .expect("Error while trying to get Users");
        }
        Some(project_id) => {
            let target_project = projects::table
                .filter(projects::id.eq(project_id))
                .select(Project::as_select())
                .get_result(&mut conn)
                .expect("Error while trying to get Project");

            user_list = UserProjects::belonging_to(&target_project)
                .inner_join(users::table)
                .select(User::as_select())
                .load(&mut conn)
                .expect("Error while trying to get Users");
        }
    }

    return user_list;
}

pub async fn get_user(pool: web::Data<DbPool>, user_id: i32) -> User {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    return users::table
        .find(user_id)
        .get_result(&mut conn)
        .expect("Error while trying to get Project");
}

pub async fn create_users(pool: web::Data<DbPool>, creatable_users: web::Json<Vec<CreatableUser>>, new_users: Vec<NewUser>) -> Vec<User> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let created_users = diesel::insert_into(users::table)
        .values(&new_users)
        .get_results::<User>(&mut conn)
        .expect("Error saving new post");

    let users_to_associate_to_projects: Vec<CreatableUser> = creatable_users
        .clone()
        .into_iter()
        .filter(|u| u.project_id.is_some()).collect();

    users_to_associate_to_projects.iter().map(|u| {
        let user = created_users.iter().find(|cu| cu.name == u.name).unwrap();
        NewUserProjects {
            project_id: u.project_id.unwrap(),
            user_id: user.id
        }
    }).for_each(|u| {
        diesel::insert_into(schema::user_projects::table)
            .values(u)
            .execute(&mut conn)
            .expect("associating users with projects");
    });

    return created_users;
}

pub async fn delete_user(pool: web::Data<DbPool>, user_id: i32) -> () {
    use schema::users::dsl::*;

    let mut conn = pool.get().expect("couldn't get db connection from pool");

    diesel::delete(users.find(user_id))
        .execute(&mut conn)
        .expect("Error deleting user");
}

pub async fn patch_user(pool: web::Data<DbPool>, user_id: i32, payload: PatchableUser) -> () {
    use schema::users::dsl::*;

    let mut conn = pool.get().expect("couldn't get db connection from pool");

    diesel::update(users.find(user_id))
        .set(name.eq(&payload.name))
        .execute(&mut conn)
        .expect("Error while updating user amount");
}