use std::sync::Arc;

use rocket::{http::Status, post, routes, serde::json::Json, Route, State};

use crate::{application::usecase::user_tag_usecase::UserTagUseCase, domain::dto::user_tag_dto::UserTagDto, infrastructure::{faring::authentication::AuthenticatedUser, http::response_type::response_type::{ErrorResponse, Response, SuccessResponse}, mysql::repositories::impl_user_x_tag_repository::ImplUserTagRepository}};


pub fn user_tag_routes() -> Vec<Route> {
    routes![
        add_user_tag
    ]
}



#[post("/", data = "<user_tag>")]
pub async fn add_user_tag(
    user: AuthenticatedUser,
    user_tag: Json<UserTagDto>,
    user_tag_usecase: &State<Arc<UserTagUseCase<ImplUserTagRepository>>>,
) 
    -> Response<String> 
{
    let result = user_tag_usecase.create_user_tag(user.id, &user_tag.tagName).await;
    match result {
        Ok(_) => Ok(SuccessResponse((Status::Ok, "User tag added successfully".to_string()))),
        Err(_) => Err(ErrorResponse((Status::BadRequest, "Error adding user tag".to_string())))
    }
    
}
