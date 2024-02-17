use std::sync::Arc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::lib::entry::vo;
use crate::lib::mapping::select_team_record_by_id;

use super::Date;
use super::TeamAvatars;
use super::User;
use rocket::futures::lock::Mutex;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Team {
    pub name: String,
    pub members: Vec<String>,
    pub owner: String,
    pub avatar: TeamAvatars,
    pub description: String,
    pub date: String,
    pub todos: Vec<String>,
}

impl Team {
    pub fn members(&self) -> Vec<String> {
        self.members.clone()
    }
    pub fn add_member(&mut self, member: &str) {
        self.members.push(member.to_string());
    }
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
            todos: Vec::new(),
        }
    }
    pub async fn get(id: &str) -> Option<(String, Team)> {
        select_team_record_by_id(id).await
    }
    pub fn push_todo(&mut self, todo_id: &str) {
        self.todos.push(todo_id.to_string());
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
            todos: Vec::new(),
        }
    }
}

impl From<vo::Team> for Team {
    fn from(value: vo::Team) -> Self {
        let members = value
            .members
            .into_iter()
            .map(|member| member.username().to_string())
            .collect::<Vec<String>>();

        let todos = value
            .todos
            .into_iter()
            .map(|todo| todo.id().to_string())
            .collect::<Vec<String>>();
        Team {
            name: value.name,
            members,
            owner: value.owner,
            avatar: value.avatar,
            description: value.description,
            date: value.date,
            todos,
        }
    }
}
