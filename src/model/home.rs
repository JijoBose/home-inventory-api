use diesel::{Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::homes;

/// User details.
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = homes)]
pub struct Home {
    pub id: String,
    pub title: String,
    pub body: String
}

/// New user details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewHome {
    pub title: String,
    pub body: String,
}

// impl NewHome {
//     /// Constructs new user details from name.
//     #[cfg(test)] // only needed in tests
//     pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
//         Self { title: title.into(), body: body.into() }
//     }
// }
