extern crate diesel;

use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use utoipa::{
	openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
	Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

use crate::{
	projects::web::project_web::{create_project, get_projects},
	users::web::user_web::{create_users, delete_user, get_users, update_user_name},
	expenses::web::expense_web::{create_expense, delete_expense, get_expense},
	user_project_web::get_user_projects,
	payments::web::payment_web::get_payments,
	payments::domain::payment_model::Payment,
	users::domain::user_model::User
};


pub mod models;
pub mod schema;
mod query_strings;
mod user_project_web;
mod expenses;
mod users;
mod projects;
mod payments;

#[path = "../tests/user_projects/user_projects_web_test.rs"] mod user_projects_web_test;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn hello() -> impl Responder {
	HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	// let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	let manager = ConnectionManager::<PgConnection>::new("postgres://jbosi:password@localhost/hcount");

	// TODO move to dedicated file
	#[derive(OpenApi)]
	#[openapi(
		paths(
			payments::web::payment_web::get_payments,
			users::web::user_web::get_users,
			users::web::user_web::get_user,
		),
		components(
			schemas(Payment),
			schemas(User)
		),
	)]
	struct ApiDoc;
	
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
					.service(get_payments)
					// .service(get_expense_payments)
					// .service(get_users_by_project_id)
			)
			.service(
				SwaggerUi::new("/swagger-ui/{_:.*}")
					.url("/api-docs/openapi.json", ApiDoc::openapi()),
			)
	})
		.bind(("localhost", 8080))?
		.run()
		.await
}

