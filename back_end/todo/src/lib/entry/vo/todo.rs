use crate::lib::entry::dto::{self, Annex, Date, ITagProps, Priorities, Priority, Status, User};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Todo {
    id: String,
    name: String,
    priority: Priorities,
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
    is_focus: bool,
}

impl From<dto::Todo> for Todo {
    fn from(value: dto::Todo) -> Self {
        Todo {
            id: value.id,
            name: value.name,
            priority: value.priority,
            reviewers: Vec::new(),
            performers: Vec::new(),
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

impl Todo {
    fn select_reviewers(&self) -> () {}
}
