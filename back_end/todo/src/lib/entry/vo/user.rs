use crate::lib::{
    entry::dto::{self, Avatars},
    mapping::{select_team_record_by_id, select_user_by_username},
};
use rocket::serde::{Deserialize, Serialize};

use super::{todo::TodoBox, Team, Todo};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserPersonalSetting {
    name: String,
    email: String,
    #[serde(rename(serialize = "sendEmail"))]
    #[serde(rename(deserialize = "sendEmail"))]
    send_email: bool,
    #[serde(rename(serialize = "sendMsg"))]
    #[serde(rename(deserialize = "sendMsg"))]
    send_msg: bool,
}

impl UserPersonalSetting {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn send_email(&self) -> bool {
        self.send_email
    }
    pub fn send_msg(&self) -> bool {
        self.send_msg
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct User {
    username: String,
    name: String,
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
    todos: Option<TodoBox>,
    teams: Option<Vec<Team>>,
    #[serde(rename(serialize = "sendEmail"))]
    #[serde(rename(deserialize = "sendEmail"))]
    send_email: bool,
    #[serde(rename(serialize = "sendMsg"))]
    #[serde(rename(deserialize = "sendMsg"))]
    send_msg: bool,
}

impl User {
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn easy_from(user: dto::User) -> Self {
        User {
            username: user.username,
            name: user.name,
            avatar: user.avatar,
            email: user.email,
            team_number: user.team_number,
            todo_number: user.todo_number,
            total_todo: user.total_todo,
            todos: None,
            teams: None,
            send_email: user.send_email,
            send_msg: user.send_msg,
        }
    }

    pub async fn from(value: dto::User) -> Self {
        let todos = TodoBox::from(value.todos).await;

        let teams = match value.teams {
            Some(teams) => {
                let mut team_vos = Vec::new();
                for team_id in teams {
                    let (id, team) = select_team_record_by_id(&team_id).await.unwrap();
                    let members = team.members();
                    let mut team = Team::from(team);
                    let _ = team.set_id(&id);
                    let mut convert_members = Vec::new();
                    for member in members {
                        let user = select_user_by_username(&member).await.unwrap();
                        let user = User::easy_from(user);
                        convert_members.push(user);
                    }

                    let _ = team.set_members(convert_members);
                    team_vos.push(team);
                }
                Some(team_vos)
            }
            None => None,
        };

        // let teams = value.teams.unwrap();

        User {
            username: value.username,
            name: value.name,
            avatar: value.avatar,
            email: value.email,
            team_number: value.team_number,
            todo_number: value.todo_number,
            total_todo: value.total_todo,
            todos: Some(todos),
            teams,
            send_email: value.send_email,
            send_msg: value.send_msg,
        }
    }
}
