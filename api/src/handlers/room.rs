use std::sync::Arc;

use crate::AppState;
use axum::extract::State;
use axum::{extract::Path, http::StatusCode, Json};
use entity::room;
use entity::room::Entity as RoomEntity;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::models::room::{CreateRoom, Room};

pub async fn list_rooms(
    State(database): State<Arc<AppState>>,
    Path(house_id): Path<Uuid>,
) -> Result<Json<Vec<Room>>, StatusCode> {
    let house_id = house_id.to_owned();
    let list_rooms = RoomEntity::find()
        .filter(room::Column::HouseId.eq(house_id))
        .all(&database.db)
        .await
        .map_err(|_error|
        StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_room| Room {
          id: db_room.id.to_string(),
          name: db_room.name,
          house_id: db_room.house_id.to_string(),
        })
        .collect();
    Ok(Json(list_rooms))
}

pub async fn create_rooms(
    State(database): State<Arc<AppState>>,
    Json(room_params): Json<CreateRoom>,
) -> Result<Json<Room>, StatusCode> {
    let house_uuid = Uuid::parse_str(&room_params.house_id).unwrap();
    let new_room = room::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(room_params.name),
        house_id: Set(house_uuid),
        ..Default::default()
    };

    match new_room.insert(&database.db).await {
        Ok(inserted_room) => {
            let response_json = Json(Room {
                id: inserted_room.id.to_string(),
                name: inserted_room.name,
                house_id: inserted_room.house_id.to_string(),
            });

            Ok(response_json)
        }
        Err(db_err) => {
            let status_code = match db_err {
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            Err(status_code)
        }
    }
}

pub async fn find_room(
    State(database): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Room>, StatusCode> {
    let id = id.to_owned();

    let room = RoomEntity::find_by_id(id).one(&database.db).await.unwrap();

    if let Some(room) = room {
        Ok(Json(Room {
            id: room.id.to_string(),
            name: room.name,
            house_id: room.house_id.to_string(),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
