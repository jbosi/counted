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
	create_user(&connection, &"testName".to_string(), &105.2);
}


pub fn create_user<'a>(conn: &PgConnection, name: &'a String, balance: &'a f64) -> () {
	use schema::users;

    let new_user = NewUser {
        name,
        balance
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new post");
}