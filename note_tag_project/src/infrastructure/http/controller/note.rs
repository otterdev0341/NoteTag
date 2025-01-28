use std::sync::Arc;

use rocket::{get, http::Status, post, routes, serde::json::Json, Route, State};

use crate::{application::usecase::note_usecase::NoteUseCase, domain::{dto::note_dto::{ReqCreateNoteDto, ResNoteEntryDto}, entities::user}, infrastructure::{faring::authentication::AuthenticatedUser, http::response_type::response_type::{ErrorResponse, Response, SuccessResponse}, mysql::repositories::impl_note_repository::ImplNoteRepository}};


pub fn note_routes() -> Vec<Route> {
    routes![
        create_note,
        get_note_by_id
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
#[get("/<note_id>")]
pub async fn get_note_by_id(
    user: AuthenticatedUser,
    note_id: i32,
    note_usecase: &State<Arc<NoteUseCase<ImplNoteRepository>>>,
) -> Response<Json<ResNoteEntryDto>> {
    let result = note_usecase.get_note_by_id(user.id, note_id).await;
    match result {
        Ok(note) => Ok(SuccessResponse((Status::Ok, Json(note)))),
        Err(_) => Err(ErrorResponse((Status::BadRequest, "Error getting note".to_string())))
    }
}

