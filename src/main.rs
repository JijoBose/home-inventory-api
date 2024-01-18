use std::sync::Arc;
use anyhow::Ok;
use axum::serve;
use sea_orm::{Database, DatabaseConnection};
use tokio::net::TcpListener;
use dotenvy_macro::dotenv;

pub mod routes;
pub mod api;
pub mod models;
pub mod database;

pub struct AppState {
  db: DatabaseConnection,
}

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let database_uri = dotenv!("DATABASE_URL");
    let db_connection = Database::connect(database_uri)
      .await
      .expect("Database connection failed");

    // Initialize tracing
    tracing_subscriber::fmt::init();

    let app = routes::create_routes(Arc::new(AppState { db: db_connection.clone() }));

    let listner = TcpListener::bind(&"0.0.0.0:3000").await.unwrap();
    serve(listner, app).await?;

    Ok(())
}

pub fn main() {
  let result = start();

  if let Some(err) = result.err() {
    println!("Error: {err}");
  }
}
