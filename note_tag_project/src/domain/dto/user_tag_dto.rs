use rocket::serde;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize,ToSchema, Validate)]
#[serde(crate = "rocket::serde")]
pub struct UserTagDto {
    #[validate(length(min = 1, message = "Tag must be at least 1 characters"))]
    pub tagName: String
}

#[derive(Deserialize, Serialize,ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct UserTagListDto {
    pub total_tag: i32,
    pub tag_list: Vec<UserTagDto>
}