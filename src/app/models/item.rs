use diesel::prelude::*;
use diesel::{prelude::Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::items;
use crate::app::models::room::Room;

/// Item details.
#[derive(Queryable, Serialize, Selectable, Identifiable, Associations, Debug, PartialEq, Insertable)]
#[diesel(belongs_to(Room))]
#[diesel(table_name = items)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub room_id: String,
    pub description: Option<String>,
    pub category: String,
    pub purchase_date: String,
    pub expiry_date: Option<String>,
    pub value: f64
}

/// New Item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewItem {
    pub name: String,
    pub room_id: String,
    pub description: Option<String>,
    pub category: String,
    pub purchase_date: String,
    pub expiry_date: Option<String>,
    pub value: f64
}

#[derive(Deserialize)]
pub struct ItemQuery {
    pub room_id: Uuid,
}

// validations
impl NewItem {
  pub fn validate(&self) -> Result<(), String> {
      if self.name.trim().is_empty() {
          return Err("Name is empty".to_string());
      }
      Ok(())
  }
}
