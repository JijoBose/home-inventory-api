use std::sync::Arc;

use axum::{
  routing::{delete, get, post, patch},
  Router,
};

use crate::AppState;
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
      .route("/houses", get(all_houses))
      .route("/houses", post(create_house))
      .route("/houses/:id", get(find_house))
      .route("/houses/:id", patch(update_house))
      .route("/houses/:id", delete(delete_house))
      // Rooms
      .route("/rooms/:house_id", get(list_rooms))
      .route("/rooms", post(create_rooms))
      .with_state(app_state)
}
