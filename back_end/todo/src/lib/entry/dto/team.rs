use std::sync::Arc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::lib::mapping::select_todo_record_by_id;

use super::Date;
use super::TeamAvatars;
use super::User;
use rocket::futures::lock::Mutex;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Team {
    name: String,
    members: Vec<String>,
    owner: String,
    avatar: TeamAvatars,
    description: String,
    date: String,
}

impl Team {
    pub fn new_rand(name: &str, owner: &str) -> Self {
        let date = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        Team {
            name: name.to_string(),
            members: vec![owner.to_string()],
            owner: owner.to_string(),
            avatar: TeamAvatars::Team1,
            description: String::new(),
            date: date.to_string(),
        }
    }
    pub async fn get(id: &str) -> Option<(String, Team)> {
        select_todo_record_by_id(id).await
    }
}

impl Default for Team {
    fn default() -> Self {
        Self {
            name: String::new(),
            members: Default::default(),
            owner: Default::default(),
            avatar: Default::default(),
            description: String::new(),
            date: Default::default(),
        }
    }
}
