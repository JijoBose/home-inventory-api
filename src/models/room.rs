use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Room details.
#[derive(Serialize, Debug, PartialEq)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub house_id: String,
}

#[derive(Deserialize)]
pub struct RoomQuery {
    pub house_id: Uuid,
}

/// New room details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewRoom {
    pub name: String,
    pub house_id: String,
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
