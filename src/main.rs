mod user_table;
mod expense_table;
use user_table::prelude::User;
use expense_table::prelude::Expense;

// Import the needed modules for table creation
use sea_orm::{ConnectionTrait, Database, Schema};
// Handle errors using the `https://crates.io/crates/anyhow` crate
use anyhow::Result;

// Convert this main function into async function

#[async_std::main]
async fn main() -> Result<()>{
	// Read the database environment from the `.env` file
	// let env_database_url: &str = include_str!("../.env").trim();
	// Split the env url
	// let split_url: Vec<&str> = env_database_url.split("=").collect();
	// Get item with the format `database_backend://username:password@localhost/database`
	// let database_url = split_url[1];
	let database_url: &str = "postgres://postgres:password@localhost/hcount";
	
	let db = Database::connect(database_url)
		.await
		.expect("Database connection failed");
	
	let builder = db.get_database_backend();
	let schema = Schema::new(builder);
	
	let create_table_op =  db.execute(builder.build(&schema.create_table_from_entity(User))).await;
	let create_table_op =  db.execute(builder.build(&schema.create_table_from_entity(Expense))).await;
	println!("`CREATE TABLE User` {:?}", 
		match create_table_op {
			Ok(_) => "Operation Successful".to_owned(),
			Err(e) => format!("Unsuccessful - Error {:?}", e),
		}
	);
	Ok(())
}
