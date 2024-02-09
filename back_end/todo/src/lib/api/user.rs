use crate::lib::entry::User;
use crate::lib::response::ResultJsonData;

#[get("/user/<id>")]
pub fn get_user(id: &str) -> ResultJsonData<User> {}
