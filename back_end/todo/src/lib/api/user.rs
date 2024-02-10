use crate::lib;
use crate::lib::db::{db_init, DB};
use crate::lib::entry::User;
use crate::lib::entry::{Signin, Signup};
use crate::lib::error::Error;
use crate::lib::response::ResultJsonData;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use surreal_use::core::sql::{Cond, CreateData};
use surreal_use::core::Stmt;
use surrealdb::sql::{Array, Object, Operator, Output, Value};

#[post("/user/signin", format = "application/json", data = "<user>")]
pub async fn signin(user: Json<Signin>) -> ResultJsonData<bool> {
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

    // 结果: SELECT * FROM user WHERE username = 'matt000' AND password = 'matt000'
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

    ResultJsonData::success(true)
}

#[post("/user/signup", format = "application/json", data = "<user>")]
pub async fn signup(user: Json<Signup>) -> ResultJsonData<User> {
    let user = user.0;
    let username = user.username();
    // 检查是否已经有相同的用户名
    let sql_check = Stmt::select()
        .table("user".into())
        .fields(vec!["username".into()])
        .cond(
            Cond::new()
                .left_easy("username")
                .op(Operator::Equal)
                .right(username.into()),
        )
        .to_string();

    #[derive(Serialize, Deserialize)]
    #[serde(crate = "rocket::serde")]
    struct TmpUser {
        username: String,
    }
    let mut result = DB.query(sql_check).await.unwrap();
    let check_result: Vec<TmpUser> = result.take(0_usize).unwrap();
    if check_result.len().eq(&0_usize) {
        let user = User::quick_init(user.name(), user.username(), user.password(), user.email());

        let signup_sql = Stmt::create()
            .table("user".into())
            .data(CreateData::content(user))
            .output(Output::After)
            .to_string();
        let mut result = DB.query(signup_sql).await.unwrap();
        let signup_result: Vec<User> = result.take(0_usize).unwrap();
        let res = signup_result[0].clone();
        return ResultJsonData::success(res);
    } else {
        let error = Error::ExistAccount;
        let (e_code, e_msg) = error.get();
        return ResultJsonData::define_failure(e_code, &e_msg);
    }
}
