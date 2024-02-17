use crate::lib::entry::dto::{self, TeamAvatars};

use super::User;

use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Team {
    pub id: String,
    pub name: String,
    pub members: Vec<User>,
    pub owner: String,
    pub avatar: TeamAvatars,
    pub description: String,
    pub date: String,
}

impl From<dto::Team> for Team {
    fn from(value: dto::Team) -> Self {
        Team {
            id: String::new(),
            name: value.name,
            members: Vec::new(),
            owner: value.owner,
            avatar: value.avatar,
            description: value.description,
            date: value.date,
        }
    }
}

impl Team {
    pub fn set_id(&mut self, id: &str) -> () {
        self.id = String::from(id);
    }
    pub fn set_members(&mut self, members: Vec<User>) {
        self.members = members;
    }
}
