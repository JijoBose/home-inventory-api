use std::sync::Arc;

use axum::{
  routing::{delete, get, post},
  Router,
};

use crate::{api, AppState};
use api::house::{all_houses, create_house, find_house, delete_house};

pub fn create_routes(app_state: Arc<AppState>) -> Router {

  Router::new()
      .route("/houses", get(all_houses))
      .route("/houses", post(create_house))
      .route("/houses/:id", get(find_house))
      .route("/houses/:id", delete(delete_house))
      .with_state(app_state)
}
