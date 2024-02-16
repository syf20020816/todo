use crate::lib::{
    entry::dto::{self, Annex, Date, ITagProps, Priorities, Priority, Status},
    mapping::{select_todo_record, select_user_by_username},
};
use rocket::{
    futures::{
        executor::block_on,
        future::{join, join_all},
    },
    serde::{Deserialize, Serialize},
};

use super::User;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Todo {
    id: String,
    pub name: String,
    pub priority: Priorities,
    /// 审核人
    pub reviewers: Vec<User>,
    pub performers: Vec<User>,
    pub date: Date,
    pub tags: Vec<ITagProps>,
    pub status: Status,
    pub description: Option<String>,
    pub information: Option<String>,
    /// 附件
    pub annexs: Option<Vec<Annex>>,
    #[serde(rename(serialize = "isFocus"))]
    #[serde(rename(deserialize = "isFocus"))]
    pub is_focus: bool,
}

impl Todo {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn have_reviewers(&self) -> bool {
        !self.reviewers.is_empty()
    }
    pub fn have_performers(&self) -> bool {
        !self.performers.is_empty()
    }
    pub fn is_self_todo(&self) -> bool {
        !self.have_reviewers() && !self.have_performers()
    }
    pub fn is_team_todo(&self) -> bool {
        !self.is_self_todo()
    }
    pub fn priority(&self) -> Priorities {
        self.priority.clone()
    }
    pub async fn from(value: dto::Todo, id: String) -> Self {
        let reviewers = value.reviewers;
        let performers = value.performers;
        let reviewers = convert_usernames_to_user_instances(reviewers).await;
        let performers = convert_usernames_to_user_instances(performers).await;
        // let result = join(reviewers_f, performer_f);
        // let users = block_on(result);
        let reviewers = reviewers
            .into_iter()
            .map(|item| User::easy_from(item))
            .collect::<Vec<User>>();
        let performers = performers
            .into_iter()
            .map(|item| User::easy_from(item))
            .collect::<Vec<User>>();
        Todo {
            id,
            name: value.name,
            priority: value.priority,
            reviewers,
            performers,
            date: value.date,
            tags: value.tags,
            status: value.status,
            description: value.description,
            information: value.information,
            annexs: value.annexs,
            is_focus: value.is_focus,
        }
    }
}

async fn convert_usernames_to_user_instances(usernames: Vec<String>) -> Vec<dto::User> {
    let mut res = Vec::new();
    for username in usernames {
        let query = select_user_by_username(&username).await;
        if let Some(user) = query {
            res.push(user);
        }
    }
    res
}

impl Todo {
    fn select_reviewers(&self) -> () {}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TodoBox {
    pub low: Vec<Todo>,
    pub mid: Vec<Todo>,
    pub fatal: Vec<Todo>,
    //关注
    pub focus: Vec<Todo>,
    pub history: Vec<Todo>,
}

impl TodoBox {
    pub fn push(&mut self, todo: Todo) -> () {
        let priority = todo.priority();
        let is_focus = todo.is_focus;
        if is_focus {
            self.focus.push(todo.clone());
        }
        match priority {
            Priorities::Emergent | Priorities::High => self.fatal.push(todo),
            Priorities::Mid => self.mid.push(todo),
            Priorities::Low => self.low.push(todo),
        };
    }
    pub async fn from(value: dto::TodoBox) -> Self {
        let dto::TodoBox {
            low,
            mid,
            fatal,
            focus,
            history,
        } = value;
        let mut low = convert_ids_to_todo_instances(low).await;
        let mut mid = convert_ids_to_todo_instances(mid).await;
        let mut fatal = convert_ids_to_todo_instances(fatal).await;
        let mut history = convert_ids_to_todo_instances(history).await;
        let mut focus = Vec::new();
        focus.append(&mut low.0);
        focus.append(&mut mid.0);
        focus.append(&mut fatal.0);

        TodoBox {
            low: low.1,
            mid: mid.1,
            fatal: fatal.1,
            focus,
            history: history.1,
        }
    }
}

// impl From<dto::TodoBox> for TodoBox {
//     fn from(value: dto::TodoBox) -> Self {
//         dbg!(&value);
//         let dto::TodoBox {
//             low,
//             mid,
//             fatal,
//             focus,
//             history,
//         } = value;
//         let low = convert_ids_to_todo_instances(low);
//         let mid = convert_ids_to_todo_instances(mid);
//         let res = join(low, mid);
//         let res = block_on(res);
//         // let fatal = convert_ids_to_todo_instances(fatal);
//         // let focus = convert_ids_to_todo_instances(focus);
//         // let history = convert_ids_to_todo_instances(history);
//         // let results = join_all(vec![low, mid, fatal, focus, history]);

//         // let todos = block_on(results);
//         // dbg!(&todos);
//         // TodoBox {
//         //     low: todos[0].clone(),
//         //     mid: todos[1].clone(),
//         //     fatal: todos[2].clone(),
//         //     focus: todos[3].clone(),
//         //     history: todos[4].clone(),
//         // }
//         TodoBox {
//             low: res.0,
//             mid: res.1,
//             fatal: vec![],
//             focus: vec![],
//             history: vec![],
//         }
//     }
// }

//  focus       other
//  (Vec<Todo>,Vec<Todo>)
pub async fn convert_ids_to_todo_instances(ids: Vec<String>) -> (Vec<Todo>, Vec<Todo>) {
    if ids.is_empty() {
        return (Vec::new(), Vec::new());
    }
    let mut res = Vec::new();
    let mut focus = Vec::new();
    for id in ids {
        let query = select_todo_record(id.as_str()).await;
        if let Some((id, todo)) = query {
            let todo = Todo::from(todo, id).await;
            if (todo.is_focus) {
                focus.push(todo.clone())
            }
            res.push(todo);
        }
    }
    (focus, res)
}
