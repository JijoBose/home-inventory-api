use serde::{Deserialize, Serialize};

/// House details.
#[derive(Serialize)]
pub struct House {
    pub id: String,
    pub title: String,
    pub body: String,
}

/// New House
#[derive(Deserialize)]
pub struct CreateHouse {
    pub title: String,
    pub body: String,
}

pub struct DeleteResponse {
  pub success: bool,
  pub message: String,
}


// validations
impl CreateHouse {
  pub fn validate(&self) -> Result<(), String> {
      if self.title.trim().is_empty() {
          return Err("Name is empty".to_string());
      }
      if self.body.trim().is_empty() {
          return Err("Description is empty".to_string());
      }
      Ok(())
  }
}

// impl NewHouse {
//     /// Constructs new user details from name.
//     #[cfg(test)] // only needed in tests
//     pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
//         Self { title: title.into(), body: body.into() }
//     }
// }
