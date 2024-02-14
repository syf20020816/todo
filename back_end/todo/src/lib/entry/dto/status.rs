use std::fmt::Display;

use rocket::serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
/// 待办状态
#[derive(Debug, Clone, PartialEq)]
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

struct StatusVisitor;

impl<'de> Visitor<'de> for StatusVisitor {
    type Value = Status;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("expected one of `not start`, `in progress`, `completed`, `pending`")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: rocket::serde::de::Error,
    {
        let value = v.to_lowercase();
        match value.as_str() {
            "not start" => Ok(Status::NotStart),
            "in progress" => Ok(Status::InProgress),
            "completed" => Ok(Status::Completed),
            "pending" => Ok(Status::Pending),
            _ => Err(rocket::serde::de::Error::invalid_value(
                de::Unexpected::Str(&value),
                &self,
            )),
        }
    }
}

impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(StatusVisitor)
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
