use std::sync::Arc;

use rocket::{get, http::Status, post, routes, serde::json::Json, Route, State};
use validator::Validate;
use crate::{application::usecase::user_usecase::{self, UserOperation, UserUseCase}, domain::dto::auth_dto::{ReqSignInDto, ReqSignUpDto, ResSignInDto}, infrastructure::{faring::authentication::AuthenticatedUser, http::response_type::response_type::{ErrorResponse, Response, SuccessResponse}, mysql::repositories::impl_user_repository::ImplUserRepository}};
use crate::infrastructure::faring::cors::{CORS, options};

pub fn auth_routes() -> Vec<Route> {
    routes![
        sign_in,
        sign_up,
        me,
        options,
    ]
}


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
pub async fn me(
    user: AuthenticatedUser,
    user_use_case: &State<Arc<UserUseCase<ImplUserRepository>>>
) -> Response<String> {
    
    // get username
    let result = user_use_case.get_user_by_id(user.id).await;
    match result {
        Some(user) => {
            Ok(SuccessResponse((Status::Ok, user)))
        },
        None => {
            Err(ErrorResponse((Status::BadRequest, "Error getting user".to_string())))
        }
    }

    
    
}


