use crate::database::house;
use crate::database::house::Entity as HouseEntity;

use axum::{
  http::StatusCode,
  Json, Extension, extract::Path,
};
use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait};
use uuid::Uuid;

use crate::models::house::{House, CreateHouse};


pub async fn all_houses(
  Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<Vec<House>>, StatusCode> {

    let list_houses = HouseEntity::find()
        .all(&database)
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
  Extension(database): Extension<DatabaseConnection>,
  Json(house_params): Json<CreateHouse>,
) -> Result<(), StatusCode> {

  let new_house = house::ActiveModel {
    id: Set(Uuid::new_v4().to_string()),
    title: Set(house_params.title),
    body: Set(house_params.body),
    ..Default::default()
  };

  let _result = new_house
    .save(&database)
    .await
    .unwrap();

  Ok(())
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
      id: house.id,
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
