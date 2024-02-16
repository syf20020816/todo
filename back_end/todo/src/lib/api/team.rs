use crate::lib::{
    entry::vo,
    error::Error,
    mapping::{create_team_by_name, select_user_by_username, update_user_by_username},
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
