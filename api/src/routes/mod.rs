use std::sync::Arc;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::handlers::base::{get_houses_web, root_path};
use crate::handlers::house::{all_houses, create_house, delete_house, find_house, update_house};
use crate::handlers::room::{create_rooms, list_rooms};
use crate::AppState;

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
