use super::{Annex, Date, ITagProps, Priorities, Priority, Status, User};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Todo {
    pub id: String,
    pub name: String,
    pub priority: Priorities,
    /// 审核人
    pub reviewers: Vec<String>,
    pub performers: Vec<String>,
    pub date: Date,
    pub tags: Vec<ITagProps>,
    pub status: Status,
    pub description: Option<String>,
    pub information: Option<String>,
    /// 附件
    pub annexs: Option<Vec<Annex>>,
    pub is_focus: bool,
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
            is_focus: false,
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
    history: Vec<Todo>,
}

impl Default for TodoBox {
    fn default() -> Self {
        Self {
            low: Default::default(),
            mid: Default::default(),
            fatal: Default::default(),
            focus: Default::default(),
            history: Default::default(),
        }
    }
}
