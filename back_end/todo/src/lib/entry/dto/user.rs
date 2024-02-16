use crate::lib::entry::vo;

use super::Avatars;
use super::Priorities;
use super::Team;
use super::Todo;
use super::TodoBox;
use rocket::serde::{self, Deserialize, Serialize};

// 采用Rocket框架提供给的serde进行序列化与反序列化
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub username: String,
    pub name: String,
    // #[serde(skip_serializing)]
    pub password: String,
    pub avatar: Avatars,
    pub email: String,
    #[serde(rename(serialize = "teamNumber"))]
    #[serde(rename(deserialize = "teamNumber"))]
    pub team_number: u8,
    #[serde(rename(serialize = "todoNumber"))]
    #[serde(rename(deserialize = "todoNumber"))]
    pub todo_number: u16,
    #[serde(rename(serialize = "totalTodo"))]
    #[serde(rename(deserialize = "totalTodo"))]
    pub total_todo: u16,
    pub todos: TodoBox,
    pub teams: Option<Vec<String>>,
    #[serde(rename(serialize = "sendEmail"))]
    #[serde(rename(deserialize = "sendEmail"))]
    pub send_email: bool,
    #[serde(rename(serialize = "sendMsg"))]
    #[serde(rename(deserialize = "sendMsg"))]
    pub send_msg: bool,
}

impl User {
    pub fn new() -> User {
        User::default()
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn quick_init(name: &str, username: &str, password: &str, email: &str) -> Self {
        User {
            username: username.to_string(),
            name: name.to_string(),
            password: password.to_string(),
            avatar: Avatars::default(),
            email: email.to_string(),
            team_number: 0,
            todo_number: 0,
            total_todo: 0,
            todos: TodoBox::default(),
            teams: None,
            send_email: false,
            send_msg: true,
        }
    }
    pub fn skip_pwd(&mut self) {
        self.password = String::new();
    }
    pub fn add_todo(&mut self, todo_id: String, priority: Priorities) {
        self.todo_number += 1;
        self.total_todo += 1;
        match priority {
            Priorities::Emergent | Priorities::High => self.todos.fatal.push(todo_id),
            Priorities::Mid => self.todos.mid.push(todo_id),
            Priorities::Low => self.todos.low.push(todo_id),
        };
    }
    pub fn delete_todo(&mut self, id: &str) {
        self.todo_number -= 1;
        self.total_todo -= 1;
        self.todos.remove(id);
    }
    pub fn complete_todo(&mut self, id: &str) {
        self.todo_number -= 1;
        // 从需要执行的TODO列表中移除
        let _ = self.todos.remove(id);
        // 将其加入到history中
        self.todos.history.push(id.to_string());
    }
    pub fn create_team(&mut self, id: &str) {
        match &mut self.teams {
            Some(teams) => teams.push(id.to_string()),
            None => {
                let _ = self.teams.replace(vec![id.to_string()]);
            }
        };
        self.team_number += 1;
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            username: Default::default(),
            name: Default::default(),
            password: Default::default(),
            avatar: Default::default(),
            email: Default::default(),
            team_number: 0,
            todo_number: 0,
            total_todo: 0,
            todos: Default::default(),
            teams: None,
            send_email: false,
            send_msg: true,
        }
    }
}
