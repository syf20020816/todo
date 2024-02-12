use crate::lib::entry::dto::{Avatars, User};
use crate::lib::entry::dto::{Signin, Signup};
use crate::lib::entry::vo::UserPersonalSetting;
use crate::lib::error::Error;
use crate::lib::mapping::{
    check_user_by_username, create_user, select_user_by_username, select_user_by_username_password,
    update_user_avatar, update_user_by_personal_settings,
};
use crate::lib::response::ResultJsonData;
use rocket::serde::json::Json;

#[post("/signin", format = "application/json", data = "<user>")]
pub async fn signin(user: Json<Signin>) -> ResultJsonData<User> {
    let username = user.0.username();
    let password = user.0.password();

    let query = select_user_by_username_password(username, password).await;
    if let Some(mut user) = query {
        let _ = user.skip_pwd();
        return ResultJsonData::success(user);
    }
    let e = Error::IdentityAuthentication;
    let (e_code, e_msg) = e.get();
    return ResultJsonData::define_failure(e_code, &e_msg);
}

#[post("/signup", format = "application/json", data = "<user>")]
pub async fn signup(user: Json<Signup>) -> ResultJsonData<User> {
    let user = user.0;
    let username = user.username();
    let exist = check_user_by_username(username).await;
    if exist {
        let user = User::quick_init(user.name(), user.username(), user.password(), user.email());

        let query = create_user(user).await;
        if let Some(mut user) = query {
            user.skip_pwd();
            return ResultJsonData::success(user);
        }
        return ResultJsonData::failure("Server data error: api::signup");
    } else {
        let error = Error::ExistAccount;
        let (e_code, e_msg) = error.get();
        return ResultJsonData::define_failure(e_code, &e_msg);
    }
}

#[get("/info/<username>", format = "application/json")]
pub async fn get_user_info(username: &str) -> ResultJsonData<User> {
    let query = select_user_by_username(username).await;
    if let Some(user) = query {
        return ResultJsonData::success(user);
    }
    let e = Error::IdentityAuthentication;
    let (e_code, e_msg) = e.get();
    return ResultJsonData::define_failure(e_code, &e_msg);
}

#[post("/info/<username>", format = "application/json", data = "<user>")]
pub async fn set_user_setting(
    username: &str,
    user: Json<UserPersonalSetting>,
) -> ResultJsonData<User> {
    let user = user.0;
    let query = update_user_by_personal_settings(user, username).await;
    if let Some(user) = query {
        return ResultJsonData::success(user);
    }
    let e = Error::ChangeUserSetting;
    let (e_code, e_msg) = e.get();
    return ResultJsonData::define_failure(e_code, &e_msg);
}

#[get("/info/<username>/<avatar>")]
pub async fn set_user_avatar(username: &str, avatar: &str) -> ResultJsonData<bool> {
    let avatar = Avatars::from(avatar);
    let query = update_user_avatar(username, avatar).await;
    if query {
        return ResultJsonData::success(true);
    } else {
        let e = Error::ChangeUserAvatar;
        let (e_code, e_msg) = e.get();
        return ResultJsonData::define_failure(e_code, &e_msg);
    }
}
