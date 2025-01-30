use std::sync::Arc;

use rocket::{http::Status, post, routes, Route, State};

use crate::{application::usecase::note_status::NoteStatusUseCase, infrastructure::{faring::authentication::AuthenticatedUser, http::response_type::response_type::{ErrorResponse, Response, SuccessResponse}, mysql::repositories::impl_note_x_status_repository::ImplNoteStatusRepository}};



pub fn note_status_routes() -> Vec<Route> {
    routes![
        toggle_note_status
    ]
}

#[post("/<note_id>")]
pub async fn toggle_note_status(
    user: AuthenticatedUser,
    note_status_usecase: &State<Arc<NoteStatusUseCase<ImplNoteStatusRepository>>>,
    note_id: i32,
) 
-> Response<String> {
    let result = note_status_usecase.toggle_note_status(user.id, note_id).await;
    match result {
        Ok(_) => Ok(SuccessResponse((Status::Ok, "Note status toggled successfully".to_string()))),
        Err(_) => Err(ErrorResponse((Status::BadRequest, "Error toggling note status".to_string())))
    }
}