use std::sync::Arc;

use axum::{
  routing::{delete, get, post, patch},
  Router,
};

use crate::AppState;
use crate::handlers::base::{
  root_path,
  get_houses_web
};
use crate::handlers::house::{
  all_houses,
  create_house,
  find_house,
  update_house,
  delete_house
};
use crate::handlers::room::{
  list_rooms,
  create_rooms
};

pub fn create_routes(app_state: Arc<AppState>) -> Router {

  Router::new()
      .route("/", get(root_path))
      .route("/houses", get(get_houses_web))
      // .route("/houses", post(create_house_web)) // Todo
      // API Endpoints
      .route("/api/houses", get(all_houses))
      .route("/api/houses", post(create_house))
      .route("/api/houses/:id", get(find_house))
      .route("/api/houses/:id", patch(update_house))
      .route("/api/houses/:id", delete(delete_house))
      .route("/api/rooms/:house_id", get(list_rooms))
      .route("/api/rooms", post(create_rooms))
      .with_state(app_state)
}
