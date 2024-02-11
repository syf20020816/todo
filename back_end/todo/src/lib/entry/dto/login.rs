use rocket::serde::{self, Deserialize, Serialize};

// 采用Rocket框架提供给的serde进行序列化与反序列化
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct Signin {
    username: String,
    password: String,
}

impl Signin {
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password(&self) -> &str {
        &self.password
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct Signup {
    username: String,
    name: String,
    email: String,
    password: String,
}

impl Signup {
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password(&self) -> &str {
        &self.password
    }
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}
