use super::Avatars;
use super::TodoBox;
use rocket::serde::{self, Deserialize, Serialize};

// 采用Rocket框架提供给的serde进行序列化与反序列化
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct User {
    username: String,
    name: String,
    #[serde(skip_serializing)]
    password: String,
    avatar: Avatars,
    email: String,
    #[serde(rename(serialize = "teamNumber"))]
    team_number: u8,
    #[serde(rename(serialize = "todoNumber"))]
    todo_number: u16,
    #[serde(rename(serialize = "totalTodo"))]
    total_todo: u16,
    todos: TodoBox,
}
