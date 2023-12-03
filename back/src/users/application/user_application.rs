use actix_web::web;
use actix_web::web::Query;
use diesel::prelude::*;

use crate::DbPool;
use crate::query_strings::user_query_string::UserQueryParams;
use crate::users::domain::user_model::{CreatableUser, NewUser, PatchableUser, User};
use crate::users::repository::user_repository::{create_users, delete_user, get_users, patch_user};

pub async fn get_users_app(pool: web::Data<DbPool>, params: Query<UserQueryParams>) -> Vec<User> {
    return get_users(pool.clone(), params).await;
}

pub async fn delete_user_app(pool: web::Data<DbPool>, user_id: i32) -> () {
    return delete_user(pool.clone(), user_id).await;
}

pub async fn patch_user_app(pool: web::Data<DbPool>, user_id: i32, payload: PatchableUser) -> () {
    return patch_user(pool.clone(), user_id, payload).await;
}

pub async fn create_users_app(pool: web::Data<DbPool>, creatable_users: web::Json<Vec<CreatableUser>>) -> Vec<User> {
    let new_users: Vec<NewUser> = creatable_users.iter().map(|creatable_user| NewUser {
        name: creatable_user.name.to_string()
    }).collect();

    return create_users(pool, creatable_users, new_users).await;
}