use rocket::{get, http::Status, post, serde::json::Json};
use crate::{domain::dto::auth_dto::{ReqSignInDto, ResSignInDto}, infrastructure::http::response_type::response_type::{Response, SuccessResponse}};




// Route 1

#[utoipa::path(
    post,
    path = "/sign-in",
    request_body = ReqSignInDto,
    responses(
        (status = 200, description = "User signed in successfully", body = ResSignInDto)
    )
)]
#[post("/sign-in", data = "<req_sign_in>")]
pub async fn sign_in(
    req_sign_in: Json<ReqSignInDto>
) 
-> Response<Json<ResSignInDto>>{
    todo!()
}



// Route 2
#[post("/sign-up")]
pub async fn sign_up() -> &'static str {
    todo!()
}




// Route 3
#[get("/me")]
pub async fn me() -> Response<String> {
    Ok(SuccessResponse((Status::Ok, "This is me".to_string())))
}

