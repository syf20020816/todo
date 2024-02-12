use crate::lib::entry::dto::{self, Annex, Date, ITagProps, Priorities, Priority, Status, User};
use rocket::serde::{Deserialize, Serialize};

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
