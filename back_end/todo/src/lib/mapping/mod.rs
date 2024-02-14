mod todo;
mod user;
use rocket::serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
pub use todo::*;
pub use user::*;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(crate = "rocket::serde")]
struct Record<T> {
    pub id: Thing,
    #[serde(flatten)]
    pub data: T,
}

impl<T> Record<T> {
    pub fn to_record(self) -> (String, T) {
        let Record { id, data } = self;

        (id.id.to_string(), data)
    }
}
