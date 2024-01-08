use actix_web::{
    web::ServiceConfig,
    web::{self},
};
use shuttle_actix_web::ShuttleActixWeb;

pub mod app;
pub mod schema;

use app::api::home::{add_home, all_homes, delete_home, find_home};
use app::api::item::{add_item, get_items};
use app::api::room::{add_room, get_room};

use app::db::{initial_migration, initialize_db_pool};

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    dotenvy::dotenv().ok();

    let pool = initialize_db_pool();

    initial_migration();

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(web::Data::new(pool.clone()));
        // Homes
        cfg.service(all_homes);
        cfg.service(add_home);
        cfg.service(find_home);
        cfg.service(delete_home);
        // Rooms
        cfg.service(add_room);
        cfg.service(get_room);
        // Items
        cfg.service(add_item);
        cfg.service(get_items);
    };

    Ok(config.into())
}
