use std::fmt::Display;

use rocket::serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use surrealdb::sql::value;

#[derive(Debug, Clone, PartialEq)]
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
struct TagsVisitor;

impl<'de> Visitor<'de> for TagsVisitor {
    type Value = Tags;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .write_str("a string containing 'info', 'success', 'warning', 'default', or 'danger'")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: rocket::serde::de::Error,
    {
        let value = v.to_lowercase();
        match value.as_str() {
            "info" => Ok(Tags::Info),
            "success" => Ok(Tags::Success),
            "" => Ok(Tags::Default),
            "danger" => Ok(Tags::Danger),
            "warning" => Ok(Tags::Warning),
            _ => Err(rocket::serde::de::Error::invalid_value(
                de::Unexpected::Str(&value),
                &self,
            )),
        }
    }
}

impl<'de> Deserialize<'de> for Tags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: rocket::serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(TagsVisitor)
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

#[derive(Debug, Clone, PartialEq)]
pub enum Effects {
    Dark,
    Light,
    Plain,
}

impl Serialize for Effects {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: rocket::serde::Serializer,
    {
        let res = match self {
            Effects::Dark => "dark",
            Effects::Light => "light",
            Effects::Plain => "plain",
        };
        serializer.serialize_str(res)
    }
}

struct EffectsVisitor;

impl<'de> Visitor<'de> for EffectsVisitor {
    type Value = Effects;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string containing `dark`, `light`, `plain`'")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value = v.to_lowercase();
        match value.as_str() {
            "dark" => Ok(Effects::Dark),
            "plain" => Ok(Effects::Plain),
            "light" => Ok(Effects::Light),
            _ => Err(rocket::serde::de::Error::invalid_value(
                de::Unexpected::Str(&value),
                &self,
            )),
        }
    }
}

impl<'de> Deserialize<'de> for Effects {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(EffectsVisitor)
    }
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
