pub mod models;
pub mod schema;
pub mod user_web;
pub mod expense_web;

extern crate diesel;

use diesel::pg::PgConnection;
use expense_web::{create_expense, get_expense, delete_expense};
use user_web::{get_users, create_user, update_user_name, delete_user};
use std::env;
use diesel::r2d2::ConnectionManager;
 

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
	HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	let manager = ConnectionManager::<PgConnection>::new(database_url);
	
	let pool = r2d2::Pool::builder()
		.build(manager)
		.expect("Failed to create pool.");

	HttpServer::new(move || {
		App::new()
			.app_data(web::Data::new(pool.clone()))
			.service(hello)
			.service(create_user)
			.service(get_users)
			.service(update_user_name)
			.service(delete_user)
			.service(create_expense)
			.service(get_expense)
			.service(delete_expense)
	})
		.bind(("127.0.0.1", 8080))?
		.run()
		.await
}

