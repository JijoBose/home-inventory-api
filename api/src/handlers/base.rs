use askama::Template;
use axum::{extract::State, response::IntoResponse, Form};
use sea_orm::Set;
use std::sync::Arc;
use uuid::Uuid;

use crate::AppState;
use entity::house::ActiveModel as HouseActiveModel;
use entity::house::Entity as HouseEntity;
use entity::house::Model as HouseModel;
use sea_orm::{ActiveModelTrait, EntityTrait};

pub async fn root_path() -> impl IntoResponse {
    HelloTemplate
}

pub async fn get_houses_web(State(database): State<Arc<AppState>>) -> impl IntoResponse {
    let list_houses = HouseEntity::find().all(&database.db).await.unwrap();

    Records { houses: list_houses }
}

pub async fn create_house_web(
    State(database): State<Arc<AppState>>,
    Form(form): Form<HouseModel>,
) -> impl IntoResponse {
    let new_house = HouseActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        title: Set(form.title),
        body: Set(form.body),
        ..Default::default()
    };
    let insert_response = new_house.insert(&database.db).await.unwrap();
    HouseNewTemplate { house: insert_response };
}

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate;

#[derive(Template)]
#[template(path = "houses.html")]
struct Records {
    houses: Vec<HouseModel>,
}

#[derive(Template)]
#[template(path = "house.html")]
struct HouseNewTemplate {
    house: HouseModel,
}
