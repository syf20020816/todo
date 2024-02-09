use rocket::serde::json::Json;
use surreal_use::core::sql::Cond;
use surreal_use::core::Stmt;
use surrealdb::sql::Operator;

use crate::lib::entry::Signin;
use crate::lib::entry::User;
use crate::lib::response::ResultJsonData;

#[post("/user/signin", format = "application/json", data = "<user>")]
pub fn signin(user: Json<Signin>) -> ResultJsonData<bool> {
    let username = user.0.username();
    let password = user.0.password();

    let username_cond = Cond::new()
        .left("username")
        .op(Operator::Equal)
        .right(username.into())
        .to_origin()
        .0;
    let password_cond = Cond::new()
        .left("password")
        .op(Operator::Equal)
        .right(password.into())
        .to_origin()
        .0;

    let sql = Stmt::select()
        .table("user".into())
        .field_all()
        .cond(
            Cond::new()
                .left_value(username_cond)
                .op(Operator::And)
                .right(password_cond),
        )
        .to_string();
    dbg!(sql);
    ResultJsonData::success(true)
}

// #[get("/user/<id>")]
// pub fn get_user(id: &str) -> ResultJsonData<User> {}
