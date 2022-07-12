pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use crate::models::{User, NewUser};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
	let connection = establish_connection();
	// create_user(&connection, "testNameB", &1.0);
	// delete_user(&connection, 1);
	get_users(&connection);
}


pub fn create_user(conn: &PgConnection, name: &str, balance: &f64) -> () {
	use schema::users;

    let new_user = NewUser {
        name: name.to_string(),
        balance: Some(*balance)
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new post");
}

pub fn get_users(conn: &PgConnection) -> Vec<User> {
	use schema::users::dsl::*;

	let results = users.load::<User>(conn)
		.expect("Error while trying to get Users");
	
	for result in &results {
		println!("{}", result.name)
	}

	return results
}

pub fn update_user_amount(conn: &PgConnection, user_id: i32, amount: f64) -> () {
	use schema::users::dsl::{users, balance};

	diesel::update(users.find(user_id))
		.set(balance.eq(amount))
		.execute(conn)
		.expect("Error while updating user amount");
}

pub fn delete_user(conn: &PgConnection, id_to_delete: i32) -> () {
	use schema::users::dsl::*;

	diesel::delete(users.find(id_to_delete))
		.execute(conn)
		.expect("Error deleteing user");
}