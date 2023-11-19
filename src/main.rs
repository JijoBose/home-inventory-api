use actix_web::{error, get, middleware, post, web, App, HttpResponse, HttpServer, Responder, Result};
use diesel::{prelude::*, r2d2};


mod actions;
mod models;
mod schema;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[get("/home")]
async fn all_homes(pool: web::Data<DbPool>) -> Result<impl Responder> {
    let all_homes = web::block(move || {
      let mut conn = pool.get()?;
      actions::find_all_homes(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(all_homes))
}

#[post("/home")]
async fn add_home(
  pool: web::Data<DbPool>,
  form: web::Json<models::NewHome>,
) -> Result<impl Responder> {
  let home = web::block(move || {
    let mut conn = pool.get()?;
    actions::insert_new_home(&mut conn, &form)
  })
  .await?
  .map_err(error::ErrorInternalServerError)?;

  Ok(HttpResponse::Created().json(home))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // initialize DB pool outside of `HttpServer::new` so that it is shared across all workers
    let pool = initialize_db_pool();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            // add DB pool handle to app data; enables use of `web::Data<DbPool>` extractor
            .app_data(web::Data::new(pool.clone()))
            // add request logger middleware
            .wrap(middleware::Logger::default())
            // add route handlers
            .service(all_homes)
            .service(add_home)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
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
