use std::sync::Arc;

use rocket::{http::Status, post, routes, serde::json::Json, Route, State};

use crate::{application::usecase::note_usecase::NoteUseCase, domain::{dto::note_dto::ReqCreateNoteDto, entities::user}, infrastructure::{faring::authentication::AuthenticatedUser, http::response_type::response_type::{ErrorResponse, Response, SuccessResponse}, mysql::repositories::impl_note_repository::ImplNoteRepository}};


pub fn note_routes() -> Vec<Route> {
    routes![
        create_note,
    ]
}

#[post("/", data = "<note_info>")]
pub async fn create_note(
    user: AuthenticatedUser,
    note_info: Json<ReqCreateNoteDto>,
    note_usecase: &State<Arc<NoteUseCase<ImplNoteRepository>>>,
)
-> Response<String> {
    let result = note_usecase.create_note(user.id, note_info.into_inner()).await;
    match result {
        Ok(_) => Ok(SuccessResponse((Status::Created, "Note added successfully".to_string()))),
        Err(_) => Err(ErrorResponse((Status::BadRequest, "Error adding note".to_string())))
    }
}

