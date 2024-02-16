use rocket::{http::uri::Query, serde::json::Json};

use crate::lib::{
    entry::{
        dto::{Status, Todo, TodoBox},
        vo::{self, convert_ids_to_todo_instances},
    },
    error::Error,
    mapping::{
        create_todo_by_username, delete_todo_by_id, select_todo_record, select_user_by_username,
        update_todo_by_id, update_todo_status_by_id, update_user_by_username,
    },
    response::ResultJsonData,
};

#[post("/create", format = "application/json", data = "<todo>")]
pub async fn create_todo(todo: Json<Todo>) -> ResultJsonData<vo::User> {
    let todo = todo.0;
    let username = todo.clone().owner;
    let query = create_todo_by_username(&username, todo).await;
    if let Some((id, todo)) = query {
        let mut user = select_user_by_username(&username).await.unwrap();
        let priority = todo.priority();
        let _ = user.add_todo(id, priority);
        //update user
        let update_query = update_user_by_username(user).await;
        if let Some(user) = update_query {
            let user = vo::User::from(user).await;
            return ResultJsonData::success(user);
        } else {
            let e = Error::UpdateUser;
            let (e_code, e_msg) = e.get();
            return ResultJsonData::define_failure(e_code, &e_msg);
        }
    }
    let e = Error::CreateTodo;
    let (e_code, e_msg) = e.get();
    return ResultJsonData::define_failure(e_code, &e_msg);
}

#[delete("/<username>/<id>")]
pub async fn delete_todo(username: &str, id: &str) -> ResultJsonData<vo::User> {
    let query = delete_todo_by_id(id).await;
    if query {
        let user_query = select_user_by_username(username).await;
        if let Some(mut user) = user_query {
            let _ = user.delete_todo(id);
            let update_query = update_user_by_username(user).await;
            if let Some(user) = update_query {
                let user = vo::User::from(user).await;
                return ResultJsonData::success(user);
            }
        }
    }
    let e = Error::DeleteTodo;
    let (e_code, e_msg) = e.get();
    return ResultJsonData::define_failure(e_code, &e_msg);
}

#[put("/<username>/<id>", format = "application/json", data = "<todo>")]
pub async fn update_todo(username: &str, id: &str, todo: Json<Todo>) -> ResultJsonData<vo::User> {
    let query = update_todo_by_id(&id, todo.0).await;
    if query {
        let user_query = select_user_by_username(username).await;
        if let Some(mut user) = user_query {
            let user = vo::User::from(user).await;
            return ResultJsonData::success(user);
        }
    }
    let e = Error::UpdateTodo;
    let (e_code, e_msg) = e.get();
    return ResultJsonData::define_failure(e_code, &e_msg);
}

#[get("/<id>/<status>")]
pub async fn update_todo_status(id: &str, status: &str) -> ResultJsonData<bool> {
    let query = update_todo_status_by_id(id, status).await;
    if query {
        return ResultJsonData::success(query);
    }
    let e = Error::UpdateTodo;
    let (e_code, e_msg) = e.get();
    return ResultJsonData::define_failure(e_code, &e_msg);
}

#[get("/complete/<username>/<id>")]
pub async fn complete_todo(username: &str, id: &str) -> ResultJsonData<vo::User> {
    let status = Status::Completed.to_string();
    let (code, _update_status) = update_todo_status(id, &status).await.get();
    if code == 200 {
        let user_query = select_user_by_username(username).await;
        if let Some(mut user) = user_query {
            let _ = user.complete_todo(id);
            let query = update_user_by_username(user).await;
            if let Some(user) = query {
                let user = vo::User::from(user).await;
                return ResultJsonData::success(user);
            }
        }
    }
    let e = Error::CompleteTodo;
    let (e_code, e_msg) = e.get();
    return ResultJsonData::define_failure(e_code, &e_msg);
}

#[get("/ho")]
pub async fn ho() -> ResultJsonData<bool> {
    let todos = TodoBox {
        low: vec!["38issczoupzismi6w17v".to_string()],
        mid: Default::default(),
        fatal: Default::default(),
        history: Default::default(),
        focus: Default::default(),
    };
    let todos = vo::TodoBox::from(todos).await;
    // let res = convert_ids_to_todo_instances(vec!["38issczoupzismi6w17v".to_string()]).await;

    dbg!(todos);
    ResultJsonData::success(true)
}
