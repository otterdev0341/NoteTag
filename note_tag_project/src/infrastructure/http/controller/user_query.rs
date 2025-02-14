use std::sync::Arc;

use rocket::{http::Status, post, routes, serde::json::Json, Route, State};

use crate::{application::usecase::user_query_usecase::UserQueryUsecase, domain::{dto::{note_dto::ResNoteListDto, query::QueryNoteDto}, repositories::require_implementation::trait_user_note_query_repository::UserNoteQueryRepository}, infrastructure::{faring::{authentication::AuthenticatedUser, cors::options}, http::response_type::response_type::{ErrorResponse, Response, SuccessResponse}, mysql::repositories::impl_user_note_query_repository::ImplUserNoteQueryRepository}};

pub fn user_query_routes() -> Vec<Route> {
    routes![
        user_query,
        options
    ]
}

#[post("/", data = "<query_info>")]
pub async fn user_query(
    user: AuthenticatedUser,
    query_info: Json<QueryNoteDto>,
    note_usecase: &State<Arc<UserQueryUsecase<ImplUserNoteQueryRepository>>>,
)
-> Result<Response<Json<ResNoteListDto>>, Response<String>> {
    let result = note_usecase.query_notes(user.id, query_info.into_inner()).await;
    match result {
        Ok(notes) => Ok(Response::Ok(SuccessResponse((Status::Ok, Json(notes))))),
        Err(_) => Ok(Response::Err(ErrorResponse((Status::BadRequest, "Error getting notes".to_string()))))
    }
    
}