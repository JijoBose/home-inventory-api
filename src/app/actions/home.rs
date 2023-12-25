use actix_web::web::Json;
use diesel::prelude::*;
use uuid::Uuid;

use crate::app::models::home;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_all_homes(conn: &mut PgConnection) -> Result<Vec<home::Home>, DbError> {
    use crate::schema::homes::dsl::*;

    let get_homes = homes.load::<home::Home>(conn)?;
    Ok(get_homes)
}

pub fn find_home_by_uid(conn: &mut PgConnection, uid: Uuid) -> Result<Option<home::Home>, DbError> {
    use crate::schema::homes::dsl::*;

    let home = homes
        .filter(id.eq(uid.to_string()))
        .first::<home::Home>(conn)
        .optional()?;

    Ok(home)
}

pub fn insert_new_home(
    conn: &mut PgConnection,
    form: &Json<home::NewHome>,
) -> Result<home::Home, DbError> {
    use crate::schema::homes::dsl::*;

    match form.validate() {
        Ok(_) => {
            let new_home = home::Home {
                id: Uuid::new_v4().to_string(),
                title: form.title.to_owned(),
                body: form.body.to_owned(),
            };

            diesel::insert_into(homes).values(&new_home).execute(conn)?;

            Ok(new_home)
        }
        Err(error) => Err(DbError::from(error)),
    }
}

pub fn delete_home(conn: &mut PgConnection, uid: Uuid) -> Result<String, DbError> {
    use crate::schema::homes::dsl::*;

    let result = diesel::delete(homes.filter(id.eq(uid.to_string()))).execute(conn);

    match result {
        Ok(_) => Ok("Success".to_string()),
        Err(e) => Err(DbError::from(e)),
    }
}
