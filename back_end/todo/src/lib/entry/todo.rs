use super::{Annex, Date, ITagProps, Priorities, Priority, Status, User};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Todo {
    id: String,
    name: String,
    priority: Priority,
    /// 审核人
    reviewers: Vec<User>,
    performers: Vec<User>,
    date: Date,
    tags: Vec<ITagProps>,
    status: Status,
    description: Option<String>,
    information: Option<String>,
    /// 附件
    annexs: Option<Vec<Annex>>,
}

impl Default for Todo {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            priority: Default::default(),
            reviewers: Default::default(),
            performers: Default::default(),
            date: Default::default(),
            tags: Default::default(),
            status: Default::default(),
            description: None,
            information: None,
            annexs: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TodoBox {
    low: Vec<Todo>,
    mid: Vec<Todo>,
    fatal: Vec<Todo>,
    //关注
    focus: Vec<Todo>,
}

impl Default for TodoBox {
    fn default() -> Self {
        Self {
            low: Default::default(),
            mid: Default::default(),
            fatal: Default::default(),
            focus: Default::default(),
        }
    }
}
