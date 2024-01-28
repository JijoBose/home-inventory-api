use std::sync::Arc;

use crate::AppState;
use axum::extract::State;
use axum::{extract::Path, http::StatusCode, Json};
use entity::house;
use entity::house::Entity as HouseEntity;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use uuid::Uuid;

use crate::models::house::{CreateHouse, House};

pub async fn all_houses(
    State(database): State<Arc<AppState>>,
) -> Result<Json<Vec<House>>, StatusCode> {
    let list_houses = HouseEntity::find()
        .all(&database.db)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_house| House {
            id: db_house.id,
            title: db_house.title,
            body: db_house.body,
        })
        .collect();

    Ok(Json(list_houses))
}

pub async fn update_house(
    State(database): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(house_params): Json<CreateHouse>,
) -> Result<Json<House>, StatusCode> {
    let update_house = house::ActiveModel {
        id: Set(id),
        title: Set(house_params.title),
        body: Set(house_params.body),
    };

    match update_house.update(&database.db).await {
        Ok(updated_house) => {
            let response_json = Json(House {
                id: updated_house.id,
                title: updated_house.title,
                body: updated_house.body,
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

pub async fn create_house(
    State(database): State<Arc<AppState>>,
    Json(house_params): Json<CreateHouse>,
) -> Result<Json<House>, StatusCode> {
    let new_house = house::ActiveModel {
        id: Set(Uuid::new_v4()),
        title: Set(house_params.title),
        body: Set(house_params.body),
        ..Default::default()
    };

    match new_house.insert(&database.db).await {
        Ok(inserted_house) => {
            let response_json = Json(House {
                id: inserted_house.id,
                title: inserted_house.title,
                body: inserted_house.body,
            });

            Ok(response_json)
        }
        Err(db_err) => {
            println!("{}", db_err);
            let status_code = match db_err {
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            Err(status_code)
        }
    }
}

pub async fn find_house(
    State(database): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<House>, StatusCode> {
    let id = id.to_owned();

    let house = HouseEntity::find_by_id(id).one(&database.db).await.unwrap();

    if let Some(house) = house {
        Ok(Json(House {
            id: house.id,
            title: house.title,
            body: house.body,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn delete_house(
    State(database): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<(), StatusCode> {
    let id = id.to_owned();
    let _house = HouseEntity::delete_by_id(id)
        .exec(&database.db)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR);

    Ok(())
}
