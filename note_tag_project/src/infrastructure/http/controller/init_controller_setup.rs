use rocket::fairing::AdHoc;



use super::{auth::auth_routes, note::note_routes, note_status::note_status_routes, user_query::user_query_routes, user_tag::user_tag_routes};

pub fn init_controller_setup() -> AdHoc {
    AdHoc::on_ignite("Initialize controller", |rocket| async {
        rocket
            .mount("/auth/v1", auth_routes())
            .mount("/api/v1/user_tag", user_tag_routes())
            .mount("/api/v1/note", note_routes())
            .mount("/api/v1/note_status", note_status_routes())
            .mount("/api/v1/query", user_query_routes())
    })
}


