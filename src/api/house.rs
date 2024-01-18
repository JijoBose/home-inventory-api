use std::sync::Arc;

use crate::{database::house::Entity as HouseEntity, AppState};
use crate::database::house;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{
  http::StatusCode,
  Json, Extension, extract::Path,
};
use serde_json::json;
use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait};
use uuid::Uuid;

use crate::models::house::{House, CreateHouse};


pub async fn all_houses(
  State(database): State<Arc<AppState>>,
) -> Result<Json<Vec<House>>, StatusCode> {

    let list_houses = HouseEntity::find()
        .all(&database.db)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_house| House {
          id: db_house.id.to_string(),
          title: db_house.title,
          body: db_house.body,
        })
        .collect();

    Ok(Json(list_houses))
}

pub async fn create_house(
  State(database): State<Arc<AppState>>,
  Json(house_params): Json<CreateHouse>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

  let new_house = house::ActiveModel {
    id: Set(Uuid::new_v4().to_string()),
    title: Set(house_params.title),
    body: Set(house_params.body),
    ..Default::default()
  };

  let result = new_house
    .save(&database.db)
    .await;

  match result {
    Ok(_) => {
      let house_response = json!({ "status": "success" });

      return Ok((StatusCode::CREATED, Json(house_response)));
    }
    Err(e) => {
      if e.to_string()
          .contains("duplicate key value violates unique constraint")
      {
          let error_response = serde_json::json!({
              "status": "fail",
              "message": "Note with that title already exists",
          });
          return Err((StatusCode::CONFLICT, Json(error_response)));
      }
      return Err((
          StatusCode::INTERNAL_SERVER_ERROR,
          Json(json!({"status": "error","message": format!("{:?}", e)})),
      ));
  }

  }
}

pub async fn find_house(
  Extension(database): Extension<DatabaseConnection>,
  Path(house_id): Path<Uuid>
) -> Result<Json<House>, StatusCode> {
  let house_id = house_id.to_owned();

  let house = HouseEntity::find_by_id(house_id)
    .one(&database)
    .await
    .unwrap();

  if let Some(house) = house {
    Ok(Json(House {
      id: house.id.to_string(),
      title: house.title,
      body: house.body
    }))
  } else {
    Err(StatusCode::NOT_FOUND)
  }
}

pub async fn delete_house(
  Extension(database): Extension<DatabaseConnection>,
  Path(house_id): Path<Uuid>
) -> Result<(), StatusCode> {
  let house_id = house_id.to_owned();
  HouseEntity::delete_by_id(house_id)
    .exec(&database)
    .await
    .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

  Ok(())
}
