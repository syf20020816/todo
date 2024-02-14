use rocket::serde::json::Json;

use crate::lib::{
    entry::{
        dto::{Todo, TodoBox},
        vo::{self, convert_ids_to_todo_instances},
    },
    error::Error,
    mapping::{
        create_todo_by_username, select_todo_record, select_user_by_username,
        update_user_by_username,
    },
    response::ResultJsonData,
};

#[post("/create", format = "application/json", data = "<todo>")]
pub async fn create_todo(todo: Json<Todo>) -> ResultJsonData<vo::User> {
    let todo = todo.0;
    let username = todo.clone().owner;
    let query = create_todo_by_username(&username, todo).await;
    if let Some((id, todo)) = query {
        dbg!(&id);
        let mut user = select_user_by_username(&username).await.unwrap();
        let priority = todo.priority();
        let _ = user.add_todo(id, priority);
        //update user
        let update_query = update_user_by_username(user).await;
        if let Some(user) = update_query {
            let user = vo::User::from(user).await;
            dbg!(&user);
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
