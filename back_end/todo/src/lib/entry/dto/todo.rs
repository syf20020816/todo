use crate::lib::entry::vo;

use super::{Annex, Date, ITagProps, Priorities, Priority, Status, User};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Todo {
    pub owner: String,
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
    #[serde(rename(serialize = "isFocus"))]
    #[serde(rename(deserialize = "isFocus"))]
    pub is_focus: bool,
}

impl Default for Todo {
    fn default() -> Self {
        Self {
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
            owner: "".to_string(),
        }
    }
}

impl Todo {
    pub fn set_owner(&mut self, id: String) -> () {
        self.owner = id;
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
    pub fn from(value: vo::Todo, owner: &str) -> (String, Self) {
        let id = value.id().to_string();
        let reviewers = value
            .reviewers
            .into_iter()
            .map(|x| x.username().to_string())
            .collect::<Vec<String>>();
        let performers = value
            .performers
            .into_iter()
            .map(|x| x.username().to_string())
            .collect::<Vec<String>>();

        let todo = Todo {
            owner: owner.to_string(),
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
        };

        (id, todo)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TodoBox {
    pub low: Vec<String>,
    pub mid: Vec<String>,
    pub fatal: Vec<String>,
    //关注
    pub focus: Vec<String>,
    pub history: Vec<String>,
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

impl TodoBox {
    pub fn remove(&mut self, id: &str) {
        let TodoBox {
            low,
            mid,
            fatal,
            focus,
            history,
        } = self;

        self.low = low.clone().into_iter().filter(|x| x.ne(id)).collect();
        self.mid = mid.clone().into_iter().filter(|x| x.ne(id)).collect();
        self.fatal = fatal.clone().into_iter().filter(|x| x.ne(id)).collect();
        self.focus = focus.clone().into_iter().filter(|x| x.ne(id)).collect();
    }
}
