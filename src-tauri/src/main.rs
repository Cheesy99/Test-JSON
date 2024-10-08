#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database; 

use database::create_schema;
use sqlx::{ Sqlite, SqlitePool, migrate::MigrateDatabase};
#[tokio::main]
async fn main() {
   
   let db_url = String::from("sqlite://sqlite.db");
   if !Sqlite::database_exists(&db_url).await.unwrap_or(false){  
       Sqlite::create_database(&db_url).await.unwrap(); 
       match create_schema(&db_url).await {  
           Ok(_) => println!("database created succesfully"),
           Err(e) => panic!("{}", e)
       }
   }
   let instances = SqlitePool::connect(&db_url).await.unwrap(); 
   let qry = "INSERT INTO settings (description) VALUES($1)";
  // let result = sqlx::query(&qry).bind("testing").execute(&instances).await;

   instances.close().await;

tauri::Builder::default()
   .run(tauri::generate_context!())
   .expect("error while running tauri application");
}