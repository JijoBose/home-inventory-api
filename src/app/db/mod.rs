use std::error::Error;

use diesel::{r2d2, PgConnection, Connection, pg::Pg};
use diesel_migrations::{EmbeddedMigrations, embed_migrations, MigrationHarness};


pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub fn initialize_db_pool() -> DbPool {
  let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
  let manager = r2d2::ConnectionManager::<PgConnection>::new(conn_spec);
  r2d2::Pool::builder()
      .build(manager)
      .expect("database URL should be valid path to PG DB")
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn run_migrations(connection: &mut impl MigrationHarness<Pg>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
  connection.run_pending_migrations(MIGRATIONS)?;
  Ok(())
}

pub fn initial_migration() {
  let pg_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
  let mut connection = PgConnection::establish(&pg_spec).expect("Failed to establish connection");
  let _ = run_migrations(&mut connection);
}
