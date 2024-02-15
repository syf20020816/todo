use surreal_use::core::{
    sql::{CreateData, SetField, SurrealTable, UpdateData},
    Stmt,
};
use surrealdb::sql::Output;

use crate::lib::{
    db::DB,
    entry::dto::{Status, Todo},
};

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
    let mut result = DB.query(sql).await.unwrap();
    let sql_result: Vec<Record<Todo>> = result.take(0_usize).unwrap();
    if sql_result.len() == 1 {
        let res = sql_result[0].clone();
        Some(res.to_record())
    } else {
        None
    }
}

pub async fn delete_todo_by_id(id: &str) -> bool {
    let sql = Stmt::delete()
        .table(("todo", id).into())
        .output(Output::Before)
        .to_string();
    let mut result = DB.query(sql).await.unwrap();
    let sql_result: Vec<Record<Todo>> = result.take(0_usize).unwrap();
    if sql_result.len() == 1 {
        true
    } else {
        false
    }
}

pub async fn update_todo_by_id(id: &str, todo: Todo) -> bool {
    let sql = Stmt::update()
        .table(("todo", id).into())
        .data(UpdateData::content(todo))
        .output(Output::After)
        .to_string();
    let mut result = DB.query(sql).await.unwrap();
    let sql_result: Vec<Record<Todo>> = result.take(0_usize).unwrap();
    if sql_result.len() == 1 {
        true
    } else {
        false
    }
}

pub async fn update_todo_status_by_id(id: &str, status: &str) -> bool {
    let sql = Stmt::update()
        .table(("todo", id).into())
        .data(UpdateData::set().push(SetField::new("status", None, status)))
        .output(Output::After)
        .to_string();
    let mut result = DB.query(sql).await.unwrap();
    let sql_result: Vec<Record<Todo>> = result.take(0_usize).unwrap();
    if sql_result.len() == 1 {
        true
    } else {
        false
    }
}
