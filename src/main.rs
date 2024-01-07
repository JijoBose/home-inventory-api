use actix_web::{middleware::Logger, web::{self}, App, HttpServer};

pub mod app;
pub mod schema;

use app::api::home::{
  all_homes,
  add_home,
  find_home,
  delete_home
};

use app::api::room::{add_room, get_room};
use app::api::item::{add_item, get_items};

use app::db::{
  initialize_db_pool,
  initial_migration
};

// #[cfg(debug_assertions)]
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
        .service(delete_home)
        .service(add_room)
        .service(get_room)
        .service(get_items)
        .service(add_item)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
