use diesel::prelude::*;
use diesel::{prelude::Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::rooms;
use crate::app::models::home::Home;

/// Room details.
#[derive(Queryable, Serialize, Selectable, Identifiable, Associations, Debug, PartialEq, Insertable)]
#[diesel(belongs_to(Home))]
#[diesel(table_name = rooms)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub home_id: String,
}

/// New room details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewRoom {
    pub name: String,
    pub home_id: String,
}

// validations
impl NewRoom {
  pub fn validate(&self) -> Result<(), String> {
      if self.name.trim().is_empty() {
          return Err("Name is empty".to_string());
      }
      Ok(())
  }
}
