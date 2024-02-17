use rocket::serde::json::Json;

use crate::lib::{
    entry::{
        dto,
        vo::{self, Team},
    },
    error::Error,
    mapping::{
        create_team_by_name, select_user_by_username, update_team_by_id, update_user_by_username,
    },
    response::ResultJsonData,
};

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
