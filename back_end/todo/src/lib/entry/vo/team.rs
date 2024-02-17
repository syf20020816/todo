use crate::lib::{
    entry::dto::{self, TeamAvatars},
    mapping::select_todo_record,
};

use super::{Todo, User};

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
    pub todos: Vec<Todo>,
}

// impl From<dto::Team> for Team {
//     fn from(value: dto::Team) -> Self {
//         Team {
//             id: String::new(),
//             name: value.name,
//             members: Vec::new(),
//             owner: value.owner,
//             avatar: value.avatar,
//             description: value.description,
//             date: value.date,
//             todos: ,

//         }
//     }
// }

impl Team {
    pub fn set_id(&mut self, id: &str) -> () {
        self.id = String::from(id);
    }
    pub fn set_members(&mut self, members: Vec<User>) {
        self.members = members;
    }
    pub async fn from(value: dto::Team) -> Self {
        let mut todos = Vec::new();
        for todo_id in value.todos {
            let query = select_todo_record(&todo_id).await;
            if let Some((id, todo)) = query {
                let todo = Todo::from(todo, id).await;

                todos.push(todo);
            }
        }
        Team {
            id: String::new(),
            name: value.name,
            members: Vec::new(),
            owner: value.owner,
            avatar: value.avatar,
            description: value.description,
            date: value.date,
            todos,
        }
    }
}
