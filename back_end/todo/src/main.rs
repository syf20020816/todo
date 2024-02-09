mod lib;

#[macro_use]
extern crate rocket;

use std::{collections::HashSet, str::FromStr};

use lib::api::signin;
use lib::response::define_excp_handler;
use rocket_cors::{AllowedMethods, AllowedOrigins, Cors, CorsOptions, Method};

#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:4000",
        "http://localhost:5173",
        "http://127.0.0.1:4000",
        "http://127.0.0.1:5173",
    ]);

    let allowed_methods: AllowedMethods = vec!["Get", "Post", "Delete", "Put"]
        .into_iter()
        .map(|s| Method::from_str(s).unwrap())
        .collect::<HashSet<Method>>();

    let cors_options = CorsOptions::default()
        .allowed_methods(allowed_methods)
        .allowed_origins(allowed_origins)
        .allow_credentials(true);

    let cors = Cors::from_options(&cors_options).expect("CORS Configuration Correct");

    rocket::build()
        .attach(cors)
        .mount("/api/v1", routes![signin])
        .register("/api/v1", catchers![define_excp_handler])
}
