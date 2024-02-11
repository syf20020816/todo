mod lib;

#[macro_use]
extern crate rocket;

use lib::api::{get_user_info, signin, signup};
use lib::cors::init_cors;
use lib::db::db_init;
use lib::response::define_excp_handler;

#[launch]
async fn rocket() -> _ {
    let _ = db_init().await;
    let cors = init_cors().expect("CORS Configuration Correct");

    rocket::build()
        .attach(cors)
        .mount("/api/v1", routes![signin, signup, get_user_info])
        .register("/api/v1", catchers![define_excp_handler])
}
