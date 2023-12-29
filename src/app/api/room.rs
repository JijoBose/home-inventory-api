use actix_web::{error, post, web, HttpResponse, Responder, Result};
use diesel::{r2d2, PgConnection};

use crate::app::actions;
use crate::app::models;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

#[post("/room")]
async fn add_room(
    pool: web::Data<DbPool>,
    form: web::Json<models::room::NewRoom>,
) -> Result<impl Responder> {
    let response = web::block(move || {
        let mut conn = pool.get()?;
        actions::room::insert_new_room(&mut conn, &form)
    })
    .await?
    .map_err(error::ErrorBadRequest)?;

    Ok(HttpResponse::Created().json(response))
}
