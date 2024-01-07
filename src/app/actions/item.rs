use actix_web::web::Json;
use diesel::prelude::*;
use uuid::Uuid;

use crate::app::models::item;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn list_items(conn: &mut PgConnection, uid: Uuid) -> Result<Vec<item::Item>, DbError> {
    use crate::schema::items::dsl::*;

    let item_list = items.filter(room_id.eq(uid.to_string())).load(conn)?;

    Ok(item_list)
}

pub fn insert_new_item(
    conn: &mut PgConnection,
    form: &Json<item::NewItem>,
) -> Result<item::Item, DbError> {
    use crate::schema::items::dsl::*;

    match form.validate() {
        Ok(_) => {
            let new_item = item::Item {
                id: Uuid::new_v4().to_string(),
                room_id: form.room_id.to_owned(),
                name: form.name.to_owned(),
                description: form.description.to_owned(),
                category: form.category.to_owned(),
                purchase_date: form.purchase_date.to_owned(),
                expiry_date: form.expiry_date.to_owned(),
                value: form.value.to_owned(),
            };

            diesel::insert_into(items).values(&new_item).execute(conn)?;

            Ok(new_item)
        }
        Err(error) => Err(DbError::from(error)),
    }
}

pub fn delete_item(conn: &mut PgConnection, uid: Uuid) -> Result<String, DbError> {
    use crate::schema::items::dsl::*;

    let result = diesel::delete(items.filter(id.eq(uid.to_string()))).execute(conn);

    match result {
        Ok(_) => Ok("Success".to_string()),
        Err(e) => Err(DbError::from(e)),
    }
}
