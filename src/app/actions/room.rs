use actix_web::web::Json;
use diesel::prelude::*;
use uuid::Uuid;

use crate::app::models::room;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn insert_new_room(
    conn: &mut PgConnection,
    form: &Json<room::NewRoom>,
) -> Result<room::Room, DbError> {
    use crate::schema::rooms::dsl::*;

    match form.validate() {
        Ok(_) => {
            let new_room = room::Room {
                id: Uuid::new_v4().to_string(),
                name: form.name.to_owned(),
                home_id: form.home_id.to_owned(),
            };

            diesel::insert_into(rooms).values(&new_room).execute(conn)?;

            Ok(new_room)
        }
        Err(error) => Err(DbError::from(error)),
    }
}

pub fn delete_room(conn: &mut PgConnection, uid: Uuid) -> Result<String, DbError> {
    use crate::schema::rooms::dsl::*;

    let result = diesel::delete(rooms.filter(id.eq(uid.to_string()))).execute(conn);

    match result {
        Ok(_) => Ok("Success".to_string()),
        Err(e) => Err(DbError::from(e)),
    }
}
