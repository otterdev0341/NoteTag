use rocket::{http::Status, Responder};
use sea_orm::DbErr;

#[derive(Responder)]
pub struct SuccessResponse<T>(pub (Status, T));

#[derive(Responder)]
pub struct ErrorResponse (pub (Status, String));

pub type Response<T> = Result<SuccessResponse<T>, ErrorResponse>;

impl From<DbErr> for ErrorResponse {
    fn from(err: DbErr) -> Self {
        ErrorResponse((Status::InternalServerError, format!("Database error: {:?}", err)))
    }
}