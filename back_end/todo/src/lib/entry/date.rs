use rocket::serde::{self, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Date {
    start: String,
    end: String,
    during: usize,
}

impl Date {
    pub fn new(start: &str, end: &str) -> Self {
        let start = start.to_string();
        let end = end.to_string();
        let during = start.parse::<usize>().unwrap() - end.parse::<usize>().unwrap();
        Date { start, end, during }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date() {}
}
