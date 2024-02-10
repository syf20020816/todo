use super::Date;
use super::TeamAvatars;
use super::User;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Team {
    name: String,
    members: Vec<User>,
    owner: User,
    avatar: TeamAvatars,
    description: String,
    date: Date,
}

impl Default for Team {
    fn default() -> Self {
        Self {
            name: String::new(),
            members: Default::default(),
            owner: Default::default(),
            avatar: Default::default(),
            description: String::new(),
            date: Date::default(),
        }
    }
}
