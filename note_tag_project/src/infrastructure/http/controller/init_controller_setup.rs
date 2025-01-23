use rocket::{fairing::AdHoc, routes, Route};

use crate::infrastructure::http::controller::auth;

pub fn init_controller_setup() -> AdHoc {
    AdHoc::on_ignite("Initialize controller", |rocket| async {
        rocket.mount("/auth", auth_routes())
    })
}

// use crate::application::controller::auth;


pub fn auth_routes() -> Vec<Route> {
    routes![
        auth::sign_in,
        auth::sign_up,
        auth::me
    ]
}

