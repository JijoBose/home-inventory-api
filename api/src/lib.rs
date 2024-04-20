use anyhow::Ok;
use axum::serve;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;
use tokio::net::TcpListener;

pub mod handlers;
pub mod models;
pub mod routes;

pub struct AppState {
    db: DatabaseConnection,
}

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let database_uri = dotenvy::var("DATABASE_URL")?;
    let db_connection = Database::connect(database_uri)
        .await
        .expect("Database connection failed");

    Migrator::up(&db_connection, None).await?;

    // Initialize tracing
    tracing_subscriber::fmt::init();

    let app = routes::create_routes(Arc::new(AppState {
        db: db_connection.clone(),
    }));

    let listener = TcpListener::bind(&"0.0.0.0:3000").await.unwrap();
    serve(listener, app).await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
