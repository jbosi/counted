pub mod models;
pub mod schema;
pub mod user_repository;

#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
 

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

// fn main() {
	// let connection = establish_connection();
	// create_user(&connection, "testNameB", &1.0);
	// delete_user(&connection, 1);
	// get_users(&connection);
// }

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
pub fn rocket() -> _ {
    rocket::build()
		.mount("/", routes![
			index,
			// create_user,
			// get_users,
			// update_user_amount,
			// delete_user
		])
}

