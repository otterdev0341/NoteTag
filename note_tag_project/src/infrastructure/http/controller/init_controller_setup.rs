use rocket::fairing::AdHoc;



use super::{auth::auth_routes, note::note_routes, user_tag::user_tag_routes};

pub fn init_controller_setup() -> AdHoc {
    AdHoc::on_ignite("Initialize controller", |rocket| async {
        rocket
            .mount("/auth/v1", auth_routes())
            .mount("/api/v1/user_tag", user_tag_routes())
            .mount("/api/v1/note", note_routes())
    })
}


