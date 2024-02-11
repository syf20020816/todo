use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub enum Annex {
    Picture,
    Audio,
    Video,
    Md,
    Other,
}
