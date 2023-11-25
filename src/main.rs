use std::error::Error;

use actix_web::{middleware::Logger, web::{self, ServiceConfig}, App, HttpServer};
use diesel::{prelude::*, r2d2, sqlite::Sqlite};
use shuttle_actix_web::ShuttleActixWeb;
use diesel_migrations::{EmbeddedMigrations, embed_migrations, MigrationHarness};

mod api;
mod actions;
mod model;
mod schema;

use api::home::{
  all_homes,
  add_home,
  find_home
};

type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

fn run_migrations(connection: &mut impl MigrationHarness<Sqlite>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
  connection.run_pending_migrations(MIGRATIONS)?;
  Ok(())
}

fn initial_migration() {
  let sqlite_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
  let mut connection = SqliteConnection::establish(&sqlite_spec).expect("Failed to establish connection");
  let _ = run_migrations(&mut connection);
}

#[cfg(debug_assertions)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let pool = initialize_db_pool();

    log::info!("starting HTTP server at http://localhost:8080");
    initial_migration();

    HttpServer::new(move || {
      let logger = Logger::default();
      App::new()
        .app_data(web::Data::new(pool.clone()))
        .wrap(logger)
        .service(all_homes)
        .service(add_home)
        .service(find_home)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(not(debug_assertions))]
#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
  dotenvy::dotenv().ok();
  let pool = initialize_db_pool();

  initial_migration();

  let config = move |cfg: &mut ServiceConfig| {
    cfg.app_data(web::Data::new(pool.clone()));
    cfg.service(all_homes);
    cfg.service(add_home);
    cfg.service(find_home);
  };
  Ok(config.into())
}

/// Initialize database connection pool based on `DATABASE_URL` environment variable.
///
/// See more: <https://docs.rs/diesel/latest/diesel/r2d2/index.html>.
fn initialize_db_pool() -> DbPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(conn_spec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
}
