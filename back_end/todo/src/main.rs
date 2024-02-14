mod lib;

#[macro_use]
extern crate rocket;

use lib::api::{create_todo, get_user_info, ho, set_user_avatar, set_user_setting, signin, signup};
use lib::cors::init_cors;
use lib::db::db_init;
use lib::response::define_excp_handler;

#[launch]
async fn rocket() -> _ {
    let _ = db_init().await;
    let cors = init_cors().expect("CORS Configuration Correct");

    rocket::build()
        .attach(cors)
        .mount(
            "/api/v1/user",
            routes![
                signin,
                signup,
                get_user_info,
                set_user_setting,
                set_user_avatar
            ],
        )
        .mount("/api/v1/todo", routes![create_todo, ho])
        .register("/api/v1", catchers![define_excp_handler])
}
