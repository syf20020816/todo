use super::Avatars;
use super::Team;
use super::TodoBox;
use rocket::serde::{self, Deserialize, Serialize};

// 采用Rocket框架提供给的serde进行序列化与反序列化
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct User {
    username: String,
    name: String,
    // #[serde(skip_serializing)]
    password: String,
    avatar: Avatars,
    email: String,
    #[serde(rename(serialize = "teamNumber"))]
    #[serde(rename(deserialize = "teamNumber"))]
    team_number: u8,
    #[serde(rename(serialize = "todoNumber"))]
    #[serde(rename(deserialize = "todoNumber"))]
    todo_number: u16,
    #[serde(rename(serialize = "totalTodo"))]
    #[serde(rename(deserialize = "totalTodo"))]
    total_todo: u16,
    todos: TodoBox,
    teams: Option<Vec<Team>>,
    #[serde(rename(serialize = "sendEmail"))]
    #[serde(rename(deserialize = "sendEmail"))]
    send_email: bool,
    #[serde(rename(serialize = "sendMsg"))]
    #[serde(rename(deserialize = "sendMsg"))]
    send_msg: bool,
}

impl User {
    pub fn new() -> User {
        User::default()
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
