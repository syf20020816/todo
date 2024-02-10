use std::fmt::Display;

use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Hash, Eq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum Avatars {
    Worker,
    Miner,
    Adventurer,
}

impl Default for Avatars {
    fn default() -> Self {
        Avatars::Worker
    }
}

impl Avatars {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn worker() -> Self {
        Avatars::Worker
    }
    pub fn miner() -> Self {
        Avatars::Miner
    }
    pub fn adventurer() -> Self {
        Avatars::Adventurer
    }
}

impl Display for Avatars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Avatars::Worker => "Worker",
            Avatars::Miner => "Miner",
            Avatars::Adventurer => "Adventurer",
        };
        f.write_str(res)
    }
}

impl From<&str> for Avatars {
    fn from(value: &str) -> Self {
        let value = value.to_lowercase();
        match value.as_str() {
            "worker" => Avatars::Worker,
            "miner" => Avatars::Miner,
            "adventurer" => Avatars::Adventurer,
            _ => panic!("Invalid Avatars"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum TeamAvatars {
    Team1,
    Team2,
    Team3,
    Team4,
}

impl Default for TeamAvatars {
    fn default() -> Self {
        TeamAvatars::Team1
    }
}
