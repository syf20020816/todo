use surreal_use::core::{
    sql::{CreateData, SurrealTable},
    Stmt,
};
use surrealdb::sql::Output;

use crate::lib::{db::DB, entry::dto::Todo};

use super::{select_user_record_by_username, Record};

pub async fn create_todo_by_username(username: &str, mut todo: Todo) -> Option<(String, Todo)> {
    let query = select_user_record_by_username(username).await;
    if let Some((id, _)) = query {
        todo.set_owner(id);
    } else {
        return None;
    }

    let sql = Stmt::create()
        .table("todo".into())
        .data(todo.into())
        .output(Output::After)
        .to_string();
    let mut result = DB.query(sql).await.unwrap();
    let sql_result: Vec<Record<Todo>> = result.take(0_usize).unwrap();
    if sql_result.len() == 1 {
        let res = sql_result[0].clone();
        Some(res.to_record())
    } else {
        None
    }
}

pub async fn select_todo_record(id: &str) -> Option<(String, Todo)> {
    let sql = Stmt::select()
        .table(("todo", id).into())
        .field_all()
        .to_string();
    dbg!(&sql);
    //没有执行
    let mut result = DB.query(sql).await.unwrap();
    dbg!(&result);
    let sql_result: Vec<Record<Todo>> = result.take(0_usize).unwrap();
    dbg!(&sql_result);
    if sql_result.len() == 1 {
        let res = sql_result[0].clone();
        Some(res.to_record())
    } else {
        None
    }
}
