use actix_web::{error, get, post, delete, web, HttpResponse, Responder, Result};
use diesel::{r2d2, PgConnection};
use uuid::Uuid;

use crate::app::actions;
use crate::app::models;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

#[post("/item")]
async fn add_item(
    pool: web::Data<DbPool>,
    form: web::Json<models::item::NewItem>,
) -> Result<impl Responder> {
    let response = web::block(move || {
        let mut conn = pool.get()?;
        actions::item::insert_new_item(&mut conn, &form)
    })
    .await?
    .map_err(error::ErrorBadRequest)?;

    Ok(HttpResponse::Created().json(response))
}

#[get("/item")]
async fn get_items(
  pool: web::Data<DbPool>,
  query: web::Query<models::item::ItemQuery>,
) -> Result<impl Responder> {
  let room_uid = query.room_id;

  let response = web::block(move || {
    let mut conn = pool.get()?;
    actions::item::list_items(&mut conn, room_uid)
  })
  .await?
  .map_err(error::ErrorBadRequest)?;

  Ok(HttpResponse::Ok().json(response))
}

#[delete("/item/{item_id}")]
async fn delete_item(
  pool: web::Data<DbPool>,
  item_id: web::Path<Uuid>,
) -> Result<impl Responder> {

  let item_id = item_id.into_inner();

  let item = web::block(move || {
    let mut conn = pool.get()?;
    actions::item::delete_item(&mut conn, item_id)
  })
  .await?
  .map_err(error::ErrorBadRequest)?;

  Ok(HttpResponse::Created().json(item))
}
