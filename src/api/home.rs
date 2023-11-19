use actix_web::{web, get, post, Result, Responder, error, HttpResponse};
use diesel::{SqliteConnection, r2d2};
use uuid::Uuid;

use crate::actions;
use crate::model::home;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[get("/home")]
async fn all_homes(pool: web::Data<DbPool>) -> Result<impl Responder> {
    let all_homes = web::block(move || {
      let mut conn = pool.get()?;
      actions::find_all_homes(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(all_homes))
}

#[get("/home/{home_id}")]
async fn find_home(pool: web::Data<DbPool>, home_id: web::Path<Uuid>) -> Result<impl Responder> {
  let home_uid = home_id.into_inner();
  let home = web::block(move || {
    let mut conn = pool.get()?;
    actions::find_home_by_uid(&mut conn, home_uid)
  })
  .await?
  .map_err(error::ErrorInternalServerError)?;

  Ok(match home {
    Some(home) => HttpResponse::Ok().json(home),
    None => HttpResponse::NotFound().body(format!("No Home found with UID: {home_uid}")),
  })
}

#[post("/home")]
async fn add_home(
  pool: web::Data<DbPool>,
  form: web::Json<home::NewHome>,
) -> Result<impl Responder> {
  let home = web::block(move || {
    let mut conn = pool.get()?;
    actions::insert_new_home(&mut conn, &form)
  })
  .await?
  .map_err(error::ErrorInternalServerError)?;

  Ok(HttpResponse::Created().json(home))
}
