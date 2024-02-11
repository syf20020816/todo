use rocket::serde::{Deserialize, Serialize};
use surreal_use::core::{
    sql::{Cond, CreateData},
    Stmt,
};
use surrealdb::sql::{Operator, Output};

use crate::lib::{db::DB, entry::dto::User};

pub async fn select_user_by_username(username: &str) -> Option<User> {
    let sql = Stmt::select()
        .table("user".into())
        .field_all()
        .cond(
            Cond::new()
                .left_easy("username")
                .op(Operator::Equal)
                .right(username.into()),
        )
        .to_string();

    let mut result = DB.query(sql).await.unwrap();
    let sql_result: Vec<User> = result.take(0_usize).unwrap();
    if sql_result.len() == 1 {
        let mut res = sql_result[0].clone();
        let _ = res.skip_pwd();
        Some(res)
    } else {
        None
    }
}

pub async fn check_user_by_username(username: &str) -> bool {
    #[derive(Serialize, Deserialize)]
    #[serde(crate = "rocket::serde")]
    struct TmpUser {
        username: String,
    }

    // 检查是否已经有相同的用户名
    let sql = Stmt::select()
        .table("user".into())
        .fields(vec!["username".into()])
        .cond(
            Cond::new()
                .left_easy("username")
                .op(Operator::Equal)
                .right(username.into()),
        )
        .to_string();

    let mut result = DB.query(sql).await.unwrap();
    let check_result: Vec<TmpUser> = result.take(0_usize).unwrap();
    check_result.len().eq(&0_usize)
}

pub async fn create_user(user: User) -> Option<User> {
    let sql = Stmt::create()
        .table("user".into())
        .data(CreateData::content(user))
        .output(Output::After)
        .to_string();
    let mut result = DB.query(sql).await.unwrap();
    let sql_result: Vec<User> = result.take(0_usize).unwrap();
    if sql_result.len() == 0 {
        return None;
    }
    let res = sql_result[0].clone();
    Some(res)
}

pub async fn select_user_by_username_password(username: &str, password: &str) -> Option<User> {
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

    // 结果类似: SELECT * FROM user WHERE username = 'matt000' AND password = 'matt000'
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
    let mut result = DB.query(sql).await.unwrap();
    let sql_result: Vec<User> = result.take(0_usize).unwrap();
    if sql_result.len() == 1 {
        let res = sql_result[0].clone();
        Some(res)
    } else {
        None
    }
}
