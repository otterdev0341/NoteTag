use std::sync::Arc;

use rocket::{delete, get, http::Status, post, put, routes, serde::json::Json, Route, State};
use sea_orm::Update;
use utoipa::openapi::tag;
use validator::Validate;

use crate::{application::usecase::user_tag_usecase::UserTagUseCase, domain::{dto::user_tag_dto::{UpdateUserTagDto, UserTagDto, UserTagListDto}, entities::user_tag}, infrastructure::{faring::{authentication::AuthenticatedUser, cors::options}, http::response_type::response_type::{ErrorResponse, Response, SuccessResponse}, mysql::repositories::impl_user_x_tag_repository::ImplUserTagRepository}};


pub fn user_tag_routes() -> Vec<Route> {
    routes![
        add_user_tag,
        get_user_tag,
        update_user_tag,
        delete_tag_from_user,
        options
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

#[get("/")]
pub async fn get_user_tag(
    user: AuthenticatedUser,
    user_tag_usecase: &State<Arc<UserTagUseCase<ImplUserTagRepository>>>,
) 
-> Response<Json<UserTagListDto>> {
    let result = user_tag_usecase.get_user_tags(user.id).await;
    match result {
        Ok(tags) => Ok(SuccessResponse((Status::Ok, Json(UserTagListDto {
            totalTag: tags.len() as usize,
            tagList: tags.iter().map(|tag| tag.to_string()).collect()
        })))),
        Err(_) => Err(ErrorResponse((Status::BadRequest, "Error getting user tags".to_string())))
    }
    
}

#[put("/", data="<user_tag>")]
pub async fn update_user_tag(
    user: AuthenticatedUser,
    user_tag_usecase: &State<Arc<UserTagUseCase<ImplUserTagRepository>>>,
    user_tag: Json<UpdateUserTagDto>
) -> Response<String> {

    if user_tag.validate().is_err() {
        return Err(ErrorResponse((Status::BadRequest, "Invalid tag name".to_string())));
    }

    let result = user_tag_usecase.update_user_tag(user.id, &user_tag.oldTagName, &user_tag.newTagName).await;
    match result {
        Ok(_) => Ok(SuccessResponse((Status::Ok, "User tag updated successfully".to_string()))),
        Err(err) => Err(ErrorResponse((Status::BadRequest, format!("Error updating user tag: {}", err))))
    }
}

#[delete("/", data = "<tag_name>")]
pub async fn delete_tag_from_user(
    user: AuthenticatedUser,
    user_tag_usecase: &State<Arc<UserTagUseCase<ImplUserTagRepository>>>,
    tag_name: Json<UserTagDto>
) -> Response<String> {

    if tag_name.validate().is_err() {
        return Err(ErrorResponse((Status::BadRequest, "Invalid tag name".to_string())));
    }

    let result = user_tag_usecase.delete_user_tag(user.id, &tag_name.tagName).await;
    match result {
        Ok(_) => Ok(SuccessResponse((Status::Ok, "User tag deleted successfully".to_string()))),
        Err(err) => Err(ErrorResponse((Status::BadRequest, format!("Error deleting user tag: {}", err))))
    }
}