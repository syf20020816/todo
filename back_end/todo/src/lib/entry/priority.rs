use std::fmt::Display;

use rocket::serde::{Deserialize, Serialize};

/// 16进制颜色值
/// 使用String
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct HexColor(pub String);

impl HexColor {
    pub fn new(color: &str) -> Self {
        Self(color.to_string())
    }
    pub fn get_color(&self) -> &str {
        &self.0
    }
}

impl Serialize for HexColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: rocket::serde::Serializer,
    {
        serializer.serialize_str(self.get_color())
    }
}

impl Display for HexColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_color())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum Priorities {
    Emergent,
    High,
    Mid,
    Low,
}

/// [Priorities.Emergent, '#E86D5E'],
/// [Priorities.High, '#F69D50'],
/// [Priorities.Mid, '#6CB6FF'],
/// [Priorities.Low, '#ADAC9A']
impl Priorities {
    pub fn get_color(&self) -> HexColor {
        let color = match self {
            Priorities::Emergent => "#E86D5E",
            Priorities::High => "#F69D50",
            Priorities::Mid => "#6CB6FF",
            Priorities::Low => "#ADAC9A",
        };
        HexColor::new(color)
    }
}

impl From<Priorities> for Priority {
    fn from(value: Priorities) -> Self {
        let color = value.get_color();
        Priority::new(color, value)
    }
}

impl Default for Priorities {
    fn default() -> Self {
        Self::Mid
    }
}

impl Display for Priorities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Priorities::Emergent => "Emergency",
            Priorities::High => "High",
            Priorities::Mid => "Mid",
            Priorities::Low => "Low",
        };
        f.write_str(res)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Priority {
    //十六进制颜色
    color: HexColor,
    name: Priorities,
}

impl Default for Priority {
    fn default() -> Self {
        Priorities::default().into()
    }
}

impl Priority {
    pub fn new(color: HexColor, name: Priorities) -> Self {
        Priority { color, name }
    }
    pub fn color(&self) -> &str {
        self.color.get_color()
    }
    pub fn name(&self) -> String {
        self.name.to_string()
    }
}

impl Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Priority{}\n\tcolor:{}\n\tname:{}\n{}",
            "{",
            self.color(),
            self.name(),
            "}"
        ))
    }
}
