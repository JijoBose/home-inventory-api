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
        // Web Endpoints
        .route("/", get(root_path))
        .route("/houses", get(get_houses_web))
        // Houses API Endpoints
        .route("/api/houses", get(all_houses))
        .route("/api/houses", post(create_house))
        .route("/api/houses/:id", get(find_house))
        .route("/api/houses/:id", patch(update_house))
        .route("/api/houses/:id", delete(delete_house))
        .route("/api/houses/:id/rooms", get(list_rooms))
        // Rooms API Endpoints
        .route("/api/rooms", post(create_rooms))
        .with_state(app_state)
}
