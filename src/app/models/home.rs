use diesel::{prelude::Insertable, Queryable, associations::Identifiable};
use serde::{Deserialize, Serialize};

use crate::schema::homes;

/// User details.
#[derive(Debug, Clone, PartialEq, Identifiable, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = homes)]
pub struct Home {
    pub id: String,
    pub title: String,
    pub body: String,
}

/// New user details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewHome {
    pub title: String,
    pub body: String,
}

// validations
impl NewHome {
  pub fn validate(&self) -> Result<(), String> {
      if self.title.trim().is_empty() {
          return Err("Name is empty".to_string());
      }
      if self.body.trim().is_empty() {
          return Err("Desciption is empty".to_string());
      }
      Ok(())
  }
}

// impl NewHome {
//     /// Constructs new user details from name.
//     #[cfg(test)] // only needed in tests
//     pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
//         Self { title: title.into(), body: body.into() }
//     }
// }
