use std::collections::HashSet;

use rocket::serde::json::Json;

use crate::lib::{
    entry::{
        dto::{self, Todo},
        vo::{self, Team},
    },
    error::Error,
    mapping::{
        create_team_by_name, create_todo_by_team, create_todo_by_username,
        select_team_record_by_id, select_user_by_username, update_team_by_id,
        update_user_by_username,
    },
    response::ResultJsonData,
};

use super::create_todo;

#[get("/<username>/<name>")]
pub async fn create_team(username: &str, name: &str) -> ResultJsonData<vo::User> {
    let query = create_team_by_name(name, username).await;

    if let Some((id, _team)) = query {
        let mut user = select_user_by_username(username).await.unwrap();
        let _ = user.create_team(&id);
        let update_query = update_user_by_username(user).await;
        if let Some(user) = update_query {
            let user = vo::User::from(user).await;
            return ResultJsonData::success(user);
        }
    }
    let e = Error::CreateTeam;
    let (e_code, e_msg) = e.get();
    return ResultJsonData::define_failure(e_code, &e_msg);
}

#[put("/<name>", format = "application/json", data = "<team>")]
pub async fn update_team_members(name: &str, team: Json<Team>) -> ResultJsonData<bool> {
    let id = team.0.id.clone();
    let mut team = dto::Team::from(team.0);
    let user_query = select_user_by_username(name).await;
    if let Some(mut user) = user_query {
        let _ = team.add_member(name);
        let _ = user.create_team(&id);
        let update_user = update_user_by_username(user).await;
        if update_user.is_some() {
            let query = update_team_by_id(&id, team).await;
            if query {
                return ResultJsonData::success(true);
            }
        }
        let e = Error::UpdateTeam;
        let (e_code, e_msg) = e.get();
        return ResultJsonData::define_failure(e_code, &e_msg);
    }
    let e = Error::AccountUnExist;
    let (e_code, e_msg) = e.get();
    return ResultJsonData::define_failure(e_code, &e_msg);
}

#[put("/", format = "application/json", data = "<team>")]
pub async fn update_team_info(team: Json<Team>) -> ResultJsonData<bool> {
    let id = team.0.id.clone();
    let team = dto::Team::from(team.0);
    let query = update_team_by_id(&id, team).await;
    if query {
        return ResultJsonData::success(true);
    }
    let e = Error::UpdateTeam;
    let (e_code, e_msg) = e.get();
    return ResultJsonData::define_failure(e_code, &e_msg);
}

#[post("/todo/<id>", format = "application/json", data = "<todo>")]
pub async fn create_team_todo(id: &str, todo: Json<Todo>) -> ResultJsonData<bool> {
    // 向审核者添加TODOS
    // 向执行者添加TODOS
    // 若审核者也是执行者需要过滤只需要添加一个
    let todo = todo.0;
    let reviewers = todo.reviewers.clone();
    let performers = todo.performers.clone();
    let team_id = id.to_string();
    let users = reviewers
        .into_iter()
        .chain(performers.into_iter())
        .collect::<HashSet<String>>();
    let users = users.into_iter().collect::<Vec<String>>();
    let create_query = create_todo_by_team(&team_id, todo.clone()).await;
    let mut flag = true;
    if let Some((todo_id, todo)) = create_query {
        //更新team
        let team_query = select_team_record_by_id(&team_id).await;

        if let Some((id, mut team)) = team_query {
            let _ = team.push_todo(&todo_id);
            let update_team = update_team_by_id(&id, team).await;
            if update_team {
                for username in users {
                    let user_query = select_user_by_username(&username).await;
                    if let Some(mut user) = user_query {
                        let priority = todo.priority();
                        let _ = user.add_todo(todo_id.clone(), priority);
                        let update_query = update_user_by_username(user).await;
                        if update_query.is_some() {
                            continue;
                        } else {
                            flag = false;
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }

    if flag {
        return ResultJsonData::success(true);
    }

    let e = Error::CreateTeamTodo;
    let (e_code, e_msg) = e.get();
    return ResultJsonData::define_failure(e_code, &e_msg);
}
