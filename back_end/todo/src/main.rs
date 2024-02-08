mod lib;

#[macro_use]
extern crate rocket;

use lib::response::define_excp_handler;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![])
        .register("/api", catchers![define_excp_handler])
}
