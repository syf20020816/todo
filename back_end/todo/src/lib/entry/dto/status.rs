use std::fmt::Display;

use rocket::serde::{Deserialize, Serialize};
/// 待办状态
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub enum Status {
    /// 未开始
    NotStart,
    /// 进行中
    InProgress,
    /// 已完成
    Completed,
    /// 阻塞中
    Pending,
}

impl Default for Status {
    fn default() -> Self {
        Status::InProgress
    }
}

impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: rocket::serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Status::NotStart => "not start",
            Status::InProgress => "in progress",
            Status::Completed => "completed",
            Status::Pending => "pending",
        };
        f.write_str(res)
    }
}
