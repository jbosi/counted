use crate::models::{User, NewUser};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::schema;
// use rocket::serde::{Serialize, Deserialize, json::Json};

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


// #[get("/users")]
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
		.expect("Error deleting user");
}