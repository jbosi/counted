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
	// create_user(&connection, "testName2", &100.2);
	// delete_user(&connection, 1);
}


pub fn create_user(conn: &PgConnection, name: &str, balance: &f64) -> () {
	use schema::users;

    let new_user = NewUser {
        name: name.to_string(),
        balance: *balance
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new post");
}

pub fn delete_user(conn: &PgConnection, id_to_delete: i32) -> () {
	use schema::users::dsl::*;

	diesel::delete(users.filter(id.eq_all(id_to_delete)))
		.execute(conn)
		.expect("Error deleteing user");
}