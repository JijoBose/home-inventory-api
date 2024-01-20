use std::sync::Arc;

use crate::database::house;
use crate::{database::house::Entity as HouseEntity, AppState};
use axum::extract::State;
use axum::{extract::Path, http::StatusCode, Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
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
) -> Result<Json<House>, StatusCode> {
    let new_house = house::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
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
            let status_code = match db_err {
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            Err(status_code)
        }
    }
}

// Todo - Database implementation required
pub async fn find_house(
    Extension(database): Extension<DatabaseConnection>,
    Path(house_id): Path<Uuid>,
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
            body: house.body,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

// Todo - Database implementation required
pub async fn delete_house(
    Extension(database): Extension<DatabaseConnection>,
    Path(house_id): Path<Uuid>,
) -> Result<(), StatusCode> {
    let house_id = house_id.to_owned();
    HouseEntity::delete_by_id(house_id)
        .exec(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
