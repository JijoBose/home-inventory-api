use actix_web::web::Json;
use diesel::prelude::*;
use uuid::Uuid;

use crate::models::{Home, NewHome};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_all_homes(conn: &mut SqliteConnection) -> Result<Vec<Home>, DbError> {
    use crate::schema::homes::dsl::*;

    let get_homes = homes.load::<Home>(conn)?;
    Ok(get_homes)
}

/// Run query using Diesel to find user by uid and return it.
// pub fn find_home_by_uid(
//     conn: &mut SqliteConnection,
//     uid: Uuid,
// ) -> Result<Option<Home>, DbError> {
//     use crate::schema::homes::dsl::*;

//     let home = homes
//         .filter(id.eq(uid.to_string()))
//         .first::<Home>(conn)
//         .optional()?;

//     Ok(home)
// }

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_home(
    conn: &mut SqliteConnection,
    form: &Json<NewHome>,
) -> Result<Home, DbError> {
    use crate::schema::homes::dsl::*;

    let new_home = Home {
        id: Uuid::new_v4().to_string(),
        title: form.title.to_owned(),
        body: form.body.to_owned()
    };

    diesel::insert_into(homes).values(&new_home).execute(conn)?;

    Ok(new_home)
}
