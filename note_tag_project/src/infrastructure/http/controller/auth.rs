use std::{result, sync::Arc};

use rocket::{get, http::Status, post, serde::json::Json, State};
use validator::Validate;
use crate::{application::usecase::user_usecase::{UserOperation, UserUseCase}, domain::{dto::auth_dto::{ReqSignInDto, ReqSignUpDto, ResSignInDto}, entities::user}, infrastructure::{faring::authentication::AuthenticatedUser, http::response_type::response_type::{ErrorResponse, Response, SuccessResponse}, mysql::repositories::impl_user_repository::ImplUserRepository}};




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
    req_sign_in: Json<ReqSignInDto>,
    user_use_case: &State<Arc<UserUseCase<ImplUserRepository>>>,
) 
-> Response<ResSignInDto>
{   

     // field empty Bad request
     if let Err(errors) = req_sign_in.validate() {
        return Err(
            ErrorResponse((Status::BadRequest, format!("Validation errors: {:?}", errors)))
        );
    }

    // find user from email
    let user = user_use_case.sign_in(req_sign_in.into_inner()).await;
    match user {
        Ok(user) => {
            Ok(SuccessResponse((Status::Ok, user)))
        },
        Err(error) => {
            Err(ErrorResponse((Status::BadRequest, format!("{:?}", error))))
        }
    }
  

    
}




// Route 2
#[post("/sign-up", data = "<req_sign_up>")]
pub async fn sign_up(
    req_sign_up: Json<ReqSignUpDto>,
    user_use_case: &State<Arc<UserUseCase<ImplUserRepository>>>,
) 
-> Response<String>
{
    // field empty Bad request
    if let Err(errors) = req_sign_up.validate() {
        return Err(
            ErrorResponse((Status::BadRequest, format!("Validation errors: {:?}", errors)))
        );
    }

    let result = user_use_case.sign_up(req_sign_up.into_inner()).await;
    match result {
        UserOperation::Success => {
            Ok(SuccessResponse((Status::Created, "Account created!".to_string())))
        },
        UserOperation::Failed => {
            Err(ErrorResponse((Status::BadRequest, "Username or email already exists".to_string())))
        },
        UserOperation::DatabaseError => {
            Err(ErrorResponse((Status::InternalServerError, "Database error".to_string())))
        }
    }
    
    
}




// Route 3
#[get("/me")]
pub async fn me(user: AuthenticatedUser) -> Response<String> {
    Ok(SuccessResponse((
        Status::Ok,
        "My user ID is: ".to_string() + user.id.to_string().as_str(),
    )))
}

