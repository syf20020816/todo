use std::fmt::Display;

use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum Tags {
    Info,
    Success,
    Warning,
    Default,
    Danger,
}

impl Default for Tags {
    fn default() -> Self {
        Tags::Default
    }
}

impl Serialize for Tags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: rocket::serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl Display for Tags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Tags::Info => "info",
            Tags::Success => "success",
            Tags::Warning => "warning",
            Tags::Default => "",
            Tags::Danger => "danger",
        };
        f.write_str(res)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
enum Effects {
    Dark,
    Light,
    Plain,
}

/// 自定义构建ElementPlus中el-tag的必要参数
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ITagProps {
    #[serde(rename(serialize = "type"))]
    r#type: Tags,
    effect: Effects,
    label: String,
}

impl ITagProps {
    pub fn new(t: Tags, effect: Effects, label: &str) -> Self {
        ITagProps {
            r#type: t,
            effect,
            label: label.to_string(),
        }
    }
}
