use rocket::{get, http::Status, post, serde::json::Json};
use utoipa::OpenApi;

use crate::{domain::dto::auth_dto::{ReqSignInDto, ResSignInDto}, infrastructure::http::response_type::response_type::{Response, SuccessResponse}};


#[derive(OpenApi)]
#[openapi(paths(crate::infrastructure::http::controller::auth::sign_in),components(schemas(ReqSignInDto, ResSignInDto)))]
pub struct ApiDoc();



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

#[post("/sign-up")]
pub async fn sign_up() -> &'static str {
    "This is sign up page"
}

#[get("/me")]
pub async fn me() -> Response<String> {
    Ok(SuccessResponse((Status::Ok, "This is me".to_string())))
}

