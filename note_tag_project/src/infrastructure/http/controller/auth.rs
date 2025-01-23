use rocket::{get, http::Status, post};

use crate::infrastructure::http::response_type::response_type::{Response, SuccessResponse};

#[post("/sign-in")]
pub async fn sign_in() -> &'static str {
    "This is sign in"
}

#[post("/sign-up")]
pub async fn sign_up() -> &'static str {
    "This is sign up page"
}

#[get("/me")]
pub async fn me() -> Response<String> {
    Ok(SuccessResponse((Status::Ok, "This is me".to_string())))
}

