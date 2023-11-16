pub mod models;
pub mod schema;
pub mod user_web;
// pub mod expense_web;
pub mod project_web;
mod query_strings;
mod user_project_web;
mod expenses;
#[path = "../tests/user_projects/user_projects_web_test.rs"] mod user_projects_web_test;

extern crate diesel;

use diesel::pg::PgConnection;
use project_web::{create_project, get_projects};
use user_web::{get_users, create_users, update_user_name, delete_user};
use diesel::r2d2::ConnectionManager;
 

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use crate::expenses::web::expense_web::{create_expense, delete_expense, get_expense};
use crate::user_project_web::get_user_projects;

#[get("/")]
async fn hello() -> impl Responder {
	HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	// let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	let manager = ConnectionManager::<PgConnection>::new("postgres://jbosi:password@localhost/hcount");
	
	let pool = r2d2::Pool::builder()
		.build(manager)
		.expect("Failed to create pool.");

	HttpServer::new(move || {
		App::new()
			.app_data(web::Data::new(pool.clone()))
			.service(
				web::scope("/api")
					.service(hello)
					.service(create_users)
					.service(get_users)
					.service(update_user_name)
					.service(delete_user)
					.service(create_expense)
					.service(get_expense)
					.service(delete_expense)
					.service(get_projects)
					.service(get_user_projects)
					.service(create_project)
					// .service(get_expense_payments)
					// .service(get_users_by_project_id)
		)
	})
		.bind(("localhost", 8080))?
		.run()
		.await
}

