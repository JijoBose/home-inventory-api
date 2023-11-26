use actix_web::{middleware::Logger, web::{self, ServiceConfig}, App, HttpServer};
use shuttle_actix_web::ShuttleActixWeb;

pub mod app;
pub mod schema;

use app::api::home::{
  all_homes,
  add_home,
  find_home
};

use app::db::{
  initialize_db_pool,
  initial_migration
};

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
