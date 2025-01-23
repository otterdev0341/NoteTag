use rocket::{http::Status, Responder};

#[derive(Responder)]
pub struct SuccessResponse<T>(pub (Status, T));

#[derive(Responder)]
pub struct ErrorResponse (pub (Status, String));

pub type Response<T> = Result<SuccessResponse<T>, ErrorResponse>;